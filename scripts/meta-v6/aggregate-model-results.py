#!/usr/bin/env python3
"""Aggregate model comparison results across all batch dirs.

Scans docs/benchmarks/outputs/meta-workflow/model-comparison/stress-*/
collects every summary.tsv, merges into single DataFrame,
outputs sorted ranking + markdown report.

Usage: python3 aggregate-model-results.py [--out REPORT.md]
"""
import argparse
import csv
import os
import sys
from pathlib import Path


def load_all_summaries(root: Path) -> list[dict]:
    rows = []
    if not root.is_dir():
        return rows
    for batch_dir in sorted(root.iterdir()):
        if not batch_dir.is_dir() or not batch_dir.name.startswith("stress-"):
            continue
        tsv = batch_dir / "summary.tsv"
        if not tsv.exists():
            continue
        with open(tsv) as f:
            reader = csv.DictReader(f, delimiter="\t")
            for r in reader:
                # Skip missing-model artifacts from aborted smoke tests
                if r.get("verdict") == "MISSING" or r.get("model", "").startswith("nonexistent"):
                    continue
                r["batch"] = batch_dir.name
                rows.append(r)
    return rows


def coerce(row: dict) -> dict:
    out = dict(row)
    # Old format used `step_NN_quality` and `iterations` was absent (1-iter run).
    # New time-based format uses `best_step_NN_quality` + `iterations` field.
    if "best_step_02_quality" not in out and "step_02_quality" in out:
        out["best_step_02_quality"] = out["step_02_quality"]
        out["best_step_03_quality"] = out["step_03_quality"]
        out.setdefault("iterations", 1)
        out.setdefault("time_used_sec", out.get("duration_sec", 0))
        out.setdefault("total_tokens",
                       int(out.get("step_02_tokens", 0)) + int(out.get("step_03_tokens", 0)))
    for k in ("iterations", "time_used_sec", "total_tokens", "total_bytes", "refusals_detected"):
        try:
            out[k] = int(out.get(k, 0) or 0)
        except (ValueError, TypeError):
            out[k] = 0
    # Normalize field name: refusals_detected -> refusals
    if "refusals" not in out:
        out["refusals"] = out.get("refusals_detected", 0)
    for k in ("best_s2_quality", "best_s3_quality"):
        src = "best_step_02_quality" if k == "best_s2_quality" else "best_step_03_quality"
        try:
            out[k] = float(out.get(src, 0) or 0)
        except (ValueError, TypeError):
            out[k] = 0.0
    return out


