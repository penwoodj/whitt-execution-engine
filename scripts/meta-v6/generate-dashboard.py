#!/usr/bin/env python3
"""Generate interactive HTML dashboard from integration test results.

Reads summary.tsv + per-prompt result.json files and renders an interactive
HTML page with: pass/fail overview, per-prompt drilldown, code compile status,
step quality metrics, before/after comparison charts.

Usage: generate-dashboard.py --run-id ID --results-dir DIR --summary-tsv TSV
"""
import argparse
import csv
import html
import json
import os
import sys
from pathlib import Path


def load_summary(tsv_path: str) -> list[dict]:
    rows = []
    if not os.path.exists(tsv_path):
        return rows
    with open(tsv_path) as f:
        reader = csv.DictReader(f, delimiter="\t")
        for r in reader:
            rows.append(r)
    return rows


def load_prompt_details(results_dir: str) -> dict[str, dict]:
    out = {}
    pdir = Path(results_dir)
    if not pdir.is_dir():
        return out
    for sub in sorted(pdir.iterdir()):
        if not sub.is_dir() or not sub.name.startswith("prompt-"):
            continue
        rj = sub / "result.json"
        sq = sub / "step-quality.json"
        cc = sub / "compile.json"
        cmp_ = sub / "comparison.json"
        entry = {}
        if rj.exists():
            entry["result"] = json.loads(rj.read_text())
        if sq.exists():
            entry["step_quality"] = json.loads(sq.read_text())
        if cc.exists():
            entry["compile"] = json.loads(cc.read_text())
        if cmp_.exists():
            entry["comparison"] = json.loads(cmp_.read_text())
        out[sub.name] = entry
    return out


CSS = """
body { font-family: -apple-system, system-ui, sans-serif; margin: 0; background: #0d1117; color: #e6edf3; }
.container { max-width: 1400px; margin: 0 auto; padding: 24px; }
header { background: #161b22; padding: 20px 32px; border-bottom: 1px solid #30363d; }
header h1 { margin: 0 0 8px 0; font-size: 24px; color: #58a6ff; }
header .meta { color: #8b949e; font-size: 13px; }
.summary-cards { display: grid; grid-template-columns: repeat(auto-fit, minmax(180px, 1fr)); gap: 12px; margin: 24px 0; }
.card { background: #161b22; border: 1px solid #30363d; border-radius: 8px; padding: 16px; }
.card .label { color: #8b949e; font-size: 12px; text-transform: uppercase; letter-spacing: 0.5px; }
.card .value { font-size: 32px; font-weight: 600; margin-top: 4px; }
.card .value.pass { color: #3fb950; }
.card .value.fail { color: #f85149; }
.card .value.warn { color: #d29922; }
table { width: 100%; border-collapse: collapse; background: #161b22; border-radius: 8px; overflow: hidden; }
th, td { padding: 10px 12px; text-align: left; border-bottom: 1px solid #30363d; font-size: 13px; }
th { background: #21262d; color: #8b949e; font-weight: 600; text-transform: uppercase; letter-spacing: 0.5px; font-size: 11px; }
tr:hover { background: #1c2128; }
.badge { display: inline-block; padding: 2px 8px; border-radius: 12px; font-size: 11px; font-weight: 600; }
.badge.pass { background: #1a4731; color: #3fb950; }
.badge.fail { background: #4a1f1f; color: #f85149; }
.badge.warn { background: #4a3a1f; color: #d29922; }
.badge.na { background: #2d333b; color: #8b949e; }
.drilldown { background: #161b22; border: 1px solid #30363d; border-radius: 8px; padding: 20px; margin: 24px 0; display: none; }
.drilldown.active { display: block; }
.drilldown h3 { margin: 0 0 12px 0; color: #58a6ff; }
.drilldown pre { background: #0d1117; border: 1px solid #30363d; padding: 12px; border-radius: 6px; overflow-x: auto; font-size: 12px; }
.collapsible { cursor: pointer; padding: 8px 0; }
.collapsible:hover { color: #58a6ff; }
.step-table { font-size: 11px; }
.step-table td { padding: 4px 8px; }
.filter-bar { margin: 16px 0; display: flex; gap: 8px; align-items: center; }
.filter-bar input, .filter-bar select { background: #161b22; border: 1px solid #30363d; color: #e6edf3; padding: 6px 10px; border-radius: 6px; font-size: 13px; }
"""

