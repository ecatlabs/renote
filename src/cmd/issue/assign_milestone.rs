use std::sync::Arc;

use async_trait::async_trait;
use clap::{App, Arg, ArgMatches, SubCommand};
use hubcaps::issues::IssueOptions;
use libcli_rs::progress::{ProgressBar, ProgressTrait};

use crate::cmd::{check_github_args, CommandSetting, CommandTrait};
use crate::component::repo::issue::IssueComponentTrait;
use crate::component::repo::RepoComponent;
use crate::config::NoteConfig;
use crate::result::CmdResult;

pub(crate) const CMD_ASSIGN_MILESTONE: &str = "assign-milestone";

pub(crate) struct AssignMilestoneCommand;

impl AssignMilestoneCommand {
    pub fn new() -> Self {
        Self {}
    }
}

#[async_trait]
impl CommandTrait for AssignMilestoneCommand {
    fn setting(&self) -> &CommandSetting {
        unimplemented!()
    }

    fn app<'a, 'b>(&self) -> App<'a, 'b> {
        SubCommand::with_name(CMD_ASSIGN_MILESTONE)
            .about("Assign issues to a milestone")
            .visible_alias("am")
            .args(&[
                Arg::with_name("query")
                    .help("Issue filter query")
                    .long_help("Issue query by https://docs.github.com/en/github/searching-for-information-on-github/searching-issues-and-pull-requests")
                    .short("q")
                    .long("query")
                    .value_delimiter(" ")
                    .takes_value(true),
                Arg::with_name("milestone")
                    .value_name("milestone")
                    .help("Milestone")
                    .required(true)
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

        let milestone = repo_component
            .get_milestone(matches.value_of("milestone").unwrap())
            .await?;
        let issues_to_update: Vec<_> = issues
            .iter()
            .map(|it| IssueOptions {
                title: it.title.clone(),
                body: it.body.clone(),
                assignee: None,
                assignees: Some(it.assignees.iter().map(|it| it.login.clone()).collect()),
                milestone: Some(milestone.number),
                labels: it.labels.iter().map(|it| it.name.clone()).collect(),
                state: it.state.clone(),
            })
            .collect();

        progress!(
            format!("Updating issues to the milestone {:?}", milestone),
            repo_component.update_issues(&issues_to_update).await?;
        );

        println!(
            "Successfully updated issues to the milestone {:?}",
            milestone
        );
        Ok(())
    }
}
