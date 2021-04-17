use async_trait::async_trait;
use clap::{App, ArgMatches, SubCommand};

use crate::cmd::{CmdResult, CommandSetting, CommandTrait};
use crate::config::{HighlightLabelConfig, NoteConfig};

pub const CMD_NODE_CONFIG: &str = "config";

pub struct NodeConfigCommand;

impl NodeConfigCommand {
    pub fn new() -> Self {
        Self
    }
}

#[async_trait]
impl CommandTrait for NodeConfigCommand {
    fn setting(&self) -> &CommandSetting {
        unimplemented!()
    }

    fn app<'a, 'b>(&self) -> App<'a, 'b> {
        SubCommand::with_name(CMD_NODE_CONFIG)
            .about("Generate template")
            .visible_alias("t")
    }

    async fn process<'a>(&self, _matches: &ArgMatches<'a>) -> CmdResult {
        let mut issue = NoteConfig::default();
        issue.highlight_labels = Some(vec![HighlightLabelConfig::default()]);

        println!("{}", serde_yaml::to_string(&issue)?);
        Ok(())
    }
}
