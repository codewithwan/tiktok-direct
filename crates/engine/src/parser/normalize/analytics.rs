use serde_json::Value;
use std::collections::BTreeMap;

use crate::models::VideoMetadata;

pub fn analytics(metadata: &VideoMetadata) -> BTreeMap<String, Value> {
    let engagement = sum_options(&[
        metadata.like_count,
        metadata.comment_count,
        metadata.repost_count,
    ]);
    BTreeMap::from([
        ("engagement_count".to_string(), engagement.into()),
        (
            "engagement_rate".to_string(),
            rate(engagement, metadata.view_count).into(),
        ),
        (
            "like_rate".to_string(),
            rate(metadata.like_count, metadata.view_count).into(),
        ),
        (
            "comment_rate".to_string(),
            rate(metadata.comment_count, metadata.view_count).into(),
        ),
        (
            "share_rate".to_string(),
            rate(metadata.repost_count, metadata.view_count).into(),
        ),
    ])
}

pub fn summary(metadata: &VideoMetadata) -> BTreeMap<String, Value> {
    BTreeMap::from([
        ("video_id".to_string(), metadata.video_id.clone().into()),
        ("url".to_string(), metadata.webpage_url.clone().into()),
        (
            "author".to_string(),
            metadata.author_unique_id.clone().into(),
        ),
        ("title".to_string(), metadata.title.clone().into()),
        ("views".to_string(), metadata.view_count.into()),
        ("likes".to_string(), metadata.like_count.into()),
        ("comments".to_string(), metadata.comment_count.into()),
        ("shares".to_string(), metadata.repost_count.into()),
        ("duration".to_string(), metadata.duration.into()),
        (
            "quality".to_string(),
            format!("{:?}", metadata.quality).to_lowercase().into(),
        ),
    ])
}

fn sum_options(values: &[Option<u64>]) -> Option<u64> {
    values
        .iter()
        .try_fold(0_u64, |sum, value| value.map(|value| sum + value))
}

fn rate(numerator: Option<u64>, denominator: Option<u64>) -> Option<f64> {
    match (numerator, denominator) {
        (Some(numerator), Some(denominator)) if denominator > 0 => {
            Some(numerator as f64 / denominator as f64)
        }
        _ => None,
    }
}
