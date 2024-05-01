use std::path::{Path, PathBuf};

use anyhow::{anyhow, ensure, Result};
use futures::{stream, Stream, StreamExt, TryStreamExt};
use gloo::net::http::Request;
use octocrab::models;
use url::Url;

use crate::{
    error::InvalidRepositoryUrl,
    github::models::{ContentsType, SubtreeModel, TreesModel},
};

use super::{blob::GitHubBlob, statistics::Statistics};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GitHubRepository {
    pub owner: String,
    pub repo: String,
}

impl GitHubRepository {
    pub const ORIGIN: &'static str = "https://github.com";
    pub const API_ORIGIN: &'static str = "https://api.github.com";
    pub const RAW_ORIGIN: &'static str = "https://raw.githubusercontent.com";

    pub fn new(owner: &str, repo: &str) -> Self {
        Self { owner: owner.to_string(), repo: repo.to_string() }
    }

    pub fn from_url(url: &Url) -> Result<Self> {
        ensure!(url.origin().unicode_serialization() == Self::ORIGIN, InvalidRepositoryUrl::CannotBeBase);
        let mut path_segments = url.path_segments().ok_or_else(|| anyhow!(InvalidRepositoryUrl::CannotBeBase))?;
        let owner = path_segments.next().ok_or_else(|| anyhow!(InvalidRepositoryUrl::CannotFindOwner))?;
        let repo = path_segments.next().ok_or_else(|| anyhow!(InvalidRepositoryUrl::CannotFindRepo))?;
        // TODO rest path
        Ok(Self::new(owner, repo))
    }

    pub fn to_url(&self) -> Result<Url> {
        let mut url = Url::parse(Self::ORIGIN)?;
        url.set_path(&[&self.owner[..], &self.repo[..]].join("/"));
        Ok(url)
    }

    pub fn api_endpoint(&self, path: &str) -> Result<Url> {
        let mut url = Url::parse(Self::API_ORIGIN)?;
        url.set_path(path);
        Ok(url)
    }

    pub fn raw_endpoint(&self, path: &str) -> Result<Url> {
        let mut url = Url::parse(Self::RAW_ORIGIN)?;
        url.set_path(path);
        Ok(url)
    }

    pub async fn trees(&self, sha: &str, recursive: bool) -> Result<TreesModel> {
        let Self { owner, repo } = &self;
        let path = format!("/repos/{owner}/{repo}/git/trees/{sha}");
        let request = Request::get(self.api_endpoint(&path)?.as_str()).query([("recursive", recursive.to_string())]);
        Ok(request.send().await?.json().await?)
    }

    pub async fn repository(&self) -> Result<models::Repository> {
        let Self { owner, repo } = &self;
        let path = format!("/repos/{owner}/{repo}");
        let request = Request::get(self.api_endpoint(&path)?.as_str());
        Ok(request.send().await?.json().await?)
    }

    pub async fn raw<A: AsRef<Path>>(&self, sha: &str, path: A) -> Result<String> {
        let Self { owner, repo } = &self;
        let path = path.as_ref().to_str().ok_or_else(|| anyhow!("// TODO error handling"))?;
        let path = format!("/{owner}/{repo}/{sha}/{path}");
        let request = Request::get(self.raw_endpoint(&path)?.as_str());
        Ok(request.send().await?.text().await?)
    }

    pub async fn walk(&self) -> impl Stream<Item = Result<GitHubBlob>> + '_ {
        // TODO zip or tar.gz
        let sha = "master";

        let TreesModel { tree, .. } = self.trees(sha, true).await.unwrap();
        let paths = tree.into_iter().filter_map(|SubtreeModel { path, contents_type, .. }| match contents_type {
            ContentsType::Tree => None,
            ContentsType::Blob => Some(PathBuf::from(path)),
            ContentsType::Commit => None,
        });

        stream::iter(paths.clone())
            .map(|path| self.raw(sha, path))
            .buffered(32) // num_cpus::get() returns 1
            .zip(stream::iter(paths))
            .map(|(raw, path)| Ok(GitHubBlob::new(path, raw?)))
            .map_ok(|blob| blob)
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
