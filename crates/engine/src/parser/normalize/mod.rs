mod analytics;
mod fields;
mod maps;

use serde_json::Value;
use std::collections::BTreeMap;

use crate::models::{ExtractionQuality, VideoMetadata};
use crate::parser::html::UrlParts;
use crate::Result;
use analytics::{analytics, summary};
use fields::{fill, string_field, u64_field};
use maps::{
    map_author_stats, map_hashtags, map_location, map_media, map_mentions, map_music, map_stats,
    map_status_flags, pick_url, raw_keys,
};

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
    let stats_v2 = object
        .and_then(|o| o.get("statsV2"))
        .unwrap_or(&Value::Null);
    let video = object.and_then(|o| o.get("video")).unwrap_or(&Value::Null);
    let music = object.and_then(|o| o.get("music")).unwrap_or(&Value::Null);
    let author_stats = object
        .and_then(|o| o.get("authorStats").or_else(|| o.get("authorStatsV2")))
        .or_else(|| author.get("stats"))
        .unwrap_or(&Value::Null);

    let mut metadata = VideoMetadata {
        input_url: input_url.to_string(),
        resolved_url: Some(final_url.to_string()),
        host: parts.host,
        pathname: parts.pathname,
        video_id: string_field(item.as_ref(), "id").or(parts.video_id),
        username: parts.username,
        author_name: string_field(Some(author), "nickname"),
        author_url: None,
        author_unique_id: string_field(Some(author), "uniqueId"),
        author_avatar_url: pick_url(author.get("avatarLarger"))
            .or_else(|| pick_url(author.get("avatarMedium")))
            .or_else(|| pick_url(author.get("avatarThumb"))),
        author_stats: map_author_stats(author_stats),
        title: string_field(item.as_ref(), "desc"),
        description: string_field(item.as_ref(), "desc"),
        hashtags: map_hashtags(item.as_ref()),
        mentions: map_mentions(item.as_ref()),
        image: pick_url(video.get("cover")),
        thumbnail_url: pick_url(video.get("cover")),
        view_count: u64_field(stats, "playCount").or_else(|| u64_field(stats_v2, "playCount")),
        like_count: u64_field(stats, "diggCount").or_else(|| u64_field(stats_v2, "diggCount")),
        repost_count: u64_field(stats, "shareCount").or_else(|| u64_field(stats_v2, "shareCount")),
        comment_count: u64_field(stats, "commentCount")
            .or_else(|| u64_field(stats_v2, "commentCount")),
        duration: u64_field(video, "duration"),
        timestamp: string_field(item.as_ref(), "createTime"),
        uploader: string_field(Some(author), "uniqueId"),
        uploader_id: string_field(Some(author), "id"),
        webpage_url: Some(final_url.to_string()),
        canonical: None,
        source,
        quality: ExtractionQuality::Failed,
        reason: None,
        missing_fields: Vec::new(),
        challenge_solved: false,
        stats: map_stats(stats),
        stats_v2: map_stats(stats_v2),
        status_flags: map_status_flags(item.as_ref()),
        location: map_location(item.as_ref()),
        analytics: BTreeMap::new(),
        summary: BTreeMap::new(),
        media: map_media(video),
        music: map_music(music),
        available_json_sources: BTreeMap::new(),
        raw_item_keys: raw_keys(item.as_ref()),
        raw_item: item,
    };
    finalize_metadata(&mut metadata);
    Ok(metadata)
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

pub fn finalize_metadata(metadata: &mut VideoMetadata) {
    metadata.quality = evaluate_quality(metadata);
    metadata.missing_fields = missing_fields(metadata);
    metadata.reason = reason(metadata);
    metadata.analytics = analytics(metadata);
    metadata.summary = summary(metadata);
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

fn missing_fields(metadata: &VideoMetadata) -> Vec<String> {
    let mut missing = Vec::new();
    if metadata.video_id.is_none() {
        missing.push("video_id");
    }
    if metadata.title.is_none() {
        missing.push("title");
    }
    if metadata.author_unique_id.is_none() && metadata.author_name.is_none() {
        missing.push("author");
    }
    if metadata.view_count.is_none() {
        missing.push("views");
    }
    if metadata.like_count.is_none() {
        missing.push("likes");
    }
    if metadata.repost_count.is_none() {
        missing.push("shares");
    }
    if metadata.comment_count.is_none() {
        missing.push("comments");
    }
    if metadata.duration.is_none() {
        missing.push("duration");
    }
    missing.into_iter().map(str::to_string).collect()
}

fn reason(metadata: &VideoMetadata) -> Option<String> {
    if metadata.source.is_none() && metadata.title.is_some() {
        Some("oembed_fallback_only".to_string())
    } else if metadata.source.is_none() {
        Some("video_item_not_found".to_string())
    } else if !metadata.missing_fields.is_empty() {
        Some("missing_baseline_fields".to_string())
    } else {
        None
    }
}

#[cfg(test)]
mod tests;
