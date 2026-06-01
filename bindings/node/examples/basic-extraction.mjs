import { extract } from "../src/index.js";

const url = process.argv[2];
if (!url) {
  console.error("usage: node examples/basic-extraction.mjs <tiktok-url>");
  process.exit(1);
}

console.log(JSON.stringify(await extract(url), null, 2));
