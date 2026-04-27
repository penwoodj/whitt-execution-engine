# Upstream Factor Critical Review #1

**Date:** 2026-04-26
**Reviewer:** Sisyphus (Ralph Loop)
**Scope:** Full plan file suite + QA documentation + existing POC implementation
**Schema Source of Truth:** [docs/schema/unified-workflow-schema.yml](../schema/unified-workflow-schema.yml) (805 lines)

---

## Executive Summary

Critical review of 8 upstream factors affecting implementation success across the Whitt Execution Engine project. Each factor evaluates alignment between plan files, QA documentation, schema specification, and existing POC code.

**Overall Status:** ⚠️ 3 ALIGNED, 3 MISALIGNED, 2 BLOCKING

---

## Factor 1: Dependency Compatibility

- **Status:** ⚠️ MISALIGNED
- **Finding:** Multiple dependency risks identified in [Cargo.toml](../../Cargo.toml)
- **Plan Ref:** [Phase 00 Task 00](../plans/00-foundation/tasks/00-cargo-project-setup.md)
- **QA Ref:** [QA-00-01](../qa/phase-00/QA-CRITERIA.md)
- **Action Required:** Audit and document version pinning strategy

### Detailed Findings

| Dependency | Version | Risk | Notes |
|-----------|---------|------|-------|
| `serde-saphyr` | 0.0.24 | HIGH | Pre-0.1, API unstable. README mentions it but code uses `serde_yaml`-style patterns. No evidence of actual serde-saphyr usage in `src/`. |
| `autoagents` | 0.3.5 | HIGH | Crates.io shows this crate may not exist or have different API. No evidence of actual usage in `src/`. Plan files reference "Agent SDK" but implementation uses custom agent code. |
| `llama-cpp-2` | 0.1.138 | MEDIUM | Pre-1.0, API evolves rapidly. Vulkan feature requires specific driver versions. |
| `askama` | 0.12 | LOW | Stable template engine. Version 0.12 is mature. |
| `llama-cpp-2` vulkan | feature | MEDIUM | Vulkan backend requires AMD/NVIDIA drivers + ICD loader. Not portable. |
| `rusqlite` | 0.32 bundled | LOW | Bundled SQLite, stable. |

### Key Issue
Cargo.toml lists `serde-saphyr` and `autoagents` as dependencies but:
1. No `use serde_saphyr` or `use autoagents` found in `src/`
2. Plan files reference these as core dependencies but POC uses `serde_yaml`-compatible patterns and custom agent implementation
3. This creates a documentation-implementation gap that will confuse implementers

---

## Factor 2: Schema Alignment

- **Status:** ⚠️ MISALIGNED
- **Finding:** Plan files reference schema sections but alignment is inconsistent
- **Plan Ref:** All phase plans reference [unified-workflow-schema.yml](../schema/unified-workflow-schema.yml)
- **QA Ref:** All QA-CRITERIA.md files have Schema Ref fields
- **Action Required:** Verify schema line references match actual schema content

### Detailed Findings

| Area | Issue | Impact |
|------|-------|--------|
| Schema line refs | QA files reference "Lines 701-710" etc. but schema is 805 lines — need to verify these match | Medium |
| Schema v2.0 features | Schema defines providers, models, execution, logging, tools, state_management, orchestration, sub_workflows, metrics, hooks — many not yet in POC | High |
| Presence=enabled pattern | Schema uses "presence = enabled" pattern but Rust structs likely use `Option<T>` — need mapping strategy | Medium |
| Variable interpolation | Schema Section 15 (`${...}` references) — plan files describe interpolation but POC uses `minijinja` for basic cases | Low |

### Key Issue
The schema defines features far beyond what's implemented. Plan files for phases 04-07 reference schema sections for features that don't exist yet (memory, search, automation, metrics). The QA files correctly identify these gaps but the plan files don't explicitly acknowledge the implementation gap.

---

## Factor 3: API Surface Coverage

- **Status:** ✅ ALIGNED (for POC scope)
- **Finding:** POC CLI covers core use cases; plan files acknowledge remaining work
- **Plan Ref:** [Phase 02 Plan](../plans/02-cli-and-llm-backend-integration/plan.md)
- **QA Ref:** [QA-02](../qa/phase-02/QA-CRITERIA.md)
- **Action Required:** None for POC; track remaining for full implementation

### CLI Commands Status

