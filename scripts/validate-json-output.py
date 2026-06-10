#!/usr/bin/env python3
"""Validate JSON output from json-bench-3step.yml workflow.

Checks:
1. File exists and is non-empty
2. Content parses as valid JSON
3. JSON has 'dependencies' key (dict)
4. JSON has 'total_count' key (number)
5. total_count matches actual dependency count
6. Each dependency has 'version' field
7. Dependencies are sorted alphabetically
8. No markdown fences or extra text

Usage:
  python3 scripts/validate-json-output.py [path/to/final-result.json]
  
Exit codes:
  0 = PASS (all checks)
  1 = FAIL (any check failed)
"""

import json
import sys
import os

def validate(filepath):
    checks = {}
    
    # Check 1: File exists and non-empty
    if not os.path.exists(filepath):
        print(f"FAIL: file not found: {filepath}")
        return False, {"file_exists": False}
    
    with open(filepath, 'r') as f:
        content = f.read().strip()
    
    if not content:
        print("FAIL: file is empty")
        return False, {"file_exists": True, "non_empty": False}
    
    checks["file_exists"] = True
    checks["non_empty"] = True
    
    # Check 2: No markdown fences
    has_fences = content.startswith("```") or content.endswith("```")
    checks["no_markdown"] = not has_fences
    if has_fences:
        print("WARN: markdown fences detected, stripping...")
        # Strip fences
        lines = content.split('\n')
        if lines[0].startswith('```'):
            lines = lines[1:]
        if lines[-1].strip() == '```':
            lines = lines[:-1]
        content = '\n'.join(lines)
    
    # Check 3: Parses as JSON
    try:
        data = json.loads(content)
        checks["valid_json"] = True
    except json.JSONDecodeError as e:
        print(f"FAIL: JSON parse error: {e}")
        checks["valid_json"] = False
        return False, checks
    
    # Check 4: Has 'dependencies' key (dict)
    has_deps = isinstance(data.get("dependencies"), dict)
    checks["has_dependencies"] = has_deps
    if not has_deps:
        print(f"FAIL: missing 'dependencies' dict. Keys found: {list(data.keys())}")
    
    # Check 5: Has 'total_count' key (number)
    has_count = isinstance(data.get("total_count"), (int, float))
    checks["has_total_count"] = has_count
    if not has_count:
        print(f"FAIL: missing 'total_count' number. Keys found: {list(data.keys())}")
    
    # Check 6: total_count matches actual count
    if has_deps and has_count:
        actual = len(data["dependencies"])
        expected = data["total_count"]
        checks["count_matches"] = (actual == expected)
        if actual != expected:
            print(f"WARN: total_count={expected} but actual dependencies={actual}")
    else:
        checks["count_matches"] = False
    
    # Check 7: Each dependency has 'version' field
    if has_deps:
        all_have_version = all(
            isinstance(v, dict) and "version" in v 
            for v in data["dependencies"].values()
        )
        checks["all_have_version"] = all_have_version
        if not all_have_version:
            missing = [k for k, v in data["dependencies"].items() 
                      if not isinstance(v, dict) or "version" not in v]
            print(f"WARN: dependencies missing 'version': {missing}")
    else:
        checks["all_have_version"] = False
    
    # Check 8: Dependencies sorted alphabetically
    if has_deps:
        dep_names = list(data["dependencies"].keys())
        checks["sorted_alphabetically"] = (dep_names == sorted(dep_names))
        if dep_names != sorted(dep_names):
            print(f"WARN: dependencies not sorted. Got: {dep_names[:5]}...")
    else:
        checks["sorted_alphabetically"] = False
    
    # Summary
    passed = all(v for v in checks.values())
    score = sum(1 for v in checks.values() if v)
    total = len(checks)
    
    status = "PASS" if passed else "PARTIAL" if score > total // 2 else "FAIL"
    print(f"\n{status}: {score}/{total} checks passed")
    print(f"Dependencies found: {len(data.get('dependencies', {}))}")
    
    for k, v in checks.items():
        sym = "✅" if v else "❌"
        print(f"  {sym} {k}")
    
    return passed, checks

if __name__ == "__main__":
    filepath = sys.argv[1] if len(sys.argv) > 1 else "./docs/benchmarks/outputs/output/final-result.json"
    passed, checks = validate(filepath)
    sys.exit(0 if passed else 1)
