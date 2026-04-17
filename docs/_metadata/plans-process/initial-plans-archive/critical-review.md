# Critical Review: All Plan Files

**Date**: 2026-04-02
**Reviewer**: OpenCode Critical Review
**Scope**: plan-00 through plan-07

---

## Executive Summary

This critical review evaluates all 8 implementation plan files for the Whitt Execution Engine transpiler project. The plans are well-structured with consistent formatting, clear phase breakdowns, and checkpoint systems. However, all plans share significant systemic gaps that will impede OpenCode single-plan execution success.

**Key Findings:**

1. **Research-Implementation Disconnect**: All plans list "Web Research" sections marked "Not Started" with no specification of WHAT findings would change implementation. Even plans referencing "COMPLETED" research plans (plan-00, plan-02, plan-04) still show Web Research as "Not Started", indicating a disconnect between research-plan artifacts and plan files.

2. **Missing OpenCode Tool Specifications**: No plan specifies which OpenCode tools to use (lsp_diagnostics, glob, grep, ast_grep, etc.). Verification commands are generic `cargo test` invocations without OpenCode-specific integration.

3. **Insufficient Dependency Specifications**: Plans list upstream dependencies but lack explicit integration points—no file paths, no struct signatures, no API contracts. An agent receiving only one plan would be unable to implement without reading other plans.

4. **Absent Mock/Stub Strategies**: All plans call external services (LLM backends, web APIs, databases, git operations) but specify no mock strategies for testing in isolation.

**Overall Verdict**: Plans are well-organized scaffolds but require significant enhancement before OpenCode can execute them as self-contained units. Rating: **NEEDS-WORK** across all plans.

---

## Cross-Plan Findings

### Systemic Issues Affecting ALL Plans

| Issue | Severity | Description |
|-------|----------|-------------|
| Web Research Placeholder | HIGH | All 8 plans have 5 "Not Started" research areas. No integration guidance provided. |
| OpenCode Tool Gaps | HIGH | No plans specify lsp_diagnostics, glob, grep, ast_grep, or other OpenCode tools. |
| Mock Strategy Absence | HIGH | External dependencies (LLMs, APIs, databases, git) lack mock/stub specifications. |
| Self-Contained Executability | MEDIUM | All plans require reading other plans for struct definitions and API contracts. |
| Agentic Workflow Alignment | MEDIUM | No plan references L1-L5 layer model or step types from agentic workflow YAMLs. |
| implement.sh Assumptions | MEDIUM | Plans reference implement.sh but its existence and checkpoint tracking is unclear. |

---

## Plan-00: Foundation Compiler Contract

### Upstream Factor Analysis

| # | Factor | Rating | Finding | Action Required |
|---|--------|--------|---------|-----------------|
| 1 | OpenCode Tool Integration | NEEDS-WORK | Lines 680-690 show generic `cargo test` commands. No mention of lsp_diagnostics for type checking during implementation, glob for file discovery, or grep for pattern search. | Add OpenCode tool usage for each phase: lsp_diagnostics for schema validation, glob for workflow discovery, read/write for code generation. |
| 2 | Upstream Dependency Completeness | PASS | Plan has no upstream dependencies (foundation phase). Correctly identifies it "Blocks: None" (lines 519-521). | None - this is correct. |
| 3 | Verification Layer Executability | NEEDS-WORK | Lines 261-354 define 4 verification layers but use generic `cargo test --lib` commands. No mock strategies for schemars validation, no stub for serde-saphyr parsing edge cases. | Add mock YAML fixtures, stub validation errors, and proptest generators for schema validation. |
| 4 | Research-to-Implementation Gap | FAIL | Lines 87-161 list 4 "Not Started" research areas despite research-plan-01-foundation.yml being marked "COMPLETED" (line 73). No mapping from research findings to implementation decisions. | Either: (a) Synthesize research findings into plan, or (b) Remove "Not Started" sections and replace with "Research Completed - See research-plan-01-foundation.yml". |
| 5 | Self-Contained Executability | NEEDS-WORK | Defines WorkflowSpec and WorkflowIR (lines 25-32) but doesn't include struct signatures or field definitions. Agent would need to infer from requirements. | Add inline struct definitions with field types and serde/schemars derives. |
| 6 | Checkpoint Granularity | PASS | Checkpoints 001-005 (lines 359-465) are appropriately scoped. Each covers 1-2 phases with clear sign-off criteria. | None. |
| 7 | implement.sh Integration | NEEDS-WORK | Lines 654-674 reference implement.sh commands (--start, --continue, --verify, --complete, --status, --review) but plan doesn't verify script exists or document checkpoint ID format. | Add checkpoint ID format (e.g., "plan-00-cp001") and verify implement.sh exists in Execution Commands section. |

