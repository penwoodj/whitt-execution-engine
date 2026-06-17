# 07 — QA & Verification

Defines per-SW success criteria, iteration gates, and the evidence-gathering protocol.

---

## 1. Iteration Gate (8-Point Check)

EVERY iteration of EVERY SW MUST pass this gate before claiming improvement.
Per `AGENTS.md` and `scripts/validate-iteration.sh`:

| # | Check | Evidence Required |
|---|-------|-------------------|
| 1 | Live workflow ran | `[workflow:end]` event in log |
| 2 | Output file exists | `test -s <output-path>` returns 0 |
| 3 | Output file non-empty | `wc -l <output-path>` > 0 |
| 4 | Output file schema-valid (if YAML) | `python3 scripts/validate-yaml.py <file>` returns VALID |
| 5 | Output file markdown-valid (if .md) | `python3 -c "import markdown; markdown.markdown(open('<file>').read())"` exits 0 |
| 6 | Log contains expected events | Each step has `[step:start]` + `[step:end]` events |
| 7 | No error-level log entries (unless expected) | `grep -c "level\":\"error" <log>` returns 0 or justified count |
| 8 | Compared against previous iteration | Metrics table in `08-ITERATION-LOG.md` shows delta |

### Gate Failure Handling
- Any check fails → DO NOT claim improvement.
- Document which check failed and why.
- Fix and re-run.

---

## 2. Per-SW Success Criteria

### SW1 (task-deconstruction)

| Criterion | Measurement | Pass Threshold |
|-----------|-------------|----------------|
| Output conforms to schema | Manual + `markdown` library parse | 100% |
| All leaf tasks < 5 pts | Parse `tasks.md`, count `>= 5pts` leaves | 0 violations |
| Every task has GWT | Parse for `Given/When/Then` per task | 100% |
| Descriptions < 100 chars | Parse task action lines | ≥ 95% |
| Task count sanity | Count `^## T` headers | 4 ≤ N ≤ 30 |
| Beats baseline | Sisyphus judgment | ≥ 3 of 5 prompts |

### SW2 (desired-output-state)

| Criterion | Measurement | Pass Threshold |
|-----------|-------------|----------------|
| Every task has state block | Cross-check vs `tasks.md` | 100% |
| Criteria are testable (binary PASS/FAIL) | Manual review | ≥ 95% |
| No vague terms | grep for "good, fast, nice, complete" (with context) | 0 unjustified |
| GWT present per task | Parse for `**Given:**`, `**When:**`, `**Then:**` | 100% |
| Anti-criteria list non-empty | Parse for `**Anti-criteria:**` followed by bullets | 100% |
| Beats baseline | Sisyphus judgment | ≥ 3 of 5 prompts |

### SW3 (agentic-categorization)

| Criterion | Measurement | Pass Threshold |
|-----------|-------------|----------------|
| Categories from C1-C10 only | Parse `**Primary category:** C(\d+)` | 100% within range |
| Exactly one primary per task | Count per task | 100% |
| "Why" references task specifics | Manual review (not generic boilerplate) | ≥ 90% |
| Every task categorized | Cross-check vs `tasks.md` | 100% |
| Pattern aligns with category | Manual review | ≥ 90% |
| Sub-workflow affinity non-empty | Parse for `**Sub-workflow affinity:**` | 100% |
| Beats baseline | Sisyphus judgment | ≥ 3 of 5 prompts |

### SW4 (yaml-substructure-translation)

| Criterion | Measurement | Pass Threshold |
|-----------|-------------|----------------|
| Zero non-YAML code blocks | `scripts/check-yaml-only-blocks.py` | 0 violations |
| All keys schema-valid | Per-substructure `validate-yaml.py` | 100% |
| `${models.qwen35}` only | grep for other model refs | 0 |
| Every step has after_step_succeeds + after_step_fails | Parse YAML | 100% |
| Every task has substructure | Cross-check vs `tasks.md` | 100% |
| depends_on references valid steps | Static analysis | 100% |
| Paths under `./docs/benchmarks/outputs/` | Parse paths | 100% |
| Beats baseline | Sisyphus judgment | ≥ 3 of 5 prompts |

### SW5 (final-workflow-assembly)

| Criterion | Measurement | Pass Threshold |
|-----------|-------------|----------------|
| YAML validates via `validate-yaml.py` | Shell | 100% |
| Every task has step in workflow | Cross-check audit | 100% |
| Every step has EXHAUSTIVE_COVERAGE: YES | Parse audit | 100% |
| Every step has DESIRED_STATE_MET: YES | Parse audit | 100% |
| No orphan steps | Audit | 0 |
| Workflow executes standalone | `whitt benchmark --workflow workflow.yml` | Exits 0 |
| Beats baseline | Sisyphus judgment | ≥ 3 of 5 prompts |

---

## 3. META End-to-End Success Criteria

| Criterion | Measurement | Pass Threshold |
|-----------|-------------|----------------|
| META runs without manual intervention | Single `whitt benchmark` invocation | 100% |
| All 5 SW outputs produced | File existence check | 5/5 |
| Final `workflow.yml` valid + executable | Shell validate + run | 100% |
| Total runtime per prompt | Wall clock | < 5 hours |
| Logs contain full trace | Every step has start/end events | 100% |

---

## 4. Baseline Comparison Protocol

For each SW × prompt pair:

1. Read Sisyphus baseline: `artifacts/sw<N>-<name>/baseline-opencode/prompt-NN-<output>`
2. Read SW output: `docs/benchmarks/outputs/meta-workflow/<RUN_ID>/sw<N>/<output>`
3. Compare on these dimensions:
   - **Completeness**: Does SW cover everything baseline covers?
   - **Specificity**: Is SW as concrete/specific as baseline?
   - **Correctness**: Any factual/logical errors in SW vs baseline?
   - **Conciseness**: Is SW appropriately terse (not verbose)?
   - **Format adherence**: Does SW follow the output schema?
4. Record verdict in `artifacts/sw<N>-<name>/iterations/iter-NNN-comparison.md`:
   ```markdown
   # Iteration NNN Comparison — SW<N>, prompt-NN

   **Baseline path:** <path>
   **SW output path:** <path>

   | Dimension | Baseline | SW Output | Winner |
   |-----------|----------|-----------|--------|
   | Completeness | ... | ... | BASELINE / SW / TIE |
   | Specificity | ... | ... | ... |
   | Correctness | ... | ... | ... |
   | Conciseness | ... | ... | ... |
   | Format | ... | ... | ... |

   **Overall verdict:** SW WINS / BASELINE WINS / TIE
   **Reasoning:** <2-3 sentences>
   ```

5. SW "wins" the prompt if it wins ≥ 3 of 5 dimensions.

---

## 5. Iteration Log Schema

`08-ITERATION-LOG.md` entries follow this format:

```markdown
## SW<N> Iteration NNN — YYYY-MM-DD HH:MM

**Run ID:** <RUN_ID>
**Prompt:** prompt-NN (<short-name>)
**Workflow YAML:** docs/benchmarks/workflows/sw<N>-<name>.yml
**Log path:** <log-path>
**Output path:** <output-path>
**Previous iteration:** NNN-1 (or "none — initial")

### 8-Point Gate Results

| # | Check | Result |
|---|-------|--------|
| 1 | Live workflow ran | ✅ / ❌ |
| 2 | Output file exists | ✅ / ❌ |
| 3 | Output non-empty | ✅ / ❌ |
| 4 | Schema/markdown valid | ✅ / ❌ |
| 5 | Log events present | ✅ / ❌ |
| 6 | No unexpected errors | ✅ / ❌ |
| 7 | Compared vs previous | ✅ / ❌ |
| 8 | Metrics delta recorded | ✅ / ❌ |

### Metrics

| Metric | Previous | Current | Delta |
|--------|----------|---------|-------|
| Task count | N | M | +X |
| Quality gate pass rate | N/6 | M/6 | +X |
| Total tokens generated | N | M | +X |
| Wall clock (sec) | N | M | +X |
| Output file size (bytes) | N | M | +X |

### Baseline Comparison

**Verdict:** SW WINS / BASELINE WINS / TIE
**Reasoning:** <sentences>

### Next Action

- [ ] Promote to golden/
- [ ] Iterate again (address: <specific failure>)
- [ ] Test on prompt-NN+1
- [ ] Advance to SW<N+1>
```

---

## 6. Final Release Gate

Before declaring the entire plan **complete**:

| # | Check | Evidence |
|---|-------|----------|
| 1 | All 5 SWs have golden artifacts for ≥ 3 prompts | `ls artifacts/*/golden/` |
| 2 | META end-to-end runs on ≥ 2 prompts | `<RUN_ID>/sw5/workflow.yml` exists |
| 3 | Final `workflow.yml` validates + executes | `whitt benchmark` exit 0 |
| 4 | All unit tests pass | `cargo test --all-features` |
| 5 | Zero clippy warnings | `cargo clippy --all-features -- -W clippy::all` |
| 6 | Release build clean | `cargo build --release --all-features` |
| 7 | Iteration log complete | `08-ITERATION-LOG.md` has entries for all SWs |
| 8 | LSP diagnostics clean on all changed files | `lsp_diagnostics` 0 errors |

Any single failure → not complete. Fix and re-verify.

---

## 7. Pre-Existing Issue Tolerance

Per `AGENTS.md`: "Do NOT fix pre-existing issues unless asked."

If a pre-existing test fails (unrelated to our changes), document it:
```
**Pre-existing failure:** `test_benchmark_old_feature` fails on main; unrelated to meta-workflow v6.
**Evidence:** git log shows failure introduced in commit XYZ; not in our changeset.
```

Do not block our completion on pre-existing issues.
