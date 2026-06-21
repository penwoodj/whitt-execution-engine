# Whitt Execution Engine — Agent Operating Rules

## Operating Modes

### Engineering Mode
Implement features, fix bugs, write code. Verify with `cargo test`, `cargo clippy`. Commit with conventional commits.

### QA Mode
Run verification suite. Find issues. Document findings in `docs/qa/phase-XX/QA-FINDINGS.md`. DO NOT fix — only document and escalate.

### Mode Switching
- **Engineering → QA**: After implementing a feature or completing a task, switch to QA mode to verify.
- **QA → Engineering**: After documenting findings, switch to Engineering mode to fix issues found.
- **Explicit switch**: State "Switching to QA mode" or "Switching to Engineering mode" before each transition.

---

## The QA → Engineer → QA Cycle

This project operates on a strict verification loop:

```
┌──────────────┐     ┌──────────────┐     ┌──────────────┐
│  QA Mode     │────→│  Engineer    │────→│  QA Mode     │
│  Find Issues │     │  Fix Issues  │     │  Verify Fix  │
└──────────────┘     └──────────────┘     └──────────────┘
       ↑                                         │
       └───────────── Issues found? ──────────────┘
                         │
                    All PASS?
                         │
                    ┌────▼────┐
                    │  DONE   │
                    └─────────┘
```

### Cycle Rules

1. **QA Mode FIRST**: Before implementing, understand what "done" looks like by reading QA criteria.
2. **Engineer Mode**: Implement minimum viable change to satisfy QA criteria.
3. **QA Mode AGAIN**: Verify the implementation against ALL QA criteria for the area.
4. **Loop**: If QA fails, back to Engineer mode. If QA passes, move to next task.
5. **Maximum 3 cycles per issue**: After 3 failed fix attempts, escalate to Oracle or user.

### Evidence Requirements

Every QA pass must produce evidence:
- **Unit tests**: `cargo test --all-features` output showing pass count
- **Clippy**: `cargo clippy --all-features -- -W clippy::all` showing 0 warnings
- **Build**: `cargo build --release --all-features` exit code 0
- **LSP diagnostics**: 0 errors on changed files
- **Manual verification**: For CLI/Docker features, actual command output

---

## QA Area Framework

### Status Levels
| Status | Meaning |
|--------|---------|
| ✅ PASS | All criteria met, evidence documented |
| ✅ PASS (Fixed) | Previously failed, fix committed and verified |
| ⚠️ PARTIAL | Some criteria met, limitations documented and accepted |
| 🔵 DEFERRED | Not in current scope, tracked for future |
| ❌ FAIL | Criteria not met, blocking issue |

### Issue Severity
| Severity | Definition | Response Time |
|----------|-----------|---------------|
| HIGH | Blocks functionality or data integrity | Fix immediately |
| MEDIUM | Partial functionality, acceptable limitation | Fix in current cycle |
| LOW | Code quality, minor enhancement | Document, fix opportunistically |

### Priority Classification
| Priority | Meaning |
|----------|---------|
| P0 | Critical — must pass for release |
| P1 | Important — should pass, non-blocking |
| P2 | Nice-to-have — track for future |

---

## Test Types

| Type | Definition | Example Command |
|------|-----------|-----------------|
| Unit | Isolated, no external deps | `cargo test test_name --lib` |
| Integration | Multi-component | `cargo test --test integration_test` |
| E2E | Full workflow | `cargo test --test e2e_integration` |
| Property | Invariant checking | `cargo test property_ --lib` |
| Build | Compilation check | `cargo build --release --all-features` |
| Clippy | Lint check | `cargo clippy --all-features -- -W clippy::all` |
| Manual | Human verification | CLI commands, Docker operations |

---

## Verification Protocol

### Before Claiming Completion

1. Run `cargo test --all-features` — ALL tests pass
2. Run `cargo clippy --all-features -- -W clippy::all` — 0 warnings
3. Run `lsp_diagnostics` on ALL changed files — 0 errors
4. Run `cargo build --release --all-features` — exit code 0
5. For CLI changes: test actual command on live system
6. Document evidence in QA findings file

### Commit Hygiene

- Use conventional commits: `feat:`, `fix:`, `ref:`, `docs:`, `test:`, `chore:`
- Subject ≤70 chars, imperative mood
- Body explains WHY, not WHAT
- Include `Co-Authored-By:` for AI-generated changes
- NEVER commit without explicit user request
- NEVER suppress type errors with `as any`, `@ts-ignore`

---

## Project Structure Awareness

