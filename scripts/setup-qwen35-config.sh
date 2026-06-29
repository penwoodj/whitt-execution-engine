#!/usr/bin/env bash
# Updates config.yml with Qwen3.5-9B settings per docs/plans/meta-workflow-generator/01-CONTEXT-AND-CONSTRAINTS.md
# Usage: ./scripts/setup-qwen35-config.sh [config-path]
set -euo pipefail

CONFIG="${1:-config.yml}"

if [[ ! -f "$CONFIG" ]]; then
  echo "ERROR: config file not found: $CONFIG" >&2
  exit 1
fi

if ! command -v yq >/dev/null 2>&1; then
  echo "ERROR: yq not installed. Install via: pip install yq OR download from https://github.com/mikefarah/yq" >&2
  exit 1
fi

# Detect yq variant (mikefarah/yq uses different syntax than kislyuk/yq)
if yq --help 2>&1 | grep -q "mikefarah"; then
  YQ_INPLACE=(-i)
else
  YQ_INPLACE=(-y)
fi

echo "Updating $CONFIG for Qwen3.5-9B (CPU-only, 32K context (2x buffer policy), Q8_0 KV cache)..."

# Model identity
yq "${YQ_INPLACE[@]}" '.model.path = "/run/media/jon/data/models/lmstudio-community/Qwen3.5-9B-GGUF/Qwen3.5-9B-Q4_K_M.gguf"' "$CONFIG"
yq "${YQ_INPLACE[@]}" '.model.models_dir = "/run/media/jon/data/models"' "$CONFIG"
yq "${YQ_INPLACE[@]}" '.model.huggingface.repo = "lmstudio-community/Qwen3.5-9B-GGUF"' "$CONFIG"
yq "${YQ_INPLACE[@]}" '.model.huggingface.filename = "Qwen3.5-9B-Q4_K_M.gguf"' "$CONFIG"
yq "${YQ_INPLACE[@]}" '.model.quantization = "Q4_K_M"' "$CONFIG"
yq "${YQ_INPLACE[@]}" '.model.parameter_count = 9000000000' "$CONFIG"

# Context — user spec: 32768 (2x buffer of 16k max response; bump to 262144 if input+response >= 131072)
yq "${YQ_INPLACE[@]}" '.context.size = 32768' "$CONFIG"
yq "${YQ_INPLACE[@]}" '.context.batch_size = 512' "$CONFIG"
yq "${YQ_INPLACE[@]}" '.context.ubatch_size = 512' "$CONFIG"

# Hardware — user spec: gpu offload 0, cpu threads 5
yq "${YQ_INPLACE[@]}" '.hardware.threads = 5' "$CONFIG"
yq "${YQ_INPLACE[@]}" '.hardware.gpu_layers = 0' "$CONFIG"
yq "${YQ_INPLACE[@]}" '.hardware.use_mmap = true' "$CONFIG"
yq "${YQ_INPLACE[@]}" '.hardware.flash_attn = true' "$CONFIG"

# Sampling
yq "${YQ_INPLACE[@]}" '.sampling.temperature = 0.7' "$CONFIG"
yq "${YQ_INPLACE[@]}" '.sampling.top_p = 0.95' "$CONFIG"
yq "${YQ_INPLACE[@]}" '.sampling.top_k = 40' "$CONFIG"

# Server — user spec: max concurrent predictions 1
yq "${YQ_INPLACE[@]}" '.server.parallel = false' "$CONFIG"
yq "${YQ_INPLACE[@]}" '.server.max_slots = 1' "$CONFIG"
yq "${YQ_INPLACE[@]}" '.server.cont_batching = false' "$CONFIG"
yq "${YQ_INPLACE[@]}" '.server.cache_prompt = false' "$CONFIG"

# KV cache — user spec: Q8_0 (Vulkan supports as of llama.cpp PR #20797)
yq "${YQ_INPLACE[@]}" '.cache.cache_type_k = "q8_0"' "$CONFIG"
yq "${YQ_INPLACE[@]}" '.cache.cache_type_v = "q8_0"' "$CONFIG"

echo
echo "=== Updated config ($CONFIG) ==="
cat "$CONFIG"

echo
echo "=== Verification ==="
echo "context.size:        $(yq -r '.context.size' "$CONFIG")  (target: 32768)"
echo "hardware.threads:    $(yq -r '.hardware.threads' "$CONFIG")  (target: 5)"
echo "hardware.gpu_layers: $(yq -r '.hardware.gpu_layers' "$CONFIG")  (target: 0)"
echo "server.max_slots:    $(yq -r '.server.max_slots' "$CONFIG")  (target: 1)"
echo "cache.cache_type_k:  $(yq -r '.cache.cache_type_k' "$CONFIG")  (target: q8_0)"
echo "cache.cache_type_v:  $(yq -r '.cache.cache_type_v' "$CONFIG")  (target: q8_0)"
echo "model.path:          $(yq -r '.model.path' "$CONFIG")"
