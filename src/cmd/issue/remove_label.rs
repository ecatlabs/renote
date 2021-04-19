use async_trait::async_trait;
use clap::{App, Arg, ArgMatches, SubCommand};

use crate::cmd::{CommandSetting, CommandTrait};
use crate::result::CmdResult;

pub(crate) const CMD_REMOVE_LABEL: &str = "remove-label";

pub(crate) struct RemoveLabelCommand;

impl RemoveLabelCommand {
    pub fn new() -> Self {
        Self {}
    }
}

#[async_trait]
impl CommandTrait for RemoveLabelCommand {
    fn setting(&self) -> &CommandSetting {
        unimplemented!()
    }

    fn app<'a, 'b>(&self) -> App<'a, 'b> {
        SubCommand::with_name(CMD_REMOVE_LABEL)
            .about("Remove label to issues")
            .visible_alias("rl")
            .args(&[
                Arg::with_name("query")
                    .help("Issue filter query")
                    .long_help("Issue query by https://docs.github.com/en/github/searching-for-information-on-github/searching-issues-and-pull-requests")
                    .short("q")
                    .long("query")
                    .value_delimiter(" ")
                    .takes_value(true),
                Arg::with_name("labels")
                    .help("Labels to remove")
                    .long("labels")
                    .value_delimiter(" ")
                    .required(true)
                    .takes_value(true),
            ])
    }

    async fn process<'a>(&self, _matches: &ArgMatches<'a>) -> CmdResult {
        todo!()
    }
}