### Source Code Map
```
src/
├── lib.rs              # Root module
├── error.rs            # Error types (151 lines)
├── config/             # YAML configuration (3 files, 1790 lines)
│   ├── mod.rs          # Config loading, validation
│   ├── provider.rs     # Provider-specific config
│   └── unified.rs      # Unified config schema
├── model/              # Model specifications (4 files, 1709 lines)
│   ├── schema.rs       # Model spec structs
│   ├── registry.rs     # Model lifecycle
│   ├── resource.rs     # Resource management
│   └── interpolation.rs # Template interpolation
├── agent/              # Agent execution engine (6 files, 2284 lines)
│   ├── tools.rs        # Tool registry, 6 tools
│   ├── executor.rs     # Step execution
│   ├── react.rs        # ReAct agent
│   ├── streaming.rs    # SSE streaming
│   ├── persistence.rs  # Workflow checkpointing
│   └── sandbox.rs      # Tool sandbox security
├── backend/            # LLM backends (3 files, 1230 lines)
│   ├── llm_backend.rs  # Backend trait
│   ├── llama_vulkan.rs # Vulkan backend
│   └── mock_backend.rs # Mock for testing
├── client/             # HTTP client (5 files, 890 lines)
│   ├── http_client.rs  # HTTP client with SSE
│   ├── model_download.rs # HuggingFace download
│   ├── docker_manager.rs # Docker management
│   ├── prompt_chain.rs # Prompt chaining
│   └── types.rs        # API types
└── bin/                # CLI binaries (3 files, 1644 lines)
    ├── whitt.rs        # Main CLI
    ├── model_chain.rs  # Model chain
    └── poc_client.rs   # PoC client
```

### Plan Phases
| Phase | Name | Tasks | Status |
|-------|------|-------|--------|
| 00 | Foundation | 12 | Plan complete |
| 01 | Core Execution Engine | 13 | Plan complete |
| 02 | CLI & LLM Backends | 13 | POC implemented |
| 03 | Quality Loops | 7 | Plan complete |
| 04 | Memory & Search | 9 | Plan complete |
| 05 | Automation | 9 | Plan complete |
| 06 | Autonomy Metrics | 10 | Plan complete |
| 07 | Final Validation | 12 | Plan complete |

### QA Documentation Map
```
docs/qa/
├── extended-poc/           # 20 QA areas, all PASS
│   ├── QA-AREAS-EXTENDED-POC.md
│   ├── QA-TEST-PROCEDURES-EXTENDED-POC.md
│   ├── QA-CONFIG-SCHEMA-EXTENDED-POC.md
│   └── findings/           # Area-specific findings
├── poc-local-llm-docker/   # 10 QA areas, all CONFIRMED WORKING
│   ├── QA-AREAS.md
│   └── findings/
└── phase-XX/               # Per-phase QA suites (being built)
    ├── QA-CRITERIA.md
    ├── QA-TEST-CASES.md
    ├── QA-FINDINGS.md
    └── CROSS-REF.md
```

---

## Critical Constraints

### Vulkan Backend
- `--no-cache-prompt` MUST be used (Vulkan cannot serialize KV cache state)
- `--cont-batching` MUST NOT be used (triggers KV cache serialization on slot release)
- KV cache uses `q8_0` (works on Vulkan/RADV with Qwen3.5-9B; f16 also safe)
- `--flash-attn on` is safe and recommended
- `--gpu-layers 99` (full offload) confirmed fastest on AMD RX 580 8GB; CPU-only unusable

### Docker
- Use base `docker/docker-compose.yml` (not AMD or NVIDIA variants) for AMD GPU
- Mount entrypoint.sh at `/entrypoint.sh:ro` (not `/app/entrypoint.sh`)
- Server entrypoint is `['tini', '--', '/entrypoint.sh']`

### Schema Source of Truth
- `docs/schema/unified-workflow-schema.yml` (805 lines) is THE source of truth
- All implementations must reference specific schema line numbers
- Schema version minimum: 2.0.0

### Provider Configuration (CRITICAL)
- This project uses **llama.cpp with Vulkan** in Docker — NOT LM Studio, NOT Ollama
- Provider key MUST be `llama_cpp_with_vulkan` (schema line 28)
- Provider config MUST use `config:` wrapper with `host:` and `port:` (schema line 29-31)
- Model host.type MUST be `llama_cpp_with_vulkan` (schema line 71)
- NEVER use `lmstudio` or `ollama` in any workflow YAML or generated output
- Validation MUST reject any provider other than `llama_cpp_with_vulkan` in current POC scope
- When adding future provider support: update validation FIRST, then add provider config

### Strict Schema Validation Rules
- **ONLY keys defined in `docs/schema/unified-workflow-schema.yml` are allowed** in workflow YAMLs
- Non-schema extensions (benchmark:, model_list:, logging:, execution:) are FORBIDDEN
- Redundant config is FORBIDDEN: if providers already defines host/port, models must NOT duplicate with connection_settings
- If `workflow_execution_strategy.load_unload` is set, `model_lifecycle.load_unload_strategy` must NOT duplicate it
- `WorkflowFile::validate()` MUST reject unknown top-level keys not in the schema
- Every new key added to YAMLs MUST have a schema line reference comment
- When deferring schema features: add `# 🔵 DEFERRED: <explanation>` comment in the YAML

### QA Discipline Rules
- **ALWAYS QA from schema source of truth** — compare YAML output line-by-line against `docs/schema/unified-workflow-schema.yml`
- **NEVER assume** a provider or config structure — read the schema first
- **Track deferred features** explicitly: mark schema sections not yet implemented with `🔵 DEFERRED` and a comment explaining what future work is needed
- **Validate before claiming done** — run `WorkflowFile::validate()` on all YAMLs before marking QA PASS
- **Redundancy check**: before writing config, check if the same value is already set at a higher scope

---

## Upstream Factor Review Template

When performing critical reviews, evaluate at least 8 factors:

