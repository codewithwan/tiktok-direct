use serde_json::Value;
use std::collections::BTreeMap;

use super::fields::{string_field, u64_field};
use crate::models::{AuthorStats, Mention};

pub fn pick_url(value: Option<&Value>) -> Option<String> {
    match value? {
        Value::String(text) => Some(text.clone()),
        Value::Array(items) => items
            .iter()
            .find_map(|item| item.as_str().map(str::to_string)),
        Value::Object(map) => ["urlList", "url_list", "urls"]
            .iter()
            .find_map(|key| pick_url(map.get(*key))),
        _ => None,
    }
}

pub fn map_stats(stats: &Value) -> BTreeMap<String, Value> {
    map_values(
        stats,
        &[
            "playCount",
            "diggCount",
            "shareCount",
            "commentCount",
            "collectCount",
        ],
    )
}

pub fn map_author_stats(value: &Value) -> AuthorStats {
    AuthorStats {
        follower_count: u64_field(value, "followerCount"),
        following_count: u64_field(value, "followingCount"),
        heart_count: u64_field(value, "heartCount"),
        video_count: u64_field(value, "videoCount"),
        digg_count: u64_field(value, "diggCount"),
    }
}

pub fn map_hashtags(item: Option<&Value>) -> Vec<String> {
    let mut tags = Vec::new();
    if let Some(items) = item
        .and_then(|value| value.get("challenges"))
        .and_then(Value::as_array)
    {
        tags.extend(
            items
                .iter()
                .filter_map(|value| string_field(Some(value), "title")),
        );
    }
    if let Some(extra) = item
        .and_then(|value| value.get("textExtra"))
        .and_then(Value::as_array)
    {
        tags.extend(extra.iter().filter_map(|value| {
            string_field(Some(value), "hashtagName").or_else(|| {
                let ty = value.get("type").and_then(Value::as_u64);
                ty.is_some_and(|ty| ty == 1)
                    .then(|| string_field(Some(value), "userUniqueId"))
                    .flatten()
            })
        }));
    }
    tags.sort();
    tags.dedup();
    tags
}

pub fn map_mentions(item: Option<&Value>) -> Vec<Mention> {
    item.and_then(|value| value.get("textExtra"))
        .and_then(Value::as_array)
        .into_iter()
        .flatten()
        .filter_map(|value| {
            let username = string_field(Some(value), "userUniqueId")
                .or_else(|| string_field(Some(value), "uniqueId"));
            let user_id = string_field(Some(value), "userId");
            (username.is_some() || user_id.is_some()).then(|| Mention {
                username,
                display_text: string_field(Some(value), "displayText"),
                user_id,
            })
        })
        .collect()
}

pub fn map_media(video: &Value) -> BTreeMap<String, Value> {
    BTreeMap::from([
        ("cover".to_string(), pick_url(video.get("cover")).into()),
        (
            "dynamic_cover".to_string(),
            pick_url(video.get("dynamicCover")).into(),
        ),
        (
            "origin_cover".to_string(),
            pick_url(video.get("originCover")).into(),
        ),
        (
            "play_addr".to_string(),
            pick_url(video.get("playAddr")).into(),
        ),
        (
            "download_addr".to_string(),
            pick_url(video.get("downloadAddr")).into(),
        ),
    ])
}

pub fn map_music(music: &Value) -> BTreeMap<String, Value> {
    BTreeMap::from([
        ("id".to_string(), string_field(Some(music), "id").into()),
        (
            "title".to_string(),
            string_field(Some(music), "title").into(),
        ),
        (
            "author".to_string(),
            string_field(Some(music), "authorName").into(),
        ),
        (
            "play_url".to_string(),
            pick_url(music.get("playUrl")).into(),
        ),
    ])
}

pub fn map_status_flags(item: Option<&Value>) -> BTreeMap<String, bool> {
    let keys = [
        "isAd",
        "isPrivate",
        "secret",
        "isOriginal",
        "isAigc",
        "isReviewing",
    ];
    keys.iter()
        .map(|key| ((*key).to_string(), bool_field(item, key).unwrap_or(false)))
        .collect()
}

pub fn map_location(item: Option<&Value>) -> BTreeMap<String, Value> {
    let mut map = BTreeMap::new();
    for key in ["locationCreated", "contentLocation", "poi"] {
        if let Some(value) = item.and_then(|item| item.get(key)) {
            map.insert(key.to_string(), value.clone());
        }
    }
    map
}

pub fn raw_keys(value: Option<&Value>) -> Vec<String> {
    let mut keys: Vec<String> = value
        .and_then(Value::as_object)
        .map(|o| o.keys().cloned().collect())
        .unwrap_or_default();
    keys.sort();
    keys
}

fn map_values(value: &Value, keys: &[&str]) -> BTreeMap<String, Value> {
    keys.iter()
        .filter_map(|key| {
            value
                .get(*key)
                .map(|value| ((*key).to_string(), value.clone()))
        })
        .collect()
}

fn bool_field(value: Option<&Value>, key: &str) -> Option<bool> {
    value?
        .get(key)
        .and_then(|value| value.as_bool().or_else(|| value.as_u64().map(|v| v != 0)))
}
