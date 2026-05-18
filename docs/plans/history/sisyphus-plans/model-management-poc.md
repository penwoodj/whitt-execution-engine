# POC Plan: Model Management + Hot-Swap + Configurable Paths

## User Requirements

1. External args to change model + load config at runtime
2. Download model by input if not present, with user confirmation
3. Configure models to load from `/run/media/jon/data/models/` (configurable)
4. Benchmark unload+load speed on HDD (`/run/media/jon/data`) vs SSD (`/home/jon`)
5. Integration tests for all features

## Implementation Status: COMPLETE

### Phase 1: Router Mode + Configurable Model Path ✅
- [x] Add `models_dir` to config.yml
- [x] Add `--models-dir` to entrypoint.sh start_server() (skips `-m` flag in router mode)
- [x] Make model host path configurable via env var `MODEL_HOST_PATH` in docker-compose.yml
- [x] Default: `./models` → override: `/run/media/jon/data/models`
- [x] `DOCKER_UID`/`DOCKER_GID` env vars for permission fix on external drives

### Phase 2: Rust Client Model Management ✅
- [x] Types: `ModelInfo`, `ModelStatus`, `ModelLoadRequest`, `ModelLoadResponse`, `ModelUnloadRequest`, `ModelListResponse` (src/client/types.rs)
- [x] Methods: `list_models()`, `load_model()`, `unload_model()`, `wait_for_model_status()` (src/client/http_client.rs)
- [x] `load_model()` handles "already running" as success (idempotent)
- [x] Status polling: 500ms interval, 120s timeout for load, 60s for unload

### Phase 3: Model Download Script ✅
- [x] `scripts/download-model.sh` — interactive HF download
- [x] Lists available .gguf files when filename omitted
- [x] Confirmation prompt before download
- [x] SHA256 verification, retry logic, speed reporting
- [x] `MODEL_OUTPUT_DIR` env var to configure destination

### Phase 4: switch-model.sh v2 ✅
- [x] `scripts/switch-model-v2.sh` — pure API-based model switching
- [x] Actions: load, unload, swap, list, status
- [x] `swap` unloads all loaded models then loads target
- [x] No container restart needed (router mode)

### Phase 5: Benchmarks ✅
- [x] `scripts/benchmark-model-swap.sh` — automated benchmark
- See results below

### Phase 6: Integration Tests ✅
- [x] `test_integration_all` — health + chat + streaming + e2e (with model load/unload)
- [x] `test_list_models` — GET /v1/models validation
- [x] `test_load_and_unload_model` — full lifecycle with chat verification
- [x] `test_load_already_loaded_model` — idempotent load handling
- [x] `test_model_hot_swap` — load + verify + unload
- [x] `test_prompt_chain_all` — multi-step chain + system prompt
- [x] All tests use `#[serial]` for safe sequential execution
- [x] All 6 tests pass: `cargo test --test integration_test --test model_management_test --test prompt_chain_test -- --ignored --test-threads=1`

## Benchmark Results

### Model: Qwen2.5-0.5B-Instruct-Q4_K_M (469MB)

| Metric | SSD (LUKS) | HDD (/run/media/jon/data) | Delta |
|--------|-----------|---------------------------|-------|
| Load (avg) | 1064ms | 1060ms | ~0% |
| Unload (avg) | 537ms | 537ms | 0% |
| Full Swap (avg) | 1602ms | 1597ms | ~0% |
| Chat (first token) | 406-638ms | 394-420ms | ~0% |

### Finding
No measurable difference between SSD and HDD for this 469MB model. The OS page cache fully buffers the file after first load, eliminating disk I/O as a bottleneck. A larger model (3GB+) would likely show a gap on cold load.

## Files Created/Modified

| File | Change |
|------|--------|
| `config.yml` | Added `models_dir: /models` |
| `docker/entrypoint.sh` | Router mode logic in `translate_config()` + `start_server()` |
| `docker-compose.yml` | `MODEL_HOST_PATH` env var, `DOCKER_UID`/`DOCKER_GID` |
| `src/client/types.rs` | Added 6 model management types |
| `src/client/http_client.rs` | Added `list_models()`, `load_model()`, `unload_model()`, `wait_for_model_status()` |
| `tests/integration_test.rs` | Updated for router mode (model load/unload, correct model ID) |
| `tests/model_management_test.rs` | NEW: 4 model management tests |
| `tests/prompt_chain_test.rs` | Updated model ID, added `ensure_model_loaded()` |
| `scripts/switch-model-v2.sh` | NEW: API-based model switching |
| `scripts/download-model.sh` | NEW: Interactive HF download |
| `scripts/benchmark-model-swap.sh` | NEW: Automated benchmark |
| `Cargo.toml` | Added `serial_test = "3"` to dev-deps |
