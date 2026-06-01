import unittest
import tiktok_direct
from tiktok_direct import TikTokExtractor, extract, InvalidURLError

URL = "https://vt.tiktok.com/ZSxvYRvoR/"

class TestTikTokExtractor(unittest.TestCase):
    def test_module_exports(self):
        self.assertTrue(hasattr(tiktok_direct, "TikTokExtractor"))
        self.assertTrue(hasattr(tiktok_direct, "extract"))
        self.assertTrue(hasattr(tiktok_direct, "download"))
        self.assertTrue(hasattr(tiktok_direct, "TikTokDirectError"))
        self.assertTrue(hasattr(tiktok_direct, "InvalidURLError"))
        self.assertTrue(hasattr(tiktok_direct, "ChallengeError"))
        self.assertTrue(hasattr(tiktok_direct, "DownloadError"))

    def test_invalid_url_raises_error(self):
        extractor = TikTokExtractor()
        with self.assertRaises(InvalidURLError):
            extractor.extract("invalid-url")

        with self.assertRaises(InvalidURLError):
            extract("invalid-url")

    def test_extract_real_url(self):
        extractor = TikTokExtractor()
        res = extractor.extract(URL)

        self.assertEqual(res["quality"], "complete")
        self.assertTrue(res["video_id"])
        self.assertTrue(res["title"])

if __name__ == "__main__":
    unittest.main()