1. **Dependency Compatibility** — Do crate versions align? Any known breaking changes?
2. **Schema Alignment** — Does implementation match unified-workflow-schema.yml?
3. **API Surface Coverage** — Are all planned CLI commands implemented?
4. **Test Coverage Gaps** — Are there untested code paths?
5. **Documentation Accuracy** — Do docs match actual behavior?
6. **Performance Baselines** — Are benchmarks defined and passing?
7. **Security Posture** — Are sandbox/permission checks adequate?
8. **Cross-Platform Compatibility** — Does it work on Linux/macOS/Windows?

Document findings in format:
```markdown
### Factor N: [Name]
- **Status**: ✅ ALIGNED / ⚠️ MISALIGNED / ❌ BLOCKING
- **Finding**: [description]
- **Plan Ref**: [link to plan file]
- **QA Ref**: [link to QA criteria]
- **Action Required**: [what needs to happen]
```

---

## Remaining Work — Live System Completion

**Full gap analysis**: `.opencode-handoff.md` contains the complete remaining work plan.

### Phase Summary (MUST complete in order)

| Phase | Description | Status | Key Gap |
|-------|-------------|--------|---------|
| 1 | Fix output directory structure & JSON content | ✅ RESOLVED | `outputs/output/` works, JSON = model text, json_parsable works via context |
| 2 | Fix benchmark YAML files | ✅ DONE | Prompts unified, hook configs identical, shell resource hooks added |
| 3 | Make all execution hook-driven | ✅ NEARLY DONE | 12/12 actions, 7/10 triggers fully wired, 2/10 partial (GWT logging), 1/10 blocked (streaming) |
| 4 | Implement remaining userflows | ❌ NOT STARTED | 20 userflows — specs only, need business logic (UF05=95%, UF16=85%, UF18=90%) |
| 5 | Write extensive QA documentation | ❌ NOT STARTED | Need live system test procedures, hook coverage, userflow coverage |
| 6 | Execute QA and iterate | ❌ NOT STARTED | Run all tests on live Docker system, fix failures |
| 7 | Final verification | ✅ PASSING | 567 tests, 0 failures, 0 clippy errors, build clean |

### Critical Bugs to Fix First

1. ~~**Output path mismatch**: YAML says `outputs/json/` but files go to `outputs/output/`~~ → **RESOLVED**: YAMLs use `outputs/output/`, files go there.
2. ~~**JSON content unknown**: No test verifies output = model text only (no metadata)~~ → **RESOLVED**: JSON files contain valid model text.
3. ~~**Missing parsability log**: benchmark.log doesn't contain `json_parsable` field~~ → **NOT A BUG**: `AfterStepSucceedsContext.get_field("json_parsable")` works — parses output as JSON dynamically.
4. ~~**Hardcoded behaviors**: 9 categories of execution logic not controllable via YAML hooks~~ → **RESOLVED**: Shell action added, `run_model_inference()` extracted, deprecated path isolated.
5. ~~**Stub hook context**: HookContext populated with empty data~~ → **NOT A BUG**: All contexts populated with actual execution state (step_name, model_name, output, duration_ms, etc.)

### Architecture Principle

**ALL execution behavior MUST come from YAML hooks.** If a feature can't be expressed via schema properties + hooks, it doesn't belong in the engine. No exceptions.

---

## Generator Iteration Protocol (MANDATORY)

When iterating on the YAML workflow generator, every iteration MUST follow this process:

### Required Steps (no skipping)

```
1. Generate YAML workflow
2. Validate YAML (schema_valid=true in logs)
3. Run live workflow (`whitt benchmark --workflow <file>`)
4. Inspect logs (`./scripts/analyze-run.sh LOG OUTPUT`)
5. Bug/correctness analysis
6. Quality analysis
7. Validate iteration gate (`./scripts/validate-iteration.sh LOG OUTPUT YAML [PREV_LOG]`)
8. Document in WORKFLOW_RELIABILITY_TRACKING.md
9. Determine next action from allowed list
```

### Mandatory Evidence Gate

Run `./scripts/validate-iteration.sh` — all 8 checks MUST pass before claiming improvement.

If logs are missing or unclear:
> "Cannot validate improvement because logs are insufficient."

### Allowed Next Actions

| Action | Trigger |
|--------|---------|
| Fix code bug | Engine correctness failures |
| Improve generated YAML | Schema validation failure |
| Improve generator logic | Generator produces low-quality YAML |
| Improve hooks | Hook actions misbehave |
| Improve logging | Cannot extract required metrics |
| Improve test workflow | Need better test coverage |
| Stop — acceptable | All gates pass |

### When NOT to Claim Improvement

- Did not run a live workflow → cannot validate
- Log file missing or empty → cannot validate
- No `[workflow:end]` event → run incomplete
- `./scripts/validate-iteration.sh` failed → fix issues first
- No comparison against previous iteration → no improvement proof

### Tracking

All iterations tracked in `WORKFLOW_RELIABILITY_TRACKING.md` with:
- YAML path, log path, run ID
- 8-point evidence checklist
- Comparison table with numeric metrics
- Next action (from allowed list only)

---

## Lifecycle Hooks & Actions — Verification Gap Analysis

**Last updated:** 2026-05-30
**Commit:** `c29dcfc` (add Shell action)

### Inventory: 10 Triggers × 12 Actions = 120 Possible Combinations

#### Triggers (WorkflowHookContext variants in `src/workflow/hooks/context.rs`)

