use serde::{Deserialize, Serialize};
use url::Url;

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct TreesModel {
    pub sha: String,
    pub url: Url,
    pub tree: Vec<SubtreeModel>,
    pub truncated: bool,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct SubtreeModel {
    pub path: String,
    pub mode: String,
    #[serde(rename = "type")]
    pub contents_type: ContentsType,
    pub sha: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<Url>,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ContentsType {
    Tree,
    Blob,
    Commit,
    // TODO: symlink ?
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct BlobsModel {
    pub sha: String,
    pub node_id: String,
    pub size: u64,
    pub url: Url,
    pub content: String,
    pub encoding: EncodingType,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EncodingType {
    Base64,
}
