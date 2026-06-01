# Node.js Consumer Test

This directory contains the Node.js consumer integration test.

The purpose of this test is to verify that the `tiktok-direct` npm package can be linked, imported, and executed identically to how an end-user would use it when installed via npm.

## Running the Test

The `package.json` is configured to map the `"tiktok-direct"` dependency directly to the local filesystem (`file:../../bindings/node`).

To run the test:

```bash
# Install the local dependency
npm install

# Run the test
npm test
```

This executes all extraction and download processes, ensuring that outputs strictly match the reference Python implementation. Media artifacts are downloaded to a local temporary directory and ignored by git to maintain a clean repository.
