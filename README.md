# tiktok-direct

Library-first TikTok public video metadata extractor.

This repository is a multi-language library workspace with a shared Rust core
and planned thin language bindings. There is no CLI target in this repo by
design.

## Goals

- Provide one shared Rust core for extraction, parsing, normalization, and media
  URL discovery.
- Expose bindings for Python, Node.js, and Go.
- Keep behavior aligned across all language packages.
- Use only public TikTok page data and public embed data.
- Avoid login, user account cookies, and authenticated scraping.

## Repository Layout

```text
tiktok-direct/
  crates/
    tiktok-direct-core/      Rust core library implementation
  bindings/
    python/                  Python binding with PyO3 and maturin
    node/                    Future Node.js binding
    go/                      Future Go binding
  TODO.md                    MVP checklist and roadmap
```

## Reference Implementation

The current Python prototype has been kept outside this folder as:

```text
tiktok-direct-python-reference/
```

Use it as a behavior reference while extending the Rust core and bindings.

## Current Status

- Rust core extracts public TikTok video metadata.
- Public web challenge handling is implemented.
- Rehydration JSON parsing is implemented.
- Public oEmbed fallback is implemented.
- MP4 and MP3 download from extracted public media URLs is implemented.
- Python binding is implemented.
- Unit tests, line-count tests, and an ignored live public URL test are present.

## Python Binding

Build the wheel:

```powershell
cd tiktok-direct\bindings\python
python -m maturin build --release
```

Use it from Python:

```python
from tiktok_direct import TikTokExtractor, download, extract

video = TikTokExtractor().extract("https://vt.tiktok.com/ZSxvYRvoR/")
same_video = extract("https://vt.tiktok.com/ZSxvYRvoR/")
print(video["view_count"], same_video["quality"])

mp4_path = download("https://vt.tiktok.com/ZSxvYRvoR/", "mp4", "downloads")
mp3_path = TikTokExtractor().download("https://vt.tiktok.com/ZSxvYRvoR/", "mp3", "downloads")
```

## Rust API

```rust
use std::path::Path;
use tiktok_direct_core::{download_media, MediaKind, TikTokExtractor};

let metadata = TikTokExtractor::new().extract("https://vt.tiktok.com/ZSxvYRvoR/")?;
let mp4 = download_media(&metadata, MediaKind::Mp4, Some(Path::new("downloads")))?;
let mp3 = download_media(&metadata, MediaKind::Mp3, Some(Path::new("downloads")))?;
```

## Rust Tests

Default tests avoid live network downloads:

```powershell
cargo test
```

Live public extraction and download tests:

```powershell
cargo test --test live_extract -- --ignored
```

## Public Data Policy

- No user login.
- No user-provided account cookies.
- No authenticated scraping.
- Public TikTok page data and public oEmbed data only.
- Temporary public challenge cookies may be generated from the public page
  response when TikTok serves a public web challenge.
