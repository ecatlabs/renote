use std::collections::{HashMap, HashSet};

use async_trait::async_trait;
use hubcaps::issues::Issue;
use serde::{Deserialize, Serialize};
use tera::{from_value, to_value, Context, Tera, Value};

use crate::component::issue::{IssueComponent, IssueComponentTrait};
use crate::config::IssueSearchConfig;
use crate::result::Result;

const ISSUE_SECTION_TEMPLATE: &'static str = r#"
{% for section in sections %}
## {{ section.title }}
  {{ section.description }}
  {% for issue in section.issues -%}
  - {{ issue.title }} ([{{ issue.id }}]({{ issue.url }})) - {{ assignees_str(value=issue.assignees) }}
  {% endfor -%}
{% if not section.issues -%}
N/A
{% endif -%}
{% endfor %}
## Contributors

{% for assignee in assignees -%}
  - @{{ assignee }}
{% endfor -%}
{% if not assignees -%}
N/A
{% endif -%}
"#;

#[derive(Serialize, Deserialize)]
struct IssueSection {
    index: i8,
    title: String,
    description: String,
    issues: Vec<IssueSummary>,
}

#[derive(Serialize, Deserialize)]
struct IssueSummary {
    id: u64,
    title: String,
    url: String,
    assignees: Vec<String>,
}

#[derive(Serialize, Deserialize)]
struct Assignee {
    name: String,
}

fn assignees_str(args: &HashMap<String, Value>) -> tera::Result<Value> {
    match args.get("value") {
        Some(val) => match from_value::<Vec<String>>(val.clone()) {
            Ok(assignees) => {
                let assignees: Vec<_> = assignees.iter().map(|it| format!("@{}", it)).collect();
                Ok(to_value(assignees.join(" ")).unwrap())
            }
            Err(_) => Err("".into()),
        },
        None => Err("".into()),
    }
}

#[async_trait]
pub(crate) trait NoteComponentTrait {
    async fn create_note(&self, config: &IssueSearchConfig) -> Result<String>;
    fn render_note(&self, config: &IssueSearchConfig, issues: &Vec<Issue>) -> Result<String>;
}

pub(crate) struct NoteComponent;

impl NoteComponent {
    pub fn new() -> impl NoteComponentTrait {
        Self
    }
}

#[async_trait]
impl NoteComponentTrait for NoteComponent {
    async fn create_note(&self, config: &IssueSearchConfig) -> Result<String> {
        let issue_component = IssueComponent::new();
        let issues = issue_component.search_issues(config).await?;

        self.render_note(config, &issues)
    }

    fn render_note(&self, config: &IssueSearchConfig, issues: &Vec<Issue>) -> Result<String> {
        let mut issue_sections: Vec<IssueSection> = vec![];

        if let Some(highlight_labels) = &config.highlight_labels {
            for (label, highlight_label_config) in highlight_labels {
                let issues: Vec<_> = issues
                    .iter()
                    .filter(|issue| {
                        let issue_labels: Vec<_> = issue.labels.iter().map(|it| &it.name).collect();
                        issue_labels.contains(&label)
                    })
                    .map(|it| IssueSummary {
                        id: it.number,
                        title: it.title.clone(),
                        url: it.html_url.clone(),
                        assignees: it.assignees.iter().map(|it| it.login.clone()).collect(),
                    })
                    .collect();

                issue_sections.push(IssueSection {
                    index: highlight_label_config.index.unwrap_or(0),
                    title: highlight_label_config
                        .title
                        .clone()
                        .unwrap_or("".to_string()),
                    description: highlight_label_config
                        .description
                        .clone()
                        .unwrap_or("".to_string()),
                    issues,
                });
            }
        }

        issue_sections.sort_by(|a, b| a.index.partial_cmp(&b.index).unwrap());

        let assignees: Vec<_> = issues.iter().flat_map(|it| &it.assignees).collect();
        let mut assignees: Vec<_> = assignees
            .into_iter()
            .map(|it| it.login.clone())
            .collect::<HashSet<_>>()
            .into_iter()
            .collect();

        if let Some(mut contributors) = config.extra_contributors.clone() {
            assignees.append(&mut contributors);
        }

        assignees.sort();

        let mut tera = Tera::default();
        tera.add_raw_template("issue-sections", ISSUE_SECTION_TEMPLATE)?;
        tera.register_function("assignees_str", assignees_str);

        let mut context = Context::new();
        context.insert("sections", &issue_sections);
        context.insert("assignees", &assignees);

        let mut output = tera.render("issue-sections", &context)?;
        output = config.note.as_ref().unwrap().replace("{content}", &output);

        Ok(output)
    }
}
