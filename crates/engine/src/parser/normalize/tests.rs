use crate::parser::html::UrlParts;
use crate::parser::normalize::normalize_item;

#[test]
fn normalizes_core_stats_and_media() {
    let item = serde_json::json!({
        "id": "123",
        "desc": "hello",
        "createTime": "1700000000",
        "author": {
            "id": "u1",
            "uniqueId": "tester",
            "nickname": "Tester",
            "avatarThumb": {"urlList": ["https://avatar"]}
        },
        "authorStats": {"followerCount": "100", "followingCount": 5, "heartCount": 200, "videoCount": 7, "diggCount": 9},
        "stats": {"playCount": 10, "diggCount": 2, "shareCount": 1, "commentCount": 3},
        "statsV2": {"playCount": "11"},
        "challenges": [{"title": "rust"}],
        "textExtra": [
            {"hashtagName": "tiktok"},
            {"userUniqueId": "friend", "displayText": "@friend", "userId": "u2"}
        ],
        "isAd": false,
        "isOriginal": true,
        "locationCreated": "ID",
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
    assert_eq!(
        metadata.author_avatar_url.as_deref(),
        Some("https://avatar")
    );
    assert_eq!(metadata.author_stats.follower_count, Some(100));
    assert_eq!(metadata.hashtags, vec!["rust", "tiktok"]);
    assert_eq!(metadata.mentions[0].username.as_deref(), Some("friend"));
    assert_eq!(metadata.status_flags.get("isOriginal"), Some(&true));
    assert_eq!(metadata.location.get("locationCreated").unwrap(), "ID");
    assert_eq!(metadata.missing_fields, Vec::<String>::new());
    assert_eq!(metadata.reason, None);
    assert_eq!(metadata.analytics.get("engagement_count").unwrap(), 6);
    assert!(metadata.raw_item_keys.contains(&"author".to_string()));
}
