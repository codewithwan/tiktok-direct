use serde_json::Value;

use crate::VideoMetadata;

use super::MediaKind;

pub fn list_media_urls(metadata: &VideoMetadata, kind: MediaKind) -> Vec<String> {
    let mut urls = Vec::new();
    match kind {
        MediaKind::Mp4 => {
            add_value(&mut urls, metadata.media.get("play_addr"));
            add_value(&mut urls, metadata.media.get("download_addr"));
            collect_path(&mut urls, metadata.raw_item.as_ref(), &["video"]);
        }
        MediaKind::Mp3 => {
            add_value(&mut urls, metadata.music.get("play_url"));
            collect_path(&mut urls, metadata.raw_item.as_ref(), &["music", "playUrl"]);
        }
        MediaKind::Thumbnail => {
            add_owned(&mut urls, metadata.thumbnail_url.clone());
            add_value(&mut urls, metadata.media.get("cover"));
            add_value(&mut urls, metadata.media.get("origin_cover"));
        }
        MediaKind::Avatar => {
            add_owned(&mut urls, metadata.author_avatar_url.clone());
        }
    }
    urls
}

fn collect_path(urls: &mut Vec<String>, value: Option<&Value>, path: &[&str]) {
    let Some(mut current) = value else {
        return;
    };
    for key in path {
        let Some(next) = current.get(*key) else {
            return;
        };
        current = next;
    }
    collect_urls(urls, current);
}

fn collect_urls(urls: &mut Vec<String>, value: &Value) {
    match value {
        Value::String(_) => add_value(urls, Some(value)),
        Value::Array(values) => values.iter().for_each(|value| collect_urls(urls, value)),
        Value::Object(map) => collect_object_urls(urls, map),
        _ => {}
    }
}

fn collect_object_urls(urls: &mut Vec<String>, map: &serde_json::Map<String, Value>) {
    for key in ["urlList", "url_list", "urls", "UrlList", "URLList"] {
        if let Some(value) = map.get(key) {
            collect_urls(urls, value);
        }
    }
    for (key, value) in map {
        let key_lower = key.to_lowercase();
        if key_lower.ends_with("addr") || key_lower.ends_with("url") || key == "bitrateInfo" {
            collect_urls(urls, value);
        }
    }
}

fn add_value(urls: &mut Vec<String>, value: Option<&Value>) {
    let Some(Value::String(url)) = value else {
        return;
    };
    if url.starts_with("http") && !urls.contains(url) {
        urls.push(url.clone());
    }
}

fn add_owned(urls: &mut Vec<String>, value: Option<String>) {
    if let Some(url) = value {
        if url.starts_with("http") && !urls.contains(&url) {
            urls.push(url);
        }
    }
}
