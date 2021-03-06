#![allow(where_clauses_object_safety)]

#[macro_use]
extern crate libcli_rs;
#[macro_use]
extern crate maplit;

use clap::{Arg, Command, Error, ErrorKind};

use crate::cmd::issue::*;
use crate::cmd::note::*;
use crate::cmd::{create_cmd, get_app_matches, CmdGroup};

mod cmd;
mod component;
mod config;
mod result;
mod util;

#[tokio::main]
async fn main() {
    let commands: CmdGroup = hashmap! {
        CMD_ISSUE => create_cmd(Box::new(IssueCommand::new())),
        CMD_NOTE => create_cmd(Box::new(NoteCommand::new())),
    };
    let sub_commands: Vec<Command> = commands.values().map(|it| it.app()).collect();

    let app = Command::new(env!("CARGO_PKG_NAME"))
        .long_version(env!("LONG_VERSION"))
        .about("A complementary Github tool to use with gh to extend note/issue/... experience")
        .subcommands(sub_commands)
        .args([
            Arg::new("token")
                .value_name("string")
                .help("GitHub personal access token")
                .global(true)
                .long("token")
                .short('t')
                .env("GITHUB_TOKEN")
                .takes_value(true),
            Arg::new("owner")
                .value_name("string")
                .help("GitHub owner")
                .global(true)
                .long("owner")
                .short('o')
                .env("GITHUB_OWNER")
                .takes_value(true),
            Arg::new("repo")
                .value_name("string")
                .help("GitHub repository")
                .global(true)
                .long("repo")
                .short('r')
                .env("GITHUB_REPO")
                .takes_value(true),
            Arg::new("log-level")
                .value_name("string")
                .help("Log level")
                .global(true)
                .long("log-level")
                .short('l')
                .takes_value(true)
                .default_value("error")
                .possible_values(["off", "error", "warn", "info", "debug", "trace"]),
            Arg::new("format")
                .value_name("format")
                .help("Output format")
                .global(true)
                .long("format")
                .short('f')
                .takes_value(true)
                .default_value("console")
                .possible_values(["console", "json", "yaml"]),
        ]);

    if let Some((cmd, matches)) = get_app_matches(app).subcommand() {
        let cmd = commands.get(cmd).unwrap();

        if let Err(err) = cmd.process(matches).await {
            Error::raw(ErrorKind::Format, err).exit();
        }
    }
}
