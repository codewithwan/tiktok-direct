# tiktok-direct

`tiktok-direct` is a library-first workspace for extracting public TikTok video metadata. It is designed around one shared Rust implementation package with thin language bindings on top.

## Overview

This repository is meant to be consumed as libraries, not as a CLI app. The extraction logic lives in one Rust implementation package, while language-specific bindings expose the same behavior to downstream consumers.

## Repository Layout

- `crates/engine` - shared Rust implementation package
- `bindings/python` - Python binding built with PyO3 and maturin
- `bindings/node` - reserved for a future Node.js binding
- `bindings/go` - reserved for a future Go binding

## Design Goals

- Keep extraction, parsing, normalization, and media discovery in one shared Rust implementation package.
- Expose the same public behavior through language bindings.
- Use only public TikTok page responses, public oEmbed data, and temporary challenge cookies derived from public challenge pages.
- Avoid login flows, user cookies, and authenticated scraping.
- Keep the repository focused on reusable library packages.

## Current Status

- Public TikTok video metadata extraction is implemented in the Rust implementation package.
- Public web challenge handling is implemented.
- Rehydration JSON parsing is implemented.
- Public oEmbed fallback is implemented.
- MP4 and MP3 download support is implemented.
- The Python binding is implemented.
- Unit tests, line-count checks, and an ignored live public test are present.

## Rust Implementation

The shared Rust package lives in `crates/engine` and exposes the implementation used by the bindings.

- resolving public TikTok URLs
- handling public web challenge pages
- parsing TikTok rehydration JSON and public metadata
- normalizing extracted fields into typed Rust data
- discovering MP4, MP3, and thumbnail media URLs
- downloading public media from the normalized extraction result

It is the implementation layer used by the language bindings and by downstream Rust consumers that want the extraction logic directly.

## Package Documentation

Implementation details, local build steps, and package-specific usage examples live in the package-level README files:

- [Python binding README](bindings/python/README.md)

## Public Data Policy

- No login flow.
- No user-provided cookies.
- No authenticated scraping.
- Public TikTok page data and public oEmbed data only.
- Temporary challenge cookies may be generated from public challenge pages when TikTok serves a challenge response.
