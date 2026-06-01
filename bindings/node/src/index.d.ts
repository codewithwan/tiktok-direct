export interface ExtractorOptions {
  userAgent?: string;
  acceptLanguage?: string;
  fetch?: typeof fetch;
  useOEmbed?: boolean;
}

export interface VideoMetadata {
  input_url: string;
  resolved_url: string;
  video_id?: string;
  username?: string;
  author_name?: string;
  author_unique_id?: string;
  title?: string;
  description?: string;
  thumbnail_url?: string;
  view_count?: number;
  like_count?: number;
  repost_count?: number;
  comment_count?: number;
  duration?: number;
  timestamp?: string;
  webpage_url: string;
  source?: string | null;
  quality: "complete" | "partial" | "failed";
  media: Record<string, unknown>;
  music: Record<string, unknown>;
}

export class TikTokExtractor {
  constructor(options?: ExtractorOptions);
  extract(url: string): Promise<VideoMetadata>;
  download(url: string, kind: "mp4" | "mp3" | "thumbnail", output?: string): Promise<string>;
}

export function extract(url: string, options?: ExtractorOptions): Promise<VideoMetadata>;

export function download(
  url: string,
  kind: "mp4" | "mp3" | "thumbnail",
  output?: string,
  options?: ExtractorOptions,
): Promise<string>;
