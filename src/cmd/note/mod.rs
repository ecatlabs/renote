use async_trait::async_trait;

pub use create::*;

use crate::cmd::note::template::{GenerateTemplateCommand, CMD_GENERATE_TEMPLATE};
use crate::cmd::{create_cmd, CommandSetting, CommandTrait};

mod create;
mod template;

pub const CMD_NOTE: &str = "note";

pub struct NoteCommand {
    setting: CommandSetting,
}

impl NoteCommand {
    pub fn new() -> Self {
        NoteCommand {
            setting: CommandSetting {
                name: CMD_NOTE,
                about: "Note commands",
                commands: hashmap! {
                    CMD_CREATE_NOTE => create_cmd(Box::new(CreateNoteCommand::new())),
                    CMD_GENERATE_TEMPLATE => create_cmd(Box::new(GenerateTemplateCommand::new())),
                },
            },
        }
    }
}

#[async_trait]
impl CommandTrait for NoteCommand {
    fn setting(&self) -> &CommandSetting {
        &self.setting
    }
}
