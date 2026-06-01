# Python Binding for TikTok-Direct

High-performance, type-safe, and event-loop safe Python bindings for the core `tiktok-direct` extraction engine built on top of native Rust.

## Core Stack
* **PyO3**: Direct binding bindings layer.
* **Maturin**: Mixed Rust/Python packaging and compiler toolchain.
* **asyncio**: Event-loop compatible wrappers for non-blocking I/O.
* **ThreadPoolExecutor**: Concurrent worker pools for batch tasks.

---

## Installation & Local Build

Use your active shell environment to build and install the bindings locally:

```powershell
# 1. Navigate to python bindings folder
cd bindings/python

# 2. Build the optimized wheel
python -m maturin build --release

# 3. Force-reinstall the freshly built wheel
python -m pip install --force-reinstall ..\..\target\wheels\tiktok_direct-0.1.0-cp312-cp312-win_amd64.whl
```

To run static type analysis and code validation:
```powershell
python -m mypy tests examples
```

---

## API & Usage Guidelines

### 1. Synchronous Extraction
Designed for standard synchronous pipelines, scripts, or offline scrapers:

```python
from tiktok_direct import TikTokExtractor, extract, download

url = "https://vt.tiktok.com/example/"

# Option A: Helper function (uses Rust browser-profile rotation)
metadata = extract(url)
print(f"Title: {metadata['title']}")
print(f"Likes: {metadata['like_count']}")

# Option B: Object-oriented Extractor with custom request settings
extractor = TikTokExtractor(
    user_agent="Mozilla/5.0 CustomUA/1.0",
    accept_language="id-ID,id;q=0.9"
)
metadata = extractor.extract(url)

# Download media files (kind='mp4' or 'mp3')
mp4_path = download(url, kind="mp4", output="downloads")
print(f"MP4 saved to: {mp4_path}")
```

### 2. Asynchronous Extraction (`asyncio`)
Fully non-blocking, making it ideal for high-throughput frameworks like **FastAPI**, **Sanic**, or discord bots:

```python
import asyncio
from tiktok_direct import AsyncTikTokExtractor, extract_async, download_async

async def main():
    url = "https://vt.tiktok.com/example/"
    
    # Helper async call
    metadata = await extract_async(url)
    print(f"Views: {metadata['view_count']}")
    
    # Async Extractor instantiation
    async_extractor = AsyncTikTokExtractor(user_agent="MyAsyncClient/2.0")
    metadata = await async_extractor.extract(url)
    
    # Async download
    mp3_path = await download_async(url, kind="mp3", output="downloads")
    print(f"Audio file saved to: {mp3_path}")

asyncio.run(main())
```

### 3. Concurrent Batch Scraping (`BatchExtractor`)
Process list collections in parallel utilizing thread pools. Individual exceptions are cleanly mapped per URL without crashing the overall queue:

```python
from tiktok_direct import BatchExtractor, InvalidURLError

urls = [
    "https://vt.tiktok.com/example/",
    "invalid-url"
]

# Instantiate with 4 concurrent thread workers
batch = BatchExtractor(max_workers=4)

# Batch Extract
results = batch.extract_many(urls)
for url, result in results.items():
    if isinstance(result, Exception):
        print(f"Failed to extract {url}: {result}")
    else:
        print(f"Successfully extracted {url}: {result['title']}")

# Batch Download
downloads = batch.download_many(urls, kind="mp4", output="downloads")
```

---

## Robust Error Handling
Custom exception classes map directly from underlying Rust network, formatting, and WAF failures:

```python
from tiktok_direct import (
    TikTokExtractor,
    TikTokDirectError,
    InvalidURLError,
    ChallengeError,
    DownloadError
)

try:
    extract("https://not-tiktok.com/vid")
except InvalidURLError as e:
    print("The URL layout is not supported by this extractor.")
except ChallengeError as e:
    print("Unable to bypass WAF cookies. Please rotate IPs.")
except DownloadError as e:
    print("Media candidate streams are offline or unavailable.")
except TikTokDirectError as e:
    print(f"An unexpected extraction error occurred: {e}")
```