### Summary
**Verdict**: NEEDS-WORK. Foundation plan is well-structured but has critical research-implementation disconnect. Web Research sections marked "Not Started" despite completed research plan undermine plan credibility.

---

## Plan-01: MVP Queue & Scheduler

### Upstream Factor Analysis

| # | Factor | Rating | Finding | Action Required |
|---|--------|--------|---------|-----------------|
| 1 | OpenCode Tool Integration | NEEDS-WORK | Lines 470-475 show implement.sh commands but no OpenCode tool specifications. Queue testing would benefit from glob for job discovery, bash for process management. | Add OpenCode tool usage: bash for queue CLI testing, glob for job file discovery, read/write for job artifact inspection. |
| 2 | Upstream Dependency Completeness | NEEDS-WORK | Line 10 lists "Dependencies: plan-00 (Foundation)" but doesn't specify WHAT from plan-00. Needs WorkflowSpec, WorkflowIR, and `.opencode/` storage paths. | Add explicit integration section: "From plan-00 required: WorkflowSpec (src/spec.rs), WorkflowIR (src/ir.rs), storage layer (src/storage.rs). |
| 3 | Verification Layer Executability | NEEDS-WORK | Lines 226-280 define verification layers but no mock strategies for: queue state persistence, ChatSession lifecycle, scheduler retry logic. SQLite storage needs mock or in-memory mode. | Add mock storage layer (in-memory SQLite), stub ChatSession for testing, mock scheduler clock for time-based tests. |
| 4 | Research-to-Implementation Gap | FAIL | Lines 47-111 list 5 "Not Started" research areas with no integration guidance. Questions like "What state machine patterns best model queue lifecycle?" are unanswered. | Complete research or provide decision criteria: "If research finds X, implement Y. If research finds Z, implement W." |
| 5 | Agentic Workflow Substructure Alignment | NEEDS-WORK | Plan implements "Chat as Work Container" (lines 36-43) but doesn't map to agentic workflow L1-L5 layer model. No reference to step types or composite patterns. | Add section mapping ChatSession to agentic workflow layers: L1 (step execution), L2 (queue management), etc. |
| 6 | Checkpoint Granularity | PASS | Checkpoints 001-007 (lines 285-375) are well-scoped. Each covers one phase with clear criteria. | None. |
| 7 | implement.sh Integration | NEEDS-WORK | Lines 470-475 show implement.sh commands but no checkpoint ID mapping. Plan has 7 checkpoints but no IDs specified. | Add checkpoint IDs (plan-01-cp001 through plan-01-cp007) and map to phases. |
| 8 | Self-Contained Executability | FAIL | References ChatSession struct, Queue state machine, and scheduler without definitions. Line 119-127 list tasks but no struct signatures. | Add inline struct definitions: ChatSession { id, title, status, created_at, ... }, QueueState enum, etc. |

### Summary
**Verdict**: NEEDS-WORK. Queue plan has good structure but lacks upstream dependency specifics. ChatSession and Queue state machine need inline definitions for self-contained execution.

---

## Plan-02: CLI, Backends, and Networking Boundary

### Upstream Factor Analysis