| # | Trigger | Context Struct | Runner Firing Point | FIRED IN RUNNER? |
|---|---------|---------------|---------------------|------------------|
| 1 | `before_step_starts` | `BeforeStepStartsContext` | `runner.rs:1257` | ✅ YES |
| 2 | `during_step_streaming` | `DuringStepStreamingContext` | — | ❌ NO — requires SSE streaming path (stream:false hardcoded) |
| 3 | `after_step_succeeds` | `AfterStepSucceedsContext` | `runner.rs:1340` | ✅ YES |
| 4 | `after_step_fails` | `AfterStepFailsContext` | `runner.rs:1318` | ✅ YES |
| 5 | `after_all_retries_exhausted` | `AfterAllRetriesExhaustedContext` | `runner.rs:1355` | ✅ YES — fires after after_step_fails when error contains 'attempts failed' |
| 6 | `after_step_starts` | `AfterStepStartsContext` | `runner.rs:1286` | ✅ YES |
| 7 | `before_gwt_evaluates` | `BeforeGwtEvaluatesContext` | `actions.rs:404` | ⚠️ PARTIAL — info logging only, full wire requires hook_config in execute_gwt |
| 8 | `after_gwt_evaluates` | `AfterGwtEvaluatesContext` | `actions.rs:414` | ⚠️ PARTIAL — info logging only, full wire requires hook_config in execute_gwt |
| 9 | `on_requires_failed` | `OnRequiresFailedContext` | `runner.rs:1535` | ✅ YES — checks step.requires against step_outputs |
| 10 | `after_loop_iteration_fails` | `AfterLoopIterationFailsContext` | `runner.rs:1592` | ✅ YES — fires when iteration result has error |

**Runner firing gap: 1/10 triggers fully dead (during_step_streaming), 2/10 partially wired (GWT with logging).** 7/10 fully wired.

#### Actions (HookAction variants in `src/workflow/step.rs:176-200`)

| # | Action | Execute Function | File I/O | State Mutation | Control Flow |
|---|--------|-----------------|----------|----------------|--------------|
| 1 | `Log(LogAction)` | `execute_log()` | ✅ file write + stdout | No | Continue |
| 2 | `AppendTo(AppendToAction)` | `execute_append_to()` | ✅ file append | ✅ bookmark store (Variable) | Continue |
| 3 | `SaveTo(SaveToAction)` | `execute_save_to()` | ✅ file write | ✅ bookmark store | Continue |
| 4 | `RouteTo(RouteToAction)` | `execute_route_to()` | No | No | RouteTo |
| 5 | `Bookmark(BookmarkAction)` | `execute_bookmark()` | ✅ file write | ✅ bookmark store | Continue |
| 6 | `Notify(NotifyAction)` | `execute_notify()` | No | ✅ notify_tx.try_send() | Continue |
| 7 | `Fail(FailAction)` | `execute_fail()` | No | No | Fail |
| 7b | `Shell(ShellAction)` | `execute_shell()` | No | ✅ bookmark store ("shell_output") | Continue/Fail |
| 8 | `SkipStep(bool)` | `execute_skip_step()` | No | No | SkipStep/Continue |
| 9 | `SkipRemaining(bool)` | `execute_skip_remaining()` | No | No | SkipRemaining/Continue |
| 10 | `Gwt(Vec<GwtClause>)` | `execute_gwt()` | No | No | RouteTo/Continue |
| 11 | `IterateValues(HashMap)` | — passthrough | No | No | Continue (stub) |

---

### Verification Matrix: 4 Test Levels

Legend: ✅ = verified, ❌ = not verified, ⚠️ = partial, — = N/A

#### Level 1: Unit Tests (action isolation, `src/workflow/hooks/actions.rs` + `context.rs` + `gwt.rs` + `mod.rs`)

