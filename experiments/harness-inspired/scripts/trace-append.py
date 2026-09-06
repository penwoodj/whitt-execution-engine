#!/usr/bin/env python3
"""Append a trace entry to the JSONL trace file.
Usage: trace-append.py <trace_file> <gate> <verdict> [key=value ...]
"""
import json, sys, datetime

trace_file = sys.argv[1]
gate = sys.argv[2]
verdict = sys.argv[3]
entry = {"gate": gate, "verdict": verdict, "timestamp": datetime.datetime.utcnow().isoformat() + "Z"}
for arg in sys.argv[4:]:
    k, v = arg.split("=", 1)
    entry[k] = v
with open(trace_file, "a") as f:
    f.write(json.dumps(entry) + "\n")
