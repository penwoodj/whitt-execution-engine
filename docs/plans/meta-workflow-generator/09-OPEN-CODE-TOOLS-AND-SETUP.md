# 09 — OpenCode Tools & Setup

Tooling research, install steps, and environment preparation for executing this plan.

---

## 1. Already Available (Repo-Native)

| Tool | Purpose | Location |
|------|---------|----------|
| `cargo` | Rust build/test/clippy | `Cargo.toml` |
| `python3` | Script execution | system |
| `yq` | YAML parsing in Docker entrypoint | Docker image |
| `hf` (huggingface_hub) | Model download | `pip install huggingface_hub` |
| `docker compose` | Container orchestration | system |
| `whitt` | Execution engine CLI | `cargo build --release` → `target/release/whitt` |
| `jq` | JSON parsing for API responses | system |

### Repo Scripts (Already Present)

| Script | Purpose |
|--------|---------|
| `scripts/generate-workflow.sh` | `__RUN_ID__` + `__TASK_PLACEHOLDER__` sed substitution |
| `scripts/validate-yaml.py` | Schema validation |
| `scripts/fix-generated-yaml.py` | Strip fences, re-indent, fix `workflow_id:` line 1 |
| `scripts/analyze-run.sh` | Parse benchmark log + output for QA |
| `scripts/validate-iteration.sh` | 8-point evidence gate |

---

## 2. Tools To Create (For v6)

| Tool | Purpose | Priority |
|------|---------|----------|
| `scripts/parse-task-list.py` | Parse SW1 initial breakdown → JSON for iterate_values (OPTIONAL — may not be needed if we hardcode task slots) | LOW |
| `scripts/check-yaml-only-blocks.py` | Verify SW4 output has zero non-YAML code blocks | HIGH |
| `scripts/analyze-meta-run.sh` | Summarize META run: per-SW pass/fail, total runtime, output sizes | MEDIUM |
| `scripts/setup-qwen35-config.sh` | Update `config.yml` with Qwen3.5-9B params | HIGH |

### `scripts/check-yaml-only-blocks.py` Spec

```python
#!/usr/bin/env python3
"""Exit non-zero if markdown file contains non-YAML fenced code blocks.

Usage: check-yaml-only-blocks.py <markdown-file>
"""
import re
import sys
from pathlib import Path

ALLOWED = {"yaml", "yml"}
pattern = re.compile(r"^```(\w*)\s*$", re.MULTILINE)

def main(path: str) -> int:
    text = Path(path).read_text()
    violations = []
    for i, line in enumerate(text.splitlines(), 1):
        m = re.match(r"^```(\w*)\s*$", line)
        if m:
            lang = m.group(1).lower()
            if lang and lang not in ALLOWED:
                violations.append((i, lang))
    if violations:
        for lineno, lang in violations:
            print(f"VIOLATION line {lineno}: ```{lang} (only ```yaml allowed)")
        return 1
    print(f"OK: {path} contains only YAML code blocks")
    return 0

if __name__ == "__main__":
    sys.exit(main(sys.argv[1]))
```

### `scripts/setup-qwen35-config.sh` Spec

```bash
#!/usr/bin/env bash
# Updates config.yml with Qwen3.5-9B settings.
# Usage: setup-qwen35-config.sh <config.yml-path>
set -euo pipefail
CONFIG="${1:-config.yml}"
yq -i '.context.size = 262144' "$CONFIG"
yq -i '.hosting.gpu_layers = 0' "$CONFIG"
yq -i '.hardware.threads = 5' "$CONFIG"
yq -i '.cache.cache_type_k = "q8_0"' "$CONFIG"
yq -i '.cache.cache_type_v = "q8_0"' "$CONFIG"
yq -i '.sampling.parallel = 1' "$CONFIG"  # if key exists
echo "Updated $CONFIG for Qwen3.5-9B"
yq '.' "$CONFIG"
```

---

## 3. Docker Configuration Update

### Step 1: Verify Qwen3.5-9B Available Locally

```bash
# Check if model exists at expected mount paths
ls -lh /models/qwen3.5-9b/ 2>/dev/null || echo "Need to download"
ls -lh /run/media/jon/data/models/ 2>/dev/null | grep -i qwen3.5 || echo "Not at external path either"
```

### Step 2: Download If Missing

```bash
# Pick a path; /models is the default mount in docker-compose.yml
hf download unsloth/Qwen3.5-9B-GGUF \
    --local-dir /models/qwen3.5-9b \
    --include "*UD-Q4_K_XL*"

