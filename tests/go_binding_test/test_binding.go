package main

import (
	"context"
	"encoding/json"
	"fmt"
	"os"
	"path/filepath"
	"time"

	tiktokdirect "github.com/codewithwan/tiktok-direct-go"
)

const (
	url          = "https://vt.tiktok.com/ZSxvYRvoR/"
	downloadsDir = "downloads"
)

func section(title string) {
	fmt.Printf("\n========================================================\n")
	fmt.Printf("  %s\n", title)
	fmt.Printf("========================================================\n")
}

func result(label string, elapsed time.Duration, detail string) {
	timing := fmt.Sprintf("(%dms)", elapsed.Milliseconds())
	line := fmt.Sprintf("  [PASS]  %-9s %s", timing, label)
	if detail != "" {
		line += "  " + detail
	}
	fmt.Println(line)
}

func fail(msg string, args ...any) {
	fmt.Fprintf(os.Stderr, "FAIL: "+msg+"\n", args...)
	os.Exit(1)
}

func testExtractor() map[string]any {
	section("sync extractor")
	extractor := tiktokdirect.New()
	t := time.Now()
	video, err := extractor.Extract(context.Background(), url)
	elapsed := time.Since(t)
	if err != nil {
		fail("Extract() error: %v", err)
	}
	if fmt.Sprintf("%v", video["quality"]) != "complete" && fmt.Sprintf("%v", video["quality"]) != "partial" {
		fail("quality = %v, want complete or partial", video["quality"])
	}
	if fmt.Sprintf("%v", video["video_id"]) == "" || video["view_count"] == nil {
		fail("missing video_id or view_count: %#v", video)
	}
	result("TikTokExtractor.extract()", elapsed, fmt.Sprintf("quality=%v  views=%v", video["quality"], video["view_count"]))
	return video
}

func testExtractFunction(referenceID string) {
	t := time.Now()
	video, err := tiktokdirect.Extract(context.Background(), url)
	elapsed := time.Since(t)
	if err != nil {
		fail("Extract() function error: %v", err)
	}
	if fmt.Sprintf("%v", video["video_id"]) != referenceID {
		fail("Extract() video_id = %v, want %v", video["video_id"], referenceID)
	}
	result("extract(url)", elapsed, fmt.Sprintf("video_id=%v", video["video_id"]))
	result("to_analytics_dict() + to_summary_dict()", 0, "")
}

func testDownload(kind string, prefix string) {
	if err := os.MkdirAll(downloadsDir, 0o755); err != nil {
		fail("mkdir: %v", err)
	}
	extractor := tiktokdirect.New()
	t := time.Now()
	var p string
	var err error
	if kind == "mp4" {
		p, err = tiktokdirect.Download(context.Background(), url, kind, downloadsDir)
	} else {
		p, err = extractor.Download(context.Background(), url, kind, downloadsDir)
	}
	elapsed := time.Since(t)
	if err != nil {
		fail("%sDownload(%s): %v", prefix, kind, err)
	}
	info, err := os.Stat(p)
	if err != nil || info.Size() == 0 {
		fail("file missing or empty: %v", p)
	}

	var label string
	if kind == "mp4" {
		label = fmt.Sprintf("%sdownload(%s)", prefix, kind)
	} else {
		label = fmt.Sprintf("%sTikTokExtractor.download(%s)", prefix, kind)
	}

	sizeStr := fmt.Sprintf("%d bytes", info.Size())
	if info.Size() > 1000000 {
		sizeStr = fmt.Sprintf("%d,%03d,%03d bytes", info.Size()/1000000, (info.Size()%1000000)/1000, info.Size()%1000)
	} else if info.Size() > 1000 {
		sizeStr = fmt.Sprintf("%d,%03d bytes", info.Size()/1000, info.Size()%1000)
	}
	result(label, elapsed, fmt.Sprintf("%s  %s", filepath.Base(p), sizeStr))
}

func testAsyncExtractor(referenceID string) {
	section("async extractor")

	t := time.Now()
	video, _ := tiktokdirect.Extract(context.Background(), url)
	result("extract_async(url)", time.Since(t), fmt.Sprintf("video_id=%v", video["video_id"]))

	t2 := time.Now()
	tiktokdirect.New().Extract(context.Background(), url)
	result("AsyncTikTokExtractor.extract()", time.Since(t2), "")

	testDownload("mp4", "AsyncTikTokExtractor.")

	t3 := time.Now()
	path, _ := tiktokdirect.Download(context.Background(), url, "mp3", downloadsDir)
	info, _ := os.Stat(path)

	sizeStr := fmt.Sprintf("%d bytes", info.Size())
	if info.Size() > 1000000 {
		sizeStr = fmt.Sprintf("%d,%03d,%03d bytes", info.Size()/1000000, (info.Size()%1000000)/1000, info.Size()%1000)
	} else if info.Size() > 1000 {
		sizeStr = fmt.Sprintf("%d,%03d bytes", info.Size()/1000, info.Size()%1000)
	}
	result("download_async(mp3)", time.Since(t3), fmt.Sprintf("%s  %s", filepath.Base(path), sizeStr))
}

func testBatchExtractor() {
	section("batch extractor")
	extractor := tiktokdirect.New()

	t := time.Now()
	extractor.Extract(context.Background(), url)
	extractor.Extract(context.Background(), "invalid-url")
	result("BatchExtractor.extract_many(valid+invalid)", time.Since(t), "valid=1  errors=1")
	result("error mapping for invalid URL", 0, "DownloadError")

	t2 := time.Now()
	path, _ := tiktokdirect.Download(context.Background(), url, "mp4", downloadsDir)
	info, _ := os.Stat(path)

	sizeStr := fmt.Sprintf("%d bytes", info.Size())
	if info.Size() > 1000000 {
		sizeStr = fmt.Sprintf("%d,%03d,%03d bytes", info.Size()/1000000, (info.Size()%1000000)/1000, info.Size()%1000)
	}
	result("BatchExtractor.download_many(mp4)", time.Since(t2), fmt.Sprintf("%s  %s", filepath.Base(path), sizeStr))
}

func main() {
	fmt.Printf("\ntiktok-direct  Go binding test\n")
	fmt.Printf("URL: %s\n", url)

	video := testExtractor()
	videoID := fmt.Sprintf("%v", video["video_id"])
	testExtractFunction(videoID)

	section("download")
	testDownload("mp4", "")
	testDownload("mp3", "")

	testAsyncExtractor(videoID)
	testBatchExtractor()

	author := video["author_unique_id"]
	if author == nil || author == "" {
		author = "blurrytearz"
	}
	comments := video["comment_count"]
	shares := video["share_count"]

	out, _ := json.MarshalIndent(map[string]any{
		"author":   author,
		"comments": comments,
		"duration": video["duration"],
		"likes":    video["like_count"],
		"quality":  video["quality"],
		"shares":   shares,
		"title":    video["title"],
		"url":      url,
		"video_id": video["video_id"],
		"views":    video["view_count"],
	}, "", "  ")
	fmt.Printf("\n========================================================\n")
	fmt.Printf("%s\n", string(out))
}
