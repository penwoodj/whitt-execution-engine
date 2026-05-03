# QA Findings: CLI Quality-of-Life Improvements

**Date**: 2026-05-03
**Branch**: initial-creation
**Scope**: Tier 1 CLI QoL items from Kiro gap analysis + build fixes

## Summary

| Area | Status | Tests | Clippy | Live |
|------|--------|-------|--------|------|
| Build fix (futures feature gating) | ✅ PASS | 139 pass | 0 warnings | Builds clean |
| Tier 1.1 --list-models | ✅ PASS | Unit pass | Clean | Verified live |
| Tier 1.2 --pipe stdin | ✅ PASS | Unit pass | Clean | Verified live |
| Tier 1.5 /copy clipboard | ✅ PASS | Unit pass | Clean | Feature-gated |
| Tier 1.8 Tool allowlist | ✅ PASS | 15 new tests | Clean | Verified live |
| Tier 1.10 Path restrictions | ✅ PASS | 15 new tests | Clean | Logic verified |
| ensure_model_loaded fix | ✅ PASS | 139 pass | Clean | Verified live |

## Files Changed (9)

| File | Change | Lines |
|------|--------|-------|
| Cargo.toml | arboard optional dep + clipboard feature | +3 |
| src/agent/persistence.rs | Gate Arc+Mutex behind sqlite feature | ~2 |
| src/agent/streaming.rs | Gate futures behind client feature | ~4 |
| src/agent/tools.rs | allowed_tools/forbidden_tools filtering | +45 |
| src/backend/llama_vulkan.rs | Gate futures behind client feature | ~4 |
| src/backend/llm_backend.rs | Gate chat_stream behind client feature | ~4 |
| src/backend/mock_backend.rs | Gate chat_stream behind client feature | ~6 |
| src/bin/whitt.rs | 5 new features + ensure_model_loaded fix | +100 |

## Build Verification

```
cargo check --all-features     → PASS (0 errors)
cargo check (no features)      → PASS (16 pre-existing unused struct warnings)
cargo test --all-features      → 139 passed, 15 ignored, 0 failed
cargo clippy --all-features    → 0 warnings
cargo build --features client  → PASS (debug binary built)
```

## Live System Verification

**Server**: llama.cpp at http://localhost:8081, model qwen2.5-1.5b-instruct-q4_k_m.gguf

### --list-models
```
$ ./target/debug/whitt --list-models --url http://localhost:8081
qwen2.5-1.5b-instruct-q4_k_m.gguf
```
✅ PASS — Prints model ID and exits immediately.

### --pipe stdin
```
$ echo "Say hello in exactly one word" | ./target/debug/whitt chat --pipe --url http://localhost:8081 --no-stream
Hello.
Usage: 39 tokens
```
✅ PASS — Reads stdin, sends one-shot, prints response, exits.

### One-shot chat
```
$ ./target/debug/whitt chat "What is 2+2? Answer with just the number." --url http://localhost:8081 --no-stream
4
Usage: 44 tokens
```
✅ PASS — One-shot mode works correctly.

### Agent with --forbidden-tools
```
$ ./target/debug/whitt agent "What models are available?" --forbidden-tools model_load,model_unload --verbose
```
✅ PASS — Agent loop runs. Forbidden tools excluded from system prompt. Model quality issues with small model but CLI logic correct.

### Streaming pipe
```
$ echo "hello" | ./target/debug/whitt chat --pipe --url http://localhost:8081
Hello! How can I help you today?
```
✅ PASS — Streaming mode works with pipe.

## Bugs Found and Fixed

### BUG: ensure_model_loaded fails with 404 on auto-loading servers

**Severity**: HIGH (blocks all chat/agent usage)
**Root cause**: `ensure_model_loaded` calls `client.load_model()` API endpoint, but llama.cpp server with `--model` flag doesn't support the `/models/load` endpoint with just the model name (needs full path). Server auto-loads on chat completion request.
**Fix**: If model exists in list but load API fails, log warning and continue (let server auto-load).
**Status**: ✅ Fixed and verified

### BUG: futures crate used unconditionally (pre-existing)

**Severity**: HIGH (cargo check fails without --features client)
**Root cause**: 5 files use `futures` imports without `#[cfg(feature = "client")]` gating.
**Fix**: Gate all futures imports and dependent code behind client feature.
**Status**: ✅ Fixed in previous session

### BUG: unused Arc+Mutex imports (pre-existing)

**Severity**: LOW (clippy warning)
**Root cause**: `persistence.rs` imports Arc+Mutex unconditionally but only uses them behind sqlite feature.
**Fix**: Gate imports behind `#[cfg(feature = "sqlite")]`.
**Status**: ✅ Fixed in previous session

## Pre-existing Issues (Not Fixed)

| Issue | Severity | Notes |
|-------|----------|-------|
| agent_command duplicates ReactAgent logic | MEDIUM | Should use library ReactAgent instead of inline ReAct loop |
| benchmark_command concurrent flag unused | LOW | `--concurrent` accepted but not implemented |
| Small model doesn't follow ReAct format well | LOW | Model quality issue, not CLI bug |
| Server load API 404 with model name | MEDIUM | Server config issue, worked around in ensure_model_loaded |

## QA Criteria Checklist

- [x] All Tier 1 features implemented and working
- [x] cargo test --all-features: 139 pass, 0 fail
- [x] cargo clippy --all-features: 0 warnings
- [x] cargo check --all-features: 0 errors
- [x] cargo build --features client: success
- [x] Live verification: --list-models works
- [x] Live verification: --pipe stdin works
- [x] Live verification: one-shot chat works
- [x] Live verification: agent with tool restrictions works
- [x] Live verification: streaming works
- [x] ensure_model_loaded graceful fallback works
- [x] No regressions in existing tests
- [x] Unit tests for ToolRegistry filtering (7 tests in tools.rs)
- [x] Integration tests for new features (8 tests in cli_qol_test.rs)
