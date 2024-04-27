use anyhow::Result;
use http::Uri;
use octocrab::{models, service::middleware::base_uri::BaseUriLayer, AuthState};
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

    pub async fn get_repository_(&self) -> Result<models::Repository> {
        let GitHubRepository { owner, repo } = &self.repository;
        let path = format!("/repos/{owner}/{repo}");
        Ok(gloo::net::http::Request::get(self.endpoint(&path)?.as_str()).send().await?.json().await?)
    }

    async fn handle(request: http::Request<String>) -> Result<http::Response<String>> {
        let (parts, body) = request.into_parts();
        let method = match parts.method {
            http::Method::GET => gloo::net::http::Method::GET,
            http::Method::POST => gloo::net::http::Method::POST,
            http::Method::PUT => gloo::net::http::Method::PUT,
            http::Method::DELETE => gloo::net::http::Method::DELETE,
            _ => todo!(), // TODO
        };
        let response = gloo::net::http::RequestBuilder::new(&parts.uri.to_string())
            .method(method)
            .body(body)
            .unwrap()
            .send()
            .await
            .unwrap();
        Ok(hyper::Response::builder()
            .status(response.status())
            .body(response.body().unwrap().as_string().unwrap())
            .unwrap())
        // let response = http::Response::new("\"Hello, World!\"".to_string());
        // Ok(response)
    }

    pub async fn get_repository(&self) -> Result<models::Repository> {
        let GitHubRepository { owner, repo } = &self.repository;
        let http_client = tower::service_fn(Self::handle);
        let octo = octocrab::OctocrabBuilder::new_empty()
            .with_service(http_client)
            .with_layer(&BaseUriLayer::new(Uri::from_static("https://api.github.com"))) // !Send ...
            // .with_layer(&ExtraHeadersLayer::new(Arc::new(vec![(USER_AGENT, "octocrab".parse().unwrap())])))
            .with_auth(AuthState::None)
            .build() // Buffer::new use tokio::spawn ...
            .unwrap();
        Ok(octo.repos(owner, repo).get().await?)
    }
}
