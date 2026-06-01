# Python Consumer Test

This directory contains the Python consumer integration test.

The purpose of this test is to verify that the `tiktok_direct` Python package (built using PyO3 and Maturin) can be imported and executed exactly as an end-user would use it after installing it from a wheel or PyPI.

## Prerequisites

Before running this test, you must build and install the `tiktok_direct` package in your current Python environment (virtualenv).

```bash
cd ../../bindings/python
maturin develop --release
```
*(Or use `pip install .` depending on your workflow)*

## Running the Test

Run the test script directly using Python:

```bash
python test_binding.py
```

This will run all 4 integration sections:
1. `sync extractor`
2. `download`
3. `async extractor`
4. `batch extractor`

All downloaded media files are isolated to an OS temporary directory and do not pollute the repository.
