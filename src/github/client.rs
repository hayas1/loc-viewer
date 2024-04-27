use anyhow::{anyhow, Result};
use graphql_client::GraphQLQuery;
use octocrab::models;
use url::Url;

use super::repository::GitHubRepository;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "gql/schema.json",
    query_path = "gql/repository.gql",
    response_derives = "Debug,Clone,PartialEq"
)]
struct Repository;

pub struct GitHubApiClient {
    pub repository: GitHubRepository,
}

impl GitHubApiClient {
    pub const ORIGIN: &'static str = "https://api.github.com";
    pub const URL: &'static str = "https://api.github.com/graphql";

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

    pub async fn repository(&self) -> Result<repository::ResponseData> {
        let data = Repository::build_query(repository::Variables {
            owner: self.repository.owner.clone(),
            name: self.repository.repo.clone(),
        });
        let query = serde_json::json!(data).to_string();
        let request = gloo::net::http::RequestBuilder::new(Self::URL)
            .header("Authorization", "Bearer xxxxxxxxxxxxxxxxxxxxxxxxxxxxx")
            .method(gloo::net::http::Method::POST)
            .body(&query)?;
        let response = request.send().await?.json().await?;
        Ok(response)
    }
}
