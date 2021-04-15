use std::collections::HashMap;
use std::env::args;

use anyhow::Result;
use async_trait::async_trait;
use clap::{App, ArgMatches, SubCommand};

pub mod issue;
pub mod note;

pub type CmdResult = Result<()>;
pub type CmdBox = Box<dyn CommandTrait + Send + Sync>;
pub type CmdGroup = HashMap<&'static str, CmdBox>;

pub struct CommandSetting {
    name: &'static str,
    about: &'static str,
    commands: CmdGroup,
}

#[async_trait]
pub trait CommandTrait {
    fn setting(&self) -> &CommandSetting;

    fn app<'a, 'b>(&self) -> App<'a, 'b> {
        let sub_commands: Vec<_> = self
            .setting()
            .commands
            .values()
            .map(|it| it.app())
            .collect();

        SubCommand::with_name(self.setting().name)
            .about(self.setting().about)
            .subcommands(sub_commands)
    }

    async fn process<'a>(&self, matches: &ArgMatches<'a>) -> CmdResult {
        if let (command_name, Some(sub_matches)) = matches.subcommand() {
            self.setting()
                .commands
                .get(command_name)
                .unwrap()
                .process(sub_matches)
                .await?;

            return Ok(());
        }

        println!("{}", matches.usage());
        Ok(())
    }
}

pub fn get_app_matches<'a, 'b>(app: App<'a, 'b>) -> ArgMatches<'a> {
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