def render_markdown(rows: list[dict]) -> str:
    if not rows:
        return "# Model Aggregation Report\n\nNo data found.\n"

    # Deduplicate: keep BEST result per (model, config) across batches
    seen = {}
    for r in rows:
        r = coerce(r)
        key = (r.get("model", ""), r.get("config", ""))
        if key not in seen:
            seen[key] = r
        else:
            prev = seen[key]
            # Keep one with higher combined score (s2+s3)
            if (r["best_s2_quality"] + r["best_s3_quality"]) > (
                prev["best_s2_quality"] + prev["best_s3_quality"]
            ):
                seen[key] = r
    unique = list(seen.values())

    # Sort: by best_s3 desc (most reliable signal), then s2 desc, then iters desc
    unique.sort(
        key=lambda r: (
            r["best_s3_quality"],
            r["best_s2_quality"],
            r["iterations"],
        ),
        reverse=True,
    )

    lines = []
    lines.append("# Model Aggregation Report — Stress Test")
    lines.append("")
    lines.append(
        f"**Generated:** {os.popen('date -Iseconds').read().strip()}\n"
        f"**Total unique (model, config) pairs:** {len(unique)}\n"
        f"**Source batches:** {len({r['batch'] for r in unique})}\n"
    )
    lines.append("## Methodology\n")
    lines.append(
        "- Each model runs the `model-stress-test.yml` workflow repeatedly within a "
        "180s time budget.\n"
        "- Per-iteration outputs land in `iter-N/`.\n"
        "- Quality scores from step_02 (long-form code gen) and step_03 "
        "(multi-perspective eval) are tracked.\n"
        "- `best_sN_quality` = highest quality_score seen across iterations for step N.\n"
        "- Verdict: PASS requires best_s2_quality > 0.5 AND refusals < iterations.\n"
    )

    # Ranking table
    lines.append("## Ranking (sorted by best_s3 desc, then s2 desc)\n")
    lines.append(
        "| Rank | Model | Config | Iters | best_s2 | best_s3 | tokens | bytes | refusals | verdict |"
    )
    lines.append("|------|-------|--------|-------|---------|---------|--------|-------|----------|---------|")
    for i, r in enumerate(unique, 1):
        lines.append(
            f"| {i} | `{r['model']}` | {r.get('config', '?')} | {r['iterations']} | "
            f"{r['best_s2_quality']:.3f} | **{r['best_s3_quality']:.3f}** | "
            f"{r['total_tokens']} | {r['total_bytes']} | {r['refusals']} | "
            f"{r.get('verdict', '?')} |"
        )

    # Top 5 contenders section
    lines.append("\n## Top 5 Contenders\n")
    for i, r in enumerate(unique[:5], 1):
        lines.append(f"### #{i} `{r['model']}` ({r.get('config', '?')})")
        lines.append(
            f"- best_s2_quality: **{r['best_s2_quality']:.3f}**\n"
            f"- best_s3_quality: **{r['best_s3_quality']:.3f}**\n"
            f"- iterations in 180s: {r['iterations']}\n"
            f"- total tokens: {r['total_tokens']}\n"
            f"- total bytes: {r['total_bytes']}\n"
            f"- refusals detected: {r['refusals']}\n"
            f"- verdict: {r.get('verdict', '?')}\n"
            f"- source batch: `{r['batch']}`\n"
        )

    # Insights
    lines.append("## Insights\n")
    s3_perfect = [r for r in unique if r["best_s3_quality"] >= 0.99]
    s2_above_0_4 = [r for r in unique if r["best_s2_quality"] >= 0.4]
    s2_above_0_3 = [r for r in unique if r["best_s2_quality"] >= 0.3]
    zero_refusal = [r for r in unique if r["refusals"] == 0 and r["iterations"] > 0]
    most_iters = max(unique, key=lambda r: r["iterations"]) if unique else None
    lines.append(
        f"- **s3 quality = 1.0:** {len(s3_perfect)} models — "
        f"{', '.join(r['model'] for r in s3_perfect)}\n"
        f"- **s2 quality >= 0.4:** {len(s2_above_0_4)} models\n"
        f"- **s2 quality >= 0.3:** {len(s2_above_0_3)} models — "
        f"{', '.join(r['model'] for r in s2_above_0_3)}\n"
        f"- **zero refusals (and ran):** {len(zero_refusal)} models\n"
    )
    if most_iters:
        lines.append(
            f"- **most iterations in 180s:** `{most_iters['model']}` "
            f"with {most_iters['iterations']}\n"
        )

    # Per-config breakdown
    by_config = {}
    for r in unique:
        c = r.get("config", "unknown")
        by_config.setdefault(c, []).append(r)
    lines.append("\n## Per-Config Summary\n")
    lines.append("| config | models tested | avg best_s3 | avg best_s2 | avg iters |")
    lines.append("|--------|---------------|-------------|-------------|-----------|")
    for c, rs in sorted(by_config.items()):
        n = len(rs)
        avg_s3 = sum(r["best_s3_quality"] for r in rs) / n if n else 0
        avg_s2 = sum(r["best_s2_quality"] for r in rs) / n if n else 0
        avg_it = sum(r["iterations"] for r in rs) / n if n else 0
        lines.append(
            f"| {c} | {n} | {avg_s3:.3f} | {avg_s2:.3f} | {avg_it:.1f} |"
        )

    # Recommendation
    lines.append("\n## Recommendation\n")
    if unique:
        top = unique[0]
        lines.append(
            f"**Top overall:** `{top['model']}` "
            f"(s3={top['best_s3_quality']:.3f}, s2={top['best_s2_quality']:.3f}, "
            f"iters={top['iterations']}, config={top.get('config', '?')})\n\n"
            "For SW3 categorization (small steps, many calls): prefer CPU for <=4GB models "
            "(avoids Vulkan overhead).\n"
            "For SW2 generation (large single-shot outputs): prefer GPU=99 for >=7B models "
            "(Vulkan helps with sustained throughput).\n"
        )

    return "\n".join(lines) + "\n"


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument(
        "--root",
        default="docs/benchmarks/outputs/meta-workflow/model-comparison",
        help="root dir of stress-* batch outputs",
    )
    ap.add_argument("--out", help="output markdown file (default: stdout)")
    args = ap.parse_args()

    rows = load_all_summaries(Path(args.root))
    md = render_markdown(rows)
    if args.out:
        Path(args.out).write_text(md)
        print(f"Wrote {args.out} ({len(md)} bytes)", file=sys.stderr)
    else:
        print(md)


if __name__ == "__main__":
    main()
