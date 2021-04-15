use anyhow::Result;
use async_trait::async_trait;
use hubcaps::issues::Issue;
use serde::{Deserialize, Serialize};
use tera::{Context, Tera};
use tokio_stream::StreamExt;

use crate::component::issue::{IssueComponent, IssueComponentTrait};
use crate::config::IssueSearchConfig;

const ISSUE_SECTION_TEMPLATE: &'static str = r#"
{% for section in sections %}
## {{ section.title }}
  {% for issue in section.issues -%}
  - {{ issue.title }} ([{{ issue.id }}]({{ issue.url }})) - {{ issue.assignees }}
  {% endfor %}
{% endfor %}
"#;

#[async_trait]
pub trait NoteComponentTrait {
    async fn create_note(&self, config: &IssueSearchConfig) -> Result<String>;
    fn render_note(&self, config: &IssueSearchConfig, issues: &Vec<Issue>) -> Result<String>;
}

pub struct NoteComponent;

impl NoteComponent {
    pub fn new() -> impl NoteComponentTrait {
        Self
    }
}

#[derive(Serialize, Deserialize)]
struct IssueSection {
    title: String,
    issues: Vec<IssueSummary>,
}

#[derive(Serialize, Deserialize)]
struct IssueSummary {
    id: u64,
    title: String,
    url: String,
    assignees: Vec<String>,
}

#[async_trait]
impl NoteComponentTrait for NoteComponent {
    async fn create_note(&self, config: &IssueSearchConfig) -> Result<String> {
        let issue_component = IssueComponent::new();
        let issues = issue_component.search_issues(config).await?;

        self.render_note(config, &issues)
    }

    fn render_note(&self, config: &IssueSearchConfig, issues: &Vec<Issue>) -> Result<String> {
        let mut tera = Tera::default();
        tera.add_raw_template("issue-sections", ISSUE_SECTION_TEMPLATE);

        let mut context = Context::new();
        let mut issue_sections: Vec<IssueSection> = vec![];

        if let Some(highlight_labels) = &config.highlight_labels {
            for label in highlight_labels {
                let issues: Vec<_> = issues
                    .iter()
                    .filter(|issue| {
                        let issue_labels: Vec<_> = issue.labels.iter().map(|it| &it.name).collect();
                        issue_labels.contains(&label)
                    })
                    .map(|it| IssueSummary {
                        id: it.id,
                        title: it.title.clone(),
                        url: it.url.clone(),
                        assignees: it.assignees.iter().map(|it| it.login.clone()).collect(),
                    })
                    .collect();

                issue_sections.push(IssueSection {
                    title: label.clone(),
                    issues,
                });
            }
        }

        context.insert("sections", &issue_sections);

        let mut output = tera.render("issue-sections", &context)?;
        output = config.note.as_ref().unwrap().replace("{content}", &output);

        Ok(output)
    }
}
