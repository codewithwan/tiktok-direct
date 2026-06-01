use base64::engine::general_purpose::STANDARD;
use base64::Engine;
use regex::Regex;
use serde_json::Value;
use sha2::{Digest, Sha256};

pub fn solve_waf_cookie(html: &str) -> Option<String> {
    let cookie_name = tag_class(html, "wci")?;
    let payload = tag_class(html, "cs")?;
    let mut challenge: Value = serde_json::from_slice(&decode_base64(&payload)?).ok()?;
    let expected = decode_base64(challenge.pointer("/v/c")?.as_str()?)?;
    let seed = decode_base64(challenge.pointer("/v/a")?.as_str()?)?;

    for number in 0..=1_000_000 {
        let mut hasher = Sha256::new();
        hasher.update(&seed);
        hasher.update(number.to_string().as_bytes());
        if hasher.finalize().as_slice() == expected.as_slice() {
            challenge["d"] = Value::String(STANDARD.encode(number.to_string()));
            let value = STANDARD.encode(serde_json::to_vec(&challenge).ok()?);
            let mut cookies = vec![format!("{cookie_name}={value}")];
            if let (Some(name), Some(value)) = (tag_class(html, "rci"), tag_class(html, "rs")) {
                cookies.push(format!("{name}={value}"));
            }
            return Some(cookies.join("; "));
        }
    }

    None
}

fn decode_base64(value: &str) -> Option<Vec<u8>> {
    let mut padded = value.to_string();
    while padded.len() % 4 != 0 {
        padded.push('=');
    }
    STANDARD.decode(padded).ok()
}

fn tag_class(html: &str, id: &str) -> Option<String> {
    let pattern = format!(
        r#"<[^>]+\bid=["']{}["'][^>]+\bclass=["']([^"']*)["']"#,
        regex::escape(id)
    );
    Regex::new(&pattern)
        .ok()?
        .captures(html)?
        .get(1)
        .map(|m| m.as_str().to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn solves_generated_challenge_cookie() {
        let seed = b"seed";
        let mut hasher = Sha256::new();
        hasher.update(seed);
        hasher.update(b"42");
        let challenge = serde_json::json!({
            "v": {"a": STANDARD.encode(seed), "b": 0, "c": STANDARD.encode(hasher.finalize())},
            "s": STANDARD.encode("sig")
        });
        let payload = STANDARD.encode(serde_json::to_vec(&challenge).unwrap());
        let html =
            format!(r#"<p id="wci" class="_wafchallengeid"></p><p id="cs" class="{payload}"></p>"#);
        let cookie = solve_waf_cookie(&html).unwrap();
        assert!(cookie.starts_with("_wafchallengeid="));
    }
}
