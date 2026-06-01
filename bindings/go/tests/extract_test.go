package tests

import (
	"context"
	"net/http"
	"net/http/httptest"
	"testing"

	tiktokdirect "github.com/codewithwan/tiktok-direct-go"
)

func TestExtractFromFixtureServer(t *testing.T) {
	server := httptest.NewServer(http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		w.Write([]byte(`<script id="SIGI_STATE" type="application/json">{
			"ItemModule":{"123":{"id":"123","desc":"hello",
			"author":{"id":"u1","uniqueId":"tester","nickname":"Tester"},
			"stats":{"playCount":10,"diggCount":2,"shareCount":1,"commentCount":3},
			"video":{"duration":7,"cover":"https://img","playAddr":["https://video"]},
			"music":{"title":"sound","playUrl":"https://audio"}}}}</script>`))
	}))
	defer server.Close()

	extractor := tiktokdirect.New()
	extractor.UseOEmbed = false
	meta, err := extractor.Extract(context.Background(), server.URL+"/@tester/video/123")
	if err != nil {
		t.Fatal(err)
	}
	if meta["video_id"] != "123" || meta["quality"] != "complete" {
		t.Fatalf("unexpected metadata: %#v", meta)
	}
}