JS = """
function toggleDrilldown(n) {
  const el = document.getElementById('dd-' + n);
  el.classList.toggle('active');
}
function filterTable() {
  const q = document.getElementById('filter').value.toLowerCase();
  const status = document.getElementById('status-filter').value;
  document.querySelectorAll('tr[data-prompt]').forEach(tr => {
    const txt = tr.textContent.toLowerCase();
    const st = tr.getAttribute('data-verdict');
    const matches_q = !q || txt.includes(q);
    const matches_s = !status || st === status;
    tr.style.display = (matches_q && matches_s) ? '' : 'none';
  });
}
"""


def render(args) -> str:
    summary = load_summary(args.summary_tsv)
    details = load_prompt_details(args.results_dir)

    total = len(summary)
    passed = sum(1 for r in summary if r.get("verdict") == "PASS")
    failed = sum(1 for r in summary if r.get("verdict") == "FAIL")
    sw_wins = sum(1 for r in summary if r.get("sw_wins", "").lower() == "true")
    zero_refusals = sum(1 for r in summary if r.get("refusals") == "0")
    rust_compile_pct = 0
    ts_compile_pct = 0
    rust_attempts = [r for r in summary if r.get("rust_compiles", "na") != "na"]
    ts_attempts = [r for r in summary if r.get("ts_compiles", "na") != "na"]
    if rust_attempts:
        rust_compile_pct = int(sum(1 for r in rust_attempts if r["rust_compiles"].lower() == "true") / len(rust_attempts) * 100)
    if ts_attempts:
        ts_compile_pct = int(sum(1 for r in ts_attempts if r["ts_compiles"].lower() == "true") / len(ts_attempts) * 100)

    pass_rate = int(passed / total * 100) if total else 0

    rows_html = []
    drilldowns_html = []
    for r in summary:
        n = r["prompt"]
        verdict_class = "pass" if r["verdict"] == "PASS" else "fail"
        sw_class = "pass" if r.get("sw_wins", "").lower() == "true" else "fail"
        shell_rate = float(r.get("shell_fail_rate", "1.0"))
        shell_class = "pass" if shell_rate < 0.10 else ("warn" if shell_rate < 0.30 else "fail")
        refusal_n = int(r.get("refusals", "999"))
        refusal_class = "pass" if refusal_n == 0 else "fail"
        step_q_class = "pass" if r.get("step_quality", "").lower() == "true" else "fail"
        rust_v = r.get("rust_compiles", "na")
        rust_class = "na" if rust_v == "na" else ("pass" if rust_v == "true" else "fail")
        ts_v = r.get("ts_compiles", "na")
        ts_class = "na" if ts_v == "na" else ("pass" if ts_v == "true" else "fail")

        rows_html.append(f"""
<tr data-prompt="{n}" data-verdict="{r['verdict']}" onclick="toggleDrilldown('{n}')">
  <td><strong>P{n}</strong></td>
  <td><span class="badge {verdict_class}">{r['verdict']}</span></td>
  <td><span class="badge {sw_class}">{r.get('sw_wins','?')}</span></td>
  <td><span class="badge {shell_class}">{shell_rate*100:.0f}%</span></td>
  <td><span class="badge {refusal_class}">{refusal_n}</span></td>
  <td><span class="badge {step_q_class}">{r.get('step_quality','?')}</span></td>
  <td><span class="badge {rust_class}">{rust_v}</span></td>
  <td><span class="badge {ts_class}">{ts_v}</span></td>
</tr>""")

        # Drilldown content
        dd = details.get(f"prompt-{n}", {})
        result = dd.get("result", {})
        sq = dd.get("step_quality", {})
        cc = dd.get("compile", {})
        cmp_ = dd.get("comparison", {})

        steps_table = ""
        if sq.get("steps"):
            steps_rows = []
            for s in sq["steps"]:
                v_class = "pass" if s["verdict"] == "OK" else "fail"
                steps_rows.append(
                    f'<tr><td>{html.escape(s["file"])}</td><td>{s["size"]}B</td>'
                    f'<td>{s["lines"]}L</td><td>{s["refusals"]}</td>'
                    f'<td>{s["placeholder"]}</td>'
                    f'<td><span class="badge {v_class}">{s["verdict"]}</span></td></tr>'
                )
            steps_table = (
                '<table class="step-table"><thead><tr>'
                '<th>File</th><th>Size</th><th>Lines</th><th>Refusals</th><th>Placeholders</th><th>Verdict</th>'
                '</tr></thead><tbody>' + "".join(steps_rows) + "</tbody></table>"
            )

        rust_block = ""
        if cc.get("rust"):
            rust_block = f'<div><strong>Rust:</strong> {cc["rust"]["error_count"]} errors, compiles={cc["rust"]["compiles"]}</div>'
        ts_block = ""
        if cc.get("typescript"):
            ts_block = f'<div><strong>TS:</strong> {cc["typescript"]["error_count"]} errors, compiles={cc["typescript"]["compiles"]}</div>'

        cmp_block = ""
        if cmp_.get("per_step"):
            cmp_rows = []
            for p in cmp_["per_step"][:20]:
                cmp_rows.append(
                    f'<tr><td>{html.escape(p["file"])}</td>'
                    f'<td>{p["before_size"]}</td><td>{p["after_size"]}</td>'
                    f'<td>{p["delta"]:+d}</td></tr>'
                )
            cmp_block = (
                '<h4>Before/After Sizes</h4>'
                '<table class="step-table"><thead><tr>'
                '<th>File</th><th>Before</th><th>After</th><th>Delta</th>'
                '</tr></thead><tbody>' + "".join(cmp_rows) + "</tbody></table>"
            )

        drilldowns_html.append(f"""
<div id="dd-{n}" class="drilldown">
  <h3>P{n} — {r['verdict']}</h3>
  <pre>{html.escape(json.dumps(result, indent=2))}</pre>
  <h4>Step Quality</h4>
  {steps_table}
  <h4>Compilation</h4>
  {rust_block}{ts_block}
  {cmp_block}
</div>""")

    overall_class = "pass" if pass_rate >= 80 else ("warn" if pass_rate >= 50 else "fail")
    return f"""<!DOCTYPE html>
<html lang="en">
<head>
<meta charset="utf-8">
<title>Integration Test Dashboard — {html.escape(args.run_id)}</title>
<style>{CSS}</style>
</head>
<body>
<header>
  <h1>Integration Test Dashboard</h1>
  <div class="meta">Run: <code>{html.escape(args.run_id)}</code> · {total} prompts · {args.results_dir}</div>
</header>
<div class="container">
  <div class="summary-cards">
    <div class="card"><div class="label">Overall Pass Rate</div><div class="value {overall_class}">{pass_rate}%</div></div>
    <div class="card"><div class="label">Passed (all 3 criteria)</div><div class="value pass">{passed}</div></div>
    <div class="card"><div class="label">Failed</div><div class="value fail">{failed}</div></div>
    <div class="card"><div class="label">SW Wins</div><div class="value">{sw_wins}/{total}</div></div>
    <div class="card"><div class="label">Zero Refusals</div><div class="value">{zero_refusals}/{total}</div></div>
    <div class="card"><div class="label">Rust Compiles</div><div class="value">{rust_compile_pct}%</div></div>
    <div class="card"><div class="label">TS Compiles</div><div class="value">{ts_compile_pct}%</div></div>
  </div>

  <div class="filter-bar">
    <input id="filter" type="text" placeholder="Filter prompts..." oninput="filterTable()">
    <select id="status-filter" onchange="filterTable()">
      <option value="">All verdicts</option>
      <option value="PASS">PASS</option>
      <option value="FAIL">FAIL</option>
    </select>
  </div>

  <table>
    <thead><tr>
      <th>Prompt</th><th>Verdict</th><th>SW Wins</th><th>Shell Fail %</th>
      <th>Refusals</th><th>Step Quality</th><th>Rust Compiles</th><th>TS Compiles</th>
    </tr></thead>
    <tbody>
      {"".join(rows_html)}
    </tbody>
  </table>

  {"".join(drilldowns_html)}
</div>
<script>{JS}</script>
</body>
</html>
"""


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("--run-id", required=True)
    ap.add_argument("--results-dir", required=True)
    ap.add_argument("--summary-tsv", required=True)
    args = ap.parse_args()
    print(render(args))


if __name__ == "__main__":
    main()
