# External Binding Tests

This directory contains external consumer tests for all `tiktok-direct` language bindings (Node.js, Go, and Python).

The purpose of these tests is to verify that the compiled packages and source modules can be properly imported, linked, and executed by an external project (acting as an end-user). This ensures that module paths, dependencies, and native bindings work correctly when installed outside the main repository workspace.

## Structure

- `node_binding_test`: Tests importing the `tiktok-direct` NPM package from a standard Node.js module.
- `go_binding_test`: Tests importing the `github.com/codewithwan/tiktok-direct-go` Go package.
- `python_binding_test`: Tests importing the `tiktok_direct` Python package (typically built with `maturin develop` or pip).

## How to Run

All test scripts enforce strict parity across languages, simulating identical extraction and download flows to guarantee consistent outputs. By default, these tests will use a known live TikTok URL and download test files to an isolated OS temporary directory (handled internally), leaving no artifacts behind.

### Node.js

Navigate to `node_binding_test` and install the local binding via npm, then run the test:

```bash
cd node_binding_test
npm install
npm test
```

### Go

Navigate to `go_binding_test` and run the go script:

```bash
cd go_binding_test
go run test_binding.go
```

### Python

Navigate to `python_binding_test` and run the script (ensure you have built/installed the wheel first):

```bash
cd python_binding_test
python test_binding.py
```

## Note on Dependencies

These tests use relative paths (e.g., `file:../bindings/node` or `replace ... => ../bindings/go`) to resolve the local bindings dynamically. Ensure you run these from within their respective directories.
