#!/usr/bin/env python3
"""Rank probed models by category, pick 5 specialists.
Reads results/model-probe/*/summary.json. Writes RANKING.md and
MODEL-PICKS.md (the stage-1 gate artifact).

Pick rules (docs/03-MODEL-SELECTION.md): argmax category accuracy,
wall tiebreak, 0/4 cannot win, one model wins at most 2 categories.
"""
import json
import sys
from pathlib import Path

HERE = Path(__file__).resolve().parent
EXP = HERE.parent
PROBE_DIR = EXP / "results/model-probe"
CATEGORIES = ["FMT", "DRV", "LOG", "PLN", "AUD"]


def frac(s):
    try:
        a, b = s.split("/")
        return int(a) / int(b) if int(b) else 0.0
    except (ValueError, AttributeError):
        return 0.0


def main():
    import argparse
    ap = argparse.ArgumentParser()
    ap.add_argument("--min-models", type=int, default=6)
    args = ap.parse_args()

    models = []
    for d in sorted(PROBE_DIR.iterdir() if PROBE_DIR.exists() else []):
        sf = d / "summary.json"
        if not (d.is_dir() and sf.exists()):
            continue
        s = json.loads(sf.read_text())
        entry = {"model": d.name, "wall_s": s.get("wall_s"),
                 "total": frac(s.get("total_pass", "0/0"))}
        for c in CATEGORIES:
            entry[c] = frac(s.get("category_pass", {}).get(c, "0/0"))
        models.append(entry)

    if len(models) < 2:
        sys.exit(f"FAIL: need >=2 probed models, found {len(models)}")
    partial = len(models) < args.min_models

    lines = ["| model | total | FMT | DRV | LOG | PLN | AUD | wall_s |",
             "|---|---|---|---|---|---|---|---|"]
    for m in models:
        lines.append(
            f"| {m['model']} | {m['total']:.2f} | {m['FMT']:.2f} | "
            f"{m['DRV']:.2f} | {m['LOG']:.2f} | {m['PLN']:.2f} | "
            f"{m['AUD']:.2f} | {m['wall_s']} |")
    rank_name = "RANKING-PARTIAL.md" if partial else "RANKING.md"
    rank_header = ("# Stage-1 probe RANKING (PARTIAL — "
                   f"{len(models)}/{args.min_models} models)\n\n"
                   if partial else "# Stage-1 probe RANKING\n\n")
    (PROBE_DIR / rank_name).write_text(
        rank_header + "\n".join(lines) + "\n")

    if partial:
        withheld = ["# MODEL-PICKS — WITHHELD (partial sweep)", "",
                    f"Only {len(models)} models probed; need "
                    f"{args.min_models}. Picks invalid until sweep completes.",
                    "", "STAGE-2 gate: CLOSED", "",
                    "Probed so far:", ""]
        (EXP / "results/MODEL-PICKS.md").write_text(
            "\n".join(withheld + lines) + "\n")
        print(f"PARTIAL: {len(models)}/{args.min_models} models — "
              "gate stays CLOSED, picks withheld")
        print("\n".join(lines))
        return 0

    picks = {}
    wins = {}
    for c in CATEGORIES:
        cands = [m for m in models if m[c] > 0]
        cands.sort(key=lambda m: (-m[c], m["wall_s"] or 9e9))
        for m in cands:
            if wins.get(m["model"], 0) < 2:
                picks[c] = {"model": m["model"], "score": m[c],
                            "wall_s": m["wall_s"]}
                wins[m["model"]] = wins.get(m["model"], 0) + 1
                break
        else:
            picks[c] = {"model": None, "score": 0.0, "wall_s": None}

    out = ["# MODEL-PICKS — stage-1 gate artifact", "",
           "5 specialists from live probe results.", "",
           "| category | model | probe score | wall_s |", "|---|---|---|---|"]
    for c in CATEGORIES:
        p = picks[c]
        out.append(f"| {c} | {p['model']} | {p['score']:.2f} | "
                   f"{p['wall_s']} |")
    distinct = {p["model"] for p in picks.values() if p["model"]}
    out += ["", f"Distinct models: {len(distinct)}", "",
            "STAGE-2 gate: OPEN (case authoring permitted)"]
    (EXP / "results/MODEL-PICKS.md").write_text("\n".join(out) + "\n")
    print("\n".join(lines))
    print()
    print("\n".join(out[4:10]))
    return 0


if __name__ == "__main__":
    sys.exit(main())
