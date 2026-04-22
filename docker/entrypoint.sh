#!/bin/bash
set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

log_info() {
    echo -e "${GREEN}[INFO]${NC} $1"
}

log_warn() {
    echo -e "${YELLOW}[WARN]${NC} $1"
}

log_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

log_debug() {
    if [ "${LLAMA_DEBUG:-0}" = "1" ]; then
        echo -e "\033[0;36m[DEBUG]${NC} $1"
    fi
}

# Function to download model from HuggingFace
download_model() {
    local model_path="$1"
    local repo="$2"
    local filename="$3"
    local branch="${4:-main}"
    local expected_sha256="${5:-}"

    # If model already exists, verify checksum if provided
    if [ -f "$model_path" ]; then
        if [ -n "$expected_sha256" ]; then
            log_info "Verifying existing model checksum..."
            local actual_sha256=$(sha256sum "$model_path" | awk '{print $1}')
            if [ "$actual_sha256" != "$expected_sha256" ]; then
                log_error "Checksum mismatch for existing model: $model_path"
                log_error "Expected: $expected_sha256"
                log_error "Actual: $actual_sha256"
                log_error "Re-downloading model..."
                rm -f "$model_path"
            else
                log_info "Model exists and checksum verified: $model_path"
                return 0
            fi
        else
            log_info "Model already exists: $model_path"
            return 0
        fi
    fi

    log_info "Downloading model: $repo/$filename"
    log_info "This may take a while depending on your network speed..."

    # Create directory for model
    mkdir -p "$(dirname "$model_path")"

    local tmp_dir="/tmp/hf_download_$$"
    mkdir -p "$tmp_dir"

    local max_retries=3
    local retry_delay=5
    local attempt=1

    while [ $attempt -le $max_retries ]; do
        log_info "Download attempt $attempt of $max_retries..."

        if command -v hf &> /dev/null; then
            if [ -n "$HUGGING_FACE_HUB_TOKEN" ]; then
                export HUGGING_FACE_HUB_TOKEN
            elif [ -n "$HF_TOKEN" ]; then
                export HUGGING_FACE_HUB_TOKEN="$HF_TOKEN"
            fi

            local hf_opts="--repo-type model --local-dir $tmp_dir"
            if [ -n "$branch" ]; then
                hf_opts="$hf_opts --revision $branch"
            fi
            if [ -n "$HUGGING_FACE_HUB_TOKEN" ]; then
                hf_opts="$hf_opts --token $HUGGING_FACE_HUB_TOKEN"
            fi
            if timeout 300 hf download \
                $hf_opts \
                "$repo" \
                "$filename"; then
                if [ -n "$expected_sha256" ]; then
                    log_info "Verifying checksum..."
                    local actual_sha256=$(sha256sum "$tmp_dir/$filename" | awk '{print $1}')
                    if [ "$actual_sha256" != "$expected_sha256" ]; then
                        log_error "Checksum verification failed!"
                        log_error "Expected: $expected_sha256"
                        log_error "Actual: $actual_sha256"
                        rm -rf "$tmp_dir"
                        return 1
                    else
                        log_info "Checksum verified successfully"
                    fi
                fi

                mv "$tmp_dir/$filename" "$model_path"
                rm -rf "$tmp_dir"
                log_info "Model downloaded successfully: $model_path"
                return 0
            else
                local exit_code=$?
                if [ $exit_code -eq 124 ]; then
                    log_error "Download timeout after 300 seconds"
                else
                    log_error "Download failed with exit code: $exit_code"
                fi

                if [ $attempt -lt $max_retries ]; then
                    log_info "Retrying in $retry_delay seconds..."
                    sleep $retry_delay
                    attempt=$((attempt + 1))
                else
                    log_error "Max retries ($max_retries) exceeded"
                    rm -rf "$tmp_dir"
                    return 1
                fi
            fi
        else
            log_error "hf not found. Cannot download model."
            rm -rf "$tmp_dir"
            exit 1
        fi
    done

    rm -rf "$tmp_dir"
    return 1
}

# Function to validate YAML config
validate_config() {
    local config_path="$1"

    if [ ! -f "$config_path" ]; then
        log_warn "Config file not found: $config_path. Using defaults."
        return 0
    fi

    log_info "Validating config: $config_path"

    # Check if yq is available (optional)
    if command -v yq &> /dev/null; then
        # Validate YAML syntax
        if ! yq eval '.' "$config_path" > /dev/null 2>&1; then
            log_error "Invalid YAML syntax in: $config_path"
            exit 1
        fi

        log_info "Config is valid"
    else
        log_warn "yq not found. Skipping detailed validation."
    fi
}

# Function to translate YAML config to environment variables
translate_config() {
    local config_path="$1"

    if [ ! -f "$config_path" ]; then
        log_warn "Config file not found. Using defaults."
        return 0
    fi

    log_info "Translating config to environment variables..."

    # Helper function to get YAML value
    get_yaml_value() {
        local path="$1"
        local default="$2"

        if command -v yq &> /dev/null; then
            value=$(yq eval "$path" "$config_path" 2>/dev/null || echo "$default")
            echo "$value"
        else
            # Fallback: use grep (less reliable)
            echo "$default"
        fi
    }

    # Model path (required)
    model_path=$(get_yaml_value ".model.path" "/models/model.gguf")
    export LLAMA_ARG_MODEL_PATH="$model_path"

    # Context size (default from llama.cpp: 2048)
    ctx_size=$(get_yaml_value ".context.size" "2048")
    export LLAMA_ARG_CTX_SIZE="$ctx_size"

    # Batch size (default from llama.cpp: 2048)
    batch_size=$(get_yaml_value ".context.batch_size" "2048")
    export LLAMA_ARG_BATCH_SIZE="$batch_size"

    # Micro-batch size (default from llama.cpp: 512)
    ubatch_size=$(get_yaml_value ".context.ubatch_size" "512")
    export LLAMA_ARG_UBATCH_SIZE="$ubatch_size"

    # CPU threads (default from llama.cpp: -1 = auto)
    n_threads=$(get_yaml_value ".hardware.threads" "4")
    if [ "$n_threads" -gt 0 ]; then
        export LLAMA_ARG_N_THREADS="$n_threads"
    fi

    # GPU layers (default from llama.cpp: auto)
    n_gpu_layers=$(get_yaml_value ".hardware.gpu_layers" "999")
    export LLAMA_ARG_N_GPU_LAYERS="$n_gpu_layers"

    # Use mmap (default from llama.cpp: true)
    use_mmap=$(get_yaml_value ".hardware.use_mmap" "true")
    if [ "$use_mmap" = "false" ]; then
        export LLAMA_ARG_USE_MMAP="0"
    fi

    # Lock memory
    lock_memory=$(get_yaml_value ".hardware.lock_memory" "false")
    if [ "$lock_memory" = "true" ]; then
        export LLAMA_ARG_LOCK_MEMORY="1"
    fi

    # Sampling temperature (default from llama.cpp: 0.80)
    temp=$(get_yaml_value ".sampling.temperature" "0.80")
    export LLAMA_ARG_TEMP="$temp"

    # Top-P (default from llama.cpp: 0.95)
    top_p=$(get_yaml_value ".sampling.top_p" "0.95")
    export LLAMA_ARG_TOP_P="$top_p"

    # Top-K (default from llama.cpp: 40)
    top_k=$(get_yaml_value ".sampling.top_k" "40")
    if [ "$top_k" -gt 0 ]; then
        export LLAMA_ARG_TOP_K="$top_k"
    fi

    # Min-P (default from llama.cpp: 0.05)
    min_p=$(get_yaml_value ".sampling.min_p" "0.05")
    if [ "$min_p" != "null" ] && [ "$min_p" != "" ]; then
        export LLAMA_ARG_MIN_P="$min_p"
    fi

    # Typical-P
    typical_p=$(get_yaml_value ".sampling.typical_p" "1.0")
    if [ "$typical_p" != "null" ] && [ "$typical_p" != "1.0" ]; then
        export LLAMA_ARG_TYPICAL_P="$typical_p"
    fi

    # Repeat penalty (default from llama.cpp: 1.00)
    repeat_penalty=$(get_yaml_value ".sampling.repeat_penalty" "1.00")
    if [ "$repeat_penalty" != "1.0" ]; then
        export LLAMA_ARG_REPEAT_PENALTY="$repeat_penalty"
    fi

    # Presence penalty
    presence_penalty=$(get_yaml_value ".sampling.presence_penalty" "null")
    if [ "$presence_penalty" != "null" ] && [ "$presence_penalty" != "" ]; then
        export LLAMA_ARG_PRESENCE_PENALTY="$presence_penalty"
    fi

    # Frequency penalty
    frequency_penalty=$(get_yaml_value ".sampling.frequency_penalty" "null")
    if [ "$frequency_penalty" != "null" ] && [ "$frequency_penalty" != "" ]; then
        export LLAMA_ARG_FREQUENCY_PENALTY="$frequency_penalty"
    fi

    # Repeat last N
    repeat_last_n=$(get_yaml_value ".sampling.repeat_last_n" "64")
    if [ "$repeat_last_n" -gt 0 ]; then
        export LLAMA_ARG_REPEAT_LAST_N="$repeat_last_n"
    fi

    # Seed
    seed=$(get_yaml_value ".sampling.seed" "0")
    # Handle null value (yq returns "null" string)
    if [ "$seed" != "null" ] && [ "$seed" -gt 0 ]; then
        export LLAMA_ARG_SEED="$seed"
    fi

    # Max tokens
    n_predict=$(get_yaml_value ".sampling.max_tokens" "512")
    export LLAMA_ARG_N_PREDICT="$n_predict"

    # Server host (default from llama.cpp: 127.0.0.1)
    # SECURITY: Default to 127.0.0.1 for production. Use 0.0.0.0 only for development.
    host=$(get_yaml_value ".server.host" "127.0.0.1")
    export LLAMA_ARG_HOST="$host"

    # Server port (default from llama.cpp: 8080)
    port=$(get_yaml_value ".server.port" "8080")
    export LLAMA_ARG_PORT="$port"

    # Parallel processing
    parallel=$(get_yaml_value ".server.parallel" "false")
    if [ "$parallel" = "true" ]; then
        export LLAMA_ARG_PARALLEL="1"
    fi

    # Timeout (default from llama.cpp: 600)
    timeout=$(get_yaml_value ".server.timeout" "600")
    export LLAMA_ARG_TIMEOUT="$timeout"

    # Max slots
    max_slots=$(get_yaml_value ".server.max_slots" "8")
    if [ "$max_slots" -gt 0 ]; then
        export LLAMA_ARG_N_SLOT="$max_slots"
    fi

    # Metrics endpoint
    metrics=$(get_yaml_value ".server.metrics" "true")
    if [ "$metrics" = "true" ]; then
        export LLAMA_ARG_METRICS="1"
    fi

    # Slots endpoint
    slots_endpoint=$(get_yaml_value ".server.slots_endpoint" "true")
    if [ "$slots_endpoint" = "true" ]; then
        export LLAMA_ARG_SLOTS_ENDPOINT="1"
    fi

    # Cache type
    cache_type_k=$(get_yaml_value ".cache.cache_type_k" "f16")
    export LLAMA_ARG_CACHE_TYPE_K="$cache_type_k"

    cache_type_v=$(get_yaml_value ".cache.cache_type_v" "f16")
    export LLAMA_ARG_CACHE_TYPE_V="$cache_type_v"

    # Log level (0=trace, 1=debug, 2=info, 3=warn, 4=error)
    log_level=$(get_yaml_value ".features.log_level" "info")
    case "$log_level" in
        trace) export LLAMA_ARG_LOG_LEVEL="0" ;;
        debug) export LLAMA_ARG_LOG_LEVEL="1" ;;
        info) export LLAMA_ARG_LOG_LEVEL="2" ;;
        warn) export LLAMA_ARG_LOG_LEVEL="3" ;;
        error) export LLAMA_ARG_LOG_LEVEL="4" ;;
    esac

    # Profiling
    profiling=$(get_yaml_value ".features.profiling" "false")
    if [ "$profiling" = "true" ]; then
        export LLAMA_ARG_PROFILING="1"
    fi

    # Verbose
    verbose=$(get_yaml_value ".features.verbose" "false")
    if [ "$verbose" = "true" ]; then
        export LLAMA_ARG_VERBOSE="1"
    fi

    # Vulkan settings (GGML_VK_* prefix, not LLAMA_ARG_*)
    visible_devices=$(get_yaml_value ".vulkan.visible_devices" "0")
    export GGML_VK_VISIBLE_DEVICES="$visible_devices"

    force_max_allocation=$(get_yaml_value ".vulkan.force_max_allocation" "0")
    if [ "$force_max_allocation" != "0" ]; then
        # Convert MB to bytes
        export GGML_VK_FORCE_MAX_ALLOCATION_SIZE="$((force_max_allocation * 1024 * 1024))"
    fi

    disable_debug=$(get_yaml_value ".vulkan.disable_debug" "true")
    if [ "$disable_debug" = "true" ]; then
        export GGML_VK_DISABLE_DEBUG="1"
    fi

    enable_validation=$(get_yaml_value ".vulkan.enable_validation" "false")
    if [ "$enable_validation" = "true" ]; then
        export GGML_VK_ENABLE_VALIDATION="1"
    fi

    log_info "Config translation complete"
    log_debug "=== Resolved Environment ==="
    log_debug "  Model: $LLAMA_ARG_MODEL_PATH"
    log_debug "  Context: $LLAMA_ARG_CTX_SIZE, Batch: $LLAMA_ARG_BATCH_SIZE, UBatch: $LLAMA_ARG_UBATCH_SIZE"
    log_debug "  Threads: $LLAMA_ARG_N_THREADS, GPU layers: $LLAMA_ARG_N_GPU_LAYERS"
    log_debug "  Temp: $LLAMA_ARG_TEMP, Top-P: $LLAMA_ARG_TOP_P, Top-K: $LLAMA_ARG_TOP_K"
    log_debug "  Host: $LLAMA_ARG_HOST, Port: $LLAMA_ARG_PORT, Timeout: $LLAMA_ARG_TIMEOUT"
    log_debug "  Vulkan device: $GGML_VK_VISIBLE_DEVICES"
    log_debug "============================"
}

# Function to validate Vulkan GPU
validate_vulkan() {
    log_info "Validating Vulkan GPU..."

    if [ -z "$GGML_VK_VISIBLE_DEVICES" ]; then
        export GGML_VK_VISIBLE_DEVICES="0"
    fi

    # Try to run llama-cli to check Vulkan
    if command -v llama-cli &> /dev/null; then
        if ! llama-cli --help 2>&1 | grep -q "vulkan"; then
            log_warn "Vulkan support not found in llama-cli. Running in CPU-only mode."
            export LLAMA_ARG_N_GPU_LAYERS="0"
        else
            log_info "Vulkan support detected. GPU: $GGML_VK_VISIBLE_DEVICES"
        fi
    else
        log_warn "llama-cli not found. Skipping Vulkan validation."
    fi
}

# Function to start llama-server
start_server() {
    log_info "Starting llama-server..."

    # Construct command arguments
    local server_args=""

    # Model path (required)
    if [ -z "$LLAMA_ARG_MODEL_PATH" ]; then
        log_error "Model path not set. Set LLAMA_ARG_MODEL_PATH or provide config."
        exit 1
    fi
    server_args="$server_args -m $LLAMA_ARG_MODEL_PATH"

    # Context size
    if [ -n "$LLAMA_ARG_CTX_SIZE" ]; then
        server_args="$server_args -c $LLAMA_ARG_CTX_SIZE"
    fi

    # Batch size
    if [ -n "$LLAMA_ARG_BATCH_SIZE" ]; then
        server_args="$server_args -b $LLAMA_ARG_BATCH_SIZE"
    fi

    # Micro-batch size
    if [ -n "$LLAMA_ARG_UBATCH_SIZE" ]; then
        server_args="$server_args -ub $LLAMA_ARG_UBATCH_SIZE"
    fi

    # CPU threads
    if [ -n "$LLAMA_ARG_N_THREADS" ] && [ "$LLAMA_ARG_N_THREADS" != "0" ]; then
        server_args="$server_args -t $LLAMA_ARG_N_THREADS"
    fi

    # GPU layers
    if [ -n "$LLAMA_ARG_N_GPU_LAYERS" ] && [ "$LLAMA_ARG_N_GPU_LAYERS" != "0" ]; then
        server_args="$server_args -ngl $LLAMA_ARG_N_GPU_LAYERS"
    fi

    # Sampling parameters
    if [ -n "$LLAMA_ARG_TEMP" ]; then
        server_args="$server_args --temp $LLAMA_ARG_TEMP"
    fi

    if [ -n "$LLAMA_ARG_TOP_P" ]; then
        server_args="$server_args --top-p $LLAMA_ARG_TOP_P"
    fi

    if [ -n "$LLAMA_ARG_TOP_K" ]; then
        server_args="$server_args --top-k $LLAMA_ARG_TOP_K"
    fi

    if [ -n "$LLAMA_ARG_MIN_P" ]; then
        server_args="$server_args --min-p $LLAMA_ARG_MIN_P"
    fi

    if [ -n "$LLAMA_ARG_TYPICAL_P" ]; then
        server_args="$server_args --typical-p $LLAMA_ARG_TYPICAL_P"
    fi

    if [ -n "$LLAMA_ARG_REPEAT_PENALTY" ]; then
        server_args="$server_args --repeat-penalty $LLAMA_ARG_REPEAT_PENALTY"
    fi

    if [ -n "$LLAMA_ARG_PRESENCE_PENALTY" ]; then
        server_args="$server_args --presence-penalty $LLAMA_ARG_PRESENCE_PENALTY"
    fi

    if [ -n "$LLAMA_ARG_FREQUENCY_PENALTY" ]; then
        server_args="$server_args --frequency-penalty $LLAMA_ARG_FREQUENCY_PENALTY"
    fi

    if [ -n "$LLAMA_ARG_REPEAT_LAST_N" ]; then
        server_args="$server_args --repeat-last-n $LLAMA_ARG_REPEAT_LAST_N"
    fi

    if [ -n "$LLAMA_ARG_SEED" ]; then
        server_args="$server_args --seed $LLAMA_ARG_SEED"
    fi

    if [ -n "$LLAMA_ARG_N_PREDICT" ]; then
        server_args="$server_args -n $LLAMA_ARG_N_PREDICT"
    fi

    # Server parameters
    if [ -n "$LLAMA_ARG_HOST" ]; then
        server_args="$server_args --host $LLAMA_ARG_HOST"
    fi

    if [ -n "$LLAMA_ARG_PORT" ]; then
        server_args="$server_args --port $LLAMA_ARG_PORT"
    fi

    if [ -n "$LLAMA_ARG_N_SLOT" ]; then
        server_args="$server_args --parallel $LLAMA_ARG_N_SLOT"
    elif [ -n "$LLAMA_ARG_PARALLEL" ]; then
        server_args="$server_args --parallel -1"
    fi

    if [ -n "$LLAMA_ARG_TIMEOUT" ]; then
        server_args="$server_args --timeout $LLAMA_ARG_TIMEOUT"
    fi

    # Cache parameters
    if [ -n "$LLAMA_ARG_CACHE_TYPE_K" ]; then
        server_args="$server_args --cache-type-k $LLAMA_ARG_CACHE_TYPE_K"
    fi

    if [ -n "$LLAMA_ARG_CACHE_TYPE_V" ]; then
        server_args="$server_args --cache-type-v $LLAMA_ARG_CACHE_TYPE_V"
    fi

    # Metrics
    if [ -n "$LLAMA_ARG_METRICS" ]; then
        server_args="$server_args --metrics"
    fi

    # Slots endpoint
    if [ -n "$LLAMA_ARG_SLOTS_ENDPOINT" ]; then
        server_args="$server_args --slots"
    fi

    log_info "Server arguments: $server_args"
    log_debug "Full command: /usr/local/bin/llama-server $server_args"

    # Start server with tini (already in ENTRYPOINT)
    exec /usr/local/bin/llama-server $server_args
}

# Graceful shutdown handler
shutdown_handler() {
    log_info "Received shutdown signal. Stopping server gracefully..."

    # Unload model via API if available
    if [ -n "$LLAMA_ARG_HOST" ] && [ -n "$LLAMA_ARG_PORT" ]; then
        local server_url="http://$LLAMA_ARG_HOST:$LLAMA_ARG_PORT"
        curl -X POST "$server_url/models/unload" 2>/dev/null || true
    fi

    log_info "Server stopped. Exiting."
    exit 0
}

# Register signal handlers
trap shutdown_handler SIGTERM SIGINT

# Main entry point
main() {
    log_info "Whitt LLM Server Entrypoint"
    log_info "================================"

    # Config path
    local config_path="/config/config.yml"

    # If no config provided, try to use default
    if [ ! -f "$config_path" ] && [ ! -f "/config/config.yaml" ]; then
        log_warn "No config file found in /config. Using defaults."
    elif [ ! -f "$config_path" ] && [ -f "/config/config.yaml" ]; then
        config_path="/config/config.yaml"
    fi

    # Validate config
    validate_config "$config_path"

    # Translate config to env vars
    translate_config "$config_path"

    # Download model if specified in config
    if command -v yq &> /dev/null; then
        repo=$(yq eval '.model.huggingface.repo // ""' "$config_path" 2>/dev/null || echo "")
        filename=$(yq eval '.model.huggingface.filename // ""' "$config_path" 2>/dev/null || echo "")
        branch=$(yq eval '.model.huggingface.branch // "main"' "$config_path" 2>/dev/null || echo "main")
        sha256=$(yq eval '.model.huggingface.sha256 // ""' "$config_path" 2>/dev/null || echo "")

        if [ -n "$repo" ] && [ -n "$filename" ]; then
            download_model "$LLAMA_ARG_MODEL_PATH" "$repo" "$filename" "$branch" "$sha256"
        fi
    fi

    # Check if model exists
    if [ ! -f "$LLAMA_ARG_MODEL_PATH" ]; then
        log_error "Model not found: $LLAMA_ARG_MODEL_PATH"
        log_error "Please provide a valid model path or configure HuggingFace download."
        exit 1
    fi

    # Validate Vulkan GPU
    validate_vulkan

    # Start server
    start_server
}

# Run main function
main "$@"
