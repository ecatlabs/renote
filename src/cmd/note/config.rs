use async_trait::async_trait;
use clap::{ArgMatches, Command};

use crate::cmd::CommandTrait;
use crate::config::{HighlightLabelConfig, NoteConfig};
use crate::result::CmdResult;

pub const CMD_NODE_CONFIG: &str = "config";

pub struct NodeConfigCommand;

impl NodeConfigCommand {
    pub fn new() -> Self {
        Self
    }
}

#[async_trait]
impl CommandTrait for NodeConfigCommand {
    fn app<'help>(&self) -> Command<'help> {
        Command::new(CMD_NODE_CONFIG)
            .about("Generate template")
            .visible_alias("t")
    }

    fn validate(&self, _matches: &ArgMatches) -> CmdResult {
        Ok(())
    }

    async fn process(&self, _matches: &ArgMatches) -> CmdResult {
        let mut issue = NoteConfig::default();
        issue.highlight_labels = Some(vec![HighlightLabelConfig::default()]);

        println!("{}", serde_yaml::to_string(&issue)?);
        Ok(())
    }
}
