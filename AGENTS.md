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
- KV cache MUST use `f16` (quantized types crash with Vulkan)
- `--flash-attn on` is safe and recommended

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

## Session Handoff Protocol

When context grows large, write handoff to `.opencode-handoff.md`:
- Objective (what was being accomplished)
- Completed (what's done and verified)
- In Progress (what was actively being worked on)
- Pending Todos (remaining work with priorities)
- Key Context (files modified, patterns followed, constraints)
- How to Continue (specific next steps)
