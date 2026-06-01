import asyncio
import json
import time
from pathlib import Path

from tiktok_direct import (
    AsyncTikTokExtractor,
    BatchExtractor,
    TikTokExtractor,
    download,
    download_async,
    extract,
    extract_async,
    to_analytics_dict,
    to_summary_dict,
)

URL = "https://www.tiktok.com/@blurrytearz/video/7645289995820895509"
DOWNLOADS_DIR = Path(__file__).resolve().parent / "downloads"

PASS = "PASS"
FAIL = "FAIL"
WARN = "WARN"


def section(title: str) -> None:
    print(f"\n{'='*56}")
    print(f"  {title}")
    print(f"{'='*56}")


def result(label: str, status: str, elapsed: float, detail: str = "") -> None:
    tag = f"[{status}]"
    timing = f"({elapsed*1000:.0f}ms)"
    line = f"  {tag:<7} {timing:<9} {label}"
    if detail:
        line += f"  {detail}"
    print(line)


# ─────────────────────────────────────────────────────────────────────────────

def test_sync_extractor() -> dict:
    section("sync extractor")
    t = time.perf_counter()
    extractor = TikTokExtractor()
    video = extractor.extract(URL)
    elapsed = time.perf_counter() - t

    assert video["quality"] == "complete", video
    assert video["video_id"]
    assert video["view_count"]
    assert video["like_count"]
    assert video["duration"]
    result("TikTokExtractor.extract()", PASS, elapsed,
           f"quality={video['quality']}  views={video['view_count']}")
    return video


def test_extract_function(reference_id: str) -> None:
    t = time.perf_counter()
    video = extract(URL)
    elapsed = time.perf_counter() - t
    assert video["video_id"] == reference_id
    result("extract(url)", PASS, elapsed, f"video_id={video['video_id']}")


def test_helper_dicts(video: dict) -> None:
    t = time.perf_counter()
    analytics = to_analytics_dict(video)
    summary = to_summary_dict(video)
    elapsed = time.perf_counter() - t
    assert analytics["video_id"] == video["video_id"]
    assert "view_count" in analytics
    assert summary["views"] == video["view_count"]
    result("to_analytics_dict() + to_summary_dict()", PASS, elapsed)


def test_download_mp4() -> Path:
    DOWNLOADS_DIR.mkdir(parents=True, exist_ok=True)
    t = time.perf_counter()
    mp4_path = Path(download(URL, "mp4", str(DOWNLOADS_DIR)))
    elapsed = time.perf_counter() - t
    assert mp4_path.is_relative_to(DOWNLOADS_DIR)
    assert mp4_path.stat().st_size > 0
    mp4_bytes = mp4_path.read_bytes()
    assert b"vide" in mp4_bytes and b"soun" in mp4_bytes
    result("download(mp4)", PASS, elapsed,
           f"{mp4_path.name}  {mp4_path.stat().st_size:,} bytes")
    return mp4_path


def test_download_mp3() -> Path:
    DOWNLOADS_DIR.mkdir(parents=True, exist_ok=True)
    extractor = TikTokExtractor()
    t = time.perf_counter()
    mp3_path = Path(extractor.download(URL, "mp3", str(DOWNLOADS_DIR)))
    elapsed = time.perf_counter() - t
    assert mp3_path.is_relative_to(DOWNLOADS_DIR)
    assert mp3_path.stat().st_size > 0
    result("TikTokExtractor.download(mp3)", PASS, elapsed,
           f"{mp3_path.name}  {mp3_path.stat().st_size:,} bytes")
    return mp3_path


async def _test_async(reference_id: str) -> None:
    section("async extractor")

    t = time.perf_counter()
    video = await extract_async(URL)
    elapsed = time.perf_counter() - t
    assert video["video_id"] == reference_id
    assert video["quality"] == "complete"
    result("extract_async(url)", PASS, elapsed, f"video_id={video['video_id']}")

    ae = AsyncTikTokExtractor()
    t = time.perf_counter()
    video2 = await ae.extract(URL)
    elapsed = time.perf_counter() - t
    assert video2["video_id"] == reference_id
    result("AsyncTikTokExtractor.extract()", PASS, elapsed)

    DOWNLOADS_DIR.mkdir(parents=True, exist_ok=True)

    t = time.perf_counter()
    mp4_async = Path(await ae.download(URL, "mp4", str(DOWNLOADS_DIR)))
    elapsed = time.perf_counter() - t
    assert mp4_async.stat().st_size > 0
    result("AsyncTikTokExtractor.download(mp4)", PASS, elapsed,
           f"{mp4_async.name}  {mp4_async.stat().st_size:,} bytes")

    t = time.perf_counter()
    mp3_async = Path(await download_async(URL, "mp3", str(DOWNLOADS_DIR)))
    elapsed = time.perf_counter() - t
    assert mp3_async.stat().st_size > 0
    result("download_async(mp3)", PASS, elapsed,
           f"{mp3_async.name}  {mp3_async.stat().st_size:,} bytes")


def test_async(reference_id: str) -> None:
    asyncio.run(_test_async(reference_id))


def test_batch_extractor(reference_id: str) -> None:
    section("batch extractor")
    batch = BatchExtractor(max_workers=2)

    t = time.perf_counter()
    results = batch.extract_many([URL, "https://not-a-tiktok.com/bad"])
    elapsed = time.perf_counter() - t

    valid = results[URL]
    assert not isinstance(valid, Exception), f"BatchExtractor failed on valid URL: {valid}"
    assert valid["video_id"] == reference_id
    result("BatchExtractor.extract_many(valid+invalid)", PASS, elapsed,
           f"valid=1  errors=1")

    bad = results["https://not-a-tiktok.com/bad"]
    assert isinstance(bad, Exception)
    result("error mapping for invalid URL", PASS, 0, type(bad).__name__)

    DOWNLOADS_DIR.mkdir(parents=True, exist_ok=True)
    t = time.perf_counter()
    dl_results = batch.download_many([URL], kind="mp4", output=str(DOWNLOADS_DIR))
    elapsed = time.perf_counter() - t

    dl = dl_results[URL]
    assert not isinstance(dl, Exception), f"batch download_many failed: {dl}"
    dl_path = Path(dl)
    assert dl_path.stat().st_size > 0
    result("BatchExtractor.download_many(mp4)", PASS, elapsed,
           f"{dl_path.name}  {dl_path.stat().st_size:,} bytes")


def main() -> int:
    print(f"\ntiktok-direct  Python binding test")
    print(f"URL: {URL}")

    video = test_sync_extractor()
    vid_id = video["video_id"]

    test_extract_function(vid_id)
    test_helper_dicts(video)

    section("download")
    test_download_mp4()
    test_download_mp3()

    test_async(vid_id)
    test_batch_extractor(vid_id)

    print(f"\n{'='*56}")
    print(json.dumps(to_summary_dict(video), indent=2))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