| # | Factor | Rating | Finding | Action Required |
|---|--------|--------|---------|-----------------|
| 1 | OpenCode Tool Integration | NEEDS-WORK | Lines 700-753 show cargo test and CLI commands but no OpenCode tool usage. Plan would benefit from: lsp_diagnostics for generated code validation, bash for CLI testing, grep for output parsing. | Add OpenCode tool specifications per phase: bash for CLI command testing, lsp_diagnostics for generated Rust code validation, glob for template discovery. |
| 2 | Upstream Dependency Completeness | NEEDS-WORK | Line 10 lists "plan-00 (Foundation), plan-01 (MVP Queue)" but doesn't specify integration points. Needs: WorkflowSpec for CLI run command, Queue APIs for CLI status command, storage paths. | Add explicit dependencies: "From plan-00: WorkflowSpec (src/spec.rs), compile_workflow_spec() (src/compiler.rs). From plan-01: Queue APIs (src/queue/api.rs)." |
| 3 | Verification Layer Executability | NEEDS-WORK | Lines 367-462 define 4 layers but no mock strategies for: external backends (llama.cpp, LM Studio, vLLM, Ollama), networking policy enforcement, provider trait mocking. | Add mock ModelProvider trait implementations, stub HTTP responses for external APIs, mock networking policy layer. |
| 4 | Research-to-Implementation Gap | FAIL | Lines 105-218 list 5 "Not Started" research areas despite research-plan-03 being marked "COMPLETED" (line 90). No findings synthesis. | Synthesize research findings into implementation decisions OR remove placeholder research sections. |
| 5 | Self-Contained Executability | NEEDS-WORK | Defines ModelProvider trait conceptually (lines 246-257) but no method signatures. ProviderCapabilities struct mentioned but not defined. | Add inline trait definition: `trait ModelProvider { fn load(&self, model: &str) -> Result<()>; fn infer(&self, prompt: &str) -> Result<String>; ... }` |
| 6 | Checkpoint Granularity | PASS | Checkpoints 1-7 (lines 467-575) are appropriately scoped with target dates and verification layers. | None. |
| 7 | implement.sh Integration | NEEDS-WORK | Lines 740-753 show implement.sh usage but no checkpoint ID format. Plan references "cp3" as example but doesn't define format. | Define checkpoint ID format: plan-02-cp01 through plan-02-cp07. |
| 8 | Mock/Stub for External Dependencies | FAIL | Plan requires connecting to llama.cpp, LM Studio, vLLM, Ollama but specifies no mock strategies. Tests would fail without running servers. | Add mock server configurations: MockLlamaCpp, MockLM Studio, MockVLLM with stub responses. |

### Summary
**Verdict**: NEEDS-WORK. CLI/Backends plan is comprehensive but critically lacks mock strategies for external LLM backends. Tests cannot run without running llama.cpp/LM Studio servers.

---

## Plan-03: Glyphnova Desktop UI and Multi-Zoom Control Plane

### Upstream Factor Analysis

