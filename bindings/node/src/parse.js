import { string } from "./values.js";

export function parseScript(page, id) {
  const pattern = `<script[^>]+id=["']${escapeRegExp(id)}["'][^>]*>(.*?)</script>`;
  const match = page.match(new RegExp(pattern, "is"));
  if (!match) return null;
  try {
    return JSON.parse(unescapeHtml(match[1].trim()));
  } catch {
    return null;
  }
}

export function selectItem(sources, videoId) {
  for (const name of ["SIGI_STATE", "__UNIVERSAL_DATA_FOR_REHYDRATION__", "__NEXT_DATA__"]) {
    const item = findItem(sources[name], videoId);
    if (item) return [name, item];
  }
  return [null, null];
}

export async function mergeOEmbed(extractor, meta, pageUrl) {
  try {
    const endpoint = `https://www.tiktok.com/oembed?url=${encodeURIComponent(pageUrl)}`;
    const { text } = await extractor.fetchText(endpoint, "application/json");
    const data = JSON.parse(text);
    meta.author_name ||= string(data.author_name);
    meta.author_url ||= string(data.author_url);
    meta.title ||= string(data.title);
    meta.thumbnail_url ||= string(data.thumbnail_url);
  } catch {}
}

function findItem(value, videoId) {
  if (Array.isArray(value)) {
    for (const child of value) {
      const item = findItem(child, videoId);
      if (item) return item;
    }
  } else if (value && typeof value === "object") {
    if (value.ItemModule) return value.ItemModule[videoId] || Object.values(value.ItemModule)[0];
    if (value.itemStruct && matches(value.itemStruct, videoId)) return value.itemStruct;
    if (matches(value, videoId)) return value;
    for (const child of Object.values(value)) {
      const item = findItem(child, videoId);
      if (item) return item;
    }
  }
  return null;
}

function matches(item, videoId) {
  const id = String(item?.id || item?.awemeId || "");
  return id && (!videoId || id === videoId) && ["video", "stats", "author", "desc"].some((key) => key in item);
}

const escapeRegExp = (text) => text.replace(/[.*+?^${}()|[\]\\]/g, "\\$&");
const unescapeHtml = (text) => text.replace(/&quot;/g, '"').replace(/&amp;/g, "&").replace(/&#x27;/g, "'");
