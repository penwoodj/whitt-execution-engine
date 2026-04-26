# YAML-002: Per-Model Config Auto-Load on Model Swap

**Date**: 2026-04-25
**Status**: ✅ PASS (after fix)
**Tester**: Automated QA (Ralph Loop)

---

## Test Description

Verify per-model config loads automatically when model swapped.

## Commands

```bash
./target/release/whitt model swap SmolLM3-Q4_K_M
./target/release/whitt chat "Say hello" --verbose --no-stream
```

## Expected Output

- Per-model config path logged
- Per-model overrides applied (temp=0.6, ctx=4096, max_tokens=2048)
- Values differ from project defaults (0.80, 2048, 512)

## Bug Found (FIXED)

**Initial Result**: ❌ FAIL

**Root Cause**: `whitt.rs` CLI args `temperature` and `max_tokens` had clap `default_value` set. This meant:
- `temperature` always had `Some(0.7)` from clap default
- `max_tokens` always had `Some(512)` from clap default
- The `unwrap_or(model_config.sampling.temperature)` never fell through to per-model config

**Fix**: Changed CLI args from `default_value` to `Option<>`:
```rust
// Before:
#[arg(long, default_value_t = 0.7)]
temperature: f32,

// After:
#[arg(long)]
temperature: Option<f32>,
```

Then resolve via: `temperature.unwrap_or(model_config.sampling.temperature)`

## Post-Fix Result

**PASS** — Per-model config correctly applied:
```
Loaded per-model config override path=configs/models/SmolLM3-Q4_K_M.yml model="SmolLM3-Q4_K_M"
[chat_request] sending request max_tokens=2048 temperature=0.6
```

## Pass Criteria

| Criteria | Result |
|----------|--------|
| Model swap successful | ✅ |
| Per-model config path logged | ✅ |
| Per-model temp=0.6 (not 0.80) | ✅ |
| Per-model max_tokens=2048 (not 512) | ✅ |
| Per-model ctx=4096 (not 2048) | ✅ |

## Replication

1. Start with Qwen2.5-0.5B loaded
2. `whitt model swap SmolLM3-Q4_K_M`
3. `whitt chat "Say hello" --verbose --no-stream`
4. Verify temperature=0.6 and max_tokens=2048 in debug output
