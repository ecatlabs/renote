use async_trait::async_trait;
use libcli_rs::progress::{ProgressBar, ProgressTrait};

pub(crate) use add_label::{AddLabelCommand, CMD_ADD_LABEL};
pub(crate) use assign_milestone::{AssignMilestoneCommand, CMD_ASSIGN_MILESTONE};
use hubcaps::issues::IssueOptions;
pub(crate) use remove_label::{RemoveLabelCommand, CMD_REMOVE_LABEL};
pub(crate) use unassign_milestone::{UnassignMilestoneCommand, CMD_UNASSIGN_MILESTONE};

use crate::cmd::issue::search::{SearchIssueCommand, CMD_ISSUE_SEARCH};
use crate::cmd::{create_cmd, CommandSetting, CommandTrait};
use crate::component::repo::issue::IssueComponentTrait;
use crate::result::Result;

mod add_label;
mod assign_milestone;
mod remove_label;
mod search;
mod unassign_milestone;

pub(crate) const CMD_ISSUE: &str = "issue";

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
    filtered_labels: &Vec<String>,
    update_type: &IssueLabelUpdateType,
) -> Result<Vec<(u64, IssueOptions)>> {
    let issues = progress!(
        "Searching issues",
        repo_component.search_issues_by_query(query).await?;
    );

    let issues_info: Vec<_> = issues
        .into_iter()
        .map(|it| {
            let mut labels: Vec<_> = it.labels.iter().map(|it| it.name.clone()).collect();

            if let IssueLabelUpdateType::Add = update_type {
                labels.append(&mut filtered_labels.clone());
            } else {
                labels = labels
                    .iter()
                    .filter(|it| !filtered_labels.contains(it))
                    .map(|it| it.clone())
                    .collect();
            }

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
