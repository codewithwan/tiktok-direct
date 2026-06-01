import asyncio
from typing import Any, Dict, Optional

class AsyncTikTokExtractor:
    """Asynchronous wrapper for TikTokExtractor that runs blocking network operations in a background thread."""

    def __init__(self, user_agent: Optional[str] = None, accept_language: Optional[str] = None) -> None:
        """Initialize a new AsyncTikTokExtractor instance."""
        from .__init__ import TikTokExtractor
        self._sync_extractor = TikTokExtractor(user_agent, accept_language)

    async def extract(self, url: str) -> Dict[str, Any]:
        """Extract public metadata from a TikTok video URL asynchronously."""
        return await asyncio.to_thread(self._sync_extractor.extract, url)

    async def download(self, url: str, kind: str, output: Optional[str] = None) -> str:
        """Download public media candidates (MP4 or MP3) asynchronously."""
        return await asyncio.to_thread(self._sync_extractor.download, url, kind, output)

async def extract_async(url: str, user_agent: Optional[str] = None, accept_language: Optional[str] = None) -> Dict[str, Any]:
    """Extract public metadata from a TikTok video URL asynchronously."""
    from .__init__ import extract
    return await asyncio.to_thread(extract, url, user_agent, accept_language)

async def download_async(
    url: str,
    kind: str,
    output: Optional[str] = None,
    user_agent: Optional[str] = None,
    accept_language: Optional[str] = None,
) -> str:
    """Download public media candidates (MP4 or MP3) asynchronously."""
    from .__init__ import download
    return await asyncio.to_thread(download, url, kind, output, user_agent, accept_language)
