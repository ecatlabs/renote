use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Serialize, Deserialize, Default, Clone)]
pub(crate) struct NoteConfig {
    pub owner: String,
    pub repo: String,
    pub token: String,
    pub state: String,
    pub note: Option<String>,
    pub milestone: Option<String>,
    pub show_contributor: bool,
    pub extra_contributors: Option<Vec<String>>,
    pub exclude_issues: Option<Vec<u64>>,
    pub sort: Option<IssueSort>,
    pub labels: Option<Vec<String>>,
    pub any_labels: Option<Vec<String>>,
    pub exclude_labels: Option<Vec<String>>,
    pub highlight_labels: Option<Vec<HighlightLabelConfig>>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub(crate) enum IssueSort {
    #[serde(rename = "asc")]
    Asc,
    #[serde(rename = "desc")]
    Desc,
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Default, Clone)]
pub(crate) struct HighlightLabelConfig {
    pub label: String,
    pub title: Option<String>,
    pub description: Option<String>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Default, Clone)]
pub(crate) struct ReleaseConfig {
    pub name: String,
    pub draft: bool,
    pub pre_release: bool,
    pub artifacts: Option<Vec<String>>,
    pub note_config: NoteConfig,
}
