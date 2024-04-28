use anyhow::Result;
use base64::prelude::*;
use std::path::PathBuf;

use super::models::{BlobsModel, EncodingType};

pub struct GitHubBlob {
    pub path: Option<PathBuf>, // TODO Path cause compile error (sized)
    pub content: String,
}

impl GitHubBlob {
    pub fn new(path: Option<PathBuf>, content: String) -> Self {
        Self { path, content }
    }

    pub fn from_model(path: Option<PathBuf>, model: BlobsModel) -> Result<Self> {
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
