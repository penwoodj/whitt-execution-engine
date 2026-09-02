#!/usr/bin/env bash
set -euo pipefail

repo_root=$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)

python3 - "$repo_root/docker/docker-compose.yml" <<'PY'
from pathlib import Path
import sys

import yaml

compose = yaml.safe_load(Path(sys.argv[1]).read_text())
environment = compose["services"]["llama-server"]["environment"]
expected = (
    "LLAMA_ARG_CTX_SIZE",
    "LLAMA_ARG_BATCH_SIZE",
    "LLAMA_ARG_UBATCH_SIZE",
    "LLAMA_ARG_CACHE_TYPE_K",
    "LLAMA_ARG_CACHE_TYPE_V",
    "LLAMA_ARG_N_GPU_LAYERS",
    "LLAMA_ARG_N_THREADS",
    "LLAMA_ARG_USE_MMAP",
    "LLAMA_ARG_FLASH_ATTN",
    "LLAMA_ARG_CONT_BATCHING",
    "LLAMA_ARG_NO_CACHE_PROMPT",
    "LLAMA_ARG_PARALLEL",
)

for name in expected:
    assert environment[name] == f"${{{name}:-}}", name
PY
