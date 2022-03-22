use std::io::stdout;
use std::sync::Arc;

use async_trait::async_trait;
use clap::{ArgMatches, Command};
use libcli_rs::output::{OutputFactory, OutputTrait};
use libcli_rs::progress::{ProgressBar, ProgressTrait};

use crate::cmd::arg::create_query_arg;
use crate::cmd::CommandTrait;
use crate::component::repo::issue::IssueComponentTrait;
use crate::component::repo::RepoComponent;
use crate::config::NoteConfig;
use crate::result::CmdResult;
use crate::util::get_output_format_from_args;

pub const CMD_ISSUE_SEARCH: &str = "search";

pub struct SearchIssueCommand;

impl SearchIssueCommand {
    pub fn new() -> Self {
        Self
    }
}

#[async_trait]
impl CommandTrait for SearchIssueCommand {
    fn app<'help>(&self) -> Command<'help> {
        Command::new(CMD_ISSUE_SEARCH)
            .about("Search issues")
            .visible_alias("s")
            .args([create_query_arg()])
    }

    async fn process(&self, matches: &ArgMatches) -> CmdResult {
        let config = NoteConfig::new(matches);
        let repo_component = RepoComponent::new(None, Arc::new(config));

        let issues = progress!(
            "Searching issues",
            repo_component.search_issues_by_query(matches.value_of("query").unwrap_or_default()).await?;
        );

        if issues.is_empty() {
            progress!("Issues not found", ());
            return Ok(());
        }

        let output_format = get_output_format_from_args(&matches)?;
        output!(output_format, .display(
            stdout(),
            &issues,
            Some(vec!["html_url", "title"]),
            None,
        ))
    }
}
