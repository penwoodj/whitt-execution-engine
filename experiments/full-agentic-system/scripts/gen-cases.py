#!/usr/bin/env python3
"""Config-driven case generator for FAS experiments.

Usage: python3 gen-cases.py --exp 07-solve [--out-dir cases] [--fixtures fixtures/fusion-fixes.yml]

Loads <exp>/config.py, calls build_cases(cfg) → list of case dicts:
  {case_id, rea_plus{...}, needs[], prompt, draft_response, auxiliary,
   tkeys, success_criteria.deterministic_checks.json_exact}

Self-verification per case (S44 T.7-9 oracle discipline):
  - word band 1000..3000
  - truth passes own json_exact (run_checks)
  - digest keeps contract + all task digits
  - worked example present + starts with marker
Writes cases/*.yml + fixtures yml with '# derivation' comments.
"""
import argparse
import importlib.util
import json
import re
import sys
from pathlib import Path

SCRIPTS = Path(__file__).resolve().parent
sys.path.insert(0, str(SCRIPTS))
import fas_lib as L  # noqa: E402
import fas_engines as E  # noqa: E402

WORD_BAND = (1000, 3000)
NUM_RE = re.compile(r"(?<![A-Za-z0-9_])(\d+(?:\.\d+)?)")


def load_config(exp):
    cfg_path = SCRIPTS.parent / exp / "config.py"
    spec = importlib.util.spec_from_file_location(
        f"cfg_{exp.replace('-', '_')}", cfg_path)
    if spec is None or spec.loader is None:
        raise RuntimeError(f"cannot load config for {exp}")
    mod = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(mod)
    return mod


def verify_case(c):
    prompt = c["prompt"]
    wc = L.word_count(prompt)
    assert WORD_BAND[0] <= wc <= WORD_BAND[1], \
        f"{c['case_id']}: word count {wc} outside {WORD_BAND}"
    assert L.WORKED_MARK in prompt, f"{c['case_id']}: worked example missing"
    assert L.CONTRACT_MARK in prompt, f"{c['case_id']}: contract missing"
    # truth passes own checks
    truth_str = json.dumps(c["truth"])
    r = L.run_checks_safe(truth_str, c["checks"])
    assert r and r.get("passed"), \
        f"{c['case_id']}: truth fails own json_exact: {truth_str}"
    # digest keeps contract + task digits
    dg = L.digest(prompt)
    assert L.CONTRACT_MARK in dg, f"{c['case_id']}: digest lost contract"
    for d in c.get("task_digits", []):
        assert re.search(rf"(?<![A-Za-z0-9_]){re.escape(str(d))}(?![0-9])", dg), \
            f"{c['case_id']}: digest lost task digit {d}"
    assert L.word_count(dg) <= 600, \
        f"{c['case_id']}: digest {L.word_count(dg)} words > 600"
    return wc


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("--exp", required=True)
    ap.add_argument("--write", action="store_true")
    a = ap.parse_args()

    cfg = load_config(a.exp)
    cases = cfg.build_cases(E, L)
    assert 30 <= len(cases) <= 150, \
        f"{a.exp}: {len(cases)} cases outside [30,150]"

    wcs = []
    for c in cases:
        c.setdefault("draft_response", "PLACEHOLDER")
        c.setdefault("auxiliary", "All facts needed are in the prompt itself.")
        wcs.append(verify_case(c))

    exp_dir = SCRIPTS.parent / a.exp
    cases_dir = exp_dir / "cases"
    fix_dir = exp_dir / "fixtures"
    if a.write:
        cases_dir.mkdir(parents=True, exist_ok=True)
        fix_dir.mkdir(parents=True, exist_ok=True)
        import yaml
        for c in cases:
            yml = {
                "case_id": c["case_id"],
                "rea_plus": c.get("rea_plus", {}),
                "needs": c.get("needs", []),
                "prompt": c["prompt"],
                "draft_response": "PLACEHOLDER",
                "auxiliary": c["auxiliary"],
                "tkeys": c.get("tkeys", ""),
                "success_criteria": {
                    "deterministic_checks": {
                        "json_exact": json.dumps(c["truth"])}},
            }
            (cases_dir / f"case-{c['case_id']}.yml").write_text(
                yaml.safe_dump(yml, sort_keys=False, width=78))
        lines = ["# computed truths (rule simulation, never typed)"]
        for c in cases:
            lines.append(f"{c['case_id']}:  # {c.get('derivation', 'engine sim')}")
            lines.append(f"  {json.dumps(c['truth'])}")
        (fix_dir / "fas-fixes.yml").write_text("\n".join(lines) + "\n")

    print(f"{a.exp}: {len(cases)} cases verified, "
          f"words {min(wcs)}-{max(wcs)}, band {WORD_BAND}")


if __name__ == "__main__":
    main()
