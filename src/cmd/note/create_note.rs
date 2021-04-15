use std::fs::File;
use std::path::Path;

use async_trait::async_trait;
use clap::{App, Arg, ArgMatches, SubCommand};

use crate::cmd::{CmdResult, CommandSetting, CommandTrait};
use crate::component::note::{NoteComponent, NoteComponentTrait};
use crate::config::IssueSearchConfig;

pub const CMD_CREATE_NOTE: &str = "create";

pub struct CreateNoteCommand;

impl CreateNoteCommand {
    pub fn new() -> Self {
        Self
    }
}

#[async_trait]
impl CommandTrait for CreateNoteCommand {
    fn setting(&self) -> &CommandSetting {
        unimplemented!()
    }

    fn app<'a, 'b>(&self) -> App<'a, 'b> {
        SubCommand::with_name(CMD_CREATE_NOTE)
            .about("Create the release note")
            .visible_alias("cn")
            .args(&[
                Arg::with_name("config")
                    .help("Issue search config yaml file")
                    .long("config")
                    .takes_value(true),
                Arg::with_name("state")
                    .help("Issue state")
                    .long("state")
                    .default_value("closed")
                    .possible_values(&["open", "closed", "all"])
                    .takes_value(true)
                    .required_unless("config"),
                Arg::with_name("note")
                    .help("Release note template")
                    .long("note")
                    .takes_value(true)
                    .required_unless("config"),
                Arg::with_name("milestone")
                    .help("Milestone to filter")
                    .long("milestone")
                    .takes_value(true)
                    .required_unless("config"),
                Arg::with_name("labels")
                    .help("Labels to filter")
                    .long("labels")
                    .takes_value(true)
                    .required_unless("config"),
                Arg::with_name("exclude_labels")
                    .help("Labels to exclude")
                    .value_name("labels")
                    .value_delimiter(",")
                    .long("exclude-labels")
                    .takes_value(true)
                    .required_unless("config"),
                Arg::with_name("highlight_labels")
                    .help("Labels to highlight")
                    .value_name("labels")
                    .value_delimiter(",")
                    .long("highlight-labels")
                    .takes_value(true)
                    .required_unless("config"),
            ])
    }

    async fn process<'a>(&self, matches: &ArgMatches<'a>) -> CmdResult {
        let mut search_config = if let Some(config_path) = matches.value_of("config") {
            let file = File::open(Path::new(config_path))
                .expect(format!("expect {} found", config_path).as_str());
            serde_yaml::from_reader(file).expect("expect node config file")
        } else {
            IssueSearchConfig {
                owner: matches
                    .value_of("owner")
                    .expect("expect github owner")
                    .to_string(),
                repo: matches
                    .value_of("repo")
                    .expect("expect github repo")
                    .to_string(),
                token: matches
                    .value_of("token")
                    .expect("expect github token")
                    .to_string(),
                state: matches.value_of("state").unwrap().to_string(),
                note: Some(matches.value_of("note").unwrap().to_string()),
                milestone: Some(matches.value_of("milestone").unwrap().to_string()),
                labels: Some(
                    matches
                        .values_of("labels")
                        .unwrap()
                        .map(|it| it.to_string())
                        .collect(),
                ),
                exclude_labels: Some(
                    matches
                        .values_of("exclude_labels")
                        .unwrap()
                        .map(|it| it.to_string())
                        .collect(),
                ),
                highlight_labels: Some(
                    matches
                        .values_of("highlight_labels")
                        .unwrap()
                        .map(|it| it.to_string())
                        .collect(),
                ),
                show_contributor: true,
            }
        };

        if search_config.token.is_empty() {
            if matches.is_present("token") {
                search_config.token = matches
                    .value_of("token")
                    .expect("expect github token")
                    .to_string();
            }
        }

        let note_component = NoteComponent::new();
        println!("{}", note_component.create_note(&search_config).await?);

        Ok(())
    }
}
