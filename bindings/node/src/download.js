import { mkdir, stat, writeFile } from "node:fs/promises";
import { dirname, join } from "node:path";

import { unique } from "./values.js";

export async function downloadMedia(extractor, meta, kind, output) {
  const urls = mediaUrls(meta, kind);
  if (!urls.length) throw new Error("no media URL is available");
  const path = await outputPath(meta, kind, output);
  for (const mediaUrl of urls) {
    try {
      const headers = { ...extractor.headers("*/*", meta.webpage_url), range: "bytes=0-" };
      const response = await extractor.fetch(mediaUrl, { headers });
      if (!response.ok) throw new Error(`${response.status} ${response.statusText}`);
      const bytes = Buffer.from(await response.arrayBuffer());
      await mkdir(dirname(path), { recursive: true });
      await writeFile(path, bytes);
      return path;
    } catch {}
  }
  throw new Error("all media candidates failed");
}

function mediaUrls(meta, kind) {
  if (kind === "mp4") {
    return unique([meta.media?.play_addr, meta.media?.download_addr, ...collectNested(meta.raw_item?.video)]);
  }
  if (kind === "mp3") return unique([meta.music?.play_url, ...collectNested(meta.raw_item?.music?.playUrl)]);
  if (kind === "thumbnail") return unique([meta.thumbnail_url, meta.media?.cover]);
  return [];
}

async function outputPath(meta, kind, output = "") {
  if (!output || output.endsWith("/") || output.endsWith("\\")) {
    return join(output || "", defaultFilename(meta, kind));
  }
  try {
    if ((await stat(output)).isDirectory()) return join(output, defaultFilename(meta, kind));
  } catch {}
  return output;
}

function defaultFilename(meta, kind) {
  const ext = kind === "thumbnail" || kind === "avatar" ? "jpg" : kind;
  return `${meta.username || "unknown"}_${meta.video_id || "tiktok"}.${ext}`;
}

function collectNested(value) {
  if (typeof value === "string") return [value];
  if (Array.isArray(value)) return value.flatMap(collectNested);
  if (!value || typeof value !== "object") return [];
  return Object.entries(value).flatMap(([key, item]) => {
    const lower = key.toLowerCase();
    if (key === "bitrateInfo" || lower.endsWith("addr") || lower.endsWith("url") || lower.includes("urllist")) {
      return collectNested(item);
    }
    return [];
  });
}