| Action | Constructed | Executed | Result Asserted | File I/O Verified | State Verified |
|--------|-------------|----------|-----------------|-------------------|----------------|
| `Log` (to_file_path) | ✅ | ✅ | ✅ Continue | ✅ file exists | — |
| `Log` (nested dirs) | ✅ | ✅ | ✅ Continue | ✅ dirs created | — |
| `Log` (stdout only) | ⚠️ implicit | ⚠️ not tested standalone | ❌ | — | — |
| `Log` (event_fields) | ✅ | ✅ | ✅ | — | — |
| `Log` (level variants) | ✅ Warning/Error/Critical/Debug | ✅ | ✅ | — | — |
| `AppendTo::FilePath` | ✅ | ✅ | ✅ Continue | ✅ file appended | — |
| `AppendTo::Variable` | ✅ | ✅ | ✅ Continue | — | ✅ bookmark stored+concatenated |
| `AppendTo::Both` | ✅ | ✅ | ✅ Continue | ✅ file appended | ✅ bookmark stored |
| `SaveTo::FilePath` | ✅ | ✅ | ✅ Continue | ✅ file content verified | — |
| `SaveTo::Variable` ($$prefix) | ✅ | ✅ | ✅ Continue | — | ✅ bookmark stored |
| `SaveTo::Both` | ✅ | ✅ | ✅ Continue | ✅ file | ✅ bookmark stored |
| `RouteTo::Single` | ✅ | ✅ | ✅ RouteTo{1 target} | — | — |
| `RouteTo::Multiple` | ✅ | ✅ | ✅ RouteTo{3 targets} | — | — |
| `Bookmark::Flag(true)` | ✅ | ✅ | ✅ Continue | — | ✅ bookmark stored |
| `Bookmark::Path("...")` | ✅ | ✅ | ✅ Continue | ✅ file exists | ✅ bookmark stored |
| `Bookmark::Detailed{path}` | ✅ | ✅ | ✅ Continue | ✅ file exists + content | ✅ bookmark stored |
| `Bookmark::Detailed{path:None}` | ✅ | ✅ | ✅ Continue | — | ✅ bookmark stored |
| `Notify` (no channel) | ✅ | ✅ | ✅ Continue | — | — |
| `Notify` (with channel) | ✅ | ✅ | ✅ Continue | — | ✅ try_send verified |
| `Fail` (with message) | ✅ | ✅ | ✅ Fail{reason} | — | — |
| `Fail` (no message) | ✅ | ✅ | ✅ Fail{default} | — | — |
| `SkipStep(true)` | ✅ | ✅ | ✅ SkipStep | — | — |
| `SkipStep(false)` | ✅ | ✅ | ✅ Continue | — | — |
| `SkipRemaining(true)` | ✅ | ✅ | ✅ SkipRemaining | — | — |
| `SkipRemaining(false)` | ✅ | ✅ | ✅ Continue | — | — |
| `Gwt` (matching clause) | ✅ | ✅ | ✅ RouteTo | — | — |
| `Gwt` (non-matching) | ✅ | ✅ | ✅ Continue | — | — |
| `Gwt` (invalid condition) | ✅ | ✅ | ✅ Continue (false) | — | — |
| `Gwt` (multiple clauses) | ✅ | ✅ | ✅ first-match semantics | — | — |
| `Gwt` (nested field access) | ✅ via gwt module | ✅ | ✅ | — | — |
| `Gwt` (arithmetic in given) | ✅ via gwt module | ✅ | ✅ | — | — |
| `IterateValues` | ✅ | ✅ passthrough | ✅ Continue | — | — |
| `Shell` (echo + bookmark) | ✅ | ✅ | ✅ Continue | — | ✅ bookmark stored |
| `Shell` (false command) | ✅ | ✅ | ✅ Fail | — | — |
| `Shell` (fail_on_error=false) | ✅ | ✅ | ✅ Continue | — | — |
| `Shell` (missing command) | ✅ | ✅ | ✅ Fail | — | — |
| `Shell` (env vars) | ✅ | ✅ | ✅ Continue | — | ✅ bookmark verified |

**Unit test coverage: ~95% of action variants.** All action variants tested including Shell. IterateValues is passthrough (future feature).

#### Context struct unit tests (`src/workflow/hooks/context.rs`)

| Context | to_json_value | get_field | trigger_name |
|---------|--------------|-----------|--------------|
| BeforeStepStarts | ✅ | ✅ | ✅ |
| DuringStepStreaming | ✅ | ✅ | ✅ |
| AfterStepSucceeds | ✅ (incl. quality_score None) | ✅ | ✅ |
| AfterStepFails | ✅ | ✅ (nested error.is_retryable) | ✅ |
| AfterAllRetriesExhausted | ✅ | ❌ | ✅ |
| AfterStepStarts | ✅ | ❌ | ✅ |
| BeforeGwtEvaluates | ✅ | ❌ | ✅ |
| AfterGwtEvaluates | ✅ | ❌ | ✅ |
| OnRequiresFailed | ✅ | ❌ | ✅ |
| AfterLoopIterationFails | ✅ | ❌ | ✅ |
| **All variants unique** | — | — | ✅ (10 unique) |

#### HookEngine unit tests (`src/workflow/hooks/mod.rs`)

| Feature | Tested |
|---------|--------|
| HookEngine::new() empty bookmarks | ✅ |
| store_bookmark / get_bookmark | ✅ |
| HookResult::merge priority | ✅ (Fail > SkipRemaining > RouteTo > SkipLoop > SkipStep > Continue) |
| is_continue() | ✅ |
| is_terminal() | ✅ (Fail, SkipRemaining) |
| Default trait | ✅ |

#### GWT expression evaluator (`src/workflow/hooks/gwt.rs`)

| Category | Tests | Coverage |
|----------|-------|----------|
| Literals (true/false/null/number/string) | ✅ 7 tests | Complete |
| Field paths (simple, nested, deep, missing) | ✅ 5 tests | Complete |
| Comparisons (==, !=, >, <, >=, <=) | ✅ all operators for num/str/bool/null | Complete |
| Logical (&&, \|\|, !) | ✅ 5 tests | Complete |
| Arithmetic (+, -, *, /) | ✅ 5 tests | Complete |
| Precedence (and vs or, comparison vs logical, arithmetic vs comparison) | ✅ 3 tests | Complete |
| Error cases (div/0, type mismatch, syntax errors, invalid escape, unterminated string) | ✅ 6 tests | Complete |
| Parenthesized expressions | ✅ | Complete |
| Complex multi-field expressions | ✅ 2 tests | Complete |

**GWT evaluator: ~95% unit test coverage.** Well-tested module.

---

#### Level 2: Integration Tests (`tests/hooks_integration.rs`)

