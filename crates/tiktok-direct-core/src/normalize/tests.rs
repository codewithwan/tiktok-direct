use crate::normalize::normalize_item;
use crate::parsing::UrlParts;

#[test]
fn normalizes_core_stats_and_media() {
    let item = serde_json::json!({
        "id": "123",
        "desc": "hello",
        "createTime": "1700000000",
        "author": {"id": "u1", "uniqueId": "tester", "nickname": "Tester"},
        "stats": {"playCount": 10, "diggCount": 2, "shareCount": 1, "commentCount": 3},
        "video": {"duration": 7, "cover": "https://img", "playAddr": ["https://video"]},
        "music": {"id": "m1", "title": "sound", "authorName": "Tester", "playUrl": "https://audio"}
    });
    let parts = UrlParts {
        host: Some("www.tiktok.com".to_string()),
        pathname: Some("/@tester/video/123".to_string()),
        video_id: Some("123".to_string()),
        username: Some("tester".to_string()),
    };
    let metadata = normalize_item(
        "input",
        "resolved",
        parts,
        Some("fixture".to_string()),
        Some(&item),
    )
    .unwrap();
    assert_eq!(metadata.view_count, Some(10));
    assert_eq!(metadata.like_count, Some(2));
    assert_eq!(metadata.duration, Some(7));
    assert_eq!(metadata.media.get("play_addr").unwrap(), "https://video");
    assert!(metadata.raw_item_keys.contains(&"author".to_string()));
}
