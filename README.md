# tiktok-direct

`tiktok-direct` is a high-performance, library-first workspace for extracting public TikTok video metadata and media. It is designed around one shared, highly-modular Rust engine core with language bindings on top.

## Repository Layout

* **`crates/engine`**: Shared core Rust package using a modular subsystem design (`net/` and `parser/`).
* **`bindings/python`**: Thin, highly-optimized Python binding powered by PyO3, Maturin, and asyncio.
* **`crates/gateway`**: Small Rust CLI bridge used by runtimes that should delegate live extraction back to the Rust engine.
* **`bindings/node`**: Usable Node.js package with TypeScript declarations, tests, examples, and live extraction routed through the Rust gateway.
* **`bindings/go`**: Usable Go module with library code, tests, and examples.

---

## Key Framework Features

* **Single Rust Browser Profile Layer**: Browser profile rotation lives in the Rust engine. Python uses it natively, and Node live extraction delegates to the Rust gateway instead of maintaining its own browser-profile logic.
* **Native Challenge Solving**: Built-in, high-speed SHA-256 cookie challenge solver to transparently bypass TikTok's Web Application Firewall (WAF).
* **Python Binding**: PyO3-backed sync helpers, async wrappers, batch extraction, downloads, helper dicts, typed stubs, examples, and unit tests.
* **Go Binding**: Importable Go module with extractor helpers, reusable extractor struct, downloads, examples, and consumer-style package tests.
* **Node Binding**: ESM package with TypeScript declarations, extractor helpers, downloads, Rust gateway bridge, examples, and Node test runner coverage.
* **External Binding Tests**: Python, Go, and Node each have live consumer tests outside the package folders with per-request timing and download output checks.

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
    let metadata = extractor.extract("https://vt.tiktok.com/example/")?;
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

Full Python API, examples, async usage, batch helpers, downloads, and error handling are documented in [bindings/python/README.md](bindings/python/README.md).

### 3. Go Library

```powershell
cd bindings/go
go test ./...
```

```go
video, err := tiktokdirect.Extract(ctx, "https://vt.tiktok.com/example/")
```

Full Go API, examples, downloads, validation, and native-engine direction are documented in [bindings/go/README.md](bindings/go/README.md).

### 4. Node.js Library

```powershell
cd ..\..
cargo build --release -p tiktok-direct-gateway
cd bindings/node
npm test
```

```js
import { extract } from "tiktok-direct";

const video = await extract("https://vt.tiktok.com/example/");
```

Full Node API, examples, gateway build notes, downloads, and validation are documented in [bindings/node/README.md](bindings/node/README.md).


## Quality Assurance & Verification
Our pipeline uses strict validation mechanisms to ensure codebase integrity:
* **Cargo Tests**: Automated execution of all Rust engine and integration suites (`cargo test --workspace`).
* **Python Tests**: Unit tests covering async offloading, concurrent batching, and stub mocks (`python -m unittest discover -s bindings/python/tests -p "test_*.py"`).
* **Go Tests**: Consumer-style package validation (`go test ./...` from `bindings/go`).
* **Node Tests**: Built-in Node test runner validation (`npm test` from `bindings/node`).
* **External Binding Tests**: Live consumer checks live under `../binding_test` and write media into each binding's own `downloads` folder.
* **Formatting**: Rust uses `cargo fmt`; Go uses `gofmt`; Node code is kept ESM-only and covered by `node --test`.
