use std::sync::Arc;

use async_trait::async_trait;
use clap::{Arg, ArgMatches, Command};
use libcli_rs::progress::{ProgressBar, ProgressTrait};

use crate::cmd::arg::create_query_arg;
use crate::cmd::issue::{create_issues_info_to_update, IssueLabelUpdateType};
use crate::cmd::CommandTrait;
use crate::component::repo::issue::IssueComponentTrait;
use crate::component::repo::RepoComponent;
use crate::config::NoteConfig;
use crate::result::CmdResult;

pub const CMD_REMOVE_LABEL: &str = "remove-label";

pub struct RemoveLabelCommand;

impl RemoveLabelCommand {
    pub fn new() -> Self {
        Self {}
    }
}

#[async_trait]
impl CommandTrait for RemoveLabelCommand {
    fn app<'help>(&self) -> Command<'help> {
        Command::new(CMD_REMOVE_LABEL)
            .about("Remove labels from issues")
            .visible_alias("rl")
            .args([
                create_query_arg(),
                Arg::new("labels")
                    .help("Labels to remove")
                    .long("labels")
                    .value_delimiter(' ')
                    .required(true)
                    .takes_value(true),
            ])
    }

    async fn process(&self, matches: &ArgMatches) -> CmdResult {
        let config = NoteConfig::new(matches);
        let repo_component = RepoComponent::new(None, Arc::new(config));
        let query = matches.value_of("query").unwrap_or_default();
        let labels: Vec<_> = matches.values_of("labels").unwrap().collect();

        let issues_to_update = create_issues_info_to_update(
            &repo_component,
            query,
            &labels,
            &IssueLabelUpdateType::Remove,
        )
        .await?;

        progress!(
            format!("Updating issues to remove the labels ({:?})", labels),
            repo_component.update_issues(&issues_to_update).await?;
        );

        println!(
            "Successfully updated issues to remove the labels {:?}",
            labels
        );
        Ok(())
    }
}
