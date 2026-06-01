import assert from "node:assert/strict";
import { test } from "node:test";

import { TikTokExtractor } from "../src/index.js";

test("extracts metadata from fixture response", async () => {
  const html = `<script id="SIGI_STATE" type="application/json">{
    "ItemModule":{"123":{"id":"123","desc":"hello",
    "author":{"id":"u1","uniqueId":"tester","nickname":"Tester"},
    "stats":{"playCount":10,"diggCount":2,"shareCount":1,"commentCount":3},
    "video":{"duration":7,"cover":"https://img","playAddr":["https://video"]},
    "music":{"title":"sound","playUrl":"https://audio"}}}}</script>`;
  const fakeFetch = async (url) => ({
    ok: true,
    url,
    text: async () => html,
  });
  const extractor = new TikTokExtractor({ fetch: fakeFetch, useOEmbed: false });
  const meta = await extractor.extract("https://www.tiktok.com/@tester/video/123");

  assert.equal(meta.video_id, "123");
  assert.equal(meta.view_count, 10);
  assert.equal(meta.quality, "complete");
});
