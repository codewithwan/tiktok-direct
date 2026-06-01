package tiktokdirect

import "strconv"

func obj(parent map[string]any, key string) map[string]any {
	if parent == nil {
		return map[string]any{}
	}
	if value, ok := parent[key].(map[string]any); ok {
		return value
	}
	return map[string]any{}
}

func str(value any) string {
	switch typed := value.(type) {
	case string:
		return typed
	case float64:
		return strconv.FormatInt(int64(typed), 10)
	default:
		return ""
	}
}

func num(value any) any {
	switch typed := value.(type) {
	case float64:
		return int64(typed)
	case string:
		if parsed, err := strconv.ParseInt(typed, 10, 64); err == nil {
			return parsed
		}
	}
	return nil
}

func choose(first, second string) string {
	if first != "" {
		return first
	}
	return second
}

func chooseNum(first, second any) any {
	if value := num(first); value != nil {
		return value
	}
	return num(second)
}

func pickURL(value any) string {
	switch typed := value.(type) {
	case string:
		return typed
	case []any:
		for _, item := range typed {
			if text, ok := item.(string); ok {
				return text
			}
		}
	case map[string]any:
		for _, key := range []string{"urlList", "url_list", "urls"} {
			if url := pickURL(typed[key]); url != "" {
				return url
			}
		}
	}
	return ""
}