| Command | Status | Notes |
|---------|--------|-------|
| `whitt chat` | ✅ Implemented | Interactive REPL with LLM |
| `whitt model list/info/download` | ✅ Implemented | Model management |
| `whitt server` | ✅ Implemented | Docker-based LLM server |
| `whitt agent` | ✅ Implemented | ReAct agent execution |
| `whitt workflow run` | ⏳ Planned | Phase 01-02 scope |
| `whitt benchmark` | ⏳ Planned | Phase 03 scope |
| `whitt schedule` | ⏳ Planned | Phase 05 scope |
| `whitt memory` | ⏳ Planned | Phase 04 scope |
| `whitt report` | ⏳ Planned | Phase 06 scope |

---

## Factor 4: Test Coverage Gaps

- **Status:** ❌ BLOCKING
- **Finding:** Validation framework requires 7 layers but POC only covers 3
- **Plan Ref:** [Validation Framework](../plans/validation-criteria/framework.md) (571 lines, 7 layers)
- **QA Ref:** All QA-CRITERIA.md files reference test commands
- **Action Required:** Implement property-based tests, E2E tests, log validation, benchmarks

### Test Coverage by Validation Layer

| Layer | Framework Requirement | Current Status | Gap |
|-------|-----------------------|----------------|-----|
| 1. Unit Tests | >90% coverage, cargo test --lib | ✅ 107 unit tests pass | Coverage % unknown (no tarpaulin) |
| 2. Integration | Multi-component communication | ✅ 8 resilience + 4 E2E | Limited integration |
| 3. Property-Based | 1000 iterations per property | ❌ MISSING | No proptest infrastructure |
| 4. E2E | Full workflow YAML→output | ⚠️ Partial | 4 E2E tests but no YAML workflow execution |
| 5. System Logs | Structured logging, scope | ❌ MISSING | No log validation tests |
| 6. Live CLI | User-facing command behavior | ⚠️ Manual only | TUTORIAL.md covers manual steps |
| 7. Benchmarks | Performance baselines | ❌ MISSING | No criterion infrastructure |

### Key Issue
The [validation framework](../plans/validation-criteria/framework.md) requires all 7 layers but the POC only has layers 1-2. Phases 03-06 plan to add layers 3-7 but there's no `proptest` or `criterion` in Cargo.toml dev-dependencies. Plan files need to include these as explicit dependencies in their task files.

---

## Factor 5: Documentation Accuracy

- **Status:** ⚠️ MISALIGNED
- **Finding:** README, Cargo.toml comments, and plan files contain stale references
- **Plan Ref:** [README.md](../../README.md) (tech stack section)
- **QA Ref:** [QA-00-01](../qa/phase-00/QA-CRITERIA.md) (project setup)
- **Action Required:** Update README tech stack, fix Cargo.toml comments

### Stale References

| Location | Issue | Fix |
|----------|-------|-----|
| [README.md Tech Stack](../../README.md) | Lists "serde-saphyr" as 1.5x faster than serde_yaml but no evidence of use | Verify and update |
| [README.md Tech Stack](../../README.md) | Lists "AutoAgents" as production agent SDK but code uses custom implementation | Remove or explain |
| [README.md Project Structure](../../README.md) | Shows `parser.rs`, `generator.rs`, `scheduler.rs` — none match actual `src/` layout | Update to match actual structure |
| [README.md Status](../../README.md) | Shows "Parser implementation (planned)" but POC has config parsing | Update status |
| [Cargo.toml line 25-27](../../Cargo.toml) | Comment says "1.5x faster than serde_yaml" for serde-saphyr | Verify or remove claim |
| [Cargo.toml line 41-43](../../Cargo.toml) | Comment says "Production agent orchestration SDK" for autoagents | Verify or remove |
| Plan files 04-06 | Reference crates that don't exist (sandlock, vidaimock) | Fixed in enrichment |

---

## Factor 6: Performance Baselines

- **Status:** ❌ BLOCKING
- **Finding:** No benchmarks defined, no performance baselines established
- **Plan Ref:** [Phase 03 Task 05](../plans/03-quality-loops/tasks/05-workflow-library.md) mentions benchmarks
- **QA Ref:** [QA-03-05](../qa/phase-03/QA-CRITERIA.md), Validation Framework Layer 7
- **Action Required:** Add criterion to dev-dependencies, define baseline metrics

### Missing Performance Infrastructure

| Metric | Required By | Current Status |
|--------|-------------|----------------|
| Step execution latency | Validation Layer 7 | ❌ Not measured |
| Memory usage per model | Schema execution.memory | ❌ Not measured |
| YAML parse time | Phase 00 | ❌ Not measured |
| Agent step throughput | Phase 01 | ❌ Not measured |
| Search query latency | Phase 04 | ❌ Not measured |
| Cron scheduling overhead | Phase 05 | ❌ Not measured |
| Dashboard refresh rate | Phase 06 | ❌ Not measured |

