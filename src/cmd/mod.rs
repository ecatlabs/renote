use std::collections::HashMap;
use std::env::args;

use anyhow::anyhow;
use async_trait::async_trait;
use clap::{ArgMatches, Command};

use crate::log::init_log;
use crate::result::{CmdResult, Result};

pub mod issue;
pub mod note;

pub type CmdBox = Box<dyn CommandTrait + Send + Sync>;
pub type CmdGroup = HashMap<&'static str, CmdBox>;

fn check_github_args(matches: &ArgMatches) -> Result<()> {
    if matches.is_present("owner") && matches.is_present("repo") && matches.is_present("token") {
        return Ok(());
    }

    Err(anyhow!("GitHub owner, repo, and token are mandatory"))
}

pub struct CommandSetting {
    name: &'static str,
    about: &'static str,
    commands: CmdGroup,
}

#[async_trait]
pub trait CommandTrait {
    fn setting(&self) -> &CommandSetting;

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

    async fn process(&self, matches: &ArgMatches) -> CmdResult {
        if let Some(log_level) = matches.value_of("log-level") {
            init_log(log_level)?;
        }

        if let Some((command_name, sub_matches)) = matches.subcommand() {
            self.setting()
                .commands
                .get(command_name)
                .unwrap()
                .process(sub_matches)
                .await?;

            return Ok(());
        }

        //FIXME
        // println!("{}", matches.usage());
        Ok(())
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