| # | Factor | Rating | Finding | Action Required |
|---|--------|--------|---------|-----------------|
| 1 | OpenCode Tool Integration | NEEDS-WORK | Lines 729-784 show test and build commands but no OpenCode tools. Frontend testing would benefit from webapp-testing skill, dev-browser for UI automation. | Add OpenCode tool usage: webapp-testing for Playwright E2E tests, dev-browser for UI automation, bash for build commands. |
| 2 | Upstream Dependency Completeness | NEEDS-WORK | Lines 655-672 list dependencies but not integration points. Needs: Queue state from plan-01, Backend API from plan-02, WorkflowIR from plan-00. | Add explicit integration: "From plan-01: QueueState (src/queue/state.rs), queue_event_stream(). From plan-02: BackendAPI (src/api/mod.rs)." |
| 3 | Verification Layer Executability | NEEDS-WORK | Lines 374-468 define 4 layers with both Rust (cargo) and JavaScript (Jest/Vitest/Playwright) tests. But no mock strategies for: WebSocket connections, scheduler state, drag-and-drop events. | Add mock WebSocket server, mock scheduler state provider, mock drag-and-drop event fixtures. |
| 4 | Research-to-Implementation Gap | FAIL | Lines 102-211 list 5 "Not Started" research areas. Desktop architecture, real-time updates, drag-and-drop, graph visualization, cross-platform deployment all need research. | Complete research OR provide decision frameworks: "If Tauri supports X, use Y. If not, consider Z." |
| 5 | Self-Contained Executability | NEEDS-WORK | Tauri project structure assumed but not defined. No sample tauri.conf.json, no React component structure, no TypeScript interfaces. | Add inline project structure: `src-tauri/`, `src/` (React), sample tauri.conf.json, TypeScript interface definitions. |
| 6 | Checkpoint Granularity | NEEDS-WORK | Checkpoints 1-8 (lines 473-598) are reasonable but Checkpoint 8 "Cross-Platform Deployment" (week 10) may be too large for single OpenCode session. | Split Checkpoint 8 into: 8a (macOS build), 8b (Windows build), 8c (Linux build). |
| 7 | Cross-Platform Testing | FAIL | Plan mentions macOS, Windows, Linux testing but provides no CI/CD strategy. OpenCode cannot test Windows/Linux from Linux environment. | Add conditional testing: macOS tests run locally, Windows/Linux tests require CI or mock validation. |
| 8 | Mock for UI Testing | NEEDS-WORK | Playwright E2E tests require running desktop app but no mock backend or stub Tauri APIs. | Add mock Tauri backend, stub IPC calls, mock file system operations. |

### Summary
**Verdict**: NEEDS-WORK. UI plan has significant complexity (Tauri + React + WebSocket + cross-platform) without sufficient mock strategies. Cross-platform testing cannot be fully automated from single platform.

---

## Plan-04: Quality Loops, Artifact Workflows, and Benchmark-Driven File-Type Expansion

### Upstream Factor Analysis

| # | Factor | Rating | Finding | Action Required |
|---|--------|--------|---------|-----------------|
| 1 | OpenCode Tool Integration | NEEDS-WORK | Lines 689-745 show test commands but no OpenCode tool usage. Quality loops would benefit from: lsp_diagnostics for code verification, bash for running external linters (eslint, mypy, shellcheck). | Add OpenCode tool usage: lsp_diagnostics for code quality checks, bash for external verifier execution, grep for error pattern detection. |
| 2 | Upstream Dependency Completeness | NEEDS-WORK | Line 10 lists 4 dependencies but no integration points. Needs: WorkflowIR for repair loops, storage layer for benchmarks, CLI for quality commands, UI for dashboard. | Add explicit integration: "From plan-00: WorkflowIR (src/ir.rs), storage (src/storage.rs). From plan-02: CLI framework (src/cli/mod.rs)." |
| 3 | Verification Layer Executability | NEEDS-WORK | Lines 351-441 define 4 layers but no mock strategies for: Verifier implementations (code, docs, config), repair loop LLM calls, benchmark execution harness. | Add mock Verifier trait implementations, mock LLM for repair suggestions, mock benchmark storage. |
| 4 | Research-to-Implementation Gap | FAIL | Lines 93-203 list 5 "Not Started" research areas despite research-plan-04 being "COMPLETED" (line 82). No findings on verification interfaces, benchmark design, repair strategies. | Synthesize research findings: "Use X pattern for verification, Y strategy for repair loops, Z format for benchmarks." |
| 5 | Self-Contained Executability | NEEDS-WORK | Defines Verifier trait concept (lines 210-227) but no method signatures. VerificationResult, VerifierCapabilities mentioned but not defined. | Add inline definitions: `trait Verifier { fn verify(&self, artifact: &Artifact) -> Result<VerificationResult>; }`, struct VerificationResult { success, failures, metrics }. |
| 6 | Checkpoint Granularity | PASS | Checkpoints 1-7 (lines 446-560) are well-scoped with clear sign-off criteria. | None. |
| 7 | External Verifier Integration | NEEDS-WORK | Plan mentions eslint, mypy, shellcheck (lines 100-106) but doesn't specify how to integrate these external tools in Rust tests. | Add bash command wrappers for external verifiers, error output parsing strategies. |
| 8 | Mock for LLM-Driven Repair | FAIL | Generate-verify-repair loops (lines 230-248) require LLM calls for repair suggestions but no mock strategy. Tests would fail without running LLM backend. | Add mock repair LLM that returns predetermined fixes based on error patterns. |

