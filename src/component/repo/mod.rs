use std::sync::Arc;

use hubcaps::Github;

use crate::component::repo::issue::IssueComponentTrait;
use crate::component::repo::release::ReleaseComponentTrait;
use crate::config::NoteConfig;

pub(crate) mod issue;
mod release;

pub(crate) struct RepoComponent {
    github: Arc<Github>,
    config: Arc<NoteConfig>,
}

impl RepoComponent {
    pub fn new(
        github: &Arc<Github>,
        config: Arc<NoteConfig>,
    ) -> impl IssueComponentTrait + ReleaseComponentTrait {
        RepoComponent {
            github: Arc::clone(github),
            config: config.clone(),
        }
    }
}
