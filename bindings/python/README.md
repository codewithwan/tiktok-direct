# Python Binding

Python binding for the shared Rust implementation package.

Stack:

- PyO3
- maturin

## Build

```powershell
python -m maturin build --release
```

## Install & Run

Use the Python environment that is already active in your shell. No virtual environment is required.

```powershell
cd tiktok-direct\bindings\python
python -m maturin build --release
python -m pip install --force-reinstall ..\..\target\wheels\tiktok_direct-0.1.0-cp312-cp312-win_amd64.whl
```

If you already had an older `tiktok-direct` installed, the `--force-reinstall` step replaces it with the local build.

Then run the binding test from the repository root:

```powershell
cd binding_test\python_binding_test
python .\test_binding.py
```

## API

```python
from tiktok_direct import TikTokExtractor, download, extract

url = "https://vt.tiktok.com/ZSxvYRvoR/"

video = TikTokExtractor().extract(url)
same_video = extract(url)

print(video["view_count"])
print(same_video["quality"])

mp4_path = download(url, "mp4", "downloads")
mp3_path = TikTokExtractor().download(url, "mp3", "downloads")
```