### Summary
**Verdict**: NEEDS-WORK. Quality loops plan has strong conceptual foundation but critically lacks mock strategies for repair loops. LLM-driven repair cannot be tested without running backend.

---

## Plan-05: Local Memory Retrieval, Web Search, and Web Scraping

### Upstream Factor Analysis

| # | Factor | Rating | Finding | Action Required |
|---|--------|--------|---------|-----------------|
| 1 | OpenCode Tool Integration | NEEDS-WORK | Lines 702-766 show commands but no OpenCode tool usage. Memory/search would benefit from: grep for artifact search, webfetch for scraping tests, glob for memory file discovery. | Add OpenCode tool usage: grep for memory search testing, webfetch for scraping validation, glob for artifact discovery. |
| 2 | Upstream Dependency Completeness | NEEDS-WORK | Line 10 lists 5 dependencies but no integration points. Needs: `.opencode/` storage from plan-00, CLI framework from plan-02, UI browser from plan-03, quality metrics from plan-04. | Add explicit integration: "From plan-00: .opencode/ structure, storage layer. From plan-02: CLI commands framework." |
| 3 | Verification Layer Executability | NEEDS-WORK | Lines 353-447 define 4 layers but no mock strategies for: search indexing (Tantivy), external search APIs (DuckDuckGo, Brave), web scraping with robots.txt, provenance tracking. | Add mock search index, mock external search APIs (HTTP stubs), mock robots.txt server, mock provenance logger. |
| 4 | Research-to-Implementation Gap | FAIL | Lines 114-224 list 5 "Not Started" research areas. Critical decisions on storage patterns, search strategies, scraping compliance, external APIs, and provenance are unguided. | Provide decision frameworks: "For storage, if SQLite supports X, use it. If not, consider RocksDB." |
| 5 | Self-Contained Executability | NEEDS-WORK | Memory storage schema mentioned (line 234) but not defined. No sample memory artifact structure, no search query syntax. | Add inline definitions: MemorySchema { structured: SQLite, unstructured: Tantivy }, sample memory artifact JSON. |
| 6 | External Service Mocking | FAIL | Plan requires DuckDuckGo API, Brave API, and web scraping (lines 272-288) but no mock strategies. Tests cannot run without network access and API compliance. | Add mock HTTP server with stub search responses, mock robots.txt server, offline scraping test fixtures. |
| 7 | Checkpoint Granularity | PASS | Checkpoints 1-7 (lines 451-571) are well-scoped. | None. |
| 8 | Compliance Testing | NEEDS-WORK | Robots.txt compliance (lines 162-179) is mentioned but no test fixtures for robots.txt parsing, no rate limiting verification. | Add sample robots.txt fixtures, rate limiting test scenarios, compliance assertion framework. |

### Summary
**Verdict**: NEEDS-WORK. Memory/search plan is technically ambitious but critically lacks mock strategies for external services. DuckDuckGo/Brave APIs and web scraping cannot be tested without network access.

---

## Plan-06: Cron Execution, Git-Branch Experimentation, and Refining Workflows

### Upstream Factor Analysis

