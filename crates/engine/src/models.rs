use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BrowserProfile {
    pub user_agent: String,
    pub accept_language: String,
    pub sec_ch_ua: Option<String>,
    pub sec_ch_ua_platform: Option<String>,
}

impl BrowserProfile {
    pub fn random() -> Self {
        crate::net::ua::generate_random_profile()
    }
}

impl Default for BrowserProfile {
    fn default() -> Self {
        Self::random()
    }
}


#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum ExtractionQuality {
    Complete,
    Partial,
    Failed,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VideoMetadata {
    pub input_url: String,
    pub resolved_url: Option<String>,
    pub host: Option<String>,
    pub pathname: Option<String>,
    pub video_id: Option<String>,
    pub username: Option<String>,
    pub author_name: Option<String>,
    pub author_url: Option<String>,
    pub author_unique_id: Option<String>,
    pub title: Option<String>,
    pub description: Option<String>,
    pub image: Option<String>,
    pub view_count: Option<u64>,
    pub like_count: Option<u64>,
    pub repost_count: Option<u64>,
    pub comment_count: Option<u64>,
    pub duration: Option<u64>,
    pub timestamp: Option<String>,
    pub uploader: Option<String>,
    pub uploader_id: Option<String>,
    pub webpage_url: Option<String>,
    pub canonical: Option<String>,
    pub thumbnail_url: Option<String>,
    pub source: Option<String>,
    pub quality: ExtractionQuality,
    pub challenge_solved: bool,
    pub stats: BTreeMap<String, Value>,
    pub media: BTreeMap<String, Value>,
    pub music: BTreeMap<String, Value>,
    pub available_json_sources: BTreeMap<String, bool>,
    pub raw_item_keys: Vec<String>,
    pub raw_item: Option<Value>,
}
