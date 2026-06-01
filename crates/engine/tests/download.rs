use std::collections::BTreeMap;

use serde_json::json;
use tiktok_direct_engine::{list_media_urls, ExtractionQuality, MediaKind, VideoMetadata};

#[test]
fn lists_mp4_candidates_from_normalized_and_raw_fields() {
    let metadata = fixture_metadata(
        BTreeMap::from([
            ("play_addr".to_string(), json!("https://cdn/video.mp4")),
            ("download_addr".to_string(), json!("https://cdn/video.mp4")),
        ]),
        BTreeMap::new(),
        json!({
            "video": {
                "bitrateInfo": [
                    {"playAddr": {"urlList": ["https://cdn/best.mp4"]}}
                ]
            }
        }),
    );

    assert_eq!(
        list_media_urls(&metadata, MediaKind::Mp4),
        vec!["https://cdn/video.mp4", "https://cdn/best.mp4"]
    );
}

#[test]
fn lists_mp3_candidates_from_music_fields() {
    let metadata = fixture_metadata(
        BTreeMap::new(),
        BTreeMap::from([("play_url".to_string(), json!("https://cdn/audio.mp3"))]),
        json!({
            "music": {
                "playUrl": {"urlList": ["https://cdn/audio-backup.mp3"]}
            }
        }),
    );

    assert_eq!(
        list_media_urls(&metadata, MediaKind::Mp3),
        vec!["https://cdn/audio.mp3", "https://cdn/audio-backup.mp3"]
    );
}

fn fixture_metadata(
    media: BTreeMap<String, serde_json::Value>,
    music: BTreeMap<String, serde_json::Value>,
    raw_item: serde_json::Value,
) -> VideoMetadata {
    VideoMetadata {
        input_url: "https://vt.tiktok.com/test/".to_string(),
        resolved_url: Some("https://www.tiktok.com/@rainzy/video/123".to_string()),
        host: Some("www.tiktok.com".to_string()),
        pathname: Some("/@rainzy/video/123".to_string()),
        video_id: Some("123".to_string()),
        username: Some("rainzy".to_string()),
        author_name: Some("Rainzy".to_string()),
        author_url: None,
        author_unique_id: Some("rainzy".to_string()),
        title: Some("caption".to_string()),
        description: Some("caption".to_string()),
        image: None,
        view_count: Some(10),
        like_count: Some(2),
        repost_count: Some(1),
        comment_count: Some(1),
        duration: Some(9),
        timestamp: Some("1".to_string()),
        uploader: Some("rainzy".to_string()),
        uploader_id: Some("author-id".to_string()),
        webpage_url: Some("https://www.tiktok.com/@rainzy/video/123".to_string()),
        canonical: None,
        thumbnail_url: None,
        source: Some("fixture".to_string()),
        quality: ExtractionQuality::Complete,
        challenge_solved: false,
        stats: BTreeMap::new(),
        media,
        music,
        available_json_sources: BTreeMap::new(),
        raw_item_keys: Vec::new(),
        raw_item: Some(raw_item),
    }
}
