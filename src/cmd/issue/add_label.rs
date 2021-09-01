use std::sync::Arc;

use async_trait::async_trait;
use clap::{App, Arg, ArgMatches, SubCommand};
use libcli_rs::progress::{ProgressBar, ProgressTrait};

use crate::cmd::issue::{create_issues_info_to_update, IssueLabelUpdateType};
use crate::cmd::{check_github_args, CommandSetting, CommandTrait};
use crate::component::repo::issue::IssueComponentTrait;
use crate::component::repo::RepoComponent;
use crate::config::NoteConfig;
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
            .about("Add labels to issues")
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

    async fn process<'a>(&self, matches: &ArgMatches<'a>) -> CmdResult {
        check_github_args(&matches)?;

        let config = NoteConfig::new(matches);
        let repo_component = RepoComponent::new(None, Arc::new(config));
        let query = matches.value_of("query").unwrap_or_default();
        let labels: Vec<_> = matches.values_of("labels").unwrap().collect();

        let issues_to_update = create_issues_info_to_update(
            &repo_component,
            query,
            &labels,
            &IssueLabelUpdateType::Add,
        )
        .await?;

        progress!(
            format!("Updating issues to add the labels ({:?})", labels),
            repo_component.update_issues(&issues_to_update).await?;
        );

        println!("Successfully updated issues to add the labels {:?}", labels);
        Ok(())
    }
}
