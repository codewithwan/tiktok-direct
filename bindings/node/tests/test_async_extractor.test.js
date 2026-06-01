import assert from "node:assert/strict";
import { test } from "node:test";
import { TikTokExtractor, extract } from "../src/index.js";

const URL = "https://vt.tiktok.com/ZSxvYRvoR/";

test("test_invalid_url_raises_error_async", async () => {
  const extractor = new TikTokExtractor();
  await assert.rejects(
    async () => await extractor.extract("invalid-url")
  );

  await assert.rejects(
    async () => await extract("invalid-url")
  );
});

test("test_extract_real_url_async", async () => {
  const extractor = new TikTokExtractor();
  const meta = await extractor.extract(URL);

  assert.equal(meta.quality, "complete");
  assert.ok(meta.video_id);
  assert.ok(meta.title);
});
