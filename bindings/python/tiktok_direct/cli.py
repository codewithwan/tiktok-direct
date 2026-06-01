import argparse
import csv
import json
import sys
from pathlib import Path
from typing import Any, Dict, Iterable, List

from . import TikTokExtractor, to_analytics_dict, to_summary_dict


def main() -> int:
    parser = argparse.ArgumentParser(prog="tiktok-direct")
    parser.add_argument("url", nargs="?", help="TikTok video URL")
    parser.add_argument("--input", help="File with one URL per line")
    parser.add_argument("--download", choices=["mp4", "mp3", "thumbnail", "avatar"])
    parser.add_argument("--output", help="Output file or directory")
    parser.add_argument("--format", choices=["json", "jsonl", "csv"], default="json")
    parser.add_argument("--summary", action="store_true")
    parser.add_argument("--raw", action="store_true")
    parser.add_argument("--raw-only", action="store_true")
    parser.add_argument("--quiet", action="store_true")
    parser.add_argument("--compact", action="store_true")
    parser.add_argument("--pretty", action="store_true")
    parser.add_argument("--max-workers", type=int, default=4)
    args = parser.parse_args()

    urls = list(read_urls(args))
    if not urls:
        parser.error("provide a URL or --input file")

    extractor = TikTokExtractor()
    if args.input:
        rows = [run_one(extractor, url, args) for url in urls]
        write_batch(rows, args)
        return 0

    row = run_one(extractor, urls[0], args)
    if args.quiet and args.download and row.get("downloaded"):
        print(row["downloaded"])
        return 0
    if row["ok"]:
        print_json(select_payload(row["metadata"], args), args)
        return 0
    print_json(row, args)
    return 1


def read_urls(args: argparse.Namespace) -> Iterable[str]:
    if args.input:
        for line in Path(args.input).read_text(encoding="utf-8").splitlines():
            line = line.strip()
            if line and not line.startswith("#"):
                yield line
    elif args.url:
        yield args.url


def run_one(extractor: TikTokExtractor, url: str, args: argparse.Namespace) -> Dict[str, Any]:
    try:
        metadata = extractor.extract(url)
        downloaded = None
        if args.download:
            downloaded = extractor.download(url, args.download, args.output)
        return {
            "ok": True,
            "url": url,
            "quality": metadata.get("quality"),
            "metadata": metadata,
            "downloaded": downloaded,
        }
    except Exception as exc:
        return {
            "ok": False,
            "url": url,
            "quality": "failed",
            "error_type": type(exc).__name__,
            "error_message": str(exc),
        }


def select_payload(metadata: Dict[str, Any], args: argparse.Namespace) -> Dict[str, Any]:
    if args.raw_only:
        return metadata.get("raw_item") or {}
    if args.summary or args.quiet:
        return to_summary_dict(metadata)
    if not args.raw:
        metadata = dict(metadata)
        metadata.pop("raw_item", None)
    return metadata


def write_batch(rows: List[Dict[str, Any]], args: argparse.Namespace) -> None:
    if args.format == "jsonl":
        text = "\n".join(json.dumps(row, ensure_ascii=False) for row in rows)
    elif args.format == "csv":
        text = rows_to_csv(rows)
    else:
        text = json.dumps(rows, ensure_ascii=False, indent=None if args.compact else 2)
    if args.output and not args.download:
        Path(args.output).write_text(text + "\n", encoding="utf-8")
    else:
        print(text)


def rows_to_csv(rows: List[Dict[str, Any]]) -> str:
    import io

    data = []
    for row in rows:
        if row["ok"]:
            item = {"ok": True, "url": row["url"], **to_analytics_dict(row["metadata"])}
        else:
            item = dict(row)
        data.append(item)
    fields = sorted({key for item in data for key in item})
    buffer = io.StringIO()
    writer = csv.DictWriter(buffer, fieldnames=fields)
    writer.writeheader()
    writer.writerows(data)
    return buffer.getvalue().rstrip()


def print_json(value: Any, args: argparse.Namespace) -> None:
    indent = None if args.compact else 2
    print(json.dumps(value, ensure_ascii=False, indent=indent))


if __name__ == "__main__":
    raise SystemExit(main())
