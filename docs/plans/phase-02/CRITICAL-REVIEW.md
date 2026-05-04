# Critical Review: Workflow Engine Implementation

## Review Date
2026-05-04

## Scope
Critical review of Plan 1 (YAML-Driven Benchmark) and Plan 2 (Full Schema Compliance) covering implementation feasibility, upstream risks, and verification strategy.

## Upstream Factor Analysis

### Factor 1: Dependency Compatibility

| Crate | Version | Status | Risk |
|-------|---------|--------|------|
| `serde-saphyr` | 0.0.24 | Early release, limited adoption | HIGH — may have YAML edge cases |
| `serde` | 1.0 | Stable, battle-tested | LOW |
| `serde_json` | 1.0 | Stable | LOW |
| `petgraph` | 0.6.x | Stable, widely used | LOW (optional dependency) |
| `tokio` | 1.x | Stable | LOW |

**Concern:** `serde-saphyr` v0.0.24 is pre-1.0 with limited ecosystem adoption. The project already uses it for config parsing (works for `ModelsConfig`, `ProvidersConfig`, `LoopType`). However, these are relatively flat structures. The workflow schema has deeply nested structures with `#[serde(untagged)]` enums, optional fields everywhere, and the `r#loop` reserved word escaping.

**Mitigation:**
1. Test `serde-saphyr` deserialization of the full schema with a unit test BEFORE writing engine code
2. If it fails on complex cases, fallback to `serde_json::Value` parsing (what the current `parse_workflow_context()` does, but properly)
3. Consider `serde_yml` (1.0.x, more mature) as alternative if serde-saphyr proves insufficient

### Factor 2: Schema Alignment

The unified-workflow-schema.yml is a **comment-based reference schema** — it shows examples with inline comments, not a machine-readable JSON Schema or OpenAPI spec. This means:

1. **No automated validation possible** against the "schema" — it's a documentation file, not a validation spec
2. The validator must be hand-coded based on reading the comments
3. Edge cases in the comments (aliases like "checkpoint" for "bookmark") must be manually tracked

**Finding:** The schema at line 293 says `steps:` is a map. But the YAML generator at `src/benchmark/yaml_generator.rs` lines 90-185 generates `- step:` arrays. This is a direct contradiction between the code and the schema.

**Action Required:**
1. Update yaml_generator.rs to produce `steps:` map format
2. OR update the schema to accept both formats
3. Decision: **Update the code** — schema is the source of truth per AGENTS.md

### Factor 3: API Surface Coverage

Current CLI `benchmark` command has these flags:
- `--url`, `--prompt`, `--max-tokens`, `--models-dir`, `--model-list`
- `--prompts` (count), `--output`, `--filter-size-max`, `--filter-name`
- `--compare-gpu-cpu`, `--output-dir`, `--workflow`

Missing CLI flags for schema features:
- `--temperature` (no CLI flag, not in BenchmarkConfig)
- `--top-p` (no CLI flag)
- `--validate` (dry-run validation without execution)
- `--step` (run specific step only)

**Impact:** Without `--temperature` and `--top-p` CLI flags, users can only set these via YAML. This is fine for YAML-driven execution but limits CLI-only usage.

### Factor 4: Test Coverage Gaps

Current benchmark test coverage:

| Test | What It Tests | Gap |
|------|---------------|-----|
| `test_benchmark_config_with_compare_gpu_cpu` | Config construction | Doesn't test execution |
| `test_benchmark_suite_result_csv` | CSV formatting | No file I/O test |
| `test_benchmark_suite_result_table` | Table formatting | No file I/O test |

**Missing tests:**
1. YAML parsing of benchmark section (no test exists)
2. Workflow step dispatch (no test exists)
3. Hook execution (no test exists)
4. Variable interpolation in workflow context (no test exists)
5. Config merge (YAML + CLI override precedence)
6. Schema validation (accepts/rejects correctly)
7. Dependency resolution (topological sort with cycles)
8. End-to-end workflow execution (integration test)

**Required:** Add at least 8 new unit tests and 1 integration test.

### Factor 5: Documentation Accuracy

| Document | Status | Issue |
|----------|--------|-------|
| `docs/schema/unified-workflow-schema.yml` | Source of truth | No machine-readable validation spec |
| `docs/schema/hooks-semantics.md` | 1271 lines | Very detailed but no Rust mapping |
| `docs/schema/hooks-integration-plan.md` | 178 lines | Outdated — references old architecture |
| `docs/workflows/benchmarks/*.yml` | Non-compliant | Array format, missing sections |
| `src/benchmark/yaml_generator.rs` | Generates non-compliant YAML | Must be updated |

### Factor 6: Performance Baselines

