#![allow(where_clauses_object_safety)]

#[macro_use]
extern crate maplit;

use std::process::exit;

use clap::{App, Arg};

use crate::cmd::issue::*;
use crate::cmd::note::*;
use crate::cmd::{create_cmd, get_app_matches, CmdGroup};

mod cmd;
mod component;
mod config;
mod log;
mod result;
mod util;

#[tokio::main]
async fn main() {
    let commands: CmdGroup = hashmap! {
        CMD_ISSUE => create_cmd(Box::new(IssueCommand::new())),
        CMD_NOTE => create_cmd(Box::new(NoteCommand::new())),
    };
    let sub_commands: Vec<_> = commands.values().map(|it| it.app()).collect();

    let app = App::new("renote")
        .version(option_env!("BUILD_VERSION").unwrap_or(""))
        .about("GitHub Release Note CLI")
        .subcommands(sub_commands)
        .args(&[
            Arg::with_name("token")
                .value_name("string")
                .help("GitHub personal access token")
                .global(true)
                .long("token")
                .short("t")
                .env("GITHUB_TOKEN")
                .takes_value(true),
            Arg::with_name("owner")
                .value_name("string")
                .help("GitHub owner")
                .global(true)
                .long("owner")
                .short("o")
                .env("GITHUB_OWNER")
                .takes_value(true),
            Arg::with_name("repo")
                .value_name("string")
                .help("GitHub repository")
                .global(true)
                .long("repo")
                .short("r")
                .env("GITHUB_REPO")
                .takes_value(true),
            Arg::with_name("log-level")
                .value_name("string")
                .help("Log level")
                .global(true)
                .long("log-level")
                .short("l")
                .takes_value(true)
                .default_value("error")
                .possible_values(&["off", "error", "warn", "info", "debug", "trace"]),
        ]);

    let matches = get_app_matches(app);
    if let (command_name, Some(sub_matches)) = matches.subcommand() {
        commands
            .get(command_name)
            .unwrap()
            .process(sub_matches)
            .await
            .expect("expect command");
    }

    exit(1);
}
