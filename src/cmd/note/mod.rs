use async_trait::async_trait;
use clap::ArgMatches;

pub use create::*;

use crate::cmd::note::config::{NodeConfigCommand, CMD_NODE_CONFIG};
use crate::cmd::{create_cmd, CommandSetting, CommandTrait};
use crate::result::CmdResult;

mod config;
mod create;

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
                    CMD_NODE_CONFIG => create_cmd(Box::new(NodeConfigCommand::new())),
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

    fn validate(&self, _matches: &ArgMatches) -> CmdResult {
        Ok(())
    }
}