### Key Issue
No `criterion` in Cargo.toml. Plan files for phases 03+ reference benchmarks but don't include setup tasks. Phase 03 (Quality Loops) should include a benchmark infrastructure task.

---

## Factor 7: Security Posture

- **Status:** ✅ ALIGNED (for POC scope)
- **Finding:** Basic sandbox exists; plan files adequately describe security requirements
- **Plan Ref:** [Phase 01 Task 05](../plans/01-core-execution-engine/tasks/05-tool-sandbox.md)
- **QA Ref:** [QA-01-05](../qa/phase-01/QA-CRITERIA.md)
- **Action Required:** None for POC; full security audit deferred to Phase 07

### Security Implementation Status

| Feature | Status | Notes |
|---------|--------|-------|
| ToolSandbox (file read) | ✅ Implemented | Path validation, allowed_paths |
| ToolSandbox (file write) | ⏳ Planned | Phase 01 Task 05 |
| Shell command sandboxing | ⏳ Planned | Phase 01 Task 05 |
| Web operation robots.txt | ⏳ Planned | Phase 04 Task 05 |
| Rate limiting | ⏳ Planned | Schema provider retry |
| Confirmation prompts | ⏳ Planned | Schema tools.*.require_confirmation |

---

## Factor 8: Cross-Platform Compatibility

- **Status:** ✅ ALIGNED (Linux-primary with documented constraints)
- **Finding:** Project targets Linux with Vulkan/AMD GPU; Docker as cross-platform layer
- **Plan Ref:** [Phase 02](../plans/02-cli-and-llm-backend-integration/plan.md)
- **QA Ref:** [QA-02-03](../qa/phase-02/QA-CRITERIA.md) (Vulkan backend)
- **Action Required:** Document platform constraints in README

### Platform Matrix

| Platform | GPU | Status | Notes |
|----------|-----|--------|-------|
| Linux AMD GPU | Vulkan | ✅ Tested | Primary target, verified on live system |
| Linux NVIDIA GPU | CUDA | ⏳ Untested | llama-cpp-2 supports cuda feature |
| Linux CPU-only | CPU | ⚠️ Partial | Should work but not tested |
| macOS | Metal | ❌ Untested | llama-cpp-2 supports metal feature |
| Windows | Vulkan/CUDA | ❌ Untested | Docker may work |

### Vulkan Constraints (Documented)
- `--no-cache-prompt` MUST be used (KV cache serialization crash)
- `--cont-batching` MUST NOT be used (triggers crash)
- KV cache MUST use `f16` (quantized types crash)
- `--flash-attn on` is safe and recommended

---

## Summary of Findings

| Factor | Status | Severity | Action |
|--------|--------|----------|--------|
| 1. Dependency Compatibility | ⚠️ MISALIGNED | HIGH | Audit serde-saphyr, autoagents usage; update Cargo.toml + README |
| 2. Schema Alignment | ⚠️ MISALIGNED | MEDIUM | Verify schema line refs in QA files; document implementation gap |
| 3. API Surface Coverage | ✅ ALIGNED | LOW | Track remaining commands for full implementation |
| 4. Test Coverage Gaps | ❌ BLOCKING | HIGH | Add proptest, criterion to Cargo.toml; implement missing test layers |
| 5. Documentation Accuracy | ⚠️ MISALIGNED | MEDIUM | Update README tech stack, project structure, status |
| 6. Performance Baselines | ❌ BLOCKING | HIGH | Add criterion; define baseline metrics in Phase 03 |
| 7. Security Posture | ✅ ALIGNED | LOW | Adequate for POC; defer full audit to Phase 07 |
| 8. Cross-Platform | ✅ ALIGNED | LOW | Document constraints; Linux-primary is acceptable |

## Action Items (Priority Order)

1. **P0:** Audit `serde-saphyr` and `autoagents` — determine if they're actually used or should be removed from Cargo.toml
2. **P0:** Add `proptest` and `criterion` to `[dev-dependencies]` in Cargo.toml
3. **P1:** Update [README.md](../../README.md) tech stack section to match actual implementation
4. **P1:** Update [README.md](../../README.md) project structure to match actual `src/` layout
5. **P1:** Verify schema line references in all QA-CRITERIA.md files against actual schema content
6. **P2:** Add benchmark infrastructure task to Phase 03 plan
7. **P2:** Document Vulkan constraints in README or deployment guide
8. **P2:** Add property-based test tasks to Phase 03 plan
