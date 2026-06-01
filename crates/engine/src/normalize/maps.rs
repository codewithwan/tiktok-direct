use serde_json::Value;
use std::collections::BTreeMap;

use super::fields::string_field;

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
