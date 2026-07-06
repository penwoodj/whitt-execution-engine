# scripts/meta-v6/env.sh
# Shared environment for META-v6 wrapper scripts. Source, don't execute.
#
# Makes the pipeline portable across machines/backends:
#   REPO                 — repo root, derived from this file's location
#                          (override with WHITT_REPO)
#   WHITT_BACKEND        — "llamacpp" (default, Docker llama.cpp on :8080)
#                          or "lmstudio" (LM Studio on :1234)
#   WHITT_LMSTUDIO_MODEL — LM Studio model id (default qwen/qwen3.5-9b)
#
# Helpers:
#   localize_workflow <in> <out> — rewrite hardcoded repo paths; for the
#       lmstudio backend also swap the provider block + model names.
#   model_flags — echoes --models-dir/--filter-name args (empty for lmstudio,
#       where the workflow's models block drives discovery).

_ENV_SH_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO="${WHITT_REPO:-$(cd "${_ENV_SH_DIR}/../.." && pwd)}"
WHITT_BACKEND="${WHITT_BACKEND:-llamacpp}"
WHITT_LMSTUDIO_MODEL="${WHITT_LMSTUDIO_MODEL:-qwen/qwen3.5-9b}"
WHITT_REPO="${REPO}"
export REPO WHITT_REPO WHITT_BACKEND WHITT_LMSTUDIO_MODEL

localize_workflow() {
    local in="$1" out="$2" tmp
    tmp="$(mktemp)"
    sed -e "s|/home/jon/code/whitt-execution-engine|${REPO}|g" "$in" > "$tmp"
    if [[ "${WHITT_BACKEND}" == "lmstudio" ]]; then
        sed -e 's/llama_cpp_with_vulkan/lmstudio/g' \
            -e 's/port: 8080/port: 1234/g' \
            -e "s|Qwen3-5-9B-Q4_K_M\.gguf|${WHITT_LMSTUDIO_MODEL}|g" \
            -e "s|Qwen3-4B-Instruct-2507-Q4_K_M\.gguf|${WHITT_LMSTUDIO_MODEL}|g" \
            "$tmp" > "${tmp}.2"
        mv "${tmp}.2" "$tmp"
    fi
    mv "$tmp" "$out"
}

model_flags() {
    if [[ "${WHITT_BACKEND}" == "lmstudio" ]]; then
        echo ""
    else
        echo "--models-dir ${REPO}/models --filter-name Qwen3-5-9B"
    fi
}
