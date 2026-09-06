#!/usr/bin/env python3
"""Top-level conf gate: prompt -> lane token (LIGHT/HEAVY/SYNTH).

Reads a probe file. Two probe formats:
  {"lane": "HEAVY"}                      -- pre-classified (spoof/dry-run)
  {"logprobs": [...], "multi_part": b}   -- live: entropy + structure cue
Entropy >= 0.8 or multi_part -> HEAVY/SYNTH; else LIGHT.
Prints a single quoted token on stdout for GWT routing.
"""
import argparse
import json
import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))
from fas_lib import conf_route  # noqa: E402

def emit_token(tok):
    """Print routing token WITHOUT trailing newline.

    Engine GWT compares raw shell stdout: `{{bookmarks.shell_output.stdout}}
    == "PASS"` after template substitution becomes `<stdout> == "PASS"`.
    A trailing newline breaks the lexer (expr error -> false). The dry-run
    simulator strips whitespace, so no-newline output stays compatible.
    """
    sys.stdout.write(f'"{tok}"')
    sys.stdout.flush()


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("--probe", required=True)
    ap.add_argument("--default", default="LIGHT")
    a = ap.parse_args()

    lane = a.default
    p = Path(a.probe)
    if p.exists():
        try:
            d = json.loads(p.read_text())
        except json.JSONDecodeError:
            d = {}
        if isinstance(d, dict):
            if "lane" in d:
                lane = str(d["lane"])
            elif "logprobs" in d:
                lp = d["logprobs"]
                heavy = conf_route(lp) == "HEAVY" if lp else False
                if d.get("multi_part"):
                    lane = "SYNTH"
                elif heavy:
                    lane = "HEAVY"
    emit_token(lane)


if __name__ == "__main__":
    main()
