package tiktokdirect

import (
	"os"
	"path/filepath"
	"strings"
)

func outputPath(meta map[string]any, kind, output string) string {
	if output == "" {
		output = defaultFilename(meta, kind)
	}
	if info, err := os.Stat(output); err == nil && info.IsDir() {
		return filepath.Join(output, defaultFilename(meta, kind))
	}
	if strings.HasSuffix(output, "/") || strings.HasSuffix(output, "\\") {
		return filepath.Join(output, defaultFilename(meta, kind))
	}
	return output
}

func defaultFilename(meta map[string]any, kind string) string {
	ext := kind
	if kind == "thumbnail" || kind == "avatar" {
		ext = "jpg"
	}
	username := str(meta["username"])
	if username == "" {
		username = "unknown"
	}
	videoID := str(meta["video_id"])
	if videoID == "" {
		videoID = "tiktok"
	}
	return username + "_" + videoID + "." + ext
}

func unique(values []string) []string {
	out := make([]string, 0, len(values))
	seen := map[string]bool{}
	for _, value := range values {
		if strings.HasPrefix(value, "http") && !seen[value] {
			out = append(out, value)
			seen[value] = true
		}
	}
	return out
}
