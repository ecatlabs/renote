use std::str::FromStr;

use anyhow::anyhow;
use clap::ArgMatches;
use hubcaps_ex::{Credentials, Github};
use libcli_rs::output::OutputFormat;

use crate::result::Result;

pub fn create_github_client(token: &str) -> Result<Github> {
    let client = Github::new(
        env!("CARGO_PKG_NAME"),
        Credentials::Token(token.to_string()),
    )?;

    Ok(client)
}

pub fn get_output_format_from_args(args: &ArgMatches) -> Result<OutputFormat> {
    let format = args.value_of("format").unwrap();
    OutputFormat::from_str(format).map_err(|err| anyhow!("{:?}", err))
}
