# Go Binding

Planned binding for the Rust core.

Possible strategies:

- C ABI wrapper around Rust core
- Manual Go port that follows Rust/Python behavior fixtures

Target API:

```go
video, err := tiktokdirect.Extract(ctx, "https://vt.tiktok.com/example/")
```

