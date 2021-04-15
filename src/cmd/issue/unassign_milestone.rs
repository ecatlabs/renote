use async_trait::async_trait;
use clap::{App, Arg, ArgMatches, SubCommand};

use crate::cmd::{CmdResult, CommandSetting, CommandTrait};

pub const CMD_UNASSIGN_MILESTONE: &str = "unassign-milestone";

pub struct UnassignMilestoneCommand;

impl UnassignMilestoneCommand {
    pub fn new() -> Self {
        Self {}
    }
}

#[async_trait]
impl CommandTrait for UnassignMilestoneCommand {
    fn setting(&self) -> &CommandSetting {
        unimplemented!()
    }

    fn app<'a, 'b>(&self) -> App<'a, 'b> {
        SubCommand::with_name(CMD_UNASSIGN_MILESTONE)
            .about("Unassign issues from a milestone")
            .visible_alias("um")
            .args(&[
                Arg::with_name("query")
                    .help("Issue filter query")
                    .long_help("Issue query by https://docs.github.com/en/github/searching-for-information-on-github/searching-issues-and-pull-requests")
                    .short("q")
                    .long("query")
                    .value_delimiter(" ")
                    .takes_value(true),
                Arg::with_name("milestone")
                    .help("Milestone")
                    .value_name("milestone")
                    .required(true)
                    .takes_value(true),
            ])
    }

    async fn process<'a>(&self, _matches: &ArgMatches<'a>) -> CmdResult {
        todo!()
    }
}
