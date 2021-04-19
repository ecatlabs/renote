use std::io::stdout;
use std::sync::Arc;

use async_trait::async_trait;
use clap::{App, Arg, ArgMatches, SubCommand};
use libcli_rs::output::{OutputFactory, OutputTrait};
use libcli_rs::progress::{ProgressBar, ProgressTrait};

use crate::cmd::{check_github_args, CommandSetting, CommandTrait};
use crate::component::repo::issue::IssueComponentTrait;
use crate::component::repo::RepoComponent;
use crate::config::NoteConfig;
use crate::result::CmdResult;
use crate::util::get_output_format_from_args;

pub(crate) const CMD_ISSUE_SEARCH: &str = "search";

pub(crate) struct SearchIssueCommand;

impl SearchIssueCommand {
    pub fn new() -> Self {
        Self
    }
}

#[async_trait]
impl CommandTrait for SearchIssueCommand {
    fn setting(&self) -> &CommandSetting {
        unimplemented!()
    }

    fn app<'a, 'b>(&self) -> App<'a, 'b> {
        SubCommand::with_name(CMD_ISSUE_SEARCH)
            .about("Search issues")
            .visible_alias("s")
            .args(&[
                Arg::with_name("query")
                    .help("Issue filter query")
                    .long_help("Issue query by https://docs.github.com/en/github/searching-for-information-on-github/searching-issues-and-pull-requests")
                    .short("q")
                    .long("query")
                    .value_delimiter(" ")
                    .takes_value(true),
            ])
    }

    async fn process<'a>(&self, matches: &ArgMatches<'a>) -> CmdResult {
        check_github_args(&matches)?;

        let config = NoteConfig::new(matches);
        let repo_component = RepoComponent::new(None, Arc::new(config));

        let issues = progress!(
            "Searching issues",
            repo_component.search_issues_by_query(matches.value_of("query").unwrap_or_default()).await?;
        );

        let output_format = get_output_format_from_args(&matches)?;
        output!(output_format, .display(
            stdout(),
            &issues,
            Some(vec!["url", "title"]),
            None,
        ))
    }
}
