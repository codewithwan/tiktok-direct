package tests

import (
	"context"
	"fmt"
	"os"
	"path/filepath"
	"strings"
	"testing"
	"time"

	tiktokdirect "github.com/codewithwan/tiktok-direct-go"
)

func TestInvalidDownloadParamsRaiseError(t *testing.T) {
	extractor := tiktokdirect.New()
	_, err := extractor.Download(context.Background(), URL, "invalid-kind", "downloads")
	if err == nil {
		t.Error("Expected error for invalid kind, got nil")
	}

	_, err = tiktokdirect.Download(context.Background(), URL, "invalid-kind", "downloads")
	if err == nil {
		t.Error("Expected error for invalid kind in package function, got nil")
	}
}

func TestDownloadRealUrl(t *testing.T) {
	outDir := filepath.Join("..", "..", "..", "temp", fmt.Sprintf("tiktok-direct-go-%d", time.Now().UnixNano()))
	err := os.MkdirAll(outDir, 0755)
	if err != nil {
		t.Fatalf("Failed to create temp dir: %v", err)
	}
	defer os.RemoveAll(outDir)

	extractor := tiktokdirect.New()
	path, err := extractor.Download(context.Background(), URL, "mp4", outDir)
	
	if err != nil {
		errStr := err.Error()
		if !strings.Contains(errStr, "no media URL") && !strings.Contains(errStr, "all media candidates failed") {
			t.Fatalf("Unexpected download error: %v", err)
		}
	} else {
		info, err := os.Stat(path)
		if err != nil {
			t.Fatalf("File not created: %v", err)
		}
		if info.Size() == 0 {
			t.Error("Expected non-empty file")
		}
	}
}
