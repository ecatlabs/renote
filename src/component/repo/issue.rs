use async_trait::async_trait;
use hubcaps::issues::{Issue, IssueListOptions, Sort, State};
use hubcaps::search::{IssuesItem, IssuesSort, SearchIssuesOptions};
use log::error;
use tokio_stream::StreamExt;

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
    async fn search_issues_by_query(&self, label: &String) -> Result<Vec<Issue>>;
    fn filter_issue(&self, issue: &Issue) -> bool;
}

#[async_trait]
impl IssueComponentTrait for RepoComponent {
    async fn list_issues(&self) -> Result<Vec<Issue>> {
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
                    error!("Failed to parse the issue: {:?}", err);
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

    async fn search_issues_by_query(&self, _label: &String) -> Result<Vec<Issue>> {
        let search_options = SearchIssuesOptions::builder()
            .sort(IssuesSort::Created)
            .build();

        let mut search_query: Vec<String> = vec![];

        if let Some(milestone) = &self.config.milestone {
            search_query.push(format!("milestone:{}", milestone));
        }
        search_query.push(format!("is:{}", &self.config.state));

        let labels = self.config.labels.clone().unwrap_or(vec![]);
        for label in &labels {
            search_query.push(format!("label:{}", label))
        }

        for label in self.config.exclude_labels.as_ref().unwrap() {
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
                    error!("Failed to parse the issue: {}", err);
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
                        error!("Failed to convert the issue item to an issue: {}", err);
                        None
                    }
                }
            })
            .collect()
            .await;
        Ok(issues)
    }

    fn filter_issue(&self, issue: &Issue) -> bool {
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
