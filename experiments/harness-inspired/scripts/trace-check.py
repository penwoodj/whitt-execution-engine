#!/usr/bin/env python3
import json, sys, datetime

with open(sys.argv[1]) as f:
    r = json.load(f)

entry = {
    "gate": sys.argv[2],
    "verdict": "pass" if r['pass'] else "fail",
    "score": r.get('score', 0),
    "checks": f"{r.get('passed', 0)}/{r.get('total', 0)}",
    "timestamp": datetime.datetime.utcnow().isoformat() + 'Z'
}
if len(sys.argv) > 4:
    entry["case"] = sys.argv[4]

with open(sys.argv[3], 'a') as f:
    f.write(json.dumps(entry) + '\n')
