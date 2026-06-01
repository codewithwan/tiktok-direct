package tiktokdirect

import (
	"context"
	"errors"
	"io"
	"net/http"
	"os"
	"path/filepath"
	"strings"
)

func Download(ctx context.Context, videoURL, kind, output string) (string, error) {
	return New().Download(ctx, videoURL, kind, output)
}

func (e *Extractor) Download(ctx context.Context, videoURL, kind, output string) (string, error) {
	var last error
	for attempt := 0; attempt < 3; attempt++ {
		meta, err := e.Extract(ctx, videoURL)
		if err != nil {
			last = err
			continue
		}
		urls := mediaURLs(meta, kind)
		if len(urls) == 0 {
			last = errors.New("no media URL is available")
			continue
		}
		path := outputPath(meta, kind, output)
		for _, mediaURL := range urls {
			if err := e.downloadOne(ctx, mediaURL, path, str(meta["webpage_url"])); err == nil {
				return path, nil
			} else {
				last = err
			}
			_ = os.Remove(path)
		}
	}
	if last != nil {
		return "", last
	}
	return "", errors.New("all media candidates failed")
}

func mediaURLs(meta map[string]any, kind string) []string {
	media := obj(meta, "media")
	music := obj(meta, "music")
	raw := obj(meta, "raw_item")
	switch kind {
	case "mp4":
		urls := []string{str(media["play_addr"]), str(media["download_addr"])}
		return unique(append(urls, collectNested(raw["video"])...))
	case "mp3":
		urls := []string{str(music["play_url"])}
		return unique(append(urls, collectNested(obj(raw, "music")["playUrl"])...))
	case "thumbnail":
		return unique([]string{str(meta["thumbnail_url"]), str(media["cover"])})
	default:
		return nil
	}
}

func (e *Extractor) downloadOne(ctx context.Context, url, path, referer string) error {
	req, err := http.NewRequestWithContext(ctx, http.MethodGet, url, nil)
	if err != nil {
		return err
	}
	req.Header.Set("Referer", referer)
	resp, err := e.Client.Do(req)
	if err != nil {
		return err
	}
	defer resp.Body.Close()
	if resp.StatusCode < 200 || resp.StatusCode >= 300 {
		return errors.New(resp.Status)
	}
	if err := os.MkdirAll(filepath.Dir(path), 0o755); err != nil {
		return err
	}
	file, err := os.Create(path)
	if err != nil {
		return err
	}
	defer file.Close()
	_, err = io.Copy(file, resp.Body)
	return err
}

func collectNested(value any) []string {
	var urls []string
	switch typed := value.(type) {
	case string:
		urls = append(urls, typed)
	case []any:
		for _, item := range typed {
			urls = append(urls, collectNested(item)...)
		}
	case map[string]any:
		for key, item := range typed {
			lower := strings.ToLower(key)
			if key == "bitrateInfo" || strings.HasSuffix(lower, "addr") || strings.HasSuffix(lower, "url") || strings.Contains(lower, "urllist") {
				urls = append(urls, collectNested(item)...)
			}
		}
	}
	return urls
}
