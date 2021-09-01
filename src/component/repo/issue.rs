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
    async fn search_issues_by_labels(&self, labels: &[String]) -> Result<Vec<Issue>>;
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

        if let Some(labels) = &self.config.labels {
            let mut found_issues = self.search_issues_by_labels(labels).await?;
            issues.append(&mut found_issues);
        }

        if let Some(labels) = &self.config.any_labels {
            for label in labels {
                let mut found_issues = self.search_issues_by_labels(&[label.clone()]).await?;
                issues.append(&mut found_issues);
            }
        }

        Ok(issues)
    }

    async fn search_issues_by_labels(&self, labels: &[String]) -> Result<Vec<Issue>> {
        debug!("searching issues by labels: {:?}", labels);

        let repo = self
            .github
            .repo(self.config.owner.clone(), self.config.repo.clone());

        let since_time = match self.config.since {
            Some(ref x) => {
                x.clone()
                // let a = NaiveDate::parse_from_str(x, "%Y-%m-%d")?;
                // a.format("YYYY-MM-DDThh:mmTZD").to_string()
            }
            None => self.get_latest_release().await?.created_at,
        };

        let mut search_options_builder = IssueListOptions::builder();
        search_options_builder
            .labels(labels.to_vec())
            .state(to_issue_state(&self.config.state))
            .sort(Sort::Created)
            .since(since_time);
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
        use std::fmt::{Write, Display};
        debug!("Searching issues by query: {}", query);

        let search_options = SearchIssuesOptions::builder()
            .sort(IssuesSort::Created)
            .build();

        fn add_filter(query: &mut String, filter: &str, args: impl Display) {
            write!(query, "{}:{} ", filter, args).unwrap();
        }

        let mut query = query.to_owned();
        if !query.is_empty() {
            query.push(' ');
        }

        add_filter(&mut query, "repo", format_args!("{}/{}", self.config.owner, self.config.repo));

        if let Some(milestone) = &self.config.milestone {
            add_filter(&mut query, "milestone", milestone);
        }

        if !self.config.state.is_empty() {
            add_filter(&mut query, "is", &self.config.state);
        }

        if let Some(labels) = &self.config.labels {
            for label in labels {
                add_filter(&mut query, "label", label);
            }
        }

        if let Some(labels) = &self.config.exclude_labels {
            for label in labels {
                add_filter(&mut query, "-label", label);
            }
        }

        if query.ends_with(' ') {
            query.pop();
        }

        let issues: Vec<_> = self
            .github
            .search()
            .issues()
            .iter(query, &search_options)
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

        // filter excluded issues
        if let Some(exclude_issue_numbers) = &self.config.exclude_issues {
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
            if issue.labels.iter().any(|l| labels.contains(&l.name)) {
                return false;
            }
        }

        true
    }
}
