import { execFile } from "node:child_process";
import { existsSync } from "node:fs";
import { dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";
import { promisify } from "node:util";

import { downloadMedia } from "./download.js";
import { mergeOEmbed, parseScript, selectItem } from "./parse.js";
import { normalize, usernameFrom, videoIdFrom } from "./normalize.js";

const execFileAsync = promisify(execFile);
const packageRoot = resolve(dirname(fileURLToPath(import.meta.url)), "..", "..", "..");
const gatewayName = process.platform === "win32" ? "tiktok-direct-gateway.exe" : "tiktok-direct-gateway";
const gatewayPath = resolve(packageRoot, "target", "release", gatewayName);

export class TikTokExtractor {
  constructor(options = {}) {
    this.acceptLanguage = options.acceptLanguage || "en-US,en;q=0.9,id;q=0.8";
    this.fetch = options.fetch;
    this.useOEmbed = options.useOEmbed ?? true;
  }

  async extract(url) {
    if (!this.fetch) return gateway("extract", [url], true);

    let best;
    let lastError;
    for (let attempt = 0; attempt < 5; attempt += 1) {
      try {
        const meta = await this.extractOnce(url);
        if (meta.quality === "complete") return meta;
        best = meta;
      } catch (error) {
        lastError = error;
      }
    }
    if (best) return best;
    throw lastError;
  }

  async extractOnce(url) {
    const { finalUrl, text } = await this.fetchText(url, "text/html");
    const videoId = videoIdFrom(finalUrl) || videoIdFrom(url);
    const username = usernameFrom(finalUrl) || usernameFrom(url);
    const sources = {
      SIGI_STATE: parseScript(text, "SIGI_STATE"),
      __UNIVERSAL_DATA_FOR_REHYDRATION__: parseScript(text, "__UNIVERSAL_DATA_FOR_REHYDRATION__"),
      __NEXT_DATA__: parseScript(text, "__NEXT_DATA__"),
    };
    const [source, item] = selectItem(sources, videoId);
    const meta = normalize(url, finalUrl, username, source, item);
    if (this.useOEmbed) await mergeOEmbed(this, meta, finalUrl);
    meta.quality = meta.source && meta.view_count != null && meta.duration != null ? "complete" : meta.title ? "partial" : "failed";
    return meta;
  }

  async download(url, kind, output = "") {
    if (!this.fetch) return gateway("download", [url, kind, output], false);

    let lastError;
    for (let attempt = 0; attempt < 3; attempt += 1) {
      try {
        return await downloadMedia(this, await this.extract(url), kind, output);
      } catch (error) {
        lastError = error;
      }
    }
    throw lastError;
  }

  async fetchText(url, accept) {
    const response = await this.fetch(url, { headers: this.headers(accept) });
    if (!response.ok) throw new Error(`${response.status} ${response.statusText}`);
    return { finalUrl: response.url || url, text: await response.text() };
  }

  headers(accept, referer) {
    const h = {
      "accept-language": this.acceptLanguage,
      "accept": accept,
      "accept-encoding": "gzip, deflate, br",
      "upgrade-insecure-requests": "1",
      "sec-fetch-dest": "document",
      "sec-fetch-mode": "navigate",
      "sec-fetch-site": "none",
      "sec-fetch-user": "?1",
      "cache-control": "max-age=0",
    };
    if (referer) h.referer = referer;
    return h;
  }
}

export async function extract(url, options) {
  return new TikTokExtractor(options).extract(url);
}

export async function download(url, kind, output, options) {
  return new TikTokExtractor(options).download(url, kind, output);
}

async function gateway(command, args, json) {
  const allArgs = [command, ...args.filter((arg) => arg !== undefined && arg !== "")];
  const result = existsSync(gatewayPath)
    ? await execFileAsync(gatewayPath, allArgs, { cwd: packageRoot })
    : await execFileAsync("cargo", ["run", "--quiet", "-p", "tiktok-direct-gateway", "--", ...allArgs], {
        cwd: packageRoot,
      });
  const output = result.stdout.trim();
  return json ? JSON.parse(output) : output;
}
