use std::fs;
use std::path::{Path, PathBuf};
use std::time::Duration;

use reqwest::blocking::Client;
use reqwest::header::{ACCEPT, ACCEPT_LANGUAGE, RANGE, REFERER, USER_AGENT};

use crate::{BrowserProfile, Result, TikTokDirectError, VideoMetadata};

mod urls;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MediaKind {
    Mp4,
    Mp3,
    Thumbnail,
    Avatar,
}

impl MediaKind {
    pub fn parse(value: &str) -> Result<Self> {
        match value {
            "mp4" => Ok(Self::Mp4),
            "mp3" => Ok(Self::Mp3),
            "thumbnail" => Ok(Self::Thumbnail),
            "avatar" => Ok(Self::Avatar),
            other => Err(TikTokDirectError::Unsupported(format!(
                "media kind must be mp4, mp3, thumbnail, or avatar, got {other}"
            ))),
        }
    }

    fn extension(self) -> &'static str {
        match self {
            Self::Mp4 => "mp4",
            Self::Mp3 => "mp3",
            Self::Thumbnail | Self::Avatar => "jpg",
        }
    }
}

pub use urls::list_media_urls;

pub fn download_media(
    metadata: &VideoMetadata,
    kind: MediaKind,
    output: Option<&Path>,
) -> Result<PathBuf> {
    download_media_with_profile(metadata, kind, output, &BrowserProfile::default(), 60)
}

pub fn download_media_with_profile(
    metadata: &VideoMetadata,
    kind: MediaKind,
    output: Option<&Path>,
    profile: &BrowserProfile,
    timeout_seconds: u64,
) -> Result<PathBuf> {
    let urls = list_media_urls(metadata, kind);
    if urls.is_empty() {
        return Err(TikTokDirectError::MediaDownload(format!(
            "no {} URL is available",
            kind.extension()
        )));
    }

    let output_path = resolve_output_path(metadata, kind, output)?;
    let client = Client::builder()
        .timeout(Duration::from_secs(timeout_seconds))
        .redirect(reqwest::redirect::Policy::limited(10))
        .build()?;
    let mut last_error = None;

    for url in urls {
        match download_one(&client, profile, metadata, &url, &output_path) {
            Ok(()) => return Ok(output_path),
            Err(err) => {
                let _ = fs::remove_file(&output_path);
                last_error = Some(err.to_string());
            }
        }
    }

    Err(TikTokDirectError::MediaDownload(last_error.unwrap_or_else(
        || "all media candidates failed".to_string(),
    )))
}

fn resolve_output_path(
    metadata: &VideoMetadata,
    kind: MediaKind,
    output: Option<&Path>,
) -> Result<PathBuf> {
    let mut path = output
        .map(Path::to_path_buf)
        .unwrap_or_else(|| default_filename(metadata, kind).into());
    if path.exists() && path.is_dir() {
        path = path.join(default_filename(metadata, kind));
    }
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }
    Ok(path)
}

fn default_filename(metadata: &VideoMetadata, kind: MediaKind) -> String {
    let username = metadata.username.as_deref().unwrap_or("unknown");
    let video_id = metadata.video_id.as_deref().unwrap_or("tiktok");
    format!("{username}_{video_id}.{}", kind.extension())
}

fn download_one(
    client: &Client,
    profile: &BrowserProfile,
    metadata: &VideoMetadata,
    url: &str,
    output: &Path,
) -> Result<()> {
    let mut request = client
        .get(url)
        .header(USER_AGENT, &profile.user_agent)
        .header(ACCEPT, "*/*")
        .header(ACCEPT_LANGUAGE, &profile.accept_language)
        .header(RANGE, "bytes=0-");
    if let Some(referer) = &metadata.webpage_url {
        request = request.header(REFERER, referer);
    }

    let response = request.send()?;
    if !response.status().is_success() {
        return Err(TikTokDirectError::MediaDownload(format!(
            "media request failed with status {}",
            response.status()
        )));
    }

    let bytes = response.bytes()?;
    fs::write(output, &bytes)?;
    if bytes.is_empty() {
        return Err(TikTokDirectError::MediaDownload(
            "downloaded media is empty".to_string(),
        ));
    }
    Ok(())
}
