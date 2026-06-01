import { download } from "../src/index.js";

const [url, kind = "mp4"] = process.argv.slice(2);
if (!url) {
  console.error("usage: node examples/download-media.mjs <tiktok-url> [mp4|mp3|thumbnail]");
  process.exit(1);
}

console.log(await download(url, kind, "downloads/"));
