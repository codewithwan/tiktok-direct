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

def _raise_mapped_exception(e: Exception) -> Any:
    msg = str(e)
    if "invalid" in msg.lower() or "parse" in msg.lower() or "url" in msg.lower() or "builder error" in msg.lower():
        raise InvalidURLError(msg) from e
    elif "challenge" in msg.lower() or "waf" in msg.lower():
        raise ChallengeError(msg) from e
    elif "download" in msg.lower() or "http" in msg.lower() or "network" in msg.lower():
        raise DownloadError(msg) from e
    else:
        raise TikTokDirectError(msg) from e

__all__ = [
    "TikTokExtractor",
    "extract",
    "download",
    "AsyncTikTokExtractor",
    "extract_async",
    "download_async",
    "BatchExtractor",
    "TikTokDirectError",
    "InvalidURLError",
    "ChallengeError",
    "DownloadError",
]

