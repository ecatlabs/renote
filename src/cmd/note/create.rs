use std::fs::File;
use std::path::Path;
use std::sync::Arc;

use async_trait::async_trait;
use clap::{App, Arg, ArgMatches, SubCommand};
use libcli_rs::progress::{ProgressBar, ProgressTrait};

use crate::cmd::{CommandSetting, CommandTrait};
use crate::component::note::{NoteComponent, NoteComponentTrait};
use crate::config::NoteConfig;
use crate::result::CmdResult;

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
            .visible_alias("c")
            .args(&[Arg::with_name("config")
                .help("Issue search config yaml file")
                .long("config")
                .required(true)
                .takes_value(true)])
    }

    async fn process<'a>(&self, matches: &ArgMatches<'a>) -> CmdResult {
        let config_path = matches.value_of("config").unwrap();
        let file = File::open(Path::new(config_path))
            .expect(format!("expect {} found", config_path).as_str());
        let mut note_config: NoteConfig =
            serde_yaml::from_reader(file).expect("expect node config file");

        // override by the global settings
        if matches.is_present("token") {
            note_config.token = matches.value_of("token").unwrap().to_string();
        }
        if matches.is_present("owner") {
            note_config.owner = matches.value_of("owner").unwrap().to_string();
        }
        if matches.is_present("repo") {
            note_config.repo = matches.value_of("repo").unwrap().to_string();
        }

        let note_component = NoteComponent::new(Arc::new(note_config));
        let output = progress!("Creating the note", note_component.create_note().await?);

        println!("{}", output);
        Ok(())
    }
}
