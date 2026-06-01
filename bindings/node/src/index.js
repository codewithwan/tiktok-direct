import { downloadMedia } from "./download.js";
import { mergeOEmbed, parseScript, selectItem } from "./parse.js";
import { normalize, usernameFrom, videoIdFrom } from "./normalize.js";

const UA = "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 Chrome/124 Safari/537.36";

export class TikTokExtractor {
  constructor(options = {}) {
    this.userAgent = options.userAgent || UA;
    this.acceptLanguage = options.acceptLanguage || "en-US,en;q=0.9,id;q=0.8";
    this.fetch = options.fetch || globalThis.fetch;
    this.useOEmbed = options.useOEmbed ?? true;
  }

  async extract(url) {
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
    return meta;
  }

  async download(url, kind, output = "") {
    return downloadMedia(this, await this.extract(url), kind, output);
  }

  async fetchText(url, accept) {
    const response = await this.fetch(url, { headers: this.headers(accept) });
    if (!response.ok) throw new Error(`${response.status} ${response.statusText}`);
    return { finalUrl: response.url || url, text: await response.text() };
  }

  headers(accept, referer) {
    const headers = {
      "user-agent": this.userAgent,
      "accept-language": this.acceptLanguage,
      accept,
    };
    if (referer) headers.referer = referer;
    return headers;
  }
}

export async function extract(url, options) {
  return new TikTokExtractor(options).extract(url);
}

export async function download(url, kind, output, options) {
  return new TikTokExtractor(options).download(url, kind, output);
}
