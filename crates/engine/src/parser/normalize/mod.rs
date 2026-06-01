mod fields;
mod maps;

use serde_json::Value;
use std::collections::BTreeMap;

use crate::models::{ExtractionQuality, VideoMetadata};
use crate::parser::html::UrlParts;
use crate::Result;
use fields::{fill, string_field, u64_field};
use maps::{map_media, map_music, map_stats, pick_url, raw_keys};

pub fn normalize_item(
    input_url: &str,
    final_url: &str,
    parts: UrlParts,
    source: Option<String>,
    item: Option<&Value>,
) -> Result<VideoMetadata> {
    let item = item.cloned();
    let object = item.as_ref().and_then(Value::as_object);
    let author = object.and_then(|o| o.get("author")).unwrap_or(&Value::Null);
    let stats = object.and_then(|o| o.get("stats")).unwrap_or(&Value::Null);
    let video = object.and_then(|o| o.get("video")).unwrap_or(&Value::Null);
    let music = object.and_then(|o| o.get("music")).unwrap_or(&Value::Null);

    Ok(VideoMetadata {
        input_url: input_url.to_string(),
        resolved_url: Some(final_url.to_string()),
        host: parts.host,
        pathname: parts.pathname,
        video_id: string_field(item.as_ref(), "id").or(parts.video_id),
        username: parts.username,
        author_name: string_field(Some(author), "nickname"),
        author_url: None,
        author_unique_id: string_field(Some(author), "uniqueId"),
        title: string_field(item.as_ref(), "desc"),
        description: string_field(item.as_ref(), "desc"),
        image: pick_url(video.get("cover")),
        thumbnail_url: pick_url(video.get("cover")),
        view_count: u64_field(stats, "playCount"),
        like_count: u64_field(stats, "diggCount"),
        repost_count: u64_field(stats, "shareCount"),
        comment_count: u64_field(stats, "commentCount"),
        duration: u64_field(video, "duration"),
        timestamp: string_field(item.as_ref(), "createTime"),
        uploader: string_field(Some(author), "uniqueId"),
        uploader_id: string_field(Some(author), "id"),
        webpage_url: Some(final_url.to_string()),
        canonical: None,
        source,
        quality: ExtractionQuality::Failed,
        challenge_solved: false,
        stats: map_stats(stats),
        media: map_media(video),
        music: map_music(music),
        available_json_sources: BTreeMap::new(),
        raw_item_keys: raw_keys(item.as_ref()),
        raw_item: item,
    })
}

pub fn merge_oembed(metadata: &mut VideoMetadata, oembed: &Value) {
    fill(
        &mut metadata.author_name,
        string_field(Some(oembed), "author_name"),
    );
    fill(
        &mut metadata.author_url,
        string_field(Some(oembed), "author_url"),
    );
    fill(&mut metadata.title, string_field(Some(oembed), "title"));
    fill(&mut metadata.description, metadata.title.clone());
    fill(
        &mut metadata.image,
        string_field(Some(oembed), "thumbnail_url"),
    );
    fill(
        &mut metadata.thumbnail_url,
        string_field(Some(oembed), "thumbnail_url"),
    );
    fill(&mut metadata.author_unique_id, metadata.username.clone());
    fill(&mut metadata.uploader, metadata.author_unique_id.clone());
}

pub fn evaluate_quality(metadata: &VideoMetadata) -> ExtractionQuality {
    let has_stats = metadata.view_count.is_some()
        || metadata.like_count.is_some()
        || metadata.repost_count.is_some()
        || metadata.comment_count.is_some();
    if metadata.source.is_some() && has_stats && metadata.duration.is_some() {
        ExtractionQuality::Complete
    } else if metadata.title.is_some() || metadata.thumbnail_url.is_some() {
        ExtractionQuality::Partial
    } else {
        ExtractionQuality::Failed
    }
}

#[cfg(test)]
mod tests;
