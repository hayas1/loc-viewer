use base64::prelude::*;
use std::path::PathBuf;

use crate::error::Result;

use super::models::{BlobsModel, EncodingType};

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct GitHubBlob {
    pub path: PathBuf,
    pub content: String,
}

impl GitHubBlob {
    pub fn new(path: PathBuf, content: String) -> Self {
        Self { path, content }
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
        let decoded = BASE64_STANDARD.decode(encoded).map_err(anyhow::Error::from)?;
        Ok(String::from_utf8_lossy(&decoded).into_owned())
    }
}
