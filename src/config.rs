use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Serialize, Deserialize, Default)]
pub(crate) struct IssueSearchConfig {
    pub owner: String,
    pub repo: String,
    pub token: String,
    pub state: String,
    pub note: Option<String>,
    pub milestone: Option<String>,
    pub labels: Option<Vec<String>>,
    pub any_labels: Option<Vec<String>>,
    pub exclude_labels: Option<Vec<String>>,
    pub highlight_labels: Option<HashMap<String, HighlightLabelConfig>>,
    pub show_contributor: bool,
    pub extra_contributors: Option<Vec<String>>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Default, Clone)]
pub(crate) struct HighlightLabelConfig {
    pub index: Option<i8>,
    pub title: Option<String>,
    pub description: Option<String>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Default)]
pub(crate) struct ReleaseConfig {
    pub name: String,
    pub draft: bool,
    pub pre_release: bool,
    pub artifacts: Option<Vec<String>>,
    pub issue_search_config: IssueSearchConfig,
}
