# Node.js Binding

Planned binding for the Rust core.

Preferred stack:

- napi-rs
- TypeScript definitions

Target API:

```js
import { TikTokExtractor } from "tiktok-direct";

const video = await new TikTokExtractor().extract("https://vt.tiktok.com/example/");
console.log(video.view_count);
```

