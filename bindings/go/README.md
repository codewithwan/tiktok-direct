# tiktok-direct-go

Go library for extracting public TikTok video metadata and downloading public media candidates.

## Layout

- `*.go`: library package `tiktokdirect`
- `tests/`: consumer-style tests that import the package
- `examples/basic_extraction`: metadata example
- `examples/download_media`: download example

## Usage

```go
video, err := tiktokdirect.Extract(ctx, "https://www.tiktok.com/@user/video/123")
```

```go
path, err := tiktokdirect.Download(ctx, url, "mp4", "downloads/")
```

Supported download kinds: `mp4`, `mp3`, `thumbnail`.

## Validate

```powershell
go test ./...
```
