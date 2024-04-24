use anyhow::Result;
use octocrab::models;
use url::Url;

use super::repository::GitHubRepository;

pub struct GitHubApiClient {
    pub repository: GitHubRepository,
}

impl GitHubApiClient {
    pub const ORIGIN: &'static str = "https://api.github.com";

    pub fn new(repository: GitHubRepository) -> Self {
        Self { repository }
    }

    pub fn endpoint(&self, path: &str) -> Result<Url> {
        let mut url = Url::parse(GitHubApiClient::ORIGIN)?;
        url.set_path(path);
        Ok(url)
    }

    pub async fn get_repository(&self) -> Result<models::Repository> {
        let GitHubRepository { owner, repo } = &self.repository;
        let path = format!("/repos/{owner}/{repo}");
        Ok(gloo::net::http::Request::get(self.endpoint(&path)?.as_str()).send().await?.json().await?)
    }
}
