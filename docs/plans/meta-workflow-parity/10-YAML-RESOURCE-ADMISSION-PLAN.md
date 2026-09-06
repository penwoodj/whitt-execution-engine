# YAML Resource Admission Implementation Plan

**Goal:** Make workflow-declared hardware requirements enforceable before model load or inference, record measured resource evidence, and require every new workflow YAML to carry an estimated then measured resource profile.

**Architecture:** Keep existing `models.*.load_params`, `min_allowed`, and `max_allowed` intact. Add a strict `workflow_execution_strategy.resource_admission` contract for required currently-available RAM, VRAM, and swap, plus per-model estimated KV, compute, host-runtime, and duration costs. Convert the YAML contract into one `resource_guard` request used by every model-load path; write observed sensor minima and outcome as a run artifact, never mutate source YAML at runtime.

**Safety:** Serial only. No subagents, parallel tools, background work, Docker, or LLM calls. Live execution waits for RAM >= 6GiB, one whitt process or fewer, healthy server, and clean crash-log scan. Engine/YAML tool actions remain enabled.

---

## Existing-State Finding

- `docs/schema/unified-workflow-schema.yml:75-114` declares model `load_params`, `max_allowed`, and `min_allowed`.
- `docs/schema/unified-workflow-schema.yml:538-556` declares workflow memory policy.
- `src/client/resource_guard.rs:343-364` admits loads using fixed Qwen assumptions: 30k context, 36 layers, 8 KV heads, 128 head dim, 512MiB compute, 1.5GiB VRAM reserve, 2.5GiB host floor.
- `src/benchmark/runner.rs:779-790` extracts YAML `load_params`, but `src/benchmark/runner.rs:4149-4178` does not consume YAML `min_allowed`, `max_allowed`, or workflow memory declarations during admission.
- Therefore: YAML resource declarations exist but do not currently enforce admission. This plan closes that gap.

## Contract

Add this strict YAML section under `workflow_execution_strategy`:

```yaml
resource_admission:                         # schema line added with implementation
  enforcement_policy: block                 # block only in current POC
  minimum_available:
    ram: 6GiB
    vram: 6GiB
    swap_free: 4GiB
  model_estimate:
    kv_cache: 2.1GiB
    compute_buffer: 512MiB
    host_runtime: 700MiB
    expected_runtime_secs: 900
  telemetry:
    write_profile: true
```

- Model weights are measured from the selected GGUF file; YAML cannot understate them.
- `model_estimate` is authored before a run, initially estimated by agent from model/load parameters and hardware profile.
- `minimum_available` is checked against current `MemAvailable`, VRAM-free, and `SwapFree` before every model load and before every inference.
- `block` returns greppable `RESOURCE_REJECT_WORKFLOW_*` errors and makes zero LLM calls.
- Telemetry writes a run-local JSON profile: declared estimate, actual model bytes, observed RAM/VRAM/swap minima, load/inference outcome, duration. Agent updates YAML's estimate only after reviewing this artifact.
- Existing `min_allowed` / `max_allowed` remain accepted; agent configuration requires them to agree with this new, unambiguous runtime-admission contract until future schema consolidation.

## File Map

- Modify: `docs/schema/unified-workflow-schema.yml` — documented contract, line references.
- Modify: `src/workflow/execution.rs` — strict deserialization types for `resource_admission`.
- Modify: `src/workflow/schema.rs` — semantic validation: block policy, required positive limits, valid resource units, estimate completeness.
- Modify: `src/model/schema.rs` — exact resource quantity parser supporting decimal GB/GiB/MB/MiB/KB/KiB; retain compatibility helper only where needed.
- Modify: `src/client/resource_guard.rs` — pure YAML-aware admission request, deterministic decision/reason codes, telemetry data types.
- Modify: `src/benchmark/runner.rs` — YAML extraction, pre-load and pre-inference enforcement on all active load paths, bounded telemetry artifact.
- Modify: `src/workflow/tests.rs`, `src/model/schema.rs` tests, `src/client/resource_guard.rs` tests, `src/benchmark/runner.rs` tests — TDD coverage.
- Create: `docs/benchmarks/workflows/resource-admission-simple.yml` — one inference + bounded shell/log hook.
- Create: `docs/benchmarks/workflows/resource-admission-long.yml` — serial multi-step workflow + bounded resource logging hook.
- Create: `docs/benchmarks/workflows/resource-admission-reject.yml` — impossible requirement; negative live proof with zero inference.
- Modify: `AGENTS.md`, `.opencode/agent/serial-safety.md` — workflow authors must estimate requirements before YAML creation, review profile after run, and record measured values.
- Modify: `docs/plans/meta-workflow-parity/STATUS.md` or current tracking file — evidence links after live validation.

## Task 1: Schema Test First

- [ ] Add a failing `src/workflow/tests.rs` case parsing `resource_admission` from YAML and asserting all fields.
- [ ] Run exact test; expect missing `resource_admission` field compile/deserialization failure.
- [ ] Add `ResourceAdmissionConfig`, `AvailableResourceMinimums`, `ModelResourceEstimate`, and `ResourceTelemetryConfig` in `src/workflow/execution.rs`; all structs use `#[serde(deny_unknown_fields)]`.
- [ ] Add workflow semantic validation: `enforcement_policy == block`; RAM/VRAM/swap positive; estimate fields positive; `expected_runtime_secs > 0`.
- [ ] Re-run exact test; expect pass.
- [ ] Add failing unknown-key and invalid-unit tests; add minimal validation; re-run passing.

