# tiktok-direct

`tiktok-direct` is a high-performance, library-first workspace for extracting public TikTok video metadata and media. It is designed around a single shared, highly-modular Rust engine core with thin, zero-dependency language bindings on top.

## Repository Layout

* **`crates/engine`**: Shared core Rust package using a modular subsystem design (`net/` and `parser/`).
* **`bindings/python`**: Thin, highly-optimized Python binding powered by PyO3, Maturin, and asyncio.
* **`bindings/node`**: Usable Node.js package with `src/`, declarations, tests, and examples.
* **`bindings/go`**: Usable Go module with library code, tests, and examples.

---

## Key Framework Features

* **Dynamic WAF Evasion / Rotation**: Integrates organic browser profile simulation (User-Agent, Accept-Language, Sec-CH-UA client-hints) driven by high-entropy nanosecond resolution.
* **Native Challenge Solving**: Built-in, high-speed SHA-256 cookie challenge solver to transparently bypass TikTok's Web Application Firewall (WAF).
* **Asynchronous Execution**: Fully native Python `asyncio` wrappers supporting non-blocking concurrent events inside modern web apps (FastAPI, Sanic, etc.).
* **Parallel Batch Pipelines**: High-speed concurrent scraping leveraging robust worker ThreadPools with isolated error mappings.
* **Clean Exception Mappings**: Native Python exception hierarchy (`InvalidURLError`, `ChallengeError`, `DownloadError`) for predictable error handling.

---

## Getting Started

### 1. Rust Engine Core
To consume the raw extraction and download components directly inside a Rust application, add it to your dependencies:

```toml
[dependencies]
tiktok-direct-engine = { package = "engine", path = "crates/engine" }
```

```rust
use tiktok_direct_engine::{TikTokExtractor, MediaKind, download_media};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let extractor = TikTokExtractor::new();
    let metadata = extractor.extract("https://vt.tiktok.com/ZSxvYRvoR/")?;
    println!("Title: {:?}", metadata.title);
    Ok(())
}
```

### 2. Python Language Binding
To compile and bind the package locally into your active Python environment:

```powershell
cd bindings/python
python -m maturin build --release
python -m pip install --force-reinstall ..\..\target\wheels\tiktok_direct-0.1.0-cp312-cp312-win_amd64.whl
```

For advanced API integrations, examples, and async configurations, see the [Python Package Documentation](bindings/python/README.md).

### 3. Go Library

```powershell
cd bindings/go
go test ./...
```

```go
video, err := tiktokdirect.Extract(ctx, "https://www.tiktok.com/@user/video/123")
```

### 4. Node.js Library

```powershell
cd bindings/node
npm test
```

```js
import { extract } from "tiktok-direct";

const video = await extract("https://www.tiktok.com/@user/video/123");
```

---

## Quality Assurance & Verification
Our pipeline uses strict validation mechanisms to ensure codebase integrity:
* **Cargo Tests**: Automated execution of all Rust engine and integration suites (`cargo test --workspace`).
* **Python Tests**: Comprehensive unit tests covering async offloading, concurrent batching, and stub mocks (`python -m unittest discover`).
* **Go Tests**: Consumer-style package validation (`go test ./...` from `bindings/go`).
* **Node Tests**: Built-in Node test runner validation (`npm test` from `bindings/node`).
* **Static Verification**: Strict type assertions through mypy (`python -m mypy tests examples`).
* **Automatic Code Formatting**: Consolidated code formatting checks integrated through Ruff pre-commit tools.
