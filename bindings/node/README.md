# tiktok-direct

Node.js library for extracting public TikTok video metadata and downloading public media candidates.

## Layout

- `src/`: library implementation and TypeScript declarations
- `tests/`: Node test runner tests
- `examples/`: runnable usage examples

## Usage

```js
import { TikTokExtractor, extract, download } from "tiktok-direct";

const video = await extract("https://www.tiktok.com/@user/video/123");
const path = await download(video.webpage_url, "mp4", "downloads/");

const extractor = new TikTokExtractor();
const metadata = await extractor.extract(video.webpage_url);
```

Supported download kinds: `mp4`, `mp3`, `thumbnail`.

## Validate

```powershell
npm test
```
