package tiktokdirect

import (
	"context"
	"errors"
	"io"
	"net/http"
	"os"
	"path/filepath"
)

func Download(ctx context.Context, videoURL, kind, output string) (string, error) {
	return New().Download(ctx, videoURL, kind, output)
}

func (e *Extractor) Download(ctx context.Context, videoURL, kind, output string) (string, error) {
	meta, err := e.Extract(ctx, videoURL)
	if err != nil {
		return "", err
	}
	urls := mediaURLs(meta, kind)
	if len(urls) == 0 {
		return "", errors.New("no media URL is available")
	}
	path := outputPath(meta, kind, output)
	for _, mediaURL := range urls {
		if err := e.downloadOne(ctx, mediaURL, path, str(meta["webpage_url"])); err == nil {
			return path, nil
		}
		_ = os.Remove(path)
	}
	return "", errors.New("all media candidates failed")
}

func mediaURLs(meta map[string]any, kind string) []string {
	media := obj(meta, "media")
	music := obj(meta, "music")
	switch kind {
	case "mp4":
		return unique([]string{str(media["play_addr"]), str(media["download_addr"])})
	case "mp3":
		return unique([]string{str(music["play_url"])})
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
	req.Header.Set("User-Agent", e.UserAgent)
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
