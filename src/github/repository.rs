use std::path::PathBuf;

use anyhow::{anyhow, ensure, Result};
use futures::{stream, Stream, StreamExt};
use url::Url;

use crate::{
    error::InvalidRepositoryUrl,
    github::{
        client::GitHubApiClient,
        models::{ContentsType, SubtreeModel, TreesModel},
    },
};

use super::{blob::GitHubBlob, statistics::Statistics};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GitHubRepository {
    pub owner: String,
    pub repo: String,
}

impl GitHubRepository {
    pub const ORIGIN: &'static str = "https://github.com";

    pub fn new(owner: &str, repo: &str) -> Self {
        Self { owner: owner.to_string(), repo: repo.to_string() }
    }

    pub fn from_url(url: &Url) -> Result<Self> {
        ensure!(url.origin().unicode_serialization() == Self::ORIGIN, InvalidRepositoryUrl::CannotBeBase);
        let mut path_segments = url.path_segments().ok_or_else(|| anyhow!(InvalidRepositoryUrl::CannotBeBase))?;
        let owner = path_segments.next().ok_or_else(|| anyhow!(InvalidRepositoryUrl::CannotFindOwner))?;
        let repo = path_segments.next().ok_or_else(|| anyhow!(InvalidRepositoryUrl::CannotFindRepo))?;
        Ok(Self::new(owner, repo))
    }

    pub fn to_url(&self) -> Result<Url> {
        let mut url = Url::parse(Self::ORIGIN)?;
        url.set_path(&[&self.owner[..], &self.repo[..]].join("/"));
        Ok(url)
    }

    pub async fn walk(&self) -> impl Stream<Item = Result<GitHubBlob>> + '_ {
        let client = GitHubApiClient::new(self); // TODO lifetime
        let sha = "master";

        let TreesModel { tree, .. } = client.trees(sha, true).await.unwrap();

        stream::iter(tree.into_iter())
            .filter_map(move |SubtreeModel { path, contents_type, .. }| async move {
                match contents_type {
                    ContentsType::Tree => None,
                    ContentsType::Blob => Some(GitHubBlob::repo_path(self, PathBuf::from(&path), sha)),
                    ContentsType::Commit => None,
                }
            })
            .buffered(32) // num_cpus::get() returns 1
    }

    pub async fn get_statistics(&self) -> Result<Statistics> {
        Statistics::get(self.clone()).await // TODO lifetime
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_repo_url() {
        let url = Url::parse("https://github.com/hayas1/loc-viewer").unwrap();
        let repo = GitHubRepository::from_url(&url).unwrap();
        assert_eq!(repo, GitHubRepository::new("hayas1", "loc-viewer"));
        assert_eq!(repo.to_url().unwrap().as_str(), "https://github.com/hayas1/loc-viewer");

        let url = Url::parse("https://github.com/hayas1/loc-viewer/").unwrap();
        let repo = GitHubRepository::from_url(&url).unwrap();
        assert_eq!(repo, GitHubRepository::new("hayas1", "loc-viewer"));
        assert_eq!(repo.to_url().unwrap().as_str(), "https://github.com/hayas1/loc-viewer");
    }
}
