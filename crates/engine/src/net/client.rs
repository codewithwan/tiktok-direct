use reqwest::blocking::Client;
use reqwest::header::{HeaderMap, HeaderValue, ACCEPT, ACCEPT_LANGUAGE, COOKIE, USER_AGENT};
use std::time::Duration;

use crate::{BrowserProfile, Result, TikTokDirectError};

#[derive(Debug, Clone)]
pub struct TikTokHttpClient {
    client: Client,
    profile: BrowserProfile,
}

impl TikTokHttpClient {
    pub fn new(profile: BrowserProfile, timeout_seconds: u64) -> Result<Self> {
        let client = Client::builder()
            .timeout(Duration::from_secs(timeout_seconds))
            .redirect(reqwest::redirect::Policy::limited(10))
            .build()?;
        Ok(Self { client, profile })
    }

    pub fn fetch_text(
        &self,
        url: &str,
        accept: &str,
        cookie: Option<&str>,
    ) -> Result<(String, String)> {
        let mut headers = self.browser_headers(accept)?;
        if let Some(cookie) = cookie {
            headers.insert(COOKIE, header_value(cookie)?);
        }

        let response = self.client.get(url).headers(headers).send()?;
        let final_url = response.url().to_string();
        let status = response.status();
        if !status.is_success() {
            return Err(TikTokDirectError::Http {
                status: status.as_u16(),
                message: status.canonical_reason().unwrap_or("unknown").to_string(),
            });
        }

        Ok((final_url, response.text()?))
    }

    fn browser_headers(&self, accept: &str) -> Result<HeaderMap> {
        let mut headers = HeaderMap::new();
        headers.insert(USER_AGENT, header_value(&self.profile.user_agent)?);
        headers.insert(ACCEPT, header_value(accept)?);
        headers.insert(
            ACCEPT_LANGUAGE,
            header_value(&self.profile.accept_language)?,
        );
        if let Some(ref sec_ch_ua) = self.profile.sec_ch_ua {
            headers.insert("sec-ch-ua", header_value(sec_ch_ua)?);
        }
        if let Some(ref sec_ch_ua_platform) = self.profile.sec_ch_ua_platform {
            headers.insert("sec-ch-ua-platform", header_value(sec_ch_ua_platform)?);
        }
        Ok(headers)
    }
}

fn header_value(value: &str) -> Result<HeaderValue> {
    HeaderValue::from_str(value).map_err(|err| TikTokDirectError::Network(err.to_string()))
}
