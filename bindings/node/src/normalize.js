import { number, pickUrl, string } from "./values.js";

export function normalize(inputUrl, finalUrl, username, source, item = {}) {
  const author = item?.author || {};
  const stats = item?.stats || {};
  const statsV2 = item?.statsV2 || {};
  const video = item?.video || {};
  const music = item?.music || {};
  const meta = {
    input_url: inputUrl,
    resolved_url: finalUrl,
    video_id: string(item.id) || videoIdFrom(finalUrl),
    username,
    author_name: string(author.nickname),
    author_unique_id: string(author.uniqueId),
    title: string(item.desc),
    description: string(item.desc),
    thumbnail_url: pickUrl(video.cover),
    view_count: number(stats.playCount) ?? number(statsV2.playCount),
    like_count: number(stats.diggCount) ?? number(statsV2.diggCount),
    repost_count: number(stats.shareCount) ?? number(statsV2.shareCount),
    comment_count: number(stats.commentCount) ?? number(statsV2.commentCount),
    duration: number(video.duration),
    timestamp: string(item.createTime),
    webpage_url: finalUrl,
    source,
    media: media(video),
    music: { title: string(music.title), play_url: pickUrl(music.playUrl) },
  };
  meta.quality = source && meta.view_count != null && meta.duration != null ? "complete" : meta.title ? "partial" : "failed";
  return meta;
}

export const videoIdFrom = (url) => url.match(/\/(?:video|photo)\/(\d+)/)?.[1] || "";
export const usernameFrom = (url) => url.match(/tiktok\.com\/@([^/?#]+)/)?.[1] || "";

function media(video) {
  return {
    cover: pickUrl(video.cover),
    play_addr: pickUrl(video.playAddr),
    download_addr: pickUrl(video.downloadAddr),
  };
}
