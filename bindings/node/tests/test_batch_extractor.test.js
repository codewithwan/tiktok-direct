import assert from "node:assert/strict";
import { test } from "node:test";
import { TikTokExtractor } from "../src/index.js";

const URL = "https://vt.tiktok.com/ZSxvYRvoR/";

test("test_extract_many_real_url", async () => {
  const extractor = new TikTokExtractor();

  const urls = [
    URL,
    "invalid-url"
  ];

  const results = await Promise.allSettled(urls.map(url => extractor.extract(url)));

  assert.equal(results.length, 2);
  
  const valid = results[0].value;
  assert.ok(valid);
  assert.equal(valid.quality, "complete");
  assert.ok(valid.video_id);

  const err = results[1].reason;
  assert.ok(err);
});
