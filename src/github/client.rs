use std::path::Path;

use anyhow::{anyhow, Result};
use url::Url;

use super::{models::TreesModel, repository::GitHubRepository};

pub struct GitHubApiClient {
    pub repository: GitHubRepository,
}

impl GitHubApiClient {
    pub const API_ORIGIN: &'static str = "https://api.github.com";
    pub const RAW_ORIGIN: &'static str = "https://raw.githubusercontent.com";

    pub fn new(repository: GitHubRepository) -> Self {
        Self { repository }
    }

    pub fn api_endpoint(&self, path: &str) -> Result<Url> {
        let mut url = Url::parse(GitHubApiClient::API_ORIGIN)?;
        url.set_path(path);
        Ok(url)
    }

    pub fn raw_endpoint(&self, path: &str) -> Result<Url> {
        let mut url = Url::parse(GitHubApiClient::RAW_ORIGIN)?;
        url.set_path(path);
        Ok(url)
    }

    pub async fn trees(&self, sha: &str, recursive: bool) -> Result<TreesModel> {
        let GitHubRepository { owner, repo } = &self.repository;
        let path = format!("/repos/{owner}/{repo}/git/trees/{sha}");
        let request = gloo::net::http::Request::get(self.api_endpoint(&path)?.as_str())
            .query([("recursive", recursive.to_string())]);
        Ok(request.send().await?.json().await?)
    }

    pub async fn raw<A: AsRef<Path>>(&self, sha: &str, path: A) -> Result<String> {
        let GitHubRepository { owner, repo } = &self.repository;
        let path = path.as_ref().to_str().ok_or_else(|| anyhow!("// TODO error handling"))?;
        let path = format!("/{owner}/{repo}/{sha}/{path}");
        let request = gloo::net::http::Request::get(self.raw_endpoint(&path)?.as_str());
        Ok(request.send().await?.text().await?)
    }
}
