# YAML Configuration Test Procedures

Specific test procedures with exact commands and expected outputs for the YAML configuration system.

---

## Prerequisites

1. Docker container running:
   ```bash
   docker ps --filter name=llama
   # Expected: whitt-llama-server Up X hours (healthy)
   ```

2. Build binaries:
   ```bash
   cargo build --bin whitt --features client
   ```

3. Swap to Qwen2.5-0.5B (baseline model):
   ```bash
   ./target/debug/whitt model swap Qwen2.5-0.5B-Instruct-Q4_K_M
   ```

---

## YAML-001: Load Project Config

**Description**: Verify project config.yml loads correctly and sets defaults.

**Prerequisites**: Container running, config.yml exists at repo root

**Command**:
```bash
./target/debug/whitt chat "Hello" --verbose --no-stream
```

**Expected Output**:
```
[INFO] Loaded project config: /path/to/config.yml
[INFO] Using context size: 2048
[INFO] Using temperature: 0.80
[INFO] Using max tokens: 512
[INFO] Requesting non-streaming response...
Hello! How can I help you today?
Tokens: 9 (prompt: 5, completion: 4)
```

**Pass Criteria**:
- ✅ Config file path logged
- ✅ Default values applied (ctx=2048, temp=0.80, max_tokens=512)
- ✅ Response generated successfully
- ✅ Token usage reported

---

## YAML-002: Per-Model Config Auto-Load on Model Swap

**Description**: Verify per-model config loads automatically when model swapped.

**Prerequisites**: Container running

**Command**:
```bash
./target/debug/whitt model swap SmolLM3-Q4_K_M
./target/debug/whitt chat "Say hello" --verbose --no-stream
```

**Expected Output**:
```
Unloading model: Qwen2.5-0.5B-Instruct-Q4_K_M
Swapping to model: SmolLM3-Q4_K_M
[INFO] Loaded per-model config override: configs/models/SmolLM3-Q4_K_M.yml
[INFO] Using per-model temperature: 0.6
[INFO] Using per-model context size: 4096
[INFO] Using per-model max tokens: 2048
Hello!
Tokens: 1 (prompt: 8, completion: 1)
```

**Pass Criteria**:
- ✅ Model swap successful
- ✅ Per-model config path logged
- ✅ Per-model overrides applied (temp=0.6, ctx=4096, max_tokens=2048)
- ✅ Values differ from project defaults (0.80, 2048, 512)

---

## YAML-003: CLI Flag Override of Config Value

**Description**: Verify CLI flags override both per-model and project config.

**Prerequisites**: Container running, SmolLM3 loaded (per-model temp=0.6)

**Command**:
```bash
./target/debug/whitt chat "Say hello" --verbose --no-stream --temperature 0.42
```

**Expected Output**:
```
[INFO] Loaded per-model config override: configs/models/SmolLM3-Q4_K_M.yml
[INFO] Using per-model temperature: 0.6
[INFO] CLI override: temperature = 0.42
[INFO] Requesting non-streaming response...
Hello!
Tokens: 1 (prompt: 8, completion: 1)
```

**Pass Criteria**:
- ✅ Per-model config loaded
- ✅ CLI override logged
- ✅ Temperature = 0.42 used (not 0.6 from config)
- ✅ Response generated successfully

---

## YAML-004: Config Validation Rejects Invalid Temperature

**Description**: Verify validation catches out-of-range temperature values.

**Prerequisites**: Container running

**Command**:
```bash
# Attempt to set invalid temperature (outside 0.0-2.0 range)
./target/debug/whitt chat "Hello" --no-stream --temperature 5.0
```

**Expected Output**:
```
Error: Validation failed
  Caused by: temperature must be between 0.0 and 2.0, got 5.0
```

**Pass Criteria**:
- ✅ Error message displayed
- ✅ Validation failure reported
- ✅ Invalid value rejected
- ✅ Request not sent to server

---

## YAML-005: Missing Per-Model Config (Graceful Degradation)

**Description**: Verify graceful degradation when per-model config doesn't exist.

**Prerequisites**: Container running, model with no per-model config

**Command**:
```bash
# Create a model entry with no corresponding config file
# (hypothetical model for testing)
echo "Qwen2.5-0.5B-Instruct-Q4_K_M" > /tmp/test-model.txt
# Note: In actual testing, verify behavior with model that truly has no config
```

