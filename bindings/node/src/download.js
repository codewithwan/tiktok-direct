import { mkdir, writeFile } from "node:fs/promises";
import { dirname, join } from "node:path";

import { unique } from "./values.js";

export async function downloadMedia(extractor, meta, kind, output) {
  const urls = mediaUrls(meta, kind);
  if (!urls.length) throw new Error("no media URL is available");
  const path = outputPath(meta, kind, output);
  for (const mediaUrl of urls) {
    try {
      const response = await extractor.fetch(mediaUrl, {
        headers: extractor.headers("*/*", meta.webpage_url),
      });
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
  if (kind === "mp4") return unique([meta.media?.play_addr, meta.media?.download_addr]);
  if (kind === "mp3") return unique([meta.music?.play_url]);
  if (kind === "thumbnail") return unique([meta.thumbnail_url, meta.media?.cover]);
  return [];
}

function outputPath(meta, kind, output = "") {
  if (!output || output.endsWith("/") || output.endsWith("\\")) {
    return join(output || "", defaultFilename(meta, kind));
  }
  return output;
}

function defaultFilename(meta, kind) {
  const ext = kind === "thumbnail" || kind === "avatar" ? "jpg" : kind;
  return `${meta.username || "unknown"}_${meta.video_id || "tiktok"}.${ext}`;
}
