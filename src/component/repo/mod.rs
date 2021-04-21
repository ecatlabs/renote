use std::sync::Arc;

use hubcaps_ex::Github;

use crate::component::repo::issue::IssueComponentTrait;
use crate::component::repo::release::ReleaseComponentTrait;
use crate::config::NoteConfig;
use crate::util::create_github_client;

pub(crate) mod issue;
pub(crate) mod release;

pub(crate) struct RepoComponent {
    github: Arc<Github>,
    config: Arc<NoteConfig>,
}

impl RepoComponent {
    pub fn new(
        github: Option<Arc<Github>>,
        config: Arc<NoteConfig>,
    ) -> impl IssueComponentTrait + ReleaseComponentTrait {
        let github = if let Some(x) = github {
            x
        } else {
            Arc::new(create_github_client(&*config.token).unwrap())
        };

        RepoComponent { github, config }
    }
}