Tests use `execute_action()` directly (not via runner). Verify end-to-end action dispatch from HookAction enum through to result.

| Test | Action | Context | File I/O | State |
|------|--------|---------|----------|-------|
| `given_all_triggers_fixture_when_read_then_contains_all_triggers` | — fixture parse | — | ✅ file read | — |
| `given_log_action_when_executed_then_file_created_with_content` | Log{to_file_path, event_fields} | BeforeStepStarts | ✅ file + content | — |
| `given_save_to_action_when_executed_then_file_contains_output` | SaveTo::FilePath | AfterStepSucceeds | ✅ file + content | — |
| `given_append_to_action_when_executed_then_content_appended_to_file` | AppendTo::FilePath | AfterStepSucceeds | ✅ append verified | — |
| `given_bookmark_action_when_executed_then_file_and_memory_stored` | Bookmark::Detailed | AfterStepSucceeds | ✅ file | ✅ bookmark |
| `given_fail_action_when_executed_then_hook_result_is_fail` | Fail{message} | BeforeStepStarts | — | — |
| `given_skip_step_action_when_executed_then_hook_result_is_skip_step` | SkipStep(true) | BeforeStepStarts | — | — |
| `given_route_to_action_when_executed_then_hook_result_routes_to_target` | RouteTo::Single | BeforeStepStarts | — | — |
| `given_gwt_matching_clause_when_executed_then_routes_to_then_target` | Gwt{1 clause} | BeforeStepStarts | — | — |
| `given_gwt_non_matching_clause_when_executed_then_continues` | Gwt{1 clause} | BeforeStepStarts | — | — |
| `given_gwt_comparison_expression_when_evaluated_then_correct` | GWT evaluate | — | — | — |
| `given_gwt_logical_and_when_evaluated_then_correct` | GWT evaluate | — | — | — |
| `given_gwt_dot_path_when_evaluated_then_navigates_nested` | GWT evaluate | — | — | — |
| `given_hook_results_merged_then_highest_priority_wins` | HookResult::merge | — | — | — |
| `given_invalid_gwt_expression_when_evaluated_then_error` | GWT evaluate | — | — | — |
| `given_missing_field_gwt_when_evaluated_then_false` | GWT evaluate | — | — | — |

**Integration test coverage: 47 tests total. 11 actions via `execute_action()`, now with 4 context types (BeforeStepStarts, AfterStepSucceeds, AfterStepFails, DuringStepStreaming).** 15 serde round-trip tests for all 12 HookAction variants. Critical regression test for LogAction swallowing bug.

**Missing integration tests:**
- Hook action parsed from YAML → deserialized → executed (serde round-trip from YAML file, not just JSON)
- Actions with AfterStepFails context (error_message extraction)
- Actions with DuringStepStreaming context (chunk_text extraction)

---

#### Level 3: End-to-End User Journeys (YAML → Runner → Execution → Output)

**E2E test = full `whitt benchmark --workflow <file>` against live Docker llama.cpp server.**

| Journey | YAML | Actions Verified | Status |
|---------|------|-----------------|--------|
| 2-step with `save_to` + `log` + template interpolation | `live-test-ministral-3b.yml` | save_to (file), log (to_file_path), log (stdout), template `{{step.*.output}}` | ✅ VERIFIED |
| Multi-model benchmark (3, 5, 15, 50 models) | `benchmark-*-models.yml` | log (stdout from after_step_succeeds), shell (free -h) | ⚠️ PARTIAL — log+shell added to YAMLs, not yet verified in live run |

**Missing E2E journeys:**
- ❌ `before_step_starts` with `skip_step: true` → verify step skipped
- ❌ `after_step_fails` with `fail` action → verify error propagation
- ❌ `after_step_fails` with GWT routing → verify conditional retry routing
- ❌ `append_to` accumulating results across multiple steps
- ❌ `bookmark: true` storing state → verify retrieval by subsequent step
- ❌ `route_to` with multiple targets → verify parallel execution
- ❌ `notify` action → verify notification sent (or logged)
- ❌ `during_step_streaming` → NOT EVEN WIRED IN RUNNER
- ❌ `after_all_retries_exhausted` → NOT EVEN WIRED IN RUNNER
- ❌ `before_gwt_evaluates` / `after_gwt_evaluates` → NOT EVEN WIRED IN RUNNER
- ❌ `on_requires_failed` → NOT EVEN WIRED IN RUNNER
- ❌ `after_loop_iteration_fails` → NOT EVEN WIRED IN RUNNER
- ❌ Multi-step workflow with depends_on + hooks at each step
- ❌ Hook action serde round-trip: YAML → serde_json → HookAction → execute_action (this was the bug we just fixed — no regression E2E test)

---

#### Level 4: Live System User Flow Testing (Full Docker stack)