| # | Factor | Rating | Finding | Action Required |
|---|--------|--------|---------|-----------------|
| 1 | OpenCode Tool Integration | NEEDS-WORK | Lines 736-813 show commands but no OpenCode tool usage. Git automation would benefit from: bash for git commands, glob for experiment discovery, lsp_diagnostics for generated workflow validation. | Add OpenCode tool usage: bash for git operations, glob for experiment branch discovery, read/write for merge proposal files. |
| 2 | Upstream Dependency Completeness | NEEDS-WORK | Line 10 lists 6 dependencies but no integration points. Needs: WorkflowIR for scheduling nodes, queue for cron execution, CLI for schedule commands, git integration patterns. | Add explicit integration: "From plan-00: WorkflowIR scheduling nodes. From plan-01: Queue execution. From plan-02: CLI schedule commands." |
| 3 | Verification Layer Executability | NEEDS-WORK | Lines 376-475 define 4 layers but no mock strategies for: cron scheduler (time-based), git operations (branching, worktrees), merge validation, rollback procedures. | Add mock clock for cron testing, mock git repository (in-memory), mock merge validator, mock rollback state. |
| 4 | Research-to-Implementation Gap | FAIL | Lines 116-230 list 5 "Not Started" research areas. Cron patterns, git experimentation frameworks, merge policies, refinement capture, and result comparison need guidance. | Provide decision frameworks: "For cron, use tokio-cron-scheduler. For git experiments, prefer worktrees over branches for isolation." |
| 5 | Self-Contained Executability | NEEDS-WORK | Experiment manifest structure mentioned (line 259) but not defined. No sample cron expression validation, no merge policy schema. | Add inline definitions: ExperimentManifest { workflow_version, branch, policy, created_at }, MergePolicy enum, sample cron expressions. |
| 6 | Git Testing Isolation | NEEDS-WORK | Plan requires git branch creation and worktree management (lines 256-272) but tests would pollute actual git repository. No isolated git testing strategy. | Add temp git repository fixtures, git sandbox for tests, cleanup procedures for test branches. |
| 7 | Checkpoint Granularity | PASS | Checkpoints 1-7 (lines 480-604) are well-scoped with clear criteria. | None. |
| 8 | Scheduling Policy Compilation | NEEDS-WORK | Phase 7 (lines 356-372) mentions compiling scheduling policies into WorkflowIR but doesn't specify IR node structure or compilation algorithm. | Add WorkflowIR scheduling node definition: SchedulingNode { cron_expr, resource_limits, failure_policy }. |

### Summary
**Verdict**: NEEDS-WORK. Automation plan requires git repository manipulation without isolated testing strategy. Cron scheduling and git experiments need mock environments.

---

## Plan-07: Autonomous Loops and Metrics-Driven FE and BE UX Expansion

### Upstream Factor Analysis

| # | Factor | Rating | Finding | Action Required |
|---|--------|--------|---------|-----------------|
| 1 | OpenCode Tool Integration | NEEDS-WORK | Lines 840-923 show commands but no OpenCode tool usage. Autonomy testing would benefit from: bash for override commands, read/write for checkpoint files, glob for metric discovery. | Add OpenCode tool usage: bash for autonomy CLI testing, read/write for checkpoint manipulation, glob for metric file discovery. |
| 2 | Upstream Dependency Completeness | NEEDS-WORK | Line 10 lists 7 dependencies but no integration points. This is the final plan and integrates ALL previous systems: WorkflowIR, Queue, CLI, UI, Quality, Memory, Automation. | Add comprehensive integration section mapping each upstream plan to specific modules and APIs. |
| 3 | Verification Layer Executability | NEEDS-WORK | Lines 430-532 define 4 layers but no mock strategies for: autonomous loop execution, metrics collection, human override, intervention logging, stop conditions, checkpoint restoration, risk assessment, confidence thresholds. | Add mock autonomous executor, mock metrics collector, mock override handler, mock checkpoint store, mock risk assessor. |
| 4 | Research-to-Implementation Gap | FAIL | Lines 120-235 list 5 "Not Started" research areas. Autonomous loop contracts, metrics frameworks, human override patterns, dashboards, and confidence thresholds are unguided. | Provide decision frameworks: "For autonomy, use bounded loops with max iterations. For metrics, use OpenTelemetry. For dashboards, use Grafana." |
| 5 | Self-Contained Executability | NEEDS-WORK | Autonomous loop contract mentioned (lines 244-257) but not defined. No sample contract structure, no stop condition schema, no risk assessment model. | Add inline definitions: AutonomousContract { goals, checkpoints, stop_conditions, max_duration }, StopCondition enum, RiskAssessment struct. |
| 6 | Checkpoint Granularity | NEEDS-WORK | Checkpoints 1-9 (lines 537-703) are reasonable but Plan has 9 phases (most complex). Checkpoint 9 "Full Autonomy System Integration" may be too large. | Split Checkpoint 9 into: 9a (core autonomy), 9b (metrics/dashboards), 9c (full integration test). |
| 7 | Human Override Testing | NEEDS-WORK | Human override controls (lines 287-304) require real-time intervention but no mock strategy for testing override during autonomous execution. | Add mock autonomous workflow that can be paused/stopped at predetermined points for override testing. |
| 8 | Metrics Mocking | FAIL | Metrics collection (lines 261-282) requires running autonomous workflows but no mock metrics strategy. Tests cannot verify metrics without actual execution. | Add mock metrics collector that generates synthetic metrics for testing dashboards and analysis. |
| 9 | Confidence Threshold Validation | NEEDS-WORK | Phase 9 (lines 409-426) mentions confidence thresholds but no validation strategy. How to test that thresholds prevent unsafe decisions? | Add test scenarios: high-risk actions blocked, low-risk actions allowed, threshold tuning verified. |

