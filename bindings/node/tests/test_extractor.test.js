import assert from "node:assert/strict";
import { test } from "node:test";
import { TikTokExtractor, extract, download } from "../src/index.js";

const URL = "https://vt.tiktok.com/ZSxvYRvoR/";

test("test_module_exports", () => {
  assert.ok(TikTokExtractor);
  assert.ok(extract);
  assert.ok(download);
});

test("test_invalid_url_raises_error", async () => {
  const extractor = new TikTokExtractor();
  await assert.rejects(
    async () => await extractor.extract("invalid-url")
  );

  await assert.rejects(
    async () => await extract("invalid-url")
  );
});

test("test_extract_real_url", async () => {
  const extractor = new TikTokExtractor();
  const meta = await extractor.extract(URL);

  assert.equal(meta.quality, "complete");
  assert.ok(meta.video_id);
  assert.ok(meta.title);
});
