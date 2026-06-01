from typing import List, Dict, Any, Optional
from concurrent.futures import ThreadPoolExecutor, as_completed

class BatchExtractor:
    """High-performance batch processing engine for extracting and downloading multiple TikTok URLs in parallel."""

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
        from .__init__ import TikTokExtractor
        self.max_workers = max_workers
        self.extractor = TikTokExtractor(user_agent, accept_language)

    def extract_many(self, urls: List[str]) -> Dict[str, Any]:
        """Extract public metadata for multiple TikTok URLs concurrently.

        Args:
            urls: A list of public TikTok video URLs.

        Returns:
            A dictionary mapping each URL to its extracted metadata dictionary or the Exception raised.
        """
        results: Dict[str, Any] = {}
        with ThreadPoolExecutor(max_workers=self.max_workers) as executor:
            future_to_url = {executor.submit(self.extractor.extract, url): url for url in urls}
            for future in as_completed(future_to_url):
                url = future_to_url[future]
                try:
                    results[url] = future.result()
                except Exception as e:
                    results[url] = e
        return results

    def download_many(self, urls: List[str], kind: str, output: Optional[str] = None) -> Dict[str, Any]:
        """Download public media for multiple TikTok URLs concurrently.

        Args:
            urls: A list of public TikTok video URLs.
            kind: The type of media to download ('mp4' or 'mp3').
            output: The destination output folder path.

        Returns:
            A dictionary mapping each URL to its downloaded file path or the Exception raised.
        """
        results: Dict[str, Any] = {}
        with ThreadPoolExecutor(max_workers=self.max_workers) as executor:
            future_to_url = {executor.submit(self.extractor.download, url, kind, output): url for url in urls}
            for future in as_completed(future_to_url):
                url = future_to_url[future]
                try:
                    results[url] = future.result()
                except Exception as e:
                    results[url] = e
        return results
