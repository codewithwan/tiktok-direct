# TikTok Direct Engine

This is the core Rust engine for `tiktok-direct`. It contains all the shared logic for resolving public TikTok URLs, solving challenges, parsing metadata, normalizing results, and downloading media.

## Architecture & Structure

The engine is highly modular:

- `challenge.rs`: Solves public web challenge pages and manages cookies.
- `download.rs`: Handles listing and downloading of candidate media URLs (MP3 and MP4).
- `extractor.rs`: Orchestrates the main extraction workflow.
- `http.rs`: Internal HTTP client with custom user-agent rotation and cookie management.
- `models.rs`: Defines strongly typed metadata schemas, extraction configurations, and browser profile options.
- `normalize/`: Standardizes raw API fields into typed structs.
- `oembed.rs`: Integrates oEmbed endpoint fallback if standard pages fail to resolve.
- `parsing.rs`: Extracts and parses raw JSON structures from page rehydration.

## Rust Usage Example

Add the crate to your `Cargo.toml`:

```toml
[dependencies]
tiktok-direct-engine = { package = "engine", path = "path/to/crates/engine" }
```

And run:

```rust
use tiktok_direct_engine::{TikTokExtractor, MediaKind, download_media};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 1. Initialize the extractor
    let extractor = TikTokExtractor::new();

    // 2. Extract video metadata
    let url = "https://vt.tiktok.com/ZSxvYRvoR/";
    let metadata = extractor.extract(url)?;

    println!("Video Title: {}", metadata.title);
    println!("Author: {}", metadata.author.username);

    // 3. Download MP4 media
    let output_dir = Some(std::path::Path::new("downloads"));
    let filepath = download_media(&metadata, MediaKind::Mp4, output_dir)?;
    println!("Downloaded MP4 to: {:?}", filepath);

    Ok(())
}
```

## Running Engine Tests

To run the engine unit and integration tests, run from the workspace root:

```powershell
cargo test -p engine
```
