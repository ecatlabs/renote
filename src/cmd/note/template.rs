use async_trait::async_trait;
use clap::{App, ArgMatches, SubCommand};

use crate::cmd::{CmdResult, CommandSetting, CommandTrait};
use crate::config::{HighlightLabelConfig, IssueSearchConfig};

pub const CMD_GENERATE_TEMPLATE: &str = "template";

pub struct GenerateTemplateCommand;

impl GenerateTemplateCommand {
    pub fn new() -> Self {
        Self
    }
}

#[async_trait]
impl CommandTrait for GenerateTemplateCommand {
    fn setting(&self) -> &CommandSetting {
        unimplemented!()
    }

    fn app<'a, 'b>(&self) -> App<'a, 'b> {
        SubCommand::with_name(CMD_GENERATE_TEMPLATE)
            .about("Generate template")
            .visible_alias("t")
    }

    async fn process<'a>(&self, _matches: &ArgMatches<'a>) -> CmdResult {
        let mut issue = IssueSearchConfig::default();
        issue.highlight_labels = Some(hashmap! {
            "example-label".to_string() => HighlightLabelConfig::default()
        });

        println!("{}", serde_yaml::to_string(&issue)?);
        Ok(())
    }
}
