use async_trait::async_trait;
use clap::{App, Arg, ArgMatches, SubCommand};

use crate::cmd::{CommandSetting, CommandTrait};
use crate::result::CmdResult;

pub(crate) const CMD_ADD_LABEL: &str = "add-label";

pub(crate) struct AddLabelCommand;

impl AddLabelCommand {
    pub fn new() -> Self {
        Self
    }
}

#[async_trait]
impl CommandTrait for AddLabelCommand {
    fn setting(&self) -> &CommandSetting {
        unimplemented!()
    }

    fn app<'a, 'b>(&self) -> App<'a, 'b> {
        SubCommand::with_name(CMD_ADD_LABEL)
            .about("Add label to issues")
            .visible_alias("al")
            .args(&[
                Arg::with_name("query")
                    .help("Issue filter query")
                    .long_help("Issue query by https://docs.github.com/en/github/searching-for-information-on-github/searching-issues-and-pull-requests")
                    .short("q")
                    .long("query")
                    .value_delimiter(" ")
                    .takes_value(true),
                Arg::with_name("labels")
                    .help("Labels to add")
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
