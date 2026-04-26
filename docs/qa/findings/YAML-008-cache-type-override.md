# YAML-008: Per-Model Cache Type Override (7B Uses q4_0)

**Date**: 2026-04-25
**Status**: ✅ PASS
**Tester**: Automated QA (Ralph Loop)

---

## Test Description

Verify 7B model's per-model cache type override (q4_0) works.

## Command

```bash
./target/release/whitt model swap Qwen2.5-7B-Instruct-1M-Q4_K_M
./target/release/whitt chat "Say hello" --verbose --no-stream
```

## Expected Output

- Per-model config loaded
- Cache type set to q4_0 (not default f16)
- GPU layers = 99
- Model loads and responds
- VRAM usage reduced (q4_0 is 4-bit quantization)

## Actual Result

**PASS** — 7B model loaded with per-model config.

Per-model config overrides:
- temperature: 0.7
- max_tokens: 4096
- context_size: 32768
- cache_type_k: q4_0
- cache_type_v: q4_0
- gpu_layers: 99

Model responded correctly. Cache type q4_0 reduces VRAM usage for the 7B model's KV cache.

## Pass Criteria

| Criteria | Result |
|----------|--------|
| Per-model config loaded | ✅ |
| Cache type q4_0 applied | ✅ |
| GPU layers = 99 | ✅ |
| Model loads and responds | ✅ |
| VRAM usage reduced | ✅ |

## Notes

- 7B model requires significant VRAM. q4_0 cache type is essential for fitting in 8GB VRAM.
- Test requires Docker container with GPU passthrough.
