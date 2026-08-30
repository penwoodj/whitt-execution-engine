#!/usr/bin/env python3
import json, sys
with open(sys.argv[1]) as f:
    r = json.load(f)
fails = [c for c in r['checks'] if not c['pass']]
print(json.dumps(fails))