package tiktokdirect

func selectItem(sources map[string]any, videoID string) (string, map[string]any) {
	for _, name := range []string{"SIGI_STATE", "__UNIVERSAL_DATA_FOR_REHYDRATION__", "__NEXT_DATA__"} {
		if item := findItem(sources[name], videoID); item != nil {
			return name, item
		}
	}
	return "", nil
}

func findItem(value any, videoID string) map[string]any {
	switch typed := value.(type) {
	case map[string]any:
		if module, ok := typed["ItemModule"].(map[string]any); ok {
			if item, ok := module[videoID].(map[string]any); ok {
				return item
			}
			for _, value := range module {
				if item, ok := value.(map[string]any); ok {
					return item
				}
			}
		}
		if item, ok := typed["itemStruct"].(map[string]any); ok && matches(item, videoID) {
			return item
		}
		if matches(typed, videoID) {
			return typed
		}
		for _, child := range typed {
			if item := findItem(child, videoID); item != nil {
				return item
			}
		}
	case []any:
		for _, child := range typed {
			if item := findItem(child, videoID); item != nil {
				return item
			}
		}
	}
	return nil
}

func matches(item map[string]any, videoID string) bool {
	id := str(item["id"])
	if id == "" {
		id = str(item["awemeId"])
	}
	if id == "" || (videoID != "" && id != videoID) {
		return false
	}
	return item["video"] != nil || item["stats"] != nil || item["author"] != nil || item["desc"] != nil
}

func normalize(inputURL, finalURL, username, source string, item map[string]any) map[string]any {
	author := obj(item, "author")
	stats := obj(item, "stats")
	statsV2 := obj(item, "statsV2")
	video := obj(item, "video")
	music := obj(item, "music")
	out := map[string]any{
		"input_url":        inputURL,
		"resolved_url":     finalURL,
		"video_id":         choose(str(item["id"]), firstMatch(`/((?:video|photo))/(\d+)`, finalURL, 2)),
		"username":         username,
		"author_name":      str(author["nickname"]),
		"author_unique_id": str(author["uniqueId"]),
		"title":            str(item["desc"]),
		"description":      str(item["desc"]),
		"thumbnail_url":    pickURL(video["cover"]),
		"view_count":       chooseNum(stats["playCount"], statsV2["playCount"]),
		"like_count":       chooseNum(stats["diggCount"], statsV2["diggCount"]),
		"repost_count":     chooseNum(stats["shareCount"], statsV2["shareCount"]),
		"comment_count":    chooseNum(stats["commentCount"], statsV2["commentCount"]),
		"duration":         num(video["duration"]),
		"timestamp":        str(item["createTime"]),
		"uploader":         str(author["uniqueId"]),
		"uploader_id":      str(author["id"]),
		"webpage_url":      finalURL,
		"source":           source,
		"raw_item":         item,
		"media": map[string]any{
			"cover":         pickURL(video["cover"]),
			"play_addr":     pickURL(video["playAddr"]),
			"download_addr": pickURL(video["downloadAddr"]),
		},
		"music": map[string]any{"play_url": pickURL(music["playUrl"]), "title": str(music["title"])},
	}
	out["quality"] = quality(out)
	return out
}

func quality(meta map[string]any) string {
	if meta["source"] != "" && meta["view_count"] != nil && meta["duration"] != nil {
		return "complete"
	}
	if meta["title"] != "" || meta["thumbnail_url"] != "" {
		return "partial"
	}
	return "failed"
}
