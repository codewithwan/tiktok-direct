from .tiktok_direct import TikTokExtractor as _TikTokExtractor, download as _download, extract as _extract
from .exceptions import TikTokDirectError, InvalidURLError, ChallengeError, DownloadError
from .async_extractor import AsyncTikTokExtractor, extract_async, download_async
from .batch import BatchExtractor
from typing import Any, Dict, Optional

class TikTokExtractor:
    def __init__(self, user_agent: Optional[str] = None, accept_language: Optional[str] = None) -> None:
        """Initialize a new TikTokExtractor instance.

        Args:
            user_agent: Optional custom User-Agent string. If not provided, a random browser profile is dynamically generated.
            accept_language: Optional custom Accept-Language string.
        """
        self._inner = _TikTokExtractor(user_agent, accept_language)

    def extract(self, url: str) -> Dict[str, Any]:
        """Extract public metadata from a TikTok video URL.

        Args:
            url: The public TikTok video URL.

        Returns:
            A dictionary containing the normalized video metadata.
        """
        try:
            return self._inner.extract(url)
        except Exception as e:
            _raise_mapped_exception(e)

    def download(self, url: str, kind: str, output: Optional[str] = None) -> str:
        """Download public media candidates (MP4 or MP3) from the video URL.

        Args:
            url: The public TikTok video URL.
            kind: The type of media to download ('mp4' or 'mp3').
            output: The destination output folder path.

        Returns:
            The absolute path of the downloaded file.
        """
        try:
            return self._inner.download(url, kind, output)
        except Exception as e:
            _raise_mapped_exception(e)

def extract(url: str, user_agent: Optional[str] = None, accept_language: Optional[str] = None) -> Dict[str, Any]:
    """Extract public metadata from a TikTok video URL.

    Args:
        url: The public TikTok video URL.
        user_agent: Optional custom User-Agent string.
        accept_language: Optional custom Accept-Language string.

    Returns:
        A dictionary containing the normalized video metadata.
    """
    try:
        return _extract(url, user_agent, accept_language)
    except Exception as e:
        _raise_mapped_exception(e)

def download(
    url: str,
    kind: str,
    output: Optional[str] = None,
    user_agent: Optional[str] = None,
    accept_language: Optional[str] = None,
) -> str:
    """Download public media candidates (MP4 or MP3) from the video URL.

    Args:
        url: The public TikTok video URL.
        kind: The type of media to download ('mp4' or 'mp3').
        output: The destination output folder path.
        user_agent: Optional custom User-Agent string.
        accept_language: Optional custom Accept-Language string.

    Returns:
        The absolute path of the downloaded file.
    """
    try:
        return _download(url, kind, output, user_agent, accept_language)
    except Exception as e:
        _raise_mapped_exception(e)

def to_analytics_dict(metadata: Dict[str, Any]) -> Dict[str, Any]:
    """Return a flat analytics dictionary suitable for CSV/spreadsheets."""
    stable = {
        "video_id": metadata.get("video_id"),
        "username": metadata.get("username"),
        "author_unique_id": metadata.get("author_unique_id"),
        "title": metadata.get("title"),
        "view_count": metadata.get("view_count"),
        "like_count": metadata.get("like_count"),
        "comment_count": metadata.get("comment_count"),
        "repost_count": metadata.get("repost_count"),
        "duration": metadata.get("duration"),
        "quality": metadata.get("quality"),
        "reason": metadata.get("reason"),
    }
    stable.update(metadata.get("analytics") or {})
    return stable

def to_summary_dict(metadata: Dict[str, Any]) -> Dict[str, Any]:
    """Return a compact summary dictionary for logs and CLI output."""
    summary = metadata.get("summary")
    if isinstance(summary, dict) and summary:
        return dict(summary)
    return {
        "video_id": metadata.get("video_id"),
        "url": metadata.get("webpage_url") or metadata.get("resolved_url"),
        "author": metadata.get("author_unique_id") or metadata.get("username"),
        "title": metadata.get("title"),
        "views": metadata.get("view_count"),
        "likes": metadata.get("like_count"),
        "comments": metadata.get("comment_count"),
        "shares": metadata.get("repost_count"),
        "duration": metadata.get("duration"),
        "quality": metadata.get("quality"),
    }

def _raise_mapped_exception(e: Exception) -> Any:
    msg = str(e)
    lower = msg.lower()
    if "invalid" in lower or "parse" in lower or "builder error" in lower:
        raise InvalidURLError(msg) from e
    elif "download" in lower or "media" in lower or "http" in lower or "network" in lower:
        raise DownloadError(msg) from e
    elif "challenge" in lower or "waf" in lower:
        raise ChallengeError(msg) from e
    elif "url" in lower:
        raise InvalidURLError(msg) from e
    else:
        raise TikTokDirectError(msg) from e

__all__ = [
    "TikTokExtractor",
    "extract",
    "download",
    "to_analytics_dict",
    "to_summary_dict",
    "AsyncTikTokExtractor",
    "extract_async",
    "download_async",
    "BatchExtractor",
    "TikTokDirectError",
    "InvalidURLError",
    "ChallengeError",
    "DownloadError",
]
