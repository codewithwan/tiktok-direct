package tests

import (
	"context"
	"testing"

	tiktokdirect "github.com/codewithwan/tiktok-direct-go"
)

const URL = "https://vt.tiktok.com/ZSxvYRvoR/"

func TestInvalidUrlRaisesError(t *testing.T) {
	extractor := tiktokdirect.New()
	_, err := extractor.Extract(context.Background(), "invalid-url")
	if err == nil {
		t.Error("Expected error for invalid url, got nil")
	}

	_, err = tiktokdirect.Extract(context.Background(), "invalid-url")
	if err == nil {
		t.Error("Expected error for invalid url in package function, got nil")
	}
}

func TestExtractRealUrl(t *testing.T) {
	extractor := tiktokdirect.New()
	meta, err := extractor.Extract(context.Background(), URL)
	if err != nil {
		t.Fatalf("Failed to extract real URL: %v", err)
	}

	if meta["quality"] != "complete" && meta["quality"] != "partial" {
		t.Errorf("Expected complete or partial, got %v", meta["quality"])
	}
	if fmtID := meta["video_id"]; fmtID == nil || fmtID == "" {
		t.Error("Expected video_id, got empty")
	}
	if fmtTitle := meta["title"]; fmtTitle == nil || fmtTitle == "" {
		t.Error("Expected title, got empty")
	}
}
