use crate::net::{fetch_oembed, solve_waf_cookie, TikTokHttpClient};
use crate::parser::{
    finalize_metadata, merge_oembed, normalize_item, parse_url_parts, select_item, PageJsonSources,
};
use crate::{BrowserProfile, ExtractionQuality, Result, VideoMetadata};

#[derive(Debug, Clone)]
pub struct TikTokExtractor {
    profile: BrowserProfile,
    rotate_profile: bool,
    timeout_seconds: u64,
}

impl TikTokExtractor {
    pub fn new() -> Self {
        Self {
            profile: BrowserProfile::default(),
            rotate_profile: true,
            timeout_seconds: 20,
        }
    }

    pub fn with_profile(profile: BrowserProfile) -> Self {
        Self {
            profile,
            rotate_profile: false,
            timeout_seconds: 20,
        }
    }

    pub fn with_timeout_seconds(mut self, timeout_seconds: u64) -> Self {
        self.timeout_seconds = timeout_seconds;
        self
    }

    pub fn extract(&self, url: &str) -> Result<VideoMetadata> {
        let mut best = None;
        let mut last_error = None;
        for _ in 0..5 {
            let metadata = match self.extract_once(url) {
                Ok(metadata) => metadata,
                Err(err) => {
                    last_error = Some(err);
                    continue;
                }
            };
            if metadata.quality == ExtractionQuality::Complete {
                return Ok(metadata);
            }
            best = Some(metadata);
        }
        if let Some(best) = best {
            Ok(best)
        } else {
            Err(last_error.expect("at least one extraction attempt ran"))
        }
    }

    fn profile_for_attempt(&self) -> BrowserProfile {
        if self.rotate_profile {
            BrowserProfile::random()
        } else {
            self.profile.clone()
        }
    }

    fn extract_once(&self, _url: &str) -> Result<VideoMetadata> {
        let client = TikTokHttpClient::new(self.profile_for_attempt(), self.timeout_seconds)?;
        let (resolved_url, mut html) = client.fetch_text(_url, "text/html", None)?;
        let mut final_url = resolved_url;
        let mut challenge_solved = false;

        if let Some(cookie) = solve_waf_cookie(&html) {
            let fetched = client.fetch_text(&final_url, "text/html", Some(&cookie))?;
            final_url = fetched.0;
            html = fetched.1;
            challenge_solved = true;
        }

        let url_parts = parse_url_parts(_url, &final_url);
        let sources = PageJsonSources::parse(&html);
        let (source, item) = select_item(&sources, url_parts.video_id.as_deref());
        let mut metadata = normalize_item(_url, &final_url, url_parts, source, item)?;

        metadata.challenge_solved = challenge_solved;
        metadata.available_json_sources = sources.available();

        let oembed = fetch_oembed(&client, &final_url)
            .or_else(|_| fetch_oembed(&client, _url))
            .ok();
        if let Some(oembed) = oembed {
            merge_oembed(&mut metadata, &oembed);
        }

        finalize_metadata(&mut metadata);
        Ok(metadata)
    }
}

impl Default for TikTokExtractor {
    fn default() -> Self {
        Self::new()
    }
}
