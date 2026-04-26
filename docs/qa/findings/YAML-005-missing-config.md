# YAML-005: Missing Per-Model Config (Graceful Degradation)

**Date**: 2026-04-25
**Status**: ✅ PASS
**Tester**: Automated QA (Ralph Loop)

---

## Test Description

Verify graceful degradation when per-model config doesn't exist.

## Method

Rename per-model config temporarily, swap to that model, verify fallback.

```bash
mv configs/models/Qwen2.5-0.5B-Instruct-Q4_K_M.yml configs/models/Qwen2.5-0.5B-Instruct-Q4_K_M.yml.bak
./target/release/whitt model swap Qwen2.5-0.5B-Instruct-Q4_K_M
# (then restore)
mv configs/models/Qwen2.5-0.5B-Instruct-Q4_K_M.yml.bak configs/models/Qwen2.5-0.5B-Instruct-Q4_K_M.yml
```

## Expected Output

- Warning logged (not error)
- Fallback to defaults
- Model loads successfully
- No crash or panic

## Actual Result

**PASS** — All criteria met.

When config file is missing:
```
Config load failed, using defaults: ... (file not found)
```

Model loads with project defaults (temp=0.80, max_tokens=512, ctx=2048).
No panic, no crash. Request completes successfully.

## Pass Criteria

| Criteria | Result |
|----------|--------|
| Warning logged (not error) | ✅ |
| Fallback to defaults | ✅ |
| Model loads successfully | ✅ |
| No crash or panic | ✅ |
