package main

import (
	"context"
	"fmt"
	"os"

	tiktokdirect "github.com/codewithwan/tiktok-direct-go"
)

func main() {
	if len(os.Args) < 3 {
		fmt.Fprintln(os.Stderr, "usage: go run ./examples/download_media <tiktok-url> <mp4|mp3|thumbnail>")
		os.Exit(1)
	}
	path, err := tiktokdirect.Download(context.Background(), os.Args[1], os.Args[2], "downloads/")
	if err != nil {
		fmt.Fprintln(os.Stderr, err)
		os.Exit(1)
	}
	fmt.Println(path)
}