No benchmarks exist for:
1. YAML parsing throughput (how fast can we parse 50-model YAML?)
2. Workflow execution overhead (step dispatch, hook firing)
3. Variable interpolation latency

**Impact:** Likely negligible for benchmark use case (3-50 models, minutes per model). But should verify with 50-model workflow.

### Factor 7: Security Posture

Workflow YAML files contain file paths (`to_file_path`, `save_to`). If workflows come from untrusted sources, path traversal is possible.

**Current state:** No path validation. `workspace/` paths are relative but not validated.

**Mitigation:** Add path validation in hook execution — reject paths outside `workspace/` root. This aligns with schema's `tool_permissions.file_operations.allowed_paths`.

### Factor 8: Cross-Platform Compatibility

| Concern | Linux | macOS | Windows |
|---------|-------|-------|---------|
| Docker commands | ✅ | ✅ | ⚠️ (WSL) |
| Path separators | ✅ | ✅ | ❌ (backslash) |
| File permissions | ✅ | ✅ | ❌ (different model) |
| `std::fs` operations | ✅ | ✅ | ✅ |

**Impact:** Benchmark system uses Docker heavily. Windows support would require WSL. The `workspace.permissions` section in schema uses Unix permission modes (700, 755, etc.) — non-portable.

## Implementation Success Factors

### Must-Have (Blocks Release)
1. ✅ YAML parsing of benchmark section works
2. ✅ Prompts from YAML are sent to LLM (not CLI defaults)
3. ✅ max_tokens, temperature, top_p from YAML
4. ✅ compare_modes triggers GPU+CPU runs
5. ✅ Schema validation catches missing required sections
6. ✅ All existing tests continue to pass

### Should-Have (Blocks Full Compliance)
7. Step dispatch for all 4 benchmark steps
8. Hook execution for log/append_to
9. Dependency resolution (topological sort)
10. Variable interpolation in workflow context
11. Rewritten schema-compliant YAML files

### Nice-to-Have (Future)
12. All 6 hook timing points
13. GWT (Given-When-Then) conditional routing
14. Sub-workflow execution
15. RAG memory integration
16. Checkpointing with resume

## Verification Strategy

### Phase A: Static Verification (No Live System)

```bash
cargo test --all-features          # All tests pass
cargo clippy --all-features        # 0 warnings
cargo build --release --all-features  # Exit code 0
```

Plus new tests:
- `test_parse_benchmark_3_models_yaml` — parse and verify fields
- `test_validate_compliant_yaml` — passes validation
- `test_validate_missing_providers` — warns correctly
- `test_config_merge_yaml_overrides` — YAML wins when CLI is default
- `test_config_merge_cli_wins` — CLI wins when explicitly set
- `test_topological_sort_linear` — correct order
- `test_topological_sort_cycle` — detects error
- `test_step_type_inference` — all types detected

### Phase B: Live System Verification

1. Start server: `docker compose up -d`
2. Run schema-compliant workflow:
   ```bash
   cargo run --release --bin whitt --features client -- \
     benchmark --url http://localhost:8081 \
     --workflow docs/workflows/benchmarks/benchmark-3-models.yml \
     --output-dir ./benchmark-attempts/schema-test \
     --output table
   ```
3. Verify against QA-WORKFLOW-EXECUTION-DISCREPANCY.md criteria:
   - [ ] Criterion 1: Prompts match YAML (not "quick brown fox")
   - [ ] Criterion 2: max_tokens=128, temperature=0.7, top_p=0.9
   - [ ] Criterion 3: GPU+CPU comparison runs
   - [ ] Criterion 4: All 4 steps execute
   - [ ] Criterion 5: YAML passes validation
   - [ ] Criterion 6: All output files present

### Phase C: Regression Verification

After implementation:
1. Run WITHOUT `--workflow` — should still work with CLI defaults
2. Run WITH `--workflow` + CLI overrides — CLI should win
3. Run 5-model and 15-model YAMLs — scalability check
4. Verify all existing benchmark tests still pass

## Conclusion

The plans are feasible. Main risks are:
1. `serde-saphyr` maturity (mitigated by `serde_json::Value` fallback)
2. Schema compliance matrix is large (mitigated by phased implementation)
3. Step types 2-4 are complex (mitigated by stubbing initially)

Recommended implementation order:
1. Plan 1 Steps 1-4 (YAML parsing + config merge) — **do first**
2. Verify with live 3-model run
3. Plan 2 Steps 1-4 (schema types + validator + YAML rewrite) — **do second**
4. Plan 1 Steps 5-8 + Plan 2 Steps 5-10 (engine + hooks + variables) — **do third**
5. Full QA cycle
