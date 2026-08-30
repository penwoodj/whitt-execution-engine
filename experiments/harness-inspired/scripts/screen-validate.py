#!/usr/bin/env python3
import yaml, sys
path = sys.argv[1]
with open(path) as f:
    c = yaml.safe_load(f)
assert 'success_criteria' in c, 'missing success_criteria'
assert 'deterministic_checks' in c['success_criteria'], 'missing deterministic_checks'
print('VALID')
