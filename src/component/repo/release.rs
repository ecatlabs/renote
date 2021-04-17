use async_trait::async_trait;
use hubcaps::releases::Release;

use crate::component::repo::RepoComponent;
use crate::result::Result;

#[async_trait]
pub(crate) trait ReleaseComponentTrait {
    async fn get_latest_release(&self) -> Result<Release>;
}

#[async_trait]
impl ReleaseComponentTrait for RepoComponent {
    async fn get_latest_release(&self) -> Result<Release> {
        let repo = self.github.repo(&self.config.owner, &self.config.repo);
        Ok(repo.releases().latest().await?)
    }
}
