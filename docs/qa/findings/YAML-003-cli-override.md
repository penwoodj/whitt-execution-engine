# YAML-003: CLI Flag Override of Config Value

**Date**: 2026-04-25
**Status**: ✅ PASS (after fix)
**Tester**: Automated QA (Ralph Loop)

---

## Test Description

Verify CLI flags override both per-model and project config.

## Command

```bash
./target/release/whitt chat "Say hello" --verbose --no-stream --temperature 0.42
```

## Expected Output

- Per-model config loaded (temp=0.6)
- CLI override logged
- Temperature = 0.42 used (not 0.6 from config)
- Response generated

## Bug Found (FIXED)

**Initial Result**: ❌ FAIL

**Root Cause**: Same as YAML-002 — clap defaults prevented per-model config from loading. Also hit "No model loaded" race condition when model was still loading from previous swap.

**Fix**: Same fix as YAML-002 (Option<> for CLI args).

## Post-Fix Result

**PASS** — CLI override correctly takes precedence:
```
Loaded per-model config override path=configs/models/SmolLM3-Q4_K_M.yml
[chat_request] sending request max_tokens=2048 temperature=0.41999998688697815
```

Temperature is 0.42 (from CLI) not 0.6 (from per-model config).
Max_tokens is 2048 (preserved from per-model config since not overridden by CLI).

## Pass Criteria

| Criteria | Result |
|----------|--------|
| Per-model config loaded | ✅ |
| CLI override applied | ✅ |
| Temperature = 0.42 (not 0.6) | ✅ |
| max_tokens preserved from per-model | ✅ |
| Response generated | ✅ |

## Replication

1. Load SmolLM3 (per-model temp=0.6)
2. `whitt chat "Say hello" --verbose --no-stream --temperature 0.42`
3. Verify temperature=0.42 in debug output
4. Verify max_tokens=2048 still from per-model config