# Verify
ls -lh /models/qwen3.5-9b/
# Expect: Qwen3.5-9B-UD-Q4_K_XL.gguf (~6.5 GB)
```

### Step 3: Update `config.yml`

```bash
./scripts/setup-qwen35-config.sh config.yml
```

Or manually edit `config.yml`:

```yaml
context:
  size: 262144
hosting:
  gpu_layers: 0
hardware:
  threads: 5
cache:
  cache_type_k: q8_0
  cache_type_v: q8_0
sampling:
  parallel: 1
  # other sampling defaults
```

### Step 4: Update Model List (if applicable)

The engine discovers models from `models_dir` or `model_list_file`. Ensure Qwen3.5-9B
is discoverable:

```bash
# Verify model discovery
whitt benchmark --models-dir /models --list-models 2>&1 | grep -i qwen3.5
```

Or add to model list file:

```bash
echo "qwen3.5-9b /models/qwen3.5-9b/Qwen3.5-9B-UD-Q4_K_XL.gguf" >> config/models.txt
```

### Step 5: Bring Up Docker

```bash
docker compose -f docker/docker-compose.yml down
docker compose -f docker/docker-compose.yml up -d --build
```

### Step 6: Verify Health

```bash
# Wait for healthcheck
until curl -sf http://localhost:8080/props > /dev/null; do
  echo "Waiting for llama-server..."
  sleep 5
done

# Inspect loaded config
curl -s http://localhost:8080/props | jq .

# Smoke test
curl -s http://localhost:8080/v1/chat/completions \
  -H "Content-Type: application/json" \
  -d '{
    "model": "qwen35",
    "messages": [{"role": "user", "content": "Reply with exactly: READY"}],
    "max_tokens": 10,
    "temperature": 0.0
  }' | jq .
```

Expected: HTTP 200, response contains "READY".

---

## 4. OpenCode Tooling (For Sisyphus Use)

Tools available in this environment that accelerate the work:

| Tool | Purpose | When to Use |
|------|---------|-------------|
| `task(category="deep")` | Delegate SW YAML authoring to subagent | When SW spec is clear and ready to implement |
| `task(subagent_type="explore")` | Codebase pattern discovery | When checking if a feature is wired |
| `task(subagent_type="librarian")` | External library research | When checking llama.cpp docs, schema refs |
| `task(subagent_type="oracle")` | Architecture consultation | If SW design hits a wall |
| `delegate(agent="momus")` | Plan review | Before executing each phase |
| `lsp_diagnostics` | Verify YAML/Rust diagnostics | After each edit |
| `compress` | Manage context length | When conversation gets long |

### Suggested Delegation Patterns

1. **Author SW YAML**: delegate to `deep` with full spec + constraints
2. **Run smoke test on YAML**: do it directly (`whitt benchmark`)
3. **Compare SW output to baseline**: do it directly (Sisyphus judgment)
4. **Update iteration log**: do it directly
5. **Author unit tests**: delegate to `deep` with confirmed engine behavior

---

## 5. Performance / Resource Budget

| Resource | Budget | Notes |
|----------|--------|-------|
| Disk | ≥ 30 GB free | 7 GB model + outputs + logs |
| RAM | ≥ 32 GB | Qwen3.5-9B + 256K KV cache |
| CPU | 5 threads dedicated | Set via `hardware.threads: 5` |
| GPU | Optional (Vulkan) | Layer offload = 0 (CPU-only) |
| Time per SW iteration | 15-60 min | Depends on prompt complexity |
| Time per META run | 2-4 hours | All 5 SWs chained |

---

## 6. Pre-Flight Checklist (Before Phase 3)

- [ ] Docker running, Qwen3.5-9B loaded, `/props` responds 200
- [ ] `config.yml` updated with locked params
- [ ] `cargo build --release` succeeds, `target/release/whitt` exists
- [ ] `whitt benchmark --help` shows expected CLI args
- [ ] Test prompts saved in `artifacts/dataset/`
- [ ] `scripts/check-yaml-only-blocks.py` created and executable
- [ ] `scripts/setup-qwen35-config.sh` created and executable
- [ ] Plan files all reviewed (this entire suite)
- [ ] `08-ITERATION-LOG.md` initialized

If any item fails, fix before proceeding to SW1 iteration.
