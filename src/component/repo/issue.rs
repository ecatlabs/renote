use anyhow::anyhow;
use async_trait::async_trait;
use log::{debug, error, info, trace};
use tokio_stream::StreamExt;

use hubcaps_ex::issues::{Issue, IssueListOptions, IssueOptions, Sort, State};
use hubcaps_ex::milestone::{Milestone, MilestoneListOptions};
use hubcaps_ex::search::{IssuesItem, IssuesSort, SearchIssuesOptions};

use crate::component::repo::release::ReleaseComponentTrait;
use crate::component::repo::RepoComponent;
use crate::config::IssueSort;
use crate::result::Result;

fn to_issue_state(state: &str) -> State {
    match state {
        "open" => State::Open,
        "closed" => State::Closed,
        _ => State::All,
    }
}

fn to_issue(issue_item: &IssuesItem) -> Result<Issue> {
    let output = serde_yaml::to_string(issue_item)?;
    Ok(serde_yaml::from_str::<Issue>(&output)?)
}

#[async_trait]
pub(crate) trait IssueComponentTrait {
    async fn list_issues(&self) -> Result<Vec<Issue>>;
    async fn search_issues_by_labels(&self, labels: &Vec<String>) -> Result<Vec<Issue>>;
    async fn search_issues_by_query(&self, query: &str) -> Result<Vec<Issue>>;
    async fn update_issues(&self, issues: &Vec<(u64, IssueOptions)>) -> Result<()>;
    async fn get_milestone(&self, milestone: &str) -> Result<Milestone>;
    fn filter_issue(&self, issue: &Issue) -> bool;
}

#[async_trait]
impl IssueComponentTrait for RepoComponent {
    async fn list_issues(&self) -> Result<Vec<Issue>> {
        debug!("listing issues");

        let mut issues: Vec<Issue> = vec![];

        let mut found_issues = self
            .search_issues_by_labels(&self.config.labels.clone().unwrap_or(vec![]))
            .await?;
        issues.append(&mut found_issues);

        for label in self.config.any_labels.clone().unwrap_or(vec![]) {
            let mut found_issues = self.search_issues_by_labels(&vec![label]).await?;
            issues.append(&mut found_issues);
        }

        Ok(issues)
    }

    async fn search_issues_by_labels(&self, labels: &Vec<String>) -> Result<Vec<Issue>> {
        debug!("searching issues by labels: {:?}", labels);

        let repo = self
            .github
            .repo(self.config.owner.clone(), self.config.repo.clone());

        let latest_release = self.get_latest_release().await?;
        let mut search_options_builder = IssueListOptions::builder();
        search_options_builder
            .labels(labels.clone())
            .state(to_issue_state(&self.config.state))
            .sort(Sort::Created)
            .since(&latest_release.created_at);
        if let Some(IssueSort::Asc) = &self.config.sort {
            search_options_builder.asc();
        }
        let search_options = search_options_builder.build();

        let issues: Vec<_> = repo
            .issues()
            .iter(&search_options)
            .filter_map(|it| {
                if let Err(err) = it {
                    error!("failed to parse the issue: {:?}", err);
                    return None;
                }
                let issue = it.unwrap();

                if self.filter_issue(&issue) {
                    Some(issue)
                } else {
                    None
                }
            })
            .collect()
            .await;

        Ok(issues)
    }

    async fn search_issues_by_query(&self, query: &str) -> Result<Vec<Issue>> {
        debug!("Searching issues by query: {}", query);

        let search_options = SearchIssuesOptions::builder()
            .sort(IssuesSort::Created)
            .build();

        let mut search_query: Vec<String> = if query.is_empty() {
            vec![]
        } else {
            query.split(" ").map(|it| it.to_string()).collect()
        };
        search_query.push(format!("repo:{}/{}", self.config.owner, self.config.repo));

        if let Some(milestone) = &self.config.milestone {
            search_query.push(format!("milestone:{}", milestone));
        }

        if !self.config.state.is_empty() {
            search_query.push(format!("is:{}", &self.config.state));
        }

        let labels = self.config.labels.clone().unwrap_or(vec![]);
        for label in &labels {
            search_query.push(format!("label:{}", label))
        }

        for label in self.config.exclude_labels.clone().unwrap_or(vec![]) {
            search_query.push(format!("-label:{}", label));
        }

        let search_query = search_query.join(" ");

        let issues: Vec<_> = self
            .github
            .search()
            .issues()
            .iter(search_query, &search_options)
            .filter_map(|it| {
                if let Err(err) = it {
                    error!("failed to parse the issue: {}", err);
                    return None;
                }

                match to_issue(&it.unwrap()) {
                    Ok(issue) => {
                        if self.filter_issue(&issue) {
                            Some(issue)
                        } else {
                            None
                        }
                    }
                    Err(err) => {
                        error!("failed to convert the issue item to an issue: {}", err);
                        None
                    }
                }
            })
            .collect()
            .await;
        Ok(issues)
    }

    async fn update_issues(&self, issues: &Vec<(u64, IssueOptions)>) -> Result<()> {
        info!("updating issues: {:?}", issues);

        let repo = self
            .github
            .repo(self.config.owner.clone(), self.config.repo.clone());

        for (issue_number, issue) in issues {
            repo.issues().update(issue_number, issue).await?;
        }

        Ok(())
    }

    async fn get_milestone(&self, milestone: &str) -> Result<Milestone> {
        debug!("getting milestone: {}", milestone);

        let repo = self
            .github
            .repo(self.config.owner.clone(), self.config.repo.clone());

        let list_options = MilestoneListOptions::builder().build();
        match repo
            .milestones()
            .list(&list_options)
            .await?
            .into_iter()
            .find(|it| it.title == milestone)
        {
            Some(m) => Ok(m),
            None => Err(anyhow!("milestone {} not found", milestone)),
        }
    }

    fn filter_issue(&self, issue: &Issue) -> bool {
        trace!("filtering issue: {:?}", issue);

        let exclude_issue_numbers = self.config.exclude_issues.clone().unwrap_or(vec![]);

        // filter excluded issues
        if !exclude_issue_numbers.is_empty() {
            if exclude_issue_numbers.contains(&issue.number) {
                return false;
            }
        }

        // filter milestone
        if let Some(milestone) = &issue.milestone {
            if let Some(expected_milestone) = &self.config.milestone {
                if milestone.title != *expected_milestone {
                    return false;
                }
            }
        } else {
            return false;
        }

        // filter exclude_labels
        if let Some(labels) = &self.config.exclude_labels {
            let issue_labels: Vec<_> = issue.labels.iter().map(|it| &it.name).collect();
            for label in labels {
                if issue_labels.contains(&label) {
                    return false;
                }
            }
        }

        true
    }
}
