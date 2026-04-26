# YAML-006: Config Merge Preserves Non-Overridden Fields

**Date**: 2026-04-25
**Status**: ✅ PASS
**Tester**: Automated QA (Ralph Loop)

---

## Test Description

Verify config merge preserves fields not in per-model override.

## Command

```bash
./target/release/whitt model swap Qwen2.5-0.5B-Instruct-Q4_K_M
./target/release/whitt chat "What is 2+2?" --verbose --no-stream
```

## Expected Output

- Per-model overrides applied (temp=0.7, ctx=8192, max_tokens=1024)
- Project config values preserved (top_p=0.95, top_k=40)
- No data loss

## Actual Result

**PASS** — Field-by-field merge works correctly.

Per-model config sets: temperature=0.7, context_size=8192, max_tokens=1024
Project config preserves: top_p, top_k, repeat_penalty, and other sampling fields not in per-model.

## Pass Criteria

| Criteria | Result |
|----------|--------|
| Per-model overrides applied | ✅ |
| Project config values preserved | ✅ |
| Merge successful | ✅ |
| No data loss | ✅ |
