import unittest
from tiktok_direct import BatchExtractor, InvalidURLError

URL = "https://vt.tiktok.com/ZSxvYRvoR/"

class TestBatchExtractor(unittest.TestCase):
    def test_extract_many_real_url(self):
        batch = BatchExtractor(max_workers=2)

        urls = [
            URL,
            "invalid-url"
        ]

        results = batch.extract_many(urls)

        self.assertEqual(len(results), 2)
        
        valid = results[URL]
        self.assertFalse(isinstance(valid, Exception))
        self.assertEqual(valid["quality"], "complete")
        self.assertTrue(valid["video_id"])

        self.assertIsInstance(results["invalid-url"], InvalidURLError)

if __name__ == "__main__":
    unittest.main()
