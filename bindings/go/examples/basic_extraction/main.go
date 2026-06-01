package main

import (
	"context"
	"encoding/json"
	"fmt"
	"os"

	tiktokdirect "github.com/codewithwan/tiktok-direct-go"
)

func main() {
	if len(os.Args) < 2 {
		fmt.Fprintln(os.Stderr, "usage: go run ./examples/basic_extraction <tiktok-url>")
		os.Exit(1)
	}
	meta, err := tiktokdirect.Extract(context.Background(), os.Args[1])
	if err != nil {
		fmt.Fprintln(os.Stderr, err)
		os.Exit(1)
	}
	_ = json.NewEncoder(os.Stdout).Encode(meta)
}
