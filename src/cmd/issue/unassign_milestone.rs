use std::sync::Arc;

use async_trait::async_trait;
use clap::{ArgMatches, Command};
use hubcaps_ex::issues::IssueOptions;
use libcli_rs::progress::{ProgressBar, ProgressTrait};

use crate::cmd::arg::create_query_arg;
use crate::cmd::{CmdResult, CommandTrait};
use crate::component::repo::issue::IssueComponentTrait;
use crate::component::repo::RepoComponent;
use crate::config::NoteConfig;

pub const CMD_UNASSIGN_MILESTONE: &str = "unassign-milestone";

pub struct UnassignMilestoneCommand;

impl UnassignMilestoneCommand {
    pub fn new() -> Self {
        Self {}
    }
}

#[async_trait]
impl CommandTrait for UnassignMilestoneCommand {
    fn app<'help>(&self) -> Command<'help> {
        Command::new(CMD_UNASSIGN_MILESTONE)
            .about("Unassign issues from a milestone")
            .visible_alias("um")
            .args([create_query_arg()])
    }

    async fn process(&self, matches: &ArgMatches) -> CmdResult {
        let config = NoteConfig::new(matches);
        let repo_component = RepoComponent::new(None, Arc::new(config));

        let issues = progress!(
            "Searching issues",
            repo_component.search_issues_by_query(matches.value_of("query").unwrap_or_default()).await?;
        );

        let issues_to_update: Vec<_> = issues
            .into_iter()
            .map(|it| {
                (
                    it.number,
                    IssueOptions {
                        title: it.title,
                        body: it.body,
                        assignee: None,
                        assignees: Some(it.assignees.into_iter().map(|it| it.login).collect()),
                        milestone: Some(0),
                        labels: it.labels.into_iter().map(|it| it.name).collect(),
                        state: it.state,
                    },
                )
            })
            .collect();

        progress!(
            "Updating issues out from the original milestone",
            repo_component.update_issues(&issues_to_update).await?;
        );

        println!("Successfully updated issues out from the original milestone");
        Ok(())
    }
}
