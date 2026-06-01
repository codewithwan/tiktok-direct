package tests

import (
	"context"
	"testing"

	tiktokdirect "github.com/codewithwan/tiktok-direct-go"
)

func TestInvalidUrlRaisesErrorAsync(t *testing.T) {
	extractor := tiktokdirect.New()
	
	errChan := make(chan error, 1)
	go func() {
		_, err := extractor.Extract(context.Background(), "invalid-url")
		errChan <- err
	}()
	
	err := <-errChan
	if err == nil {
		t.Error("Expected error for invalid url asynchronously, got nil")
	}
}

func TestExtractRealUrlAsync(t *testing.T) {
	extractor := tiktokdirect.New()
	
	type result struct {
		meta map[string]any
		err  error
	}
	resChan := make(chan result, 1)
	
	go func() {
		meta, err := extractor.Extract(context.Background(), URL)
		resChan <- result{meta, err}
	}()
	
	res := <-resChan
	if res.err != nil {
		t.Fatalf("Failed to extract real URL async: %v", res.err)
	}

	if res.meta["quality"] != "complete" && res.meta["quality"] != "partial" {
		t.Errorf("Expected complete or partial, got %v", res.meta["quality"])
	}
	if fmtID := res.meta["video_id"]; fmtID == nil || fmtID == "" {
		t.Error("Expected video_id, got empty")
	}
	if fmtTitle := res.meta["title"]; fmtTitle == nil || fmtTitle == "" {
		t.Error("Expected title, got empty")
	}
}
