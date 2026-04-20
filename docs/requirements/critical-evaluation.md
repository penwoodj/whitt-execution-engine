# 🔍 Critical Evaluation

## 📊 Overview

This document summarizes the critical technology choices for the AutoAgents SDK workflow execution engine. All decisions are based on 2025-2026 ecosystem research, performance benchmarks, and production readiness.

---

## Decision Framework

Each evaluation considers:
- Performance benchmarks (quantitative)
- Code quality (unsafe usage, maintenance)
- Ecosystem maturity (stars, downloads, active maintenance)
- Feature completeness (required capabilities)
- Community adoption (battle-tested)

**Evaluation sources:**
- `RESEARCH_UPSTREAM_SUCCESS_FACTORS.md` (April 2026, 683 lines)
- ADRs (`adr-0000` through `adr-0007`)
- Performance benchmarks from upstream crates
- RustSec advisories for security issues

---

## 1. YAML Parser: serde-saphyr

### Alternatives Evaluated

| Crate | Status | Downloads | Parse Time (25MB) | unsafe | Verdict |
|-------|--------|-----------|------------------|--------|
| **serde-saphyr** | ✅ Active, pure Rust | 230K+ (90d) | **294.83ms** | None | **CHOSEN** |
| yaml_serde | ✅ Active, official YAML org | 46K+ (90d) | ~470ms | unsafe-libyaml | Fallback for migration |
| serde_yml | ❌ Unmaintained, unsound | N/A | N/A | Yes (RUSTSEC-0067/68) | AVOID |
| yaml-rust2 | ⚠️ Maintenance only | 31M+ total | ~480ms | unsafe-libyaml | WATCH |
| serde_yaml | ❌ Deprecated | N/A | 477ms | unsafe-libyaml | Deprecated baseline |

### Why serde-saphyr?

**Advantages:**
1. **Pure Rust**: `#![forbid(unsafe_code)]` guarantees safety
2. **Fastest**: 1.6x faster than serde_yaml, 1.5x faster than yaml_serde
3. **Merge keys**: Native support for YAML merge (`<<:`), critical for layered configs
4. **Nested enums**: Rust-aligned enum handling, not string-based
5. **Garde integration**: Built-in schema validation
6. **Active maintenance**: Last push 90 days ago, used by nickel-lang, cloudflare/foundations

**Trade-offs:**
- **Evolving API**: Version 0.0.23 (as of April 2026), breaking changes possible
- **Less ecosystem**: 230K downloads vs 31M for yaml-rust2 (but yaml-rust2 is unsafe)

### Migration Path
For workflows using `serde_yaml` syntax, migrate to `yaml_serde`:
```toml
[dependencies]
serde_yaml = { package = "yaml_serde", version = "0.10" }
```
This is the official YAML org fork and drop-in compatible.

---

## 2. Template Engine: Minijinja

### Alternatives Evaluated

| Engine | Type | Big Table (500 ops) | Status | Verdict |
|--------|------|-------------------|--------|---------|
| **Minijinja** | Interpreted | ~9-13 µs | 2,511★ | **CHOSEN for YAML** |
| Askama | Precompiled | 330 µs | Mature | Used for codegen only |
| Tera | Interpreted | 856 µs | 4,167★ | Too slow |
| Handlebars | Interpreted | 3.66 ms | 355★ | 4-5x slower |

### Why Minijinja for YAML Interpolation?

**Use Case:** Runtime interpolation in unified schema (`{{ step_name.output }}`, `{{ inputs.field }}`)

**Advantages:**
1. **Jinja2 compatibility**: LLMs are trained on Jinja2 templates, syntax familiarity
2. **Lightweight**: 2.5K crate, faster than Tera (856 µs vs 9-13 µs)
3. **Rich expressions**: Filters, loops, conditionals match unified schema needs
4. **Active maintenance**: Last push 6 days ago

**Trade-offs:**
- Interpreted (not precompiled), but acceptable for small interpolation payloads
- Smaller ecosystem than Tera, but growing

### Why Askama for Code Generation?

**Use Case:** Transpiler generating Rust code from WorkflowIR (compile-time)

**Advantages:**
1. **Fastest**: 330 µs vs 856 µs (Tera)
2. **Type-safe**: Compile-time template validation, no runtime panics
3. **No string manipulation**: Strong guarantees for generated code

**Separate Concerns:**
- Minijinja = runtime interpolation (unified schema)
- Askama = compile-time codegen (transpiler internals)

---

## 3. Agent Framework: Rig

### Alternatives Evaluated

