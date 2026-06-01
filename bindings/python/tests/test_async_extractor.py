import unittest
from unittest.mock import patch
from tiktok_direct import AsyncTikTokExtractor, extract_async, InvalidURLError

class TestAsyncTikTokExtractor(unittest.IsolatedAsyncioTestCase):
    async def test_invalid_url_raises_error_async(self):
        """Test that invalid URLs raise InvalidURLError asynchronously."""
        extractor = AsyncTikTokExtractor()
        with self.assertRaises(InvalidURLError):
            await extractor.extract("invalid-url")

        with self.assertRaises(InvalidURLError):
            await extract_async("invalid-url")

    async def test_mocked_extract_async(self):
        """Test that extract_async returns correct structure when mocked."""
        from unittest.mock import MagicMock
        mock_metadata = {
            "id": "1234567890123456789",
            "title": "Mocked TikTok Video",
            "quality": "Normalized",
        }
        extractor = AsyncTikTokExtractor()
        extractor._sync_extractor.extract = MagicMock(return_value=mock_metadata)

        res = await extractor.extract("https://www.tiktok.com/@mockuser/video/1234567890123456789")

        self.assertEqual(res["id"], "1234567890123456789")
        self.assertEqual(res["title"], "Mocked TikTok Video")

if __name__ == "__main__":
    unittest.main()