## Task 2: Exact Quantity Parsing Test First

- [ ] Add failing `src/model/schema.rs` tests for `3.7GB`, `6GiB`, `512MiB`, `0.5GB`, invalid unit, and non-positive value.
- [ ] Run targeted model-schema tests; expect decimal/IEC parsing failure before implementation.
- [ ] Implement one exact byte parser returning a typed nonzero byte quantity; avoid floating-point persisted decisions by rounding once at parse boundary.
- [ ] Route existing `ResourceLimit` helpers through compatible parser behavior without silently accepting malformed limits.
- [ ] Re-run targeted tests; expect pass.

## Task 3: YAML-Aware Admission Test First

- [ ] Add failing pure `resource_guard` tests for: declared RAM rejection despite fixed floor passing; declared VRAM rejection; swap-free rejection; model weights + declared KV/compute/reserve rejection; healthy admission.
- [ ] Run only these new tests; expect missing request/config API failure.
- [ ] Add `WorkflowAdmissionRequest`, `WorkflowAdmission`, greppable `RESOURCE_REJECT_WORKFLOW_RAM`, `..._VRAM`, `..._SWAP`, and `..._MODEL_FIT` codes.
- [ ] Derive actual model weights from stat result; use declared estimated KV/compute/host runtime and declared availability minima; preserve current fixed guard as a separate hard lower bound.
- [ ] Re-run targeted guard tests; expect pass.

## Task 4: Runner Wiring Test First

- [ ] Add failing `src/benchmark/runner.rs` test: YAML `resource_admission` is extracted into `BenchmarkWorkflowConfig`.
- [ ] Add failing test: impossible YAML availability requirement produces a resource rejection before `client.load_model` / inference.
- [ ] Run those exact tests; expect failure because runner has no resource contract field.
- [ ] Extend `BenchmarkWorkflowConfig` and `load_workflow_config()` to deserialize typed contract via `WorkflowFile`, not ad-hoc raw JSON field access.
- [ ] Insert one shared admission function called before every active model load and every inference entry point; route duplicate/legacy path to shared logic so no bypass exists.
- [ ] Record telemetry with bounded one-sample-per-gate data into workflow output directory.
- [ ] Re-run exact runner tests; expect pass.

## Task 5: Workflow Fixtures and Agent Policy

- [ ] Add schema-line comments to all new YAML fields.
- [ ] Add `resource-admission-reject.yml` with impossible VRAM/RAM requirement and bounded output paths.
- [ ] Add `resource-admission-simple.yml` with measured initial estimate, one serial inference, and known bounded shell/log hooks.
- [ ] Add `resource-admission-long.yml` with measured initial estimate, serial multi-step inference, bounded resource logging, no parallelism, one model.
- [ ] Validate each YAML using `python3 scripts/meta-v6/validate-workflow.py` and `WorkflowFile::from_file` test.
- [ ] Update `AGENTS.md` and `serial-safety.md`: new workflow author must calculate declared availability + model estimates before writing YAML; after a live run, inspect profile and update YAML estimates with measured values. Never remove YAML engine tools or script actions.

## Task 6: Verification Before Live Work

- [ ] Run targeted test modules after each green step; then `cargo test --lib`.
- [ ] Run `cargo build --release --all-features`.
- [ ] Run `cargo clippy --all-features -- -W clippy::all`.
- [ ] Run diagnostics on changed Rust/YAML/config files.
- [ ] Re-read all changed files and run `git diff --check` only on touched files.

## Task 7: Serial Live Evidence

- [ ] Wait for host gate: >=6GiB available RAM; <=1 whitt; `curl /health` works; last 200 kernel lines and bounded Docker log excerpt contain no OOM/DeviceLost/Vulkan error.
- [ ] Run negative fixture first; verify resource rejection code, telemetry artifact, and zero inference markers in bounded whitt/server logs.
- [ ] Run simple fixture once; verify admission event, shell/log hook output, real inference marker, telemetry artifact, and updated measured YAML profile.
- [ ] Re-check RAM/whitt/server health.
- [ ] Run long fixture once; verify serial behavior, resource checks before every load/inference, script/hook outputs, telemetry peak values, and updated measured YAML profile.
- [ ] Critically compare estimates vs measured profile; correct YAML values and rerun both positive fixtures if values change.

## Task 8: Resume Experiment Only After Resource Gate Passes

- [ ] Resolve batch 7: rerun only three path-drift cases and `px-ana-a-f4` after admission validation; regenerate reports.
- [ ] Complete batch 7 Gates B/C/D.
- [ ] Run batches 8-10 serially with resource-admission telemetry; triage each batch, rerun stragglers only, complete gates.
- [ ] Run final regression; update evidence tracking; report verified count and any blockers without weakening oracle or admission checks.

## Completion Evidence

- New tests were observed RED before implementation and GREEN after.
- Engine refuses underspecified/unavailable YAML requirements before model loading and inference.
- Positive simple and long workflows prove real scripts/hooks remain active under engine admission.
- Measured resource artifacts exist under `docs/benchmarks/outputs/`, and fixture YAML estimates match reviewed measurements.
- 100-case experiment resumes only after resource validation passes.
