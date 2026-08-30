#!/usr/bin/env python3
"""Deterministic quality gate. Exit 0=PASS, 1=FAIL.

Usage: check-deterministic.py --artifact FILE --criteria YAML
Output: JSON {pass, checks, evidence, score, hash}
"""
import argparse, hashlib, json, os, re, sys, yaml
from pathlib import Path


def sha256_file(path):
    h = hashlib.sha256()
    with open(path, "rb") as f:
        for chunk in iter(lambda: f.read(8192), b""):
            h.update(chunk)
    return h.hexdigest()


def sha256_str(s):
    return hashlib.sha256(s.encode()).hexdigest()


def extract_last_json(text):
    candidates = []
    depth = 0
    start = None
    in_str = False
    esc = False
    for i, ch in enumerate(text):
        if in_str:
            if esc:
                esc = False
            elif ch == "\\":
                esc = True
            elif ch == '"':
                in_str = False
            continue
        if ch == '"':
            in_str = True
        elif ch == "{":
            if depth == 0:
                start = i
            depth += 1
        elif ch == "}" and depth > 0:
            depth -= 1
            if depth == 0 and start is not None:
                candidates.append(text[start:i + 1])
    for cand in reversed(candidates):
        try:
            obj = json.loads(cand)
            if isinstance(obj, dict):
                return obj
        except (json.JSONDecodeError, ValueError):
            continue
    raise json.JSONDecodeError("no JSON object found", text, 0)


def run_checks(text, criteria, artifact_dir):
    checks = []
    total = 0
    passed = 0
    dc = criteria.get("deterministic_checks", {})

    for word in dc.get("contains_required", []):
        total += 1
        found = word.lower() in text.lower()
        checks.append({"name": f"contains:{word}", "pass": found, "evidence": f"{'found' if found else 'missing'} in output"})
        if found:
            passed += 1

    for phrase in dc.get("forbidden_phrases", []):
        total += 1
        found = phrase.lower() in text.lower()
        ok = not found
        checks.append({"name": f"forbidden:{phrase}", "pass": ok, "evidence": f"{'absent' if ok else 'PRESENT'} in output"})
        if ok:
            passed += 1

    words = text.split()
    wc = len(words)
    if "max_words" in dc:
        total += 1
        ok = wc <= dc["max_words"]
        checks.append({"name": "max_words", "pass": ok, "evidence": f"{wc}/{dc['max_words']}"})
        if ok:
            passed += 1
    if "min_words" in dc:
        total += 1
        ok = wc >= dc["min_words"]
        checks.append({"name": "min_words", "pass": ok, "evidence": f"{wc}/{dc['min_words']}"})
        if ok:
            passed += 1

    lines = text.splitlines()
    lc = len(lines)
    if "max_lines" in dc:
        total += 1
        ok = lc <= dc["max_lines"]
        checks.append({"name": "max_lines", "pass": ok, "evidence": f"{lc}/{dc['max_lines']}"})
        if ok:
            passed += 1
    if "min_lines" in dc:
        total += 1
        ok = lc >= dc["min_lines"]
        checks.append({"name": "min_lines", "pass": ok, "evidence": f"{lc}/{dc['min_lines']}"})
        if ok:
            passed += 1

    if dc.get("json_valid"):
        total += 1
        try:
            json.loads(text)
            ok = True
            ev = "valid JSON"
        except (json.JSONDecodeError, ValueError) as e:
            ok = False
            ev = f"invalid JSON: {str(e)[:100]}"
        checks.append({"name": "json_valid", "pass": ok, "evidence": ev})
        if ok:
            passed += 1

    if dc.get("yaml_valid"):
        total += 1
        try:
            yaml.safe_load(text)
            ok = True
            ev = "valid YAML"
        except (yaml.YAMLError, ValueError) as e:
            ok = False
            ev = f"invalid YAML: {str(e)[:100]}"
        checks.append({"name": "yaml_valid", "pass": ok, "evidence": ev})
        if ok:
            passed += 1

    for pat in dc.get("contains_regex", []):
        total += 1
        found = bool(re.search(pat, text))
        checks.append({"name": f"regex:{pat[:50]}", "pass": found, "evidence": f"{'matched' if found else 'no match'}"})
        if found:
            passed += 1

    for fpath in dc.get("file_exists", []):
        total += 1
        full = os.path.join(artifact_dir, fpath) if not os.path.isabs(fpath) else fpath
        ok = os.path.isfile(full)
        checks.append({"name": f"file_exists:{fpath}", "pass": ok, "evidence": full})
        if ok:
            passed += 1

    if "exact_match" in dc:
        total += 1
        ok = text.strip() == dc["exact_match"].strip()
        checks.append({"name": "exact_match", "pass": ok, "evidence": f"{'match' if ok else 'mismatch'}"})
        if ok:
            passed += 1

    if "json_exact" in dc:
        total += 1

        def typed_equal(a, b):
            if isinstance(a, bool) or isinstance(b, bool):
                return isinstance(a, bool) and isinstance(b, bool) and a == b
            if isinstance(a, dict) and isinstance(b, dict):
                return a.keys() == b.keys() and all(typed_equal(a[k], b[k]) for k in a)
            if isinstance(a, list) and isinstance(b, list):
                return len(a) == len(b) and all(typed_equal(x, y) for x, y in zip(a, b))
            return type(a) is type(b) and a == b

        try:
            actual = extract_last_json(text)
            expected = json.loads(dc["json_exact"])
            ok = isinstance(actual, dict) and typed_equal(actual, expected)
            ev = "deep equal" if ok else "value or shape mismatch"
        except (json.JSONDecodeError, ValueError, TypeError) as e:
            ok = False
            ev = f"unparseable or wrong type: {str(e)[:80]}"
        checks.append({"name": "json_exact", "pass": ok, "evidence": ev, "leaky": True})
        if ok:
            passed += 1

    score = passed / total if total > 0 else 1.0
    return {"pass": all(c["pass"] for c in checks), "checks": checks, "passed": passed, "total": total, "score": round(score, 4)}


def main():
    p = argparse.ArgumentParser(description="Deterministic quality gate")
    p.add_argument("--artifact", required=True, help="Path to artifact file")
    p.add_argument("--criteria", help="Path to criteria YAML")
    p.add_argument("--out", help="Path to write JSON result")
    p.add_argument("--hash-only", action="store_true", help="Output only content hash")
    args = p.parse_args()

    if not os.path.isfile(args.artifact):
        result = {"pass": False, "checks": [{"name": "artifact_exists", "pass": False, "evidence": f"{args.artifact} not found"}], "passed": 0, "total": 1, "score": 0.0, "hash": "", "error": "artifact_missing"}
        print(json.dumps(result))
        sys.exit(1)

    text = Path(args.artifact).read_text(encoding="utf-8", errors="replace")
    content_hash = sha256_str(text)

    if args.hash_only:
        print(json.dumps({"hash": content_hash}))
        sys.exit(0)

    criteria = {}
    if os.path.isfile(args.criteria):
        with open(args.criteria) as f:
            criteria = yaml.safe_load(f) or {}
    if "deterministic_checks" not in criteria and "success_criteria" in criteria:
        criteria = criteria["success_criteria"]

    artifact_dir = str(Path(args.artifact).parent)
    result = run_checks(text, criteria, artifact_dir)
    result["hash"] = content_hash

    if args.out:
        Path(args.out).parent.mkdir(parents=True, exist_ok=True)
        Path(args.out).write_text(json.dumps(result, indent=2) + "\n")

    print(json.dumps(result))
    sys.exit(0 if result["pass"] else 1)


if __name__ == "__main__":
    main()
