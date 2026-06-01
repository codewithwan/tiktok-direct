use tiktok_direct_core::{download_media, ExtractionQuality, MediaKind, TikTokExtractor};

#[test]
#[ignore = "hits live public TikTok page"]
fn extracts_public_short_url() {
    let metadata = TikTokExtractor::new()
        .extract("https://vt.tiktok.com/ZSxvYRvoR/")
        .unwrap();

    assert_eq!(metadata.quality, ExtractionQuality::Complete);
    assert!(metadata.video_id.is_some());
    assert!(metadata.view_count.is_some());
    assert!(metadata.like_count.is_some());
    assert!(metadata.duration.is_some());
}

#[test]
#[ignore = "hits live public TikTok page and downloads media"]
fn downloads_public_mp4_and_mp3() {
    let metadata = TikTokExtractor::new()
        .extract("https://vt.tiktok.com/ZSxvYRvoR/")
        .unwrap();
    let dir = std::env::temp_dir().join("tiktok-direct-live-download-test");
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).unwrap();

    let mp4 = download_media(&metadata, MediaKind::Mp4, Some(&dir)).unwrap();
    let mp3 = download_media(&metadata, MediaKind::Mp3, Some(&dir)).unwrap();
    let mp4_bytes = std::fs::read(&mp4).unwrap();

    assert!(mp4.metadata().unwrap().len() > 0);
    assert!(mp3.metadata().unwrap().len() > 0);
    assert!(mp4_bytes.windows(4).any(|window| window == b"vide"));
    assert!(mp4_bytes.windows(4).any(|window| window == b"soun"));
}