| Framework | Stars | Providers | Status | Verdict |
|-----------|-------|-----------|--------|---------|
| **Rig** | 6,736★ | 20+ providers | ✅ Active | **CHOSEN** |
| Cognis | ~200★ | HITL support | ✅ New (Mar 2026) | Emerging |

### Why Rig?

**Advantages:**
1. **Mature ecosystem**: 6,736 stars, battle-tested in production
2. **Multi-provider**: 20+ LLM providers out of the box (OpenAI, Anthropic, Google, Ollama, LM Studio, llama.cpp)
3. **Tool calling**: Native support for MCP protocol (Model Context Protocol)
4. **Streaming**: Built-in streaming with proper backpressure handling
5. **Active development**: Frequent releases, responsive community

**Capabilities Provided:**
- Model selection and provider abstraction
- Tool registration and execution
- Streaming responses with chunk accumulation
- Retry logic with exponential backoff
- Parallel agent spawning
- Conversation state management

**Trade-offs:**
- **New framework**: Younger ecosystem compared to Python (LangChain)
- **Rust-only**: No Python bindings (yet)

---

## 4. Workflow Engine: Treadle

### Alternatives Evaluated

| Engine | Stars | Status | Features | Verdict |
|---------|-------|--------|---------|---------|
| **Treadle v0.2.0** | Mature | Stateful + HITL | **CHOSEN** |
| Dagrs | 800★ | Archived | DAG only |
| Custom | N/A | N/A | Out of scope |

### Why Treadle?

**Advantages:**
1. **Persistent workflows**: SQLite StateStore for checkpointing and resumption
2. **Human-in-the-loop**: `StageOutcome::NeedsReview` gates for manual approval
3. **DAG execution**: Backed by petgraph, cycle detection, topological sort
4. **Checkpointing**: Per-subtask state tracking for fan-out workflows
5. **Resumable**: Restart after crash without re-running completed work

**Workflow Capabilities:**
- DAG execution with dependency resolution
- Loop constructs with convergence detection
- Parallel execution where possible
- Error propagation and recovery
- Human review gates
- State persistence and resumption

