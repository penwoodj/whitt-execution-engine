# YAML-004: Config Validation Rejects Invalid Temperature

**Date**: 2026-04-25
**Status**: ✅ PASS (after fix)
**Tester**: Automated QA (Ralph Loop)

---

## Test Description

Verify validation catches out-of-range temperature values.

## Command

```bash
./target/release/whitt chat "Hello" --no-stream --temperature 5.0
```

## Expected Output

```
Error: temperature must be between 0.0 and 2.0, got 5.0
```

## Bug Found (FIXED)

**Initial Result**: ❌ FAIL

**Root Cause (Two issues)**:

1. `config/mod.rs` `validate_config()` used `tracing::warn!` instead of `anyhow::bail!` — validation logged warnings but never rejected invalid values.

2. Even after fixing validate_config to bail, CLI args (`--temperature 5.0`) were applied AFTER validation ran on the YAML config. The validation checked the config values (which were fine), but the CLI override replaced them after validation passed.

**Fix**:
1. `config/mod.rs`: Changed `tracing::warn!` → `anyhow::bail!` in `validate_config()`
2. `whitt.rs`: Added post-merge validation after CLI args are resolved:
```rust
if !(0.0..=2.0).contains(&temperature) {
    anyhow::bail!("temperature must be between 0.0 and 2.0, got {}", temperature);
}
```

## Post-Fix Result

**PASS** — Invalid temperature correctly rejected:
```
Error: temperature must be between 0.0 and 2.0, got 5
```

No request sent to server. Process exits with error code.

## Pass Criteria

| Criteria | Result |
|----------|--------|
| Error message displayed | ✅ |
| Validation failure reported | ✅ |
| Invalid value rejected | ✅ |
| Request NOT sent to server | ✅ |

## Replication

1. `whitt chat "Hello" --no-stream --temperature 5.0`
2. Observe error: `temperature must be between 0.0 and 2.0, got 5`
3. Verify no request reaches server (no debug HTTP logs)