**Alternative test (rename config temporarily)**:
```bash
mv configs/models/Qwen2.5-0.5B-Instruct-Q4_K_M.yml configs/models/Qwen2.5-0.5B-Instruct-Q4_K_M.yml.bak
./target/debug/whitt model swap Qwen2.5-0.5B-Instruct-Q4_K_M
mv configs/models/Qwen2.5-0.5B-Instruct-Q4_K_M.yml.bak configs/models/Qwen2.5-0.5B-Instruct-Q4_K_M.yml
```

**Expected Output**:
```
[WARN] No per-model config found for Qwen2.5-0.5B-Instruct-Q4_K_M, using defaults
[INFO] Using context size: 2048
[INFO] Using temperature: 0.80
```

**Pass Criteria**:
- ✅ Warning logged (not error)
- ✅ Fallback to defaults
- ✅ Model loads successfully
- ✅ No crash or panic

---

## YAML-006: Config Merge Preserves Non-Overridden Fields

**Description**: Verify config merge preserves fields not in per-model override.

**Prerequisites**: Container running

**Command**:
```bash
# Swap to Qwen2.5-0.5B (per-model has temp, ctx, max_tokens)
./target/debug/whitt model swap Qwen2.5-0.5B-Instruct-Q4_K_M
# Project config has top_p=0.95, top_k=40 (not in per-model)
./target/debug/whitt chat "What is 2+2?" --verbose --no-stream
```

**Expected Output**:
```
[INFO] Loaded per-model config override: configs/models/Qwen2.5-0.5B-Instruct-Q4_K_M.yml
[INFO] Using per-model temperature: 0.7
[INFO] Using per-model context size: 8192
[INFO] Using per-model max tokens: 1024
[INFO] Using project config top_p: 0.95
[INFO] Using project config top_k: 40
4
Tokens: 1 (prompt: 14, completion: 1)
```

**Pass Criteria**:
- ✅ Per-model overrides applied (temp=0.7, ctx=8192, max_tokens=1024)
- ✅ Project config values preserved (top_p=0.95, top_k=40)
- ✅ Merge successful
- ✅ No data loss

---

## YAML-007: All Sampling Flags from CLI

**Description**: Verify all sampling CLI flags are accepted and override config.

**Prerequisites**: Container running

**Command**:
```bash
./target/debug/whitt chat "What is 2+2?" --no-stream \
  --temperature 0.7 \
  --max-tokens 100 \
  --top-p 0.9 \
  --top-k 50 \
  --repeat-penalty 1.1 \
  --presence-penalty 0.5 \
  --frequency-penalty 0.5 \
  --stop "hello" \
  --seed 42
```

**Expected Output**:
```
[INFO] CLI override: temperature = 0.7
[INFO] CLI override: max_tokens = 100
[INFO] CLI override: top_p = 0.9
[INFO] CLI override: top_k = 50
[INFO] CLI override: repeat_penalty = 1.1
[INFO] CLI override: presence_penalty = 0.5
[INFO] CLI override: frequency_penalty = 0.5
[INFO] CLI override: stop_sequences = ["hello"]
[INFO] CLI override: seed = 42
[INFO] Requesting non-streaming response...
4
Tokens: 1 (prompt: 14, completion: 1)
```

**Pass Criteria**:
- ✅ All 9 flags accepted
- ✅ All overrides logged
- ✅ Response generated with specified parameters
- ✅ No errors

---

## YAML-008: Per-Model Cache Type Override (7B Uses q4_0)

**Description**: Verify 7B model's per-model cache type override (q4_0) works.

**Prerequisites**: Container running, sufficient VRAM for 7B model

**Command**:
```bash
./target/debug/whitt model swap Qwen2.5-7B-Instruct-1M-Q4_K_M
./target/debug/whitt chat "Say hello" --verbose --no-stream
```

**Expected Output**:
```
[INFO] Loaded per-model config override: configs/models/Qwen2.5-7B-Instruct-1M-Q4_K_M.yml
[INFO] Using per-model temperature: 0.7
[INFO] Using per-model context size: 32768
[INFO] Using per-model max tokens: 4096
[INFO] Using per-model cache_type_k: q4_0
[INFO] Using per-model cache_type_v: q4_0
[INFO] Using per-model gpu_layers: 99
Hello!
Tokens: 1 (prompt: 8, completion: 1)
```

