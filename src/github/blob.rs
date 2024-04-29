use anyhow::Result;
use base64::prelude::*;
use std::path::PathBuf;

use super::{
    client::GitHubApiClient,
    models::{BlobsModel, EncodingType},
    repository::GitHubRepository,
};

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct GitHubBlob {
    pub path: PathBuf,
    pub content: String,
}

impl GitHubBlob {
    pub fn new(path: PathBuf, content: String) -> Self {
        Self { path, content }
    }

    pub async fn repo_path(repository: GitHubRepository, path: PathBuf, sha: &str) -> Result<Self> {
        let client = GitHubApiClient::new(repository.clone());
        let raw = client.raw(sha, &path).await?;
        Ok(Self::new(path, raw))
    }

    pub fn from_model(path: PathBuf, model: BlobsModel) -> Result<Self> {
        let content = BlobsModelDecoder::new(&model).decode()?;
        Ok(Self::new(path, content))
    }
}

pub struct BlobsModelDecoder<'a> {
    model: &'a BlobsModel,
}
impl<'a> BlobsModelDecoder<'a> {
    pub fn new(model: &'a BlobsModel) -> Self {
        Self { model }
    }

    pub fn decode(&self) -> Result<String> {
        match self.model.encoding {
            EncodingType::Base64 => self.decode_base64(),
        }
    }

    pub fn decode_base64(&self) -> Result<String> {
        let mut encoded = self.model.content.as_bytes().to_owned();
        encoded.retain(|b| !b.is_ascii_whitespace());
        let s = BASE64_STANDARD.decode(encoded)?;
        Ok(String::from_utf8_lossy(&s).into_owned())
    }
}
