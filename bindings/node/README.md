# Node Binding for TikTok-Direct

Node.js library for extracting public TikTok video metadata and downloading public media candidates.

Default live extraction and downloads delegate to the Rust gateway, so browser-profile handling stays in the Rust engine. Parser-only tests can still inject a custom `fetch` implementation for deterministic fixture coverage.

## Core Stack

* **ESM package**: Published-style package entry through `src/index.js`.
* **TypeScript declarations**: Public API declarations in `src/index.d.ts`.
* **Rust gateway**: Live extraction/download path calls `tiktok-direct-gateway`.
* **Node test runner**: Fixture tests use `node --test`.
* **Examples**: Runnable CLI-style examples for extraction and downloads.

---

## Layout

* **`src/index.js`**: Public API, gateway bridge, extractor class, helper functions.
* **`src/index.d.ts`**: TypeScript declarations for options, metadata, extractor, and helpers.
* **`src/parse.js`**: HTML script extraction and raw item selection.
* **`src/normalize.js`**: Parser-only metadata normalization for fixture/custom-fetch use.
* **`src/download.js`**: Parser-only media candidate download helper for custom-fetch use.
* **`src/values.js`**: Type coercion and URL helpers.
* **`tests/`**: Node test runner fixture tests.
* **`examples/basic-extraction.mjs`**: Metadata extraction example.
* **`examples/download-media.mjs`**: Media download example.

---

## Installation & Local Build

Install dependencies from this folder:

```powershell
npm install
```

Build the Rust gateway from the workspace root:

```powershell
cargo build --release -p tiktok-direct-gateway
```

The package looks for:

* Windows: `target/release/tiktok-direct-gateway.exe`
* Linux/macOS: `target/release/tiktok-direct-gateway`

If the binary is missing in a local checkout, the package can run the gateway through Cargo.

---

## API & Usage Guidelines

### 1. Helper Extraction

```js
import { extract } from "tiktok-direct";

const video = await extract("https://vt.tiktok.com/example/");
console.log(video.title);
console.log(video.view_count);
```

### 2. Reusable Extractor

```js
import { TikTokExtractor } from "tiktok-direct";

const extractor = new TikTokExtractor({
  acceptLanguage: "en-US,en;q=0.9,id;q=0.8",
  useOEmbed: true,
});

const video = await extractor.extract("https://vt.tiktok.com/example/");
```

### 3. Download Media

```js
import { download } from "tiktok-direct";

const path = await download("https://vt.tiktok.com/example/", "mp4", "downloads/");
console.log(path);
```

Supported download kinds:

* `mp4`
* `mp3`
* `thumbnail`

### 4. Parser-Only Fixture Mode

Pass `fetch` when you want deterministic tests without calling the Rust gateway.

```js
const extractor = new TikTokExtractor({
  fetch: async (url) => ({
    ok: true,
    url,
    text: async () => "<script id=\"SIGI_STATE\" type=\"application/json\">{}</script>",
  }),
  useOEmbed: false,
});
```

### 5. Runnable Examples

```powershell
node examples/basic-extraction.mjs https://vt.tiktok.com/example/
node examples/download-media.mjs https://vt.tiktok.com/example/ mp4
```

---

## Test Coverage

Package tests:

```powershell
npm test
```

External live consumer test:

```powershell
cd ..\..\..\binding_test\node_binding_test
npm test
```

The external consumer test validates:

* `new TikTokExtractor().extract()`
* package-level `extract()`
* package-level MP4 download
* extractor MP3 download
* per-request timing output
* final JSON summary output
* media files written to `binding_test/node_binding_test/downloads`

---

## Current Scope

The current Node package is usable and routes live work through the Rust gateway. The next architecture step is replacing gateway process calls with an N-API native addon so Node can call the Rust engine in-process while keeping the public API stable.
