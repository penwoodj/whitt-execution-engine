#!/usr/bin/env python3
"""Baseline manager: promote/compare/hash-bound drift detection.

Usage:
  baseline.py promote --artifact FILE --criteria YAML --baseline-dir DIR
  baseline.py compare --artifact FILE --baseline-dir DIR
  baseline.py hash --artifact FILE
"""
import argparse, hashlib, json, os, sys
from pathlib import Path


def sha256_str(s):
    return hashlib.sha256(s.encode()).hexdigest()


def promote(artifact_path, criteria_path, baseline_dir):
    text = Path(artifact_path).read_text(encoding="utf-8", errors="replace")
    content_hash = sha256_str(text)
    size = len(text)
    words = len(text.split())
    lines = len(text.splitlines())

    meta = {
        "content_hash": content_hash,
        "size_bytes": size,
        "word_count": words,
        "line_count": lines,
        "artifact_path": str(artifact_path),
        "criteria_path": str(criteria_path) if criteria_path else None,
    }

    baseline_file = Path(baseline_dir) / "baseline.json"
    content_file = Path(baseline_dir) / "baseline.txt"
    baseline_file.parent.mkdir(parents=True, exist_ok=True)

    baseline_file.write_text(json.dumps(meta, indent=2) + "\n")
    content_file.write_text(text, encoding="utf-8")

    print(json.dumps({"action": "promoted", "hash": content_hash, "words": words, "lines": lines}))
    return 0


def compare(artifact_path, baseline_dir):
    text = Path(artifact_path).read_text(encoding="utf-8", errors="replace")
    content_hash = sha256_str(text)

    baseline_file = Path(baseline_dir) / "baseline.json"
    if not baseline_file.is_file():
        print(json.dumps({"drift": "no_baseline", "hash": content_hash}))
        return 2

    meta = json.loads(baseline_file.read_text())
    baseline_hash = meta["content_hash"]
    drift = content_hash != baseline_hash

    size = len(text)
    size_delta = size - meta["size_bytes"]
    word_delta = len(text.split()) - meta["word_count"]

    result = {
        "drift": drift,
        "hash_match": not drift,
        "current_hash": content_hash,
        "baseline_hash": baseline_hash,
        "size_delta": size_delta,
        "word_delta": word_delta,
    }
    print(json.dumps(result))
    return 1 if drift else 0


def hash_only(artifact_path):
    text = Path(artifact_path).read_text(encoding="utf-8", errors="replace")
    print(json.dumps({"hash": sha256_str(text)}))
    return 0


def main():
    p = argparse.ArgumentParser(description="Baseline manager")
    sub = p.add_subparsers(dest="cmd")

    pr = sub.add_parser("promote")
    pr.add_argument("--artifact", required=True)
    pr.add_argument("--criteria")
    pr.add_argument("--baseline-dir", required=True)

    co = sub.add_parser("compare")
    co.add_argument("--artifact", required=True)
    co.add_argument("--baseline-dir", required=True)

    ha = sub.add_parser("hash")
    ha.add_argument("--artifact", required=True)

    args = p.parse_args()
    if args.cmd == "promote":
        sys.exit(promote(args.artifact, args.criteria, args.baseline_dir))
    elif args.cmd == "compare":
        sys.exit(compare(args.artifact, args.baseline_dir))
    elif args.cmd == "hash":
        sys.exit(hash_only(args.artifact))
    else:
        p.print_help()
        sys.exit(2)


if __name__ == "__main__":
    main()