#!/usr/bin/env bash
set -euo pipefail

repo_root=$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)
entrypoint="$repo_root/docker/entrypoint.sh"
tmp_dir=$(mktemp -d)

cleanup() {
    rm -rf "$tmp_dir"
}
trap cleanup EXIT

config_path="$tmp_dir/config.yml"
harness_path="$tmp_dir/entrypoint-functions.sh"
fake_yq_path="$tmp_dir/yq"

printf '%s\n' 'context: {}' > "$config_path"

python3 - "$entrypoint" "$harness_path" <<'PY'
from pathlib import Path
import sys

source = Path(sys.argv[1]).read_text()
source = source.rsplit('main "$@"', 1)[0]
source = source.replace(
    'exec /usr/local/bin/llama-server $server_args',
    'printf "%s\\n" "$server_args" > "$ENTRYPOINT_SERVER_ARGS_CAPTURE"',
)
Path(sys.argv[2]).write_text(source)
PY

cat > "$fake_yq_path" <<'EOF'
#!/usr/bin/env bash
case "$2" in
  .context.size) printf '%s\n' 30000 ;;
  .context.batch_size) printf '%s\n' 1024 ;;
  .context.ubatch_size) printf '%s\n' 256 ;;
  .hardware.threads) printf '%s\n' 3 ;;
  .hardware.gpu_layers) printf '%s\n' 50 ;;
  .hardware.use_mmap) printf '%s\n' false ;;
  .hardware.flash_attn) printf '%s\n' true ;;
  .cache.cache_type_k|.cache.cache_type_v) printf '%s\n' f16 ;;
  .server.cache_prompt|.server.cont_batching) printf '%s\n' true ;;
  .server.max_slots) printf '%s\n' 8 ;;
  *) printf '%s\n' null ;;
esac
EOF
chmod +x "$fake_yq_path"

PATH="$tmp_dir:$PATH" \
LLAMA_ARG_CTX_SIZE=4096 \
LLAMA_ARG_BATCH_SIZE=2048 \
LLAMA_ARG_UBATCH_SIZE=512 \
LLAMA_ARG_N_THREADS=5 \
LLAMA_ARG_N_GPU_LAYERS=99 \
LLAMA_ARG_USE_MMAP=1 \
LLAMA_ARG_CACHE_TYPE_K=q8_0 \
LLAMA_ARG_CACHE_TYPE_V=q8_0 \
LLAMA_ARG_PARALLEL=1 \
LLAMA_ARG_FLASH_ATTN=0 \
LLAMA_ARG_CONT_BATCHING=0 \
LLAMA_ARG_NO_CACHE_PROMPT=1 \
LLAMA_ARG_MODEL_PATH=/models/test.gguf \
LLAMA_ARG_MODELS_DIR= \
ENTRYPOINT_SERVER_ARGS_CAPTURE="$tmp_dir/server-args" \
bash -c '
    set -eo pipefail
    source "$1"
    test "$(command -v yq)" = "$3"
    test "$(get_yaml_value .context.size 2048 "$2")" = 30000
    translate_config "$2"
    test "$LLAMA_ARG_CTX_SIZE" = 4096
    test "$LLAMA_ARG_BATCH_SIZE" = 2048
    test "$LLAMA_ARG_UBATCH_SIZE" = 512
    test "$LLAMA_ARG_N_THREADS" = 5
    test "$LLAMA_ARG_N_GPU_LAYERS" = 99
    test "$LLAMA_ARG_USE_MMAP" = 1
    test "$LLAMA_ARG_CACHE_TYPE_K" = q8_0
    test "$LLAMA_ARG_CACHE_TYPE_V" = q8_0
    test "$LLAMA_ARG_N_SLOT" = 1
    start_server
    server_args=$(<"$ENTRYPOINT_SERVER_ARGS_CAPTURE")
    case "$server_args" in *"--flash-attn on"*|*"--cont-batching"*|*"--cache-prompt"*) exit 1 ;; esac
    case "$server_args" in *"--mmap"*"--flash-attn off"*"--no-cache-prompt"*"--cache-ram 0"*"--parallel 1"*) ;; *) exit 1 ;; esac
' bash "$harness_path" "$config_path" "$fake_yq_path"
