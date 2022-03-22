use std::collections::HashMap;
use std::env::args;

use anyhow::anyhow;
use async_trait::async_trait;
use clap::{ArgMatches, Command};

use crate::result::{CmdResult, Result};
use crate::util::init_log;

mod arg;
pub mod issue;
pub mod note;

pub type CmdBox = Box<dyn CommandTrait + Send + Sync>;
pub type CmdGroup = HashMap<&'static str, CmdBox>;

pub struct CommandSetting {
    name: &'static str,
    about: &'static str,
    commands: CmdGroup,
}

#[async_trait]
pub trait CommandTrait {
    fn setting(&self) -> &CommandSetting {
        unimplemented!()
    }

    fn app<'help>(&self) -> Command<'help> {
        let sub_commands: Vec<_> = self
            .setting()
            .commands
            .values()
            .map(|it| it.app())
            .collect();

        Command::new(self.setting().name)
            .about(self.setting().about)
            .subcommands(sub_commands)
    }

    fn validate(&self, matches: &ArgMatches) -> CmdResult {
        check_github_args(&matches)
    }

    async fn process(&self, matches: &ArgMatches) -> CmdResult {
        if let Some(log_level) = matches.value_of("log-level") {
            init_log(log_level)?;
        }

        if let Some((cmd, sub_matches)) = matches.subcommand() {
            let cmd = self.setting().commands.get(cmd).unwrap();

            let result = self.validate(matches);
            if result.is_err() {
                return result;
            }

            return cmd.process(sub_matches).await;
        }

        Ok(self.app().print_help()?)
    }
}

pub fn get_app_matches(app: Command) -> ArgMatches {
    let mut args = args();
    match args {
        _ if args.len() == 1 => {
            app.get_matches_from(vec![args.nth(0).unwrap(), "help".to_string()])
        }

        _ => app.get_matches(),
    }
}

pub fn create_cmd(c: CmdBox) -> CmdBox {
    c
}

fn check_github_args(matches: &ArgMatches) -> Result<()> {
    if matches.is_present("owner") && matches.is_present("repo") && matches.is_present("token") {
        return Ok(());
    }

    Err(anyhow!("GitHub owner, repo, and token are mandatory"))
}
