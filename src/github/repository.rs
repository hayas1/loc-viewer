use anyhow::{anyhow, ensure, Result};
use url::Url;

use crate::error::InvalidRepositoryUrl;

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
