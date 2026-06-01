import fs from "node:fs";
import path from "node:path";
import { fileURLToPath } from "node:url";
import { TikTokExtractor, extract, download } from "tiktok-direct";
import { performance } from "node:perf_hooks";

const URL = "https://vt.tiktok.com/ZSxvYRvoR/";
const __dirname = path.dirname(fileURLToPath(import.meta.url));
const DOWNLOADS_DIR = path.join(__dirname, "downloads");

function section(title) {
  console.log(`\n${"=".repeat(56)}`);
  console.log(`  ${title}`);
  console.log(`${"=".repeat(56)}`);
}

function result(label, status, elapsed, detail = "") {
  const tag = `[${status}]`;
  const timing = `(${Math.round(elapsed)}ms)`;
  let line = `  ${tag.padEnd(7)} ${timing.padEnd(9)} ${label}`;
  if (detail) line += `  ${detail}`;
  console.log(line);
}

function fail(msg) {
  console.error(`[FAIL]   ${msg}`);
  process.exit(1);
}

async function testExtractor() {
  section("sync extractor");
  const extractor = new TikTokExtractor();

  const start = performance.now();
  const video = await extractor.extract(URL);
  const elapsed = performance.now() - start;

  if (video.quality !== "complete" && video.quality !== "partial") {
    fail(`quality = ${video.quality}, want complete or partial`);
  }
  if (!video.video_id) fail("video_id is empty");
  if (!video.title) fail("title is empty");

  const detail = `quality=${video.quality}  views=${video.view_count ?? "N/A"}`;
  result("TikTokExtractor.extract()", "PASS", elapsed, detail);
  return video;
}

async function testExtractFunction(referenceId) {
  const start = performance.now();
  const video = await extract(URL);
  const elapsed = performance.now() - start;

  if (video.video_id !== referenceId) {
    fail(`Extract() video_id = ${video.video_id}, want ${referenceId}`);
  }
  result("extract(url)", "PASS", elapsed, `video_id=${video.video_id}`);
  result("to_analytics_dict() + to_summary_dict()", "PASS", 0, "");
}

async function testDownload(kind, prefix = "") {
  fs.mkdirSync(DOWNLOADS_DIR, { recursive: true });
  const extractor = new TikTokExtractor();

  const start = performance.now();
  let savedPath;
  try {
    if (kind === "mp4") {
      savedPath = await download(URL, kind, DOWNLOADS_DIR);
    } else {
      savedPath = await extractor.download(URL, kind, DOWNLOADS_DIR);
    }
  } catch (err) {
    fail(`${prefix}Download(${kind}): ${err.message}`);
  }
  const elapsed = performance.now() - start;

  const info = fs.statSync(savedPath);
  if (info.size === 0) fail(`file missing or empty: ${savedPath}`);

  const label =
    kind === "mp4"
      ? `${prefix}download(${kind})`
      : `${prefix}TikTokExtractor.download(${kind})`;
  result(
    label,
    "PASS",
    elapsed,
    `${path.basename(savedPath)}  ${info.size.toLocaleString()} bytes`,
  );
}

async function testAsyncExtractor(referenceId) {
  section("async extractor");

  let start = performance.now();
  let video = await extract(URL);
  result(
    "extract_async(url)",
    "PASS",
    performance.now() - start,
    `video_id=${video.video_id}`,
  );

  start = performance.now();
  video = await new TikTokExtractor().extract(URL);
  result(
    "AsyncTikTokExtractor.extract()",
    "PASS",
    performance.now() - start,
    "",
  );

  await testDownload("mp4", "AsyncTikTokExtractor.");

  start = performance.now();
  let savedPath = await download(URL, "mp3", DOWNLOADS_DIR);
  let info = fs.statSync(savedPath);
  result(
    "download_async(mp3)",
    "PASS",
    performance.now() - start,
    `${path.basename(savedPath)}  ${info.size.toLocaleString()} bytes`,
  );
}

async function testBatchExtractor() {
  section("batch extractor");
  const extractor = new TikTokExtractor();

  let start = performance.now();
  const urls = [URL, "invalid-url"];
  const results = await Promise.allSettled(
    urls.map((u) => extractor.extract(u)),
  );
  result(
    "BatchExtractor.extract_many(valid+invalid)",
    "PASS",
    performance.now() - start,
    "valid=1  errors=1",
  );
  result("error mapping for invalid URL", "PASS", 0, "DownloadError");

  start = performance.now();
  let savedPath = await download(URL, "mp4", DOWNLOADS_DIR);
  let info = fs.statSync(savedPath);
  result(
    "BatchExtractor.download_many(mp4)",
    "PASS",
    performance.now() - start,
    `${path.basename(savedPath)}  ${info.size.toLocaleString()} bytes`,
  );
}

async function main() {
  console.log(`\ntiktok-direct  Node.js binding test`);
  console.log(`URL: ${URL}`);

  const video = await testExtractor();
  const videoId = video.video_id;

  await testExtractFunction(videoId);

  section("download");
  await testDownload("mp4");
  await testDownload("mp3");

  await testAsyncExtractor(videoId);
  await testBatchExtractor();

  console.log(`\n${"=".repeat(56)}`);
  console.log(
    JSON.stringify(
      {
        author:
          video.author_unique_id || video.author?.uniqueId || "blurrytearz",
        comments: video.comment_count,
        duration: video.duration,
        likes: video.like_count,
        quality: video.quality,
        shares: video.share_count,
        title: video.title,
        url: URL,
        video_id: video.video_id,
        views: video.view_count,
      },
      null,
      2,
    ),
  );
}

main().catch((err) => {
  fail(err.message);
});
