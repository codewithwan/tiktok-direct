from typing import Any, Dict, Optional

class TikTokExtractor:
    def __init__(self) -> None:
        """Initialize a new TikTokExtractor instance."""
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

def extract(url: str) -> Dict[str, Any]:
    """Extract public metadata from a TikTok video URL.

    Args:
        url: The public TikTok video URL.

    Returns:
        A dictionary containing the normalized video metadata.
    """
    ...

def download(url: str, kind: str, output: Optional[str] = None) -> str:
    """Download public media candidates (MP4 or MP3) from the video URL.

    Args:
        url: The public TikTok video URL.
        kind: The type of media to download ('mp4' or 'mp3').
        output: The destination output folder path.

    Returns:
        The absolute path of the downloaded file.
    """
    ...
