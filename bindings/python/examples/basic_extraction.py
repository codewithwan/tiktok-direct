#!/usr/bin/env python3
"""
Basic metadata extraction example using tiktok-direct Python bindings.
"""

from tiktok_direct import TikTokExtractor, extract


def main():
    # Example video URL (uses a public video)
    url = "https://vt.tiktok.com/ZSxvYRvoR/"

    print(f"Extracting metadata from: {url}\n")

    # Method 1: Using the functional `extract` helper
    try:
        metadata = extract(url)
        print("--- Extraction Method 1: Functional ---")
        print(f"ID: {metadata.get('id')}")
        print(f"Title: {metadata.get('title')}")
        print(f"Author: @{metadata.get('author', {}).get('username')}")
        print(f"View Count: {metadata.get('view_count')}")
        print(f"Like Count: {metadata.get('like_count')}")
        print("-" * 40)
    except Exception as e:
        print(f"Error during functional extraction: {e}")

    # Method 2: Using the OOP `TikTokExtractor` class
    try:
        extractor = TikTokExtractor()
        metadata_oop = extractor.extract(url)
        print("\n--- Extraction Method 2: Object-Oriented ---")
        print(f"Music Title: {metadata_oop.get('music', {}).get('title')}")
        print(f"Duration: {metadata_oop.get('duration')}s")
        print(f"Quality: {metadata_oop.get('quality')}")
        print("-" * 40)
    except Exception as e:
        print(f"Error during OOP extraction: {e}")


if __name__ == "__main__":
    main()