| User Flow | Steps | Hook Triggers | Status |
|-----------|-------|---------------|--------|
| UF-LIVE-01: Single model, save_to + log | Load → Infer → Hook(save_to, log) → Unload | after_step_succeeds | ✅ VERIFIED |
| UF-LIVE-02: Two-step with interpolation | Step1(infer+save) → Interpolate → Step2(infer+save) | after_step_succeeds × 2 | ✅ VERIFIED |
| UF-LIVE-03: Skip step via before_step_starts | before_step_starts(skip_step:true) → verify step skipped | before_step_starts | ❌ NOT TESTED |
| UF-LIVE-04: Fail on error via after_step_fails | Bad model → after_step_fails(log+fail) → verify error | after_step_fails | ✅ VERIFIED (iter5b) |
| UF-LIVE-05: GWT conditional routing | Step with quality_score → GWT evaluates → route_to branch | after_step_succeeds (GWT) | ❌ NOT TESTED |
| UF-LIVE-06: Bookmark persistence | Step1(bookmark:true) → Step2 reads bookmark | after_step_succeeds (bookmark) | ✅ VERIFIED (iter5a, iter5c) |
| UF-LIVE-07: Append accumulation | 3 steps each append_to same file | after_step_succeeds × 3 | ❌ NOT TESTED |
| UF-LIVE-08: Multi-model benchmark | 3+ models, hook fires for each | after_step_succeeds × N | ⚠️ PARTIAL (log only) |
| UF-LIVE-09: Notify coordination | Step1(notify) → parent workflow receives | after_step_succeeds (notify) | ❌ NOT TESTED (notify_tx stub) |
| UF-LIVE-10: Streaming hooks | during_step_streaming fires per chunk | during_step_streaming | ❌ IMPOSSIBLE (not wired) |
| UF-LIVE-11: Retry exhaustion | Step fails 3× → after_all_retries_exhausted fires | after_all_retries_exhausted | ❌ IMPOSSIBLE (not wired) |
| UF-LIVE-12: Dependency chain failure | Step1 fails → on_requires_failed fires for Step2 | on_requires_failed | ❌ IMPOSSIBLE (not wired) |
| UF-LIVE-13: Multi-step with interpolation | 2-step (generate+summarize), {{step.X.output}} resolved | before_workflow, after_step_succeeds × 2, after_workflow | ✅ VERIFIED (iter5a) |
| UF-LIVE-14: Shell action chain | 2-step with shell echo between, bookmark+interpolation | after_step_succeeds (shell) | ✅ VERIFIED (iter5c) |
| UF-LIVE-15: Workflow boundary hooks | before_workflow + after_workflow fire at correct points | before_workflow, after_workflow | ✅ VERIFIED (iter5a, 5b, 5c) |

---

### Summary: What Remains Unverified

#### By Category

| Category | Total Items | Unit ✅ | Integration ✅ | E2E ✅ | Live ✅ |
|----------|-------------|---------|---------------|--------|---------|
| **Actions (12 types)** | 12 | 12 (all have some test) | 11 (IterateValues missing) | 5 (Log, SaveTo, Bookmark, Shell, Fail) | 5 (Log, SaveTo, Bookmark, Shell, Fail) |
| **Action variants (30+)** | ~35 | ~33 | ~16 | ~8 | ~7 |
| **Triggers fired in runner** | 10 | 7 fully + 2 partial | 4 | 4 | 4 |
| **Triggers defined but NOT wired** | 1 | 1 (during_step_streaming) | 0 | 0 | 0 |
| **Context structs** | 10 | 10 (to_json) | 4 | 4 | 4 |
| **HookResult variants** | 6 | 6 | 4 | 2 | 2 |
| **GWT expressions** | ~40 patterns | ~35 | 5 | 0 | 0 |

#### Critical Gaps (Must Fix Before Release)

1. **1/10 triggers not wired: `during_step_streaming`** — Requires SSE streaming path (stream:false is hardcoded in benchmark_single_model). Architectural change needed to access chunk-level hooks from LlamaHttpClient::chat_completion.

2. **2/10 triggers partially wired: `before_gwt_evaluates` and `after_gwt_evaluates`** — Info-level logging only at GWT evaluation points. Full wire requires passing hook_config through execute_action dispatch chain into execute_gwt.

3. **IterateValues is a passthrough** — `execute_action()` returns Continue without any logic. Future feature.

4. **No E2E regression test for HookAction serde from YAML** — 14 serde round-trip tests exist for JSON, but no test reads a YAML file → parses hooks → executes them. A YAML-specific deserialization regression would go undetected.

#### RESOLVED Gaps (Previously Critical, Now Fixed)

5. ~~**7/10 triggers not wired**~~ → **RESOLVED**: 7/10 fully wired, 2/10 partially wired (GWT logging), 1/10 blocked (streaming architecture). Commits: `d7cb8d6`, `74c73b0`.

6. ~~**Notify action is a stub**~~ → **RESOLVED**: `execute_notify()` now uses `tx.try_send()` when channel present. Commit: `d7d9251`.

7. ~~**AppendTo::Variable is a stub**~~ → **RESOLVED**: `execute_append_to()` for Variable variant now stores/concatenates in engine.bookmarks. Commit: `d7d9251`.

8. ~~**SaveTo::Both not tested**~~ → **RESOLVED**: Unit test + integration test added. Commits: `d7d9251`, `951ce2f`.

9. ~~**Bookmark::Path(string) not tested**~~ → **RESOLVED**: Unit test + integration test added. Commits: `d7d9251`, `951ce2f`.

10. ~~**Log level variants not tested**~~ → **RESOLVED**: Warning, Error, Critical, Debug all tested. Commit: `d7d9251`.

11. ~~**Multi-clause GWT not tested**~~ → **RESOLVED**: Unit test for first-match semantics. Commit: `d7d9251`.