**Trade-offs:**
- **Single-process**: Not designed for distributed execution (we'll run one per workflow)
- **SQLite dependency**: Additional storage layer, but lightweight

---

## 5. Template Interpolation vs. Hooks

**Decision:** Unified schema uses **`when` hooks** instead of explicit interpolation section.

### Comparison

| Approach | Example | Pros | Cons |
|-----------|---------|-------|-------|
| **Explicit interpolation** | `prompts: "{{analyze}}"` | Clear separation | Verbose, step-locked |
| **When hooks** | `when: run_analysis | true` | Flexible, can be conditional | Implicit, requires docs |

### Why When Hooks?

**Advantages:**
1. **Auto-created variables**: Step outputs automatically named after step
2. **Substructure access**: `{{step_name.output.raw_text}}`, `.response`, `.metadata`
3. **Conditional**: Can gate execution without blocking pipeline
4. **Per-step**: Each step can have independent hooks

**Example:**
```yaml
steps:
  - name: analyze_code
    tools: [llm_generate]
    when:
      - run_analysis: true
      - quality_threshold_met: true
```

---

## 6. Tool Sandboxing: Sandlock

### Alternatives Evaluated

| Approach | Coverage | Linux Support | Verdict |
|----------|-----------|--------------|---------|
| **Sandlock v0.5.0** | Landlock + seccomp | Linux 6.12+, Rust 1.70+ | **CHOSEN** |
| landlock standalone | Landlock only | Linux 5.13+ | Partial |
| seccomp only | Syscall filtering | Linux 4.x+ | Manual effort |
| None | N/A | N/A | AVOID |

### Why Sandlock?

**Advantages:**
1. **Combined**: Landlock (filesystem/network/IPC) + seccomp (syscall filtering)
2. **Resource limits**: Memory, CPU, time budgets per-tool
3. **No-supervisor mode**: Landlock + deny-only seccomp for stricter enforcement
4. **Pipeline chaining**: `|` operator for chained sandboxing rules
5. **Production-ready**: Used by container runtimes

**Capabilities:**
- Path guards (no outside project directory)
- Dependency allowlists (only approved crates)
- Import restriction enforcement
- Per-tool resource budgets
- Timeout enforcement

**Trade-offs:**
- **Linux-only**: No Windows/macOS support (but these don't have fine-grained permissions anyway)
- **Setup complexity**: Requires policy configuration per tool

---

## 7. Streaming: sseer + ndjson_stream

### Alternatives Evaluated

| Crate | Performance | Provider Support | Verdict |
|-------|--------------|----------------|---------|
| **sseer** | 3.1-3.9x faster | SSE (OpenAI/Anthropic) | **CHOSEN for SSE** |
| ndjson_stream | Native | NDJSON (Ollama) | **CHOSEN for NDJSON** |
| eventsource-stream | Baseline | SSE | 1x speed |

### Why sseer for SSE?

**Benchmarks (10K chunks):**
- sseer: 0.43-0.58ms (3.1-3.9x faster than baseline)
- eventsource-stream: 1.67-2.24ms

**Advantages:**
1. **Performance**: 3x faster chunk delivery
2. **Backpressure**: Proper async handling
3. **Provider-native**: Works with OpenAI SSE and Anthropic EventStream

**Why ndjson_stream for Ollama?**

Ollama outputs NDJSON (newline-delimited JSON), not SSE. Using native crate avoids overhead.

---

## 8. Testing Stack: VidaiMock + proptest

### Alternatives Evaluated

| Need | Tool | Evidence | Verdict |
|------|------|-----------|---------|
| **Mock LLM** | VidaiMock v0.1.3 | 50K+ RPS, realistic TTFT | **CHOSEN** |
| **PBT** | proptest v1.11.0 | Standard Rust PBT framework | **CHOSEN** |
| **Traffic sim** | llmsim | Docs.rs, well-documented | Use as needed |

### Why VidaiMock?

**Advantages:**
1. **Physics-accurate streaming**: Realistic token-by-token delivery
2. **Zero-config**: ~7MB binary, no setup required
3. **Chaos testing**: Latency, failures, malformed responses
4. **Provider-native**: OpenAI SSE, Anthropic EventStream, Gemini

**Use Cases:**
- Unit tests: Mock isolated components
- Integration tests: Mock full workflow scenarios
- Load testing: 50K+ RPS benchmark mode

### Why proptest?

**Advantages:**
1. **Stateful testing**: `RuleBasedStateMachine` for workflow state transitions
2. **Shrinking**: Minimizes counterexamples to minimal failing cases
3. **Randomized input**: Explores edge cases

**Properties to Test:**
- Roundtrip: encode → decode = original
- Invariants: DAG has no cycles, all nodes reachable
- Idempotency: apply operation twice = apply once
- Monotonicity: quality scores only increase in verify-repair loops

---

## 9. Async Runtime: Tokio

### Alternatives Evaluated

| Runtime | Downloads | Focus | Status | Verdict |
|----------|-----------|-------|--------|---------|
| **tokio** | 5M+ stars/mo | Production-proven, ecosystem | **CHOSEN** |
| async-std | 13K downloads/mo | Simpler, std-like | Growing | Good for simple apps |

### Why Tokio?

**Advantages:**
1. **Mature ecosystem**: 5M+ stars, production-proven
2. **Broad ecosystem**: async-std, reqwest, axum, tonic all tokio-based
3. **Critical features**: Timers, channels, spawning, io_uring
4. **Future-proof**: Dominant async runtime, unlikely to be displaced

**Trade-offs:**
- **Complexity**: Steeper learning curve than async-std
- **Overkill**: For simple CLI apps, async-std may suffice

---

## Summary: Chosen Stack

| Layer | Choice | Rationale |
|-------|---------|-----------|
| **YAML Parsing** | serde-saphyr | Pure Rust, fastest, merge keys, garde integration |
| **Template Interpolation** | Minijinja | Jinja2 compatible (LLM training familiarity), lightweight |
| **Code Generation** | Askama | Precompiled, type-safe, no string manipulation |
| **Agent Framework** | Rig | 20+ providers, MCP support, mature ecosystem |
| **Workflow Engine** | Treadle v0.2.0 | Persistent, resumable, HITL support |
| **Sandboxing** | Sandlock v0.5.0 | Landlock + seccomp, resource limits |
| **Streaming** | sseer + ndjson_stream | 3x faster, provider-native |
| **Testing** | VidaiMock + proptest | Realistic streaming + property-based |
| **Async Runtime** | Tokio | Production-proven, broad ecosystem |

---

## Out of Scope (Not Evaluated)

- **Language targets**: Python, Go, JavaScript (future extensions)
- **Distributed execution**: Multi-worker coordination (local-only for now)
- **Monitoring**: Tracing, metrics collection (separate concern)
- **CLI frameworks**: clp-derivative, dialoguer (user preference)

---

## References

- **Upstream Research**: `RESEARCH_UPSTREAM_SUCCESS_FACTORS.md` (April 2026)
- **ADRs**: `adr-0000` through `adr-0007`
- **Transpiler Architecture**: `transpiler_architecture.md`
- **Unified Schema**: `unified-workflow-schema.yml` v2.0
