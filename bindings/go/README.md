# Go Binding for TikTok-Direct

Usable Go library for extracting public TikTok video metadata and downloading public media candidates.

The Go package is intentionally structured as a normal Go module today, with a clear path toward Rust engine integration through cgo/FFI later. Browser-profile rotation must remain owned by the Rust engine/gateway path, not duplicated inside this Go package.

## Core Stack

* **Go module**: Importable package `github.com/codewithwan/tiktok-direct-go`.
* **net/http**: Standard Go HTTP client with `context.Context` support.
* **Fixture tests**: `httptest` coverage for parser and normalization behavior.
* **Examples**: Runnable CLI-style examples for extraction and downloads.

---

## Layout

* **`tiktokdirect.go`**: Main extractor, HTML fetch, JSON script parsing, oEmbed merge.
* **`normalize.go`**: Converts raw TikTok item data into a normalized map.
* **`download.go`**: Resolves media candidates and writes MP4/MP3/thumbnail files.
* **`values.go`**: Type coercion helpers for strings, numbers, maps, URLs, and unique lists.
* **`files.go`**: Output path and filename helpers.
* **`tests/`**: Consumer-style package tests that import the module.
* **`examples/basic_extraction/`**: Metadata extraction example.
* **`examples/download_media/`**: Media download example.

---

## Installation & Local Use

From another Go module, use a local replace while developing:

```go
require github.com/codewithwan/tiktok-direct-go v0.1.0

replace github.com/codewithwan/tiktok-direct-go => ../tiktok-direct/bindings/go
```

Validate the package from this folder:

```powershell
go test ./...
```

---

## API & Usage Guidelines

### 1. Synchronous Extraction

```go
package main

import (
	"context"
	"fmt"

	tiktokdirect "github.com/codewithwan/tiktok-direct-go"
)

func main() {
	video, err := tiktokdirect.Extract(context.Background(), "https://vt.tiktok.com/example/")
	if err != nil {
		panic(err)
	}
	fmt.Println(video["title"])
	fmt.Println(video["view_count"])
}
```

### 2. Reusable Extractor

Use `New()` when you want to reuse the same HTTP client and settings.

```go
extractor := tiktokdirect.New()
extractor.UseOEmbed = true
extractor.AcceptLang = "en-US,en;q=0.9,id;q=0.8"

video, err := extractor.Extract(context.Background(), "https://vt.tiktok.com/example/")
```

### 3. Download Media

```go
path, err := tiktokdirect.Download(
	context.Background(),
	"https://vt.tiktok.com/example/",
	"mp4",
	"downloads/",
)
```

Supported download kinds:

* `mp4`
* `mp3`
* `thumbnail`

The output argument can be a directory or a file path. Directory outputs are resolved to the default filename.

### 4. Runnable Examples

```powershell
go run ./examples/basic_extraction https://vt.tiktok.com/example/
go run ./examples/download_media https://vt.tiktok.com/example/ mp4
```

---

## Test Coverage

Package tests:

```powershell
go test ./...
```

External live consumer test:

```powershell
cd ..\..\..\binding_test\go_binding_test
go run test_binding.go
```

The external consumer test validates:

* `Extractor.Extract()`
* package-level `Extract()`
* MP4 download
* MP3 download
* per-request timing output
* final JSON summary output
* media files written to `binding_test/go_binding_test/downloads`

---

## Current Scope

The Go package is usable as a standalone Go library. The next architecture step is a cgo/FFI bridge into the Rust engine so live extraction, retries, browser-profile rotation, and downloads all share the same core implementation.
