#!/usr/bin/env python3
"""Poll /models until target model reports loaded. Prints final state.
Usage: wait-loaded.py --model NAME [--timeout 120] [--include]
"""
import argparse
import json
import sys
import time
import urllib.request


def states():
    with urllib.request.urlopen("http://localhost:8080/models",
                                timeout=5) as r:
        return {m["id"]: m["status"]["value"]
                for m in json.load(r)["data"]}


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("--model", required=True)
    ap.add_argument("--timeout", type=int, default=120)
    args = ap.parse_args()

    deadline = time.time() + args.timeout
    while time.time() < deadline:
        try:
            st = states()
            matches = {k: v for k, v in st.items() if args.model in k}
            exact = matches.get(args.model)
            if exact == "loaded":
                print("loaded")
                return 0
            if exact == "loading":
                time.sleep(5)
                continue
            print(f"state:{exact or matches or 'absent'}")
            return 1
        except Exception:
            time.sleep(5)
    print("timeout")
    return 1


if __name__ == "__main__":
    sys.exit(main())
