use std::str::FromStr;

use anyhow::anyhow;
use clap::ArgMatches;
use hubcaps_ex::{Credentials, Github};
use libcli_rs::output::OutputFormat;

use crate::result::Result;

pub(crate) fn create_github_client(token: &str) -> Result<Github> {
    let client = Github::new("renote", Credentials::Token(token.to_string()))?;
    Ok(client)
}

pub(crate) fn get_output_format_from_args(args: &ArgMatches) -> Result<OutputFormat> {
    match OutputFormat::from_str(args.value_of("format").unwrap()) {
        Ok(o) => Ok(o),
        Err(err) => Err(anyhow!("{:?}", err)),
    }
}
