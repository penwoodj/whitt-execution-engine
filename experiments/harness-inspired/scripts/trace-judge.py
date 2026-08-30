#!/usr/bin/env python3
import json, sys, datetime

verdict_file = sys.argv[1]
trace_file = sys.argv[2]
v = open(verdict_file).read().strip()
verd = "pass" if "VERDICT: PASS" in v else "fail"
entry = {"gate": "judge", "verdict": verd, "raw": v[:300], "timestamp": datetime.datetime.utcnow().isoformat() + "Z"}
if len(sys.argv) > 3:
    entry["case"] = sys.argv[3]
with open(trace_file, "a") as f:
    f.write(json.dumps(entry) + "\n")
