#!/usr/bin/env python3
"""Analyze meta-workflow run logs and extract iteration metrics.

Usage: ./scripts/analyze-meta-run.py <RUN_DIR>

Extracts: step durations, token counts, verdicts, file outputs.
"""
import json
import re
import sys
from pathlib import Path


def parse_log_line(line: str) -> dict | None:
    """Parse a benchmark log line into structured data."""
    match = re.match(r"\[(\d+) (\w+)\] (.+)", line)
    if not match:
        return None
    timestamp, level, rest = match.groups()
    # Try to extract JSON payload
    json_match = re.search(r"(\{.+\})", rest)
    payload = json.loads(json_match.group(1)) if json_match else {}
    # Extract step name from non-JSON prefix
    step_match = re.match(r"step_name=(\S+)", rest)
    step_name = step_match.group(1) if step_match else payload.get("step_name", "?")
    return {
        "timestamp": int(timestamp),
        "level": level,
        "step_name": step_name,
        "payload": payload,
    }


def analyze_run(run_dir: Path) -> dict:
    """Analyze a single benchmark run directory."""
    result = {
        "run_id": run_dir.name,
        "sub_workflows": {},
    }
    for sw_dir in sorted(run_dir.iterdir()):
        if not sw_dir.is_dir() or not sw_dir.name.startswith(("sw", "meta")):
            continue
        log_file = run_dir / "logs" / f"{sw_dir.name}.log"
        if not log_file.exists():
            continue
        entries = []
        for line in log_file.read_text().splitlines():
            parsed = parse_log_line(line)
            if parsed:
                entries.append(parsed)
        steps = {}
        for entry in entries:
            step = entry["step_name"]
            if step not in steps:
                steps[step] = {"status": "unknown", "duration_ms": 0, "tokens": 0, "errors": []}
            if entry["level"] == "ERROR":
                steps[step]["status"] = "failed"
                steps[step]["errors"].append(entry["payload"].get("error_message", ""))
            elif entry["level"] == "INFO" and "duration_ms" in entry["payload"]:
                steps[step]["status"] = "success"
                steps[step]["duration_ms"] = entry["payload"].get("duration_ms", 0)
                steps[step]["tokens"] = entry["payload"].get("token_count", 0)
        # Check for output files
        output_files = {}
        for f in sw_dir.iterdir():
            if f.is_file():
                output_files[f.name] = {"size_bytes": f.stat().st_size}
        result["sub_workflows"][sw_dir.name] = {
            "step_count": len(steps),
            "steps": steps,
            "output_files": output_files,
            "total_duration_ms": sum(s["duration_ms"] for s in steps.values()),
        }
    return result


def print_report(analysis: dict) -> None:
    """Print human-readable analysis report."""
    print(f"# Run Analysis: {analysis['run_id']}\n")
    total_steps = 0
    total_duration = 0
    total_errors = 0
    for sw_name, sw_data in analysis["sub_workflows"].items():
        print(f"## {sw_name}")
        print(f"  Steps: {sw_data['step_count']}")
        print(f"  Duration: {sw_data['total_duration_ms'] / 1000:.1f}s")
        for step_name, step_data in sw_data["steps"].items():
            status_icon = "✅" if step_data["status"] == "success" else "❌"
            print(f"    {status_icon} {step_name}: {step_data['status']} ({step_data['duration_ms'] / 1000:.1f}s, {step_data['tokens']} tok)")
            if step_data["errors"]:
                for err in step_data["errors"]:
                    print(f"       ERROR: {err[:100]}")
        print(f"  Output files: {len(sw_data['output_files'])}")
        for fname, fdata in sw_data["output_files"].items():
            print(f"    {fname}: {fdata['size_bytes']} bytes")
        print()
        total_steps += sw_data["step_count"]
        total_duration += sw_data["total_duration_ms"]
        total_errors += sum(1 for s in sw_data["steps"].values() if s["status"] == "failed")
    print(f"## TOTAL: {total_steps} steps, {total_duration / 1000:.1f}s, {total_errors} errors")


if __name__ == "__main__":
    if len(sys.argv) != 2:
        print("Usage: analyze-meta-run.py <RUN_DIR>")
        sys.exit(1)
    run_dir = Path(sys.argv[1])
    if not run_dir.exists():
        print(f"Error: {run_dir} does not exist")
        sys.exit(1)
    analysis = analyze_run(run_dir)
    print_report(analysis)