### Summary
**Verdict**: NEEDS-WORK. Autonomy plan is the most complex and integrates all previous plans. Critically lacks mock strategies for autonomous execution and metrics collection. Cannot be tested without full system running.

---

## Consolidated Action Items

| Priority | Plan | Factor | Action |
|----------|------|--------|--------|
| P0 | ALL | Research-to-Implementation Gap | Either complete Web Research sections OR replace with "Research Completed - See research-plan-XX.yml" and synthesize key findings inline. |
| P0 | ALL | Mock/Stub Strategies | Add mock strategies for ALL external dependencies: LLM backends, web APIs, databases, git operations, file systems. |
| P0 | 02, 04, 05, 07 | External Service Mocking | Plans 02 (LLM backends), 04 (repair LLM), 05 (search APIs, scraping), 07 (autonomous execution) cannot be tested without mock services. Critical blocker. |
| P1 | ALL | OpenCode Tool Integration | Add OpenCode tool specifications to each phase: lsp_diagnostics for type checking, glob for file discovery, grep for pattern search, bash for command execution. |
| P1 | ALL | Self-Contained Executability | Add inline struct definitions, trait signatures, and API contracts. Agent should not need to read other plans. |
| P1 | 01-07 | Upstream Dependency Completeness | Add explicit integration sections with file paths: "From plan-XX required: StructName (path/to/file.rs), function_name (path/to/file.rs)." |
| P2 | 03 | Cross-Platform Testing | Add conditional testing strategy: macOS tests run locally, Windows/Linux require CI or mock validation. |
| P2 | 06 | Git Testing Isolation | Add temp git repository fixtures and git sandbox for isolated testing. |
| P2 | ALL | implement.sh Integration | Verify implement.sh exists, document checkpoint ID format, add checkpoint ID mapping to phases. |
| P3 | ALL | Agentic Workflow Alignment | Map plan components to L1-L5 layer model from agentic workflow YAMLs. |
| P3 | 03, 07 | Checkpoint Granularity | Split overly large checkpoints (03-cp8, 07-cp9) into smaller units. |

---

## Methodology Notes

This review evaluated each plan against 8 upstream factors for OpenCode single-plan execution success:

1. **OpenCode Tool Integration**: Whether plans specify lsp_diagnostics, bash, read/write/edit, glob, grep, ast_grep usage.
2. **Upstream Dependency Completeness**: Explicit file paths and API contracts for dependencies.
3. **Verification Layer Executability**: Mock/stub strategies for external dependencies.
4. **Research-to-Implementation Gap**: Whether research findings guide implementation decisions.
5. **Self-Contained Executability**: Whether agent can execute with plan file alone.
6. **Checkpoint Granularity**: Whether checkpoints fit in single OpenCode session (~2 hours).
7. **implement.sh Integration**: Consistent checkpoint IDs and script verification.
8. **Additional factors per plan**: Domain-specific concerns (cross-platform, git isolation, external APIs).

---

**Review Complete**: 2026-04-02
**Next Review**: After addressing P0 and P1 action items
