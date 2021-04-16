use crate::result::Result;
use hubcaps::{Credentials, Github};

pub(crate) fn create_github_client(token: &str) -> Result<Github> {
    let client = Github::new("renote", Credentials::Token(token.to_string()))?;
    Ok(client)
}
