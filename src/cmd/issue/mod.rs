use async_trait::async_trait;

pub(crate) use add_label::{AddLabelCommand, CMD_ADD_LABEL};
pub(crate) use assign_milestone::{AssignMilestoneCommand, CMD_ASSIGN_MILESTONE};
pub(crate) use remove_label::{RemoveLabelCommand, CMD_REMOVE_LABEL};
pub(crate) use unassign_milestone::{UnassignMilestoneCommand, CMD_UNASSIGN_MILESTONE};

use crate::cmd::{create_cmd, CommandSetting, CommandTrait};

mod add_label;
mod assign_milestone;
mod remove_label;
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
