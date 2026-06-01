#!/usr/bin/env python3
"""
Media download example using tiktok-direct Python bindings.
"""

import os
from tiktok_direct import TikTokExtractor, download


def main():
    # Example video URL (uses a public video)
    url = "https://vt.tiktok.com/example/"
    output_dir = "downloads"

    # Ensure output directory exists
    os.makedirs(output_dir, exist_ok=True)

    print(f"Downloading media for: {url}")
    print(f"Destination folder: {output_dir}\n")

    # Method 1: Download MP4 (Video) using the functional helper
    try:
        print("Downloading MP4 video...")
        mp4_path = download(url, "mp4", output_dir)
        print(f"Successfully downloaded MP4 to: {mp4_path}")
    except Exception as e:
        print(f"Failed to download MP4: {e}")

    # Method 2: Download MP3 (Audio) using the TikTokExtractor class
    try:
        print("\nDownloading MP3 audio...")
        extractor = TikTokExtractor()
        mp3_path = extractor.download(url, "mp3", output_dir)
        print(f"Successfully downloaded MP3 to: {mp3_path}")
    except Exception as e:
        print(f"Failed to download MP3: {e}")


if __name__ == "__main__":
    main()
