# Go Consumer Test

This directory contains the Go consumer integration test.

The purpose of this test is to verify that the `github.com/codewithwan/tiktok-direct-go` Go package can be imported and executed by an external project correctly.

## Running the Test

The `go.mod` file uses the `replace` directive to point directly to the local Go binding (`../../bindings/go`), meaning you do not need to pull it from remote to test changes.

To run the test:

```bash
go run test_binding.go
```

This test mirrors the 4 execution blocks found in the Python reference test to guarantee output parity across all bindings. Downloaded artifacts are placed in a localized `downloads/` directory that is automatically ignored by Git.
