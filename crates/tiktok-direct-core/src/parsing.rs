use html_escape::decode_html_entities;
use regex::Regex;
use serde_json::Value;
use std::collections::BTreeMap;
use url::Url;

#[derive(Debug, Clone)]
pub struct UrlParts {
    pub host: Option<String>,
    pub pathname: Option<String>,
    pub video_id: Option<String>,
    pub username: Option<String>,
}

#[derive(Debug, Clone)]
pub struct PageJsonSources {
    pub sigi_state: Option<Value>,
    pub universal_data: Option<Value>,
    pub next_data: Option<Value>,
}

impl PageJsonSources {
    pub fn parse(html: &str) -> Self {
        Self {
            sigi_state: json_script(html, "SIGI_STATE"),
            universal_data: json_script(html, "__UNIVERSAL_DATA_FOR_REHYDRATION__"),
            next_data: json_script(html, "__NEXT_DATA__"),
        }
    }

    pub fn available(&self) -> BTreeMap<String, bool> {
        BTreeMap::from([
            ("SIGI_STATE".to_string(), self.sigi_state.is_some()),
            (
                "__UNIVERSAL_DATA_FOR_REHYDRATION__".to_string(),
                self.universal_data.is_some(),
            ),
            ("__NEXT_DATA__".to_string(), self.next_data.is_some()),
        ])
    }
}

pub fn parse_url_parts(input_url: &str, resolved_url: &str) -> UrlParts {
    let parsed = Url::parse(resolved_url).ok();
    UrlParts {
        host: parsed
            .as_ref()
            .and_then(|url| url.host_str().map(str::to_string)),
        pathname: parsed.as_ref().map(|url| url.path().to_string()),
        video_id: video_id(resolved_url).or_else(|| video_id(input_url)),
        username: username(resolved_url).or_else(|| username(input_url)),
    }
}

pub fn select_item<'a>(
    sources: &'a PageJsonSources,
    video_id: Option<&str>,
) -> (Option<String>, Option<&'a Value>) {
    for (name, source) in [
        ("SIGI_STATE", sources.sigi_state.as_ref()),
        (
            "__UNIVERSAL_DATA_FOR_REHYDRATION__",
            sources.universal_data.as_ref(),
        ),
        ("__NEXT_DATA__", sources.next_data.as_ref()),
    ] {
        if let Some(item) = source.and_then(|value| find_item(value, video_id)) {
            return (Some(name.to_string()), Some(item));
        }
    }
    (None, None)
}

fn json_script(html: &str, id: &str) -> Option<Value> {
    let pattern = format!(
        r#"(?is)<script[^>]+id=["']{}["'][^>]*>(.*?)</script>"#,
        regex::escape(id)
    );
    let raw = Regex::new(&pattern).ok()?.captures(html)?.get(1)?.as_str();
    serde_json::from_str(&decode_html_entities(raw)).ok()
}

fn find_item<'a>(value: &'a Value, video_id: Option<&str>) -> Option<&'a Value> {
    if let Some(module) = value.get("ItemModule").and_then(Value::as_object) {
        if let Some(id) = video_id {
            if let Some(item) = module.get(id) {
                return Some(item);
            }
        }
        return module.values().next();
    }

    for candidate in walk_objects(value) {
        if let Some(item_struct) = candidate.get("itemStruct") {
            if is_matching_item(item_struct, video_id) {
                return Some(item_struct);
            }
        }
        if is_matching_item(candidate, video_id) {
            return Some(candidate);
        }
    }
    None
}

fn is_matching_item(value: &Value, video_id: Option<&str>) -> bool {
    let id = value
        .get("id")
        .or_else(|| value.get("awemeId"))
        .and_then(Value::as_str);
    let has_shape = ["video", "stats", "author", "desc"]
        .iter()
        .any(|key| value.get(key).is_some());
    has_shape && id.is_some_and(|id| video_id.is_none_or(|wanted| wanted == id))
}

fn walk_objects(value: &Value) -> Vec<&Value> {
    let mut values = Vec::new();
    if value.is_object() {
        values.push(value);
        for child in value.as_object().unwrap().values() {
            values.extend(walk_objects(child));
        }
    } else if let Some(items) = value.as_array() {
        for child in items {
            values.extend(walk_objects(child));
        }
    }
    values
}

fn video_id(url: &str) -> Option<String> {
    Regex::new(r"/(?:video|photo)/(\d+)")
        .ok()?
        .captures(url)?
        .get(1)
        .map(|m| m.as_str().to_string())
}

fn username(url: &str) -> Option<String> {
    Regex::new(r"tiktok\.com/@([^/?#]+)")
        .ok()?
        .captures(url)?
        .get(1)
        .map(|m| m.as_str().to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_video_id_and_username() {
        let parts = parse_url_parts("", "https://www.tiktok.com/@rainzy/video/123456");
        assert_eq!(parts.video_id.as_deref(), Some("123456"));
        assert_eq!(parts.username.as_deref(), Some("rainzy"));
    }
}
