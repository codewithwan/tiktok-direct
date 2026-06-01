import unittest
from unittest.mock import MagicMock
from tiktok_direct import BatchExtractor, InvalidURLError

class TestBatchExtractor(unittest.TestCase):
    def test_mocked_extract_many(self):
        """Test that extract_many concurrently processes multiple URLs when mocked."""
        mock_metadata_1 = {
            "id": "12345",
            "title": "Video 1",
        }
        mock_metadata_2 = {
            "id": "67890",
            "title": "Video 2",
        }

        batch = BatchExtractor(max_workers=2)

        def side_effect(url):
            if "vid1" in url:
                return mock_metadata_1
            elif "vid2" in url:
                return mock_metadata_2
            else:
                raise InvalidURLError("Invalid URL")

        batch.extractor.extract = MagicMock(side_effect=side_effect)

        urls = [
            "https://www.tiktok.com/@user/video/vid1",
            "https://www.tiktok.com/@user/video/vid2",
            "invalid-url"
        ]

        results = batch.extract_many(urls)

        self.assertEqual(len(results), 3)
        self.assertEqual(results["https://www.tiktok.com/@user/video/vid1"]["title"], "Video 1")
        self.assertEqual(results["https://www.tiktok.com/@user/video/vid2"]["title"], "Video 2")
        self.assertIsInstance(results["invalid-url"], InvalidURLError)

if __name__ == "__main__":
    unittest.main()
