use hubcaps::{Credentials, Github};

use crate::result::Result;

pub(crate) fn create_github_client(token: &str) -> Result<Github> {
    let client = Github::new("renote", Credentials::Token(token.to_string()))?;
    Ok(client)
}
