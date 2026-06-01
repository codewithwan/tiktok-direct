from typing import Any, Dict, List, Optional

class TikTokDirectError(Exception):
    """Base exception for tiktok-direct."""
    pass

class InvalidURLError(TikTokDirectError):
    """Raised when URL format is invalid."""
    pass

class ChallengeError(TikTokDirectError):
    """Raised when challenge solving fails."""
    pass

class DownloadError(TikTokDirectError):
    """Raised when media download fails."""
    pass

class TikTokExtractor:
    def __init__(self, user_agent: Optional[str] = None, accept_language: Optional[str] = None) -> None:
        """Initialize a new TikTokExtractor instance.

        Args:
            user_agent: Optional custom User-Agent string. If not provided, a random browser profile is dynamically generated.
            accept_language: Optional custom Accept-Language string.
        """
        ...

    def extract(self, url: str) -> Dict[str, Any]:
        """Extract public metadata from a TikTok video URL.

        Args:
            url: The public TikTok video URL.

        Returns:
            A dictionary containing the normalized video metadata.
        """
        ...

    def download(self, url: str, kind: str, output: Optional[str] = None) -> str:
        """Download public media candidates (MP4 or MP3) from the video URL.

        Args:
            url: The public TikTok video URL.
            kind: The type of media to download ('mp4' or 'mp3').
            output: The destination output folder path.

        Returns:
            The absolute path of the downloaded file.
        """
        ...

class AsyncTikTokExtractor:
    def __init__(self, user_agent: Optional[str] = None, accept_language: Optional[str] = None) -> None:
        """Initialize a new AsyncTikTokExtractor instance.

        Args:
            user_agent: Optional custom User-Agent string.
            accept_language: Optional custom Accept-Language string.
        """
        ...

    async def extract(self, url: str) -> Dict[str, Any]:
        """Extract public metadata from a TikTok video URL asynchronously."""
        ...

    async def download(self, url: str, kind: str, output: Optional[str] = None) -> str:
        """Download public media candidates (MP4 or MP3) from the video URL asynchronously."""
        ...

class BatchExtractor:
    def __init__(
        self,
        max_workers: int = 4,
        user_agent: Optional[str] = None,
        accept_language: Optional[str] = None,
    ) -> None:
        """Initialize a new BatchExtractor.

        Args:
            max_workers: Maximum number of concurrent worker threads.
            user_agent: Optional custom User-Agent string.
            accept_language: Optional custom Accept-Language string.
        """
        ...

    def extract_many(self, urls: List[str]) -> Dict[str, Any]:
        """Extract public metadata for multiple TikTok URLs concurrently.

        Args:
            urls: A list of public TikTok video URLs.

        Returns:
            A dictionary mapping each URL to its extracted metadata dictionary or the Exception raised.
        """
        ...

    def download_many(self, urls: List[str], kind: str, output: Optional[str] = None) -> Dict[str, Any]:
        """Download public media for multiple TikTok URLs concurrently.

        Args:
            urls: A list of public TikTok video URLs.
            kind: The type of media to download ('mp4' or 'mp3').
            output: The destination output folder path.

        Returns:
            A dictionary mapping each URL to its downloaded file path or the Exception raised.
        """
        ...

def extract(url: str, user_agent: Optional[str] = None, accept_language: Optional[str] = None) -> Dict[str, Any]:
    """Extract public metadata from a TikTok video URL.

    Args:
        url: The public TikTok video URL.
        user_agent: Optional custom User-Agent string.
        accept_language: Optional custom Accept-Language string.

    Returns:
        A dictionary containing the normalized video metadata.
    """
    ...

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
    ...

def to_analytics_dict(metadata: Dict[str, Any]) -> Dict[str, Any]:
    """Return a flat analytics dictionary suitable for CSV/spreadsheets."""
    ...

def to_summary_dict(metadata: Dict[str, Any]) -> Dict[str, Any]:
    """Return a compact summary dictionary for logs and CLI output."""
    ...

async def extract_async(url: str, user_agent: Optional[str] = None, accept_language: Optional[str] = None) -> Dict[str, Any]:
    """Extract public metadata from a TikTok video URL asynchronously.

    Args:
        url: The public TikTok video URL.
        user_agent: Optional custom User-Agent string.
        accept_language: Optional custom Accept-Language string.

    Returns:
        A dictionary containing the normalized video metadata.
    """
    ...

async def download_async(
    url: str,
    kind: str,
    output: Optional[str] = None,
    user_agent: Optional[str] = None,
    accept_language: Optional[str] = None,
) -> str:
    """Download public media candidates (MP4 or MP3) asynchronously.

    Args:
        url: The public TikTok video URL.
        kind: The type of media to download ('mp4' or 'mp3').
        output: The destination output folder path.
        user_agent: Optional custom User-Agent string.
        accept_language: Optional custom Accept-Language string.

    Returns:
        The absolute path of the downloaded file.
    """
    ...



