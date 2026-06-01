use thiserror::Error;

pub type Result<T> = std::result::Result<T, TikTokDirectError>;

#[derive(Debug, Error)]
pub enum TikTokDirectError {
    #[error("network error: {0}")]
    Network(String),

    #[error("http error {status}: {message}")]
    Http { status: u16, message: String },

    #[error("challenge error: {0}")]
    Challenge(String),

    #[error("parse error: {0}")]
    Parse(String),

    #[error("unsupported feature: {0}")]
    Unsupported(String),

    #[error("media download error: {0}")]
    MediaDownload(String),
}

impl From<reqwest::Error> for TikTokDirectError {
    fn from(value: reqwest::Error) -> Self {
        Self::Network(value.to_string())
    }
}

impl From<serde_json::Error> for TikTokDirectError {
    fn from(value: serde_json::Error) -> Self {
        Self::Parse(value.to_string())
    }
}

impl From<std::io::Error> for TikTokDirectError {
    fn from(value: std::io::Error) -> Self {
        Self::MediaDownload(value.to_string())
    }
}