#### Important Gaps (Should Fix)

12. **No multi-action trigger tested end-to-end** — Integration test for HookResult::merge exists, but no E2E test verifies 3+ actions firing on same trigger in runner.

13. **before_step_starts → skip_step not tested in E2E** — Unit test passes, but no E2E test verifies that a step is actually skipped by the runner when the hook returns SkipStep.

14. **after_step_fails → GWT conditional routing not tested in E2E** — Critical for retry logic.

15. **Template interpolation only tested via live system** — `{{step.step_1_generate.output}}` works in live test but has no unit test.

#### Nice-to-Have Gaps

16. **Benchmark YAMLs have limited hook configs** — Only `after_step_succeeds` log hooks. No YAML exercises save_to, bookmark, append_to, route_to, fail, skip_step, GWT in benchmark mode.

17. **No fixture for negative/invalid hook YAMLs** — `tests/fixtures/hooks/negative-invalid.yml` exists but no test reads it.

18. **No performance test for hook execution** — How many microseconds per action? No benchmark.

---

### File Reference Map

| File | Lines | What It Contains |
|------|-------|-----------------|
| `src/workflow/step.rs:174-200` | HookAction enum (12 variants) |
| `src/workflow/step.rs:202-323` | Action structs (LogAction, SaveToAction, ShellAction, etc.) |
| `src/workflow/hooks/context.rs` | 10 context structs + WorkflowHookContext enum |
| `src/workflow/hooks/actions.rs` | execute_action() dispatcher + all execute_* functions + unit tests |
| `src/workflow/hooks/mod.rs` | HookResult enum (6 variants) + HookEngine + merge logic |
| `src/workflow/hooks/gwt.rs` | GWT expression evaluator (lexer + parser + evaluator + 35 tests) |
| `src/benchmark/runner.rs:813-848` | execute_hooks_for_trigger() — the single firing point |
| `src/benchmark/runner.rs:1249-1380` | Trigger calls (before_step_starts, after_step_starts, after_step_fails, after_all_retries_exhausted, after_step_succeeds) |
| `src/benchmark/runner.rs:1535-1610` | on_requires_failed + after_loop_iteration_fails in iteration loop |
| `tests/hooks_integration.rs` | 46 integration tests (was 22) |
| `tests/fixtures/hooks/all-triggers.yml` | Fixture with all 13 trigger types × 10 actions |
| `tests/fixtures/hooks/bookmark-notify.yml` | Bookmark + notify fixture |
| `tests/fixtures/hooks/gwt-expressions.yml` | GWT expression fixture |
| `tests/fixtures/hooks/skip-actions.yml` | Skip step/remaining fixture |
| `tests/fixtures/hooks/negative-invalid.yml` | Invalid hook YAML fixture (unused) |

---

## Session Handoff Protocol

When context grows large, write handoff to `.opencode-handoff.md`:
- Objective (what was being accomplished)
- Completed (what's done and verified)
- In Progress (what was actively being worked on)
- Pending Todos (remaining work with priorities)
- Key Context (files modified, patterns followed, constraints)
- How to Continue (specific next steps)

---

## Active Objective: Meta-Workflow Parity (2026-06-21)

**Goal:** Meta-workflow-v6 generator + execution engine MUST produce workflows that accomplish prompt objectives at quality ≥ opencode baseline (single-shot with max tool usage).

**Plan suite:** `docs/plans/meta-workflow-parity/`
- `00-MASTER-PLAN.md` — overview, exit criteria
- `01-BASELINE-METHODOLOGY.md` — how baselines established
- `02-VALIDATION-CRITERIA.md` — strict pass/fail criteria
- `03-GAP-ANALYSIS.md` — brutally honest current state
- `04-ITERATION-STRATEGY.md` — cycle plan
- `05-RESOURCE-CONSTRAINTS.md` — hardware/safety limits
- `06-EVALUATION-FRAMEWORK.md` — critical evaluation
- `07-WEB-RESEARCH-LOG.md` — assumption testing log

**Known critical gap (2026-06-21):** Engine lacks tool access from workflow steps. Only 6 tools registered (`model_list`, `model_load`, `model_unload`, `chat`, `file_read`, `final_answer`); `file_read` only accessible to ReAct agent, not workflow steps. Schema declares `shell_exec`/`file_write`/`web_fetch`/`grep` but NEVER IMPLEMENTED in `src/agent/tools.rs`. Result: workflows ask model "Read src/foo.rs" → model outputs refusal text → output artifacts are garbage.

**Iteration cycle (max 3):**
1. **Cycle 1:** Template rewrite (shell hooks for file content injection)
2. **Cycle 2:** Engine `ShellTool` implementation (if Cycle 1 insufficient)
3. **Cycle 3:** Engine `FileWriteTool`/`GrepTool` implementation (last resort)

**Parity threshold:** 8/11 prompts achieve objective = acceptable. Escalate to user if <8/11 after Cycle 3.

**Operating principles for this objective:**
- **Caveman mode active** for all plan files + iteration logs (token efficiency)
- **Live system testing required** — every claim backed by actual workflow execution
- **Brutal honesty** — refusal text = HARD FAIL, not "partial pass"
- **No new dependencies without user approval**
- **Hardware safety** — Docker restart per prompt, RAM <80%, GPU <7GB
