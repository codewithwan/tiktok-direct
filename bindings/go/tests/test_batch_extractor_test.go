package tests

import (
	"context"
	"sync"
	"testing"

	tiktokdirect "github.com/codewithwan/tiktok-direct-go"
)

func TestExtractManyRealUrl(t *testing.T) {
	extractor := tiktokdirect.New()

	urls := []string{
		URL,
		"invalid-url",
	}

	type result struct {
		url  string
		meta map[string]any
		err  error
	}

	var wg sync.WaitGroup
	resChan := make(chan result, len(urls))

	for _, u := range urls {
		wg.Add(1)
		go func(targetURL string) {
			defer wg.Done()
			meta, err := extractor.Extract(context.Background(), targetURL)
			resChan <- result{targetURL, meta, err}
		}(u)
	}

	wg.Wait()
	close(resChan)

	results := make(map[string]result)
	for r := range resChan {
		results[r.url] = r
	}

	if len(results) != 2 {
		t.Errorf("Expected 2 results, got %d", len(results))
	}

	valid := results[URL]
	if valid.err != nil {
		t.Errorf("Expected valid extraction, got error: %v", valid.err)
	}
	if valid.meta["quality"] != "complete" && valid.meta["quality"] != "partial" {
		t.Errorf("Expected complete or partial, got %v", valid.meta["quality"])
	}
	if fmtID := valid.meta["video_id"]; fmtID == nil || fmtID == "" {
		t.Error("Expected video_id, got empty")
	}

	invalid := results["invalid-url"]
	if invalid.err == nil {
		t.Error("Expected error for invalid URL, got nil")
	}
}
