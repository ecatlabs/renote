use std::collections::{HashMap, HashSet};
use std::fs;
use std::sync::Arc;

use async_trait::async_trait;
use hubcaps_ex::issues::Issue;
use log::{debug, info, trace};
use serde::{Deserialize, Serialize};
use tera::{from_value, to_value, Context, Tera, Value};

use crate::component::repo::issue::IssueComponentTrait;
use crate::component::repo::RepoComponent;
use crate::config::NoteConfig;
use crate::result::Result;
use crate::util::create_github_client;

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
    async fn create_note(&self) -> Result<String>;
    fn render_note(&self, issues: &Vec<Issue>) -> Result<String>;
}

pub(crate) struct NoteComponent {
    config: Arc<NoteConfig>,
}

impl NoteComponent {
    pub fn new(config: Arc<NoteConfig>) -> impl NoteComponentTrait {
        NoteComponent { config }
    }
}

#[async_trait]
impl NoteComponentTrait for NoteComponent {
    async fn create_note(&self) -> Result<String> {
        info!("creating note");

        let github = Arc::new(create_github_client(&self.config.token)?);
        let repo_component = RepoComponent::new(Some(github), self.config.clone());

        let issues = repo_component.list_issues().await?;
        self.render_note(&issues)
    }

    fn render_note(&self, issues: &Vec<Issue>) -> Result<String> {
        info!("rendering note: issue count: {}", issues.len());

        // let mut issue_sections: Vec<IssueSection> = vec![];
        let mut issue_sections: HashMap<String, IssueSection> = hashmap! {};
        let highlight_labels = self.config.highlight_labels.clone().unwrap_or_default();

        'outer: for issue in issues.iter() {
            debug!("processing issue: {:?}", issue);

            let issue_labels: Vec<_> = issue.labels.iter().map(|it| &it.name).collect();

            let issue_summary = IssueSummary {
                id: issue.number,
                title: issue.title.clone(),
                url: issue.html_url.clone(),
                assignees: issue.assignees.iter().map(|it| it.login.clone()).collect(),
            };

            for (index, label_config) in highlight_labels.iter().enumerate() {
                if issue_labels.contains(&&label_config.label) {
                    if let Some(s) = issue_sections.get_mut(&label_config.label) {
                        s.issues.push(issue_summary);
                    } else {
                        issue_sections.insert(
                            label_config.label.clone(),
                            IssueSection {
                                index: index as i8,
                                title: label_config.title.clone().unwrap_or("".to_string()),
                                description: label_config
                                    .description
                                    .clone()
                                    .unwrap_or("".to_string()),
                                issues: vec![issue_summary],
                            },
                        );
                    }

                    continue 'outer;
                }
            }

            if let Some(s) = issue_sections.get_mut("misc") {
                s.issues.push(issue_summary);
            } else {
                issue_sections.insert(
                    "misc".to_string(),
                    IssueSection {
                        index: 100,
                        title: "Misc".to_string(),
                        description: "".to_string(),
                        issues: vec![issue_summary],
                    },
                );
            }
        }

        let mut issue_sections: Vec<_> = issue_sections.values().collect();
        issue_sections.sort_by(|a, b| a.index.partial_cmp(&b.index).unwrap());

        let assignees: Vec<_> = issues.iter().flat_map(|it| &it.assignees).collect();
        let mut assignees: Vec<_> = assignees
            .into_iter()
            .map(|it| it.login.clone())
            .collect::<HashSet<_>>()
            .into_iter()
            .collect();

        if let Some(contributors) = self.config.extra_contributors.clone() {
            for contributor in contributors {
                if !assignees.contains(&contributor) {
                    assignees.push(contributor.clone());
                }
            }
        }

        assignees.sort();

        let mut tera = Tera::default();
        tera.add_raw_template("issue-sections", ISSUE_SECTION_TEMPLATE)?;
        tera.register_function("assignees_str", assignees_str);

        let mut context = Context::new();
        context.insert("sections", &issue_sections);
        context.insert("assignees", &assignees);

        trace!("prepared render context: {:?}", context);

        let mut output = tera.render("issue-sections", &context)?;
        if let Some(mut note_template) = self.config.note.clone() {
            if let Ok(f) = fs::metadata(&note_template) {
                if f.is_file() {
                    note_template = fs::read_to_string(&note_template)?;
                }
            }

            if !note_template.is_empty() {
                output = note_template.replace("{content}", &output);
            }
        }

        Ok(output)
    }
}
