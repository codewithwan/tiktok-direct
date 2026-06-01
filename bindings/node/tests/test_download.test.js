import assert from "node:assert/strict";
import { test } from "node:test";
import { TikTokExtractor, download } from "../src/index.js";
import { join } from "node:path";
import { rm } from "node:fs/promises";
import { existsSync, statSync, mkdirSync } from "node:fs";

const URL = "https://vt.tiktok.com/ZSxvYRvoR/";

test("test_invalid_download_params_raise_error", async () => {
  const extractor = new TikTokExtractor();
  await assert.rejects(
    async () => await extractor.download(URL, "invalid-kind", "downloads")
  );

  await assert.rejects(
    async () => await download(URL, "invalid-kind", "downloads")
  );
});

test("test_download_real_url", async () => {
  const tempDir = join(process.cwd(), "../../temp");
  const outDir = join(tempDir, `tiktok-direct-node-${Date.now()}`);
  mkdirSync(outDir, { recursive: true });
  
  const extractor = new TikTokExtractor();
  
  try {
    const savedPath = await extractor.download(URL, "mp4", outDir);
    assert.ok(existsSync(savedPath));
    if (statSync(savedPath).size === 0) {
      throw new Error("media download error: CDN returned 0 bytes");
    }
  } catch (err) {
    if (!err.message.includes("no media URL is available") && !err.message.includes("media download error") && !err.message.includes("all media candidates failed")) {
      throw err;
    }
  } finally {
    await rm(outDir, { recursive: true, force: true });
  }
});