**Pass Criteria**:
- ✅ Per-model config loaded
- ✅ Cache type set to q4_0 (not default f16)
- ✅ GPU layers = 99
- ✅ Model loads and responds
- ✅ VRAM usage reduced (q4_0 is 4-bit quantization)

---

## YAML-009: Vulkan Env Vars from Config

**Description**: Verify Vulkan environment variables are set from config.

**Prerequisites**: Container running with AMD GPU

**Command**:
```bash
# Check Vulkan env vars in container
docker exec whitt-llama-server env | grep VULKAN
```

**Expected Output**:
```
VULKAN_VISIBLE_DEVICES=0
VULKAN_DISABLE_DEBUG=1
VULKAN_ENABLE_VALIDATION=0
```

**Alternative test (verify GPU detection works)**:
```bash
./target/debug/whitt server gpu
```

**Expected Output**:
```
Detected GPU: AMD
Recommended docker compose command:
  docker compose -f docker-compose.amd.yml up -d
```

**Pass Criteria**:
- ✅ Vulkan env vars set from config
- ✅ VULKAN_VISIBLE_DEVICES=0
- ✅ VULKAN_DISABLE_DEBUG=1
- ✅ GPU detection works

---

## YAML-010: Default Values When Config Fields Omitted

**Description**: Verify defaults apply when config fields are omitted.

**Prerequisites**: Container running

**Command**:
```bash
# Create minimal config with only one field
cat > /tmp/minimal-config.yml << 'EOF'
sampling:
  temperature: 0.7
EOF

# Set MODEL_NAME and use minimal config (simulated)
./target/debug/whitt chat "Hello" --verbose --no-stream
```

**Expected Output** (using project defaults + any overrides):
```
[INFO] Loaded project config: /path/to/config.yml
[INFO] Using temperature: 0.7
[INFO] Using default context size: 2048
[INFO] Using default max tokens: 512
[INFO] Using default top_p: 0.95
[INFO] Using default top_k: 40
Hello!
Tokens: 1 (prompt: 5, completion: 1)
```

**Pass Criteria**:
- ✅ Temperature = 0.7 (from config)
- ✅ Context size = 2048 (default)
- ✅ Max tokens = 512 (default)
- ✅ top_p = 0.95 (default)
- ✅ top_k = 40 (default)
- ✅ No nil pointer errors

---

## Additional Edge Cases

### YAML-011: Invalid YAML Syntax

**Command**:
```bash
echo "invalid: yaml: content" > configs/models/test.yml
```

**Expected**: Error on load, graceful degradation

### YAML-012: Empty Per-Model Config

**Command**:
```bash
echo "" > configs/models/test.yml
```

**Expected**: Treated as no config, defaults used

### YAML-013: Circular Config References

**Command**:
```yaml
# Attempt to create circular reference (not supported)
a:
  b: ${a}
```

**Expected**: Error or resolution failure

---

## Test Execution Checklist

- [ ] YAML-001: Load Project Config
- [ ] YAML-002: Per-Model Config Auto-Load
- [ ] YAML-003: CLI Flag Override
- [ ] YAML-004: Config Validation
- [ ] YAML-005: Missing Config Graceful Degradation
- [ ] YAML-006: Config Merge Preserves Fields
- [ ] YAML-007: All Sampling Flags from CLI
- [ ] YAML-008: Per-Model Cache Type Override
- [ ] YAML-009: Vulkan Env Vars from Config
- [ ] YAML-010: Default Values on Omitted Fields

---

## Success Criteria

**Overall Test Suite Pass**:
- ✅ All 10 core tests pass
- ✅ No crashes or panics
- ✅ All expected logs present
- ✅ Validation errors caught appropriately
- ✅ Graceful degradation verified

**Known Issues Documented**:
- Invalid YAML keys silently ignored (expected in POC)
- No JSON Schema validation yet (expected in POC)

---

## References

- Config schema: `src/config/mod.rs`
- Per-model configs: `configs/models/*.yml`
- Project config: `config.yml`
- Entrypoint: `entrypoint.sh` (lines 585-626)
- QA overview: `docs/qa/QA-YAML-CONFIG.md`
- General QA instructions: `docs/QA.md` (Section 9)
