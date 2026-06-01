import unittest
from unittest.mock import MagicMock, patch
from tiktok_direct import TikTokExtractor, download


class TestTikTokDownload(unittest.TestCase):
    def test_invalid_download_params_raise_error(self):
        """Test that invalid media kinds or directories raise exceptions."""
        extractor = TikTokExtractor()

        # Invalid kind (should be mp3 or mp4)
        with self.assertRaises(Exception):
            extractor.download("https://vt.tiktok.com/ZSxvYRvoR/", "invalid-kind")

        with self.assertRaises(Exception):
            download("https://vt.tiktok.com/ZSxvYRvoR/", "invalid-kind")

    @patch("tiktok_direct.TikTokExtractor.download")
    def test_mocked_download(self, mock_download):
        """Test download function returns local path when mocked."""
        mock_download.return_value = "downloads/video.mp4"

        extractor = TikTokExtractor()
        path = extractor.download("https://vt.tiktok.com/ZSxvYRvoR/", "mp4", "downloads")

        self.assertEqual(path, "downloads/video.mp4")
        mock_download.assert_called_once_with("https://vt.tiktok.com/ZSxvYRvoR/", "mp4", "downloads")


if __name__ == "__main__":
    unittest.main()
