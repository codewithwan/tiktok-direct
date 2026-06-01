import unittest
from unittest.mock import MagicMock, patch
import tiktok_direct
from tiktok_direct import TikTokExtractor, extract, InvalidURLError, TikTokDirectError

class TestTikTokExtractor(unittest.TestCase):
    def test_module_exports(self):
        """Test that the python binding exports the expected classes and functions."""
        self.assertTrue(hasattr(tiktok_direct, "TikTokExtractor"))
        self.assertTrue(hasattr(tiktok_direct, "extract"))
        self.assertTrue(hasattr(tiktok_direct, "download"))
        self.assertTrue(hasattr(tiktok_direct, "TikTokDirectError"))
        self.assertTrue(hasattr(tiktok_direct, "InvalidURLError"))
        self.assertTrue(hasattr(tiktok_direct, "ChallengeError"))
        self.assertTrue(hasattr(tiktok_direct, "DownloadError"))

    def test_invalid_url_raises_error(self):
        """Test that invalid URLs raise InvalidURLError."""
        extractor = TikTokExtractor()
        with self.assertRaises(InvalidURLError):
            extractor.extract("invalid-url")

        with self.assertRaises(InvalidURLError):
            extract("invalid-url")


    @patch("tiktok_direct.TikTokExtractor.extract")
    def test_mocked_extract(self, mock_extract):
        """Test that extract returns correct structure when mocked."""
        mock_metadata = {
            "id": "1234567890123456789",
            "title": "Mocked TikTok Video",
            "author": {"username": "mockuser", "nickname": "Mock User"},
            "duration": 15,
            "quality": "Normalized",
        }
        mock_extract.return_value = mock_metadata

        extractor = TikTokExtractor()
        res = extractor.extract("https://www.tiktok.com/@mockuser/video/1234567890123456789")

        self.assertEqual(res["id"], "1234567890123456789")
        self.assertEqual(res["title"], "Mocked TikTok Video")
        self.assertEqual(res["author"]["username"], "mockuser")
        self.assertEqual(res["quality"], "Normalized")


if __name__ == "__main__":
    unittest.main()
