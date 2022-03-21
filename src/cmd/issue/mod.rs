use async_trait::async_trait;
use hubcaps_ex::issues::IssueOptions;
use libcli_rs::progress::{ProgressBar, ProgressTrait};

pub use add_label::{AddLabelCommand, CMD_ADD_LABEL};
pub use assign_milestone::{AssignMilestoneCommand, CMD_ASSIGN_MILESTONE};
pub use remove_label::{CMD_REMOVE_LABEL, RemoveLabelCommand};
pub use unassign_milestone::{CMD_UNASSIGN_MILESTONE, UnassignMilestoneCommand};

use crate::cmd::{CommandSetting, CommandTrait, create_cmd};
use crate::cmd::issue::search::{CMD_ISSUE_SEARCH, SearchIssueCommand};
use crate::component::repo::issue::IssueComponentTrait;
use crate::result::Result;

mod add_label;
mod assign_milestone;
mod remove_label;
mod search;
mod unassign_milestone;

pub const CMD_ISSUE: &str = "issue";

pub struct IssueCommand {
    setting: CommandSetting,
}

impl IssueCommand {
    pub fn new() -> Self {
        IssueCommand {
            setting: CommandSetting {
                name: CMD_ISSUE,
                about: "Issue commands",
                commands: hashmap! {
                    CMD_ADD_LABEL => create_cmd(Box::new(AddLabelCommand::new())),
                    CMD_REMOVE_LABEL => create_cmd(Box::new(RemoveLabelCommand::new())),
                    CMD_ASSIGN_MILESTONE => create_cmd(Box::new(AssignMilestoneCommand::new())),
                    CMD_UNASSIGN_MILESTONE => create_cmd(Box::new(UnassignMilestoneCommand::new())),
                    CMD_ISSUE_SEARCH => create_cmd(Box::new(SearchIssueCommand::new())),
                },
            },
        }
    }
}

#[async_trait]
impl CommandTrait for IssueCommand {
    fn setting(&self) -> &CommandSetting {
        &self.setting
    }
}

enum IssueLabelUpdateType {
    Add,
    Remove,
}

async fn create_issues_info_to_update(
    repo_component: &(dyn IssueComponentTrait + Send + Sync),
    query: &str,
    filtered_labels: &[&str],
    update_type: &IssueLabelUpdateType,
) -> Result<Vec<(u64, IssueOptions)>> {
    let issues = progress!(
        "Searching issues",
        repo_component.search_issues_by_query(query).await?;
    );

    let issues_info = issues
        .into_iter()
        .map(|it| {
            let iter = it.labels.iter().map(|it| it.name.clone());
            let labels = if matches!(update_type, IssueLabelUpdateType::Add) {
                iter.chain(filtered_labels.iter().map(|l| l.to_string()))
                    .collect()
            } else {
                iter.filter(|l| !filtered_labels.contains(&l.as_str()))
                    .collect()
            };

            (
                it.number,
                IssueOptions {
                    title: it.title,
                    body: it.body,
                    assignee: None,
                    assignees: Some(it.assignees.into_iter().map(|it| it.login).collect()),
                    milestone: it.milestone.map(|it| it.number),
                    labels,
                    state: it.state,
                },
            )
        })
        .collect();

    Ok(issues_info)
}
