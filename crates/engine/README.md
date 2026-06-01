# TikTok Direct Engine

This is the core Rust engine for `tiktok-direct`. It contains all the shared logic for resolving public TikTok URLs, solving challenges, parsing metadata, normalizing results, and downloading media.

## Architecture & Structure

The engine is built around a modern, highly modular subsystem design:

* **`net/` (Networking Layer)**:
  * `client.rs`: Manages high-performance HTTP transport (`reqwest`), custom header injection, and cookie containers.
  * `ua.rs`: Dynamically constructs organic, WAF-evading browser user-agent profiles and platform client hints based on system entropy.
  * `challenge.rs`: Solves native public TikTok Web Application Firewall (WAF) cookie challenges asynchronously.
  * `oembed.rs`: Consumes the public oEmbed endpoints as a reliable fallback when pages are severely restricted.
* **`parser/` (Parsing & Extraction Layer)**:
  * `html.rs`: Heavy-duty HTML scanner extracting rehydration JSON states (`SIGI_STATE`, `__UNIVERSAL_DATA_FOR_REHYDRATION__`, `__NEXT_DATA__`).
  * `normalize/`: Translates raw unstructured JSON values into clean, standardized types and evaluates extraction completeness metrics.
* **Core API**:
  * `extractor.rs`: Primary orchestration pipeline managing requests, WAF solving, extraction, and fallback mechanisms.
  * `download.rs`: Media candidate resolution and parallel mp3/mp4 file downloads.
  * `models.rs`: Strongly typed representation of `VideoMetadata`, `BrowserProfile`, and validation flags.
  * `error.rs`: Strongly typed `TikTokDirectError` enum mappings.

## Rust Usage Example

Add the crate to your `Cargo.toml`:

```toml
[dependencies]
tiktok-direct-engine = { package = "engine", path = "path/to/crates/engine" }
```

And run:

```rust
use std::path::Path;
use tiktok_direct_engine::{TikTokExtractor, MediaKind, download_media};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 1. Initialize the extractor
    let extractor = TikTokExtractor::new();

    // 2. Extract video metadata
    let url = "https://vt.tiktok.com/ZSxvYRvoR/";
    let metadata = extractor.extract(url)?;

    if let Some(title) = &metadata.title {
        println!("Video Title: {}", title);
    }
    if let Some(username) = &metadata.username {
        println!("Author: {}", username);
    }

    // 3. Download MP4 media
    let output_dir = Some(Path::new("downloads"));
    let filepath = download_media(&metadata, MediaKind::Mp4, output_dir)?;
    println!("Downloaded MP4 to: {:?}", filepath);

    Ok(())
}
```

## Running Engine Tests

To run the engine unit and integration tests, run from the workspace root or engine directory:

```powershell
cargo test -p engine
```
