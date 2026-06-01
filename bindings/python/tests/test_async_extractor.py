import unittest
from tiktok_direct import AsyncTikTokExtractor, extract_async, InvalidURLError

URL = "https://vt.tiktok.com/ZSxvYRvoR/"

class TestAsyncTikTokExtractor(unittest.IsolatedAsyncioTestCase):
    async def test_invalid_url_raises_error_async(self):
        extractor = AsyncTikTokExtractor()
        with self.assertRaises(InvalidURLError):
            await extractor.extract("invalid-url")

        with self.assertRaises(InvalidURLError):
            await extract_async("invalid-url")

    async def test_extract_real_url_async(self):
        extractor = AsyncTikTokExtractor()
        res = await extractor.extract(URL)

        self.assertEqual(res["quality"], "complete")
        self.assertTrue(res["video_id"])
        self.assertTrue(res["title"])

if __name__ == "__main__":
    unittest.main()
