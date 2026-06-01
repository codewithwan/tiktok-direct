package tiktokdirect

import (
	"context"
	"encoding/json"
	"errors"
	"fmt"
	"html"
	"io"
	"net/http"
	"net/url"
	"regexp"
	"strings"
	"time"
)

type Extractor struct {
	Client        *http.Client
	AcceptLang    string
	UseOEmbed     bool
	CommentPolicy string
}

func New() *Extractor {
	return &Extractor{
		Client:     &http.Client{Timeout: 20 * time.Second},
		AcceptLang: "en-US,en;q=0.9,id;q=0.8",
		UseOEmbed:  true,
	}
}

func Extract(ctx context.Context, videoURL string) (map[string]any, error) {
	return New().Extract(ctx, videoURL)
}

func (e *Extractor) Extract(ctx context.Context, videoURL string) (map[string]any, error) {
	var best map[string]any
	var last error
	for attempt := 0; attempt < 5; attempt++ {
		meta, err := e.extractOnce(ctx, videoURL)
		if err != nil {
			last = err
			continue
		}
		if fmt.Sprintf("%v", meta["quality"]) == "complete" {
			return meta, nil
		}
		best = meta
	}
	if best != nil {
		return best, nil
	}
	return nil, last
}

func (e *Extractor) extractOnce(ctx context.Context, videoURL string) (map[string]any, error) {
	finalURL, body, err := e.fetchText(ctx, videoURL, "text/html")
	if err != nil {
		return nil, err
	}
	videoID := firstMatch(`/((?:video|photo))/(\d+)`, finalURL, 2)
	if videoID == "" {
		videoID = firstMatch(`/((?:video|photo))/(\d+)`, videoURL, 2)
	}
	username := firstMatch(`tiktok\.com/@([^/?#]+)`, finalURL, 1)
	sources := map[string]any{
		"SIGI_STATE":                         parseScript(body, "SIGI_STATE"),
		"__UNIVERSAL_DATA_FOR_REHYDRATION__": parseScript(body, "__UNIVERSAL_DATA_FOR_REHYDRATION__"),
		"__NEXT_DATA__":                      parseScript(body, "__NEXT_DATA__"),
	}
	source, item := selectItem(sources, videoID)
	meta := normalize(videoURL, finalURL, username, source, item)
	if e.UseOEmbed {
		mergeOEmbed(ctx, e, meta, finalURL)
	}
	meta["quality"] = quality(meta)
	return meta, nil
}

func (e *Extractor) fetchText(ctx context.Context, target, accept string) (string, string, error) {
	req, err := http.NewRequestWithContext(ctx, http.MethodGet, target, nil)
	if err != nil {
		return "", "", err
	}
	req.Header.Set("Accept-Language", e.AcceptLang)
	req.Header.Set("Accept", accept)
	resp, err := e.Client.Do(req)
	if err != nil {
		return "", "", err
	}
	defer resp.Body.Close()
	if resp.StatusCode < 200 || resp.StatusCode >= 300 {
		return "", "", errors.New(resp.Status)
	}
	bytes, err := io.ReadAll(resp.Body)
	return resp.Request.URL.String(), string(bytes), err
}

func parseScript(page, id string) any {
	pattern := `(?is)<script[^>]+id=["']` + regexp.QuoteMeta(id) + `["'][^>]*>(.*?)</script>`
	raw := firstMatch(pattern, page, 1)
	if raw == "" {
		return nil
	}
	var out any
	if json.Unmarshal([]byte(html.UnescapeString(raw)), &out) == nil {
		return out
	}
	return nil
}

func firstMatch(pattern, text string, index int) string {
	match := regexp.MustCompile(pattern).FindStringSubmatch(text)
	if len(match) > index {
		return html.UnescapeString(strings.TrimSpace(match[index]))
	}
	return ""
}

func mergeOEmbed(ctx context.Context, e *Extractor, meta map[string]any, pageURL string) {
	endpoint := "https://www.tiktok.com/oembed?url=" + url.QueryEscape(pageURL)
	_, body, err := e.fetchText(ctx, endpoint, "application/json")
	if err != nil {
		return
	}
	var payload map[string]any
	if json.Unmarshal([]byte(body), &payload) != nil {
		return
	}
	fill(meta, "author_name", str(payload["author_name"]))
	fill(meta, "author_url", str(payload["author_url"]))
	fill(meta, "title", str(payload["title"]))
	fill(meta, "thumbnail_url", str(payload["thumbnail_url"]))
}

func fill(meta map[string]any, key string, value string) {
	if value != "" && (meta[key] == nil || meta[key] == "") {
		meta[key] = value
	}
}
