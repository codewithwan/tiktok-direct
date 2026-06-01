use serde_json::Value;
use url::form_urlencoded;

use crate::http::TikTokHttpClient;
use crate::Result;

pub fn fetch_oembed(client: &TikTokHttpClient, url: &str) -> Result<Value> {
    let encoded: String = form_urlencoded::byte_serialize(url.as_bytes()).collect();
    let endpoint = format!("https://www.tiktok.com/oembed?url={encoded}");
    let (_, payload) = client.fetch_text(&endpoint, "application/json", None)?;
    Ok(serde_json::from_str(&payload)?)
}
