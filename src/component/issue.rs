use anyhow::Result;
use async_trait::async_trait;
use hubcaps::issues::{Issue, IssueListOptions, Sort};

use hubcaps::search::{IssuesSort, SearchIssuesOptions};
use tokio_stream::StreamExt;

use crate::component::to_issue_state;
use crate::config::IssueSearchConfig;
use crate::util::create_github_client;

pub struct IssueComponent;

impl IssueComponent {
    pub fn new() -> impl IssueComponentTrait {
        Self
    }
}

#[async_trait]
pub trait IssueComponentTrait {
    async fn search_issues(&self, config: &IssueSearchConfig) -> Result<Vec<Issue>>;
    async fn search_issues_by_label(
        &self,
        config: &IssueSearchConfig,
        label: &String,
    ) -> Result<Vec<Issue>>;
    async fn search_issues_by_query(
        &self,
        config: &IssueSearchConfig,
        label: &String,
    ) -> Result<Vec<Issue>>;
}

#[async_trait]
impl IssueComponentTrait for IssueComponent {
    async fn search_issues(&self, config: &IssueSearchConfig) -> Result<Vec<Issue>> {
        let mut labels = config.labels.clone().unwrap_or(vec![]);
        labels.append(&mut config.highlight_labels.clone().unwrap_or(vec![]));

        let mut issues: Vec<Issue> = vec![];
        for label in labels {
            let mut found_issues = self.search_issues_by_label(config, &label).await?;
            issues.append(&mut found_issues);
        }

        Ok(issues)
    }

    async fn search_issues_by_label(
        &self,
        config: &IssueSearchConfig,
        label: &String,
    ) -> Result<Vec<Issue>> {
        let client = create_github_client(&config.token)?;
        let repo = client.repo(config.owner.clone(), config.repo.clone());

        let search_options = IssueListOptions::builder()
            .labels(vec![label])
            .state(to_issue_state(&config.state))
            .sort(Sort::Created)
            .build();

        let issues: Vec<_> = repo
            .issues()
            .iter(&search_options)
            .filter(|it| it.is_ok())
            .filter_map(|it| {
                let issue = it.unwrap();

                // filter milestone
                if let Some(milestone) = &config.milestone {
                    if let Some(issue_milestone) = &issue.milestone {
                        if &issue_milestone.title != milestone {
                            return None;
                        }
                    } else {
                        return None;
                    }
                }

                // filter exclude_labels
                if let Some(labels) = &config.exclude_labels {
                    let issue_labels: Vec<_> = issue.labels.iter().map(|it| &it.name).collect();
                    for label in labels {
                        if issue_labels.contains(&label) {
                            return None;
                        }
                    }
                }

                Some(issue)
            })
            .collect()
            .await;

        Ok(issues)
    }

    async fn search_issues_by_query(
        &self,
        config: &IssueSearchConfig,
        _label: &String,
    ) -> Result<Vec<Issue>> {
        let search_options = SearchIssuesOptions::builder()
            .sort(IssuesSort::Created)
            .build();

        let mut search_query: Vec<String> = vec![];

        if let Some(milestone) = &config.milestone {
            search_query.push(format!("milestone:{}", milestone));
        }
        search_query.push(format!("is:{}", &config.state));

        let mut labels = config.labels.clone().unwrap_or(vec![]);
        labels.append(&mut config.highlight_labels.clone().unwrap_or(vec![]));

        for label in &labels {
            search_query.push(format!("label:{}", label))
        }

        for label in config.exclude_labels.as_ref().unwrap() {
            search_query.push(format!("label:{}-", label));
        }

        let client = create_github_client(&config.token)?;
        let search_query = search_query.join(" ");

        let issues: Vec<_> = client
            .search()
            .issues()
            .iter(search_query, &search_options)
            .map(|it| {
                let issue: Issue =
                    serde_yaml::from_str(&serde_yaml::to_string(&it.unwrap()).unwrap()).unwrap();
                issue
            })
            .collect()
            .await;
        Ok(issues)
    }
}
