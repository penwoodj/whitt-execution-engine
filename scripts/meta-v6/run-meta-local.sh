#!/bin/bash
# scripts/meta-v6/run-meta-local.sh
# One command: prompt → generated workflow.yml → executed agentic workflow.
#
#   SW1-SW5 generate the workflow from your prompt, post-processors repair it,
#   then the engine executes it — all against a local model server, with live
#   progress on stderr. Backend defaults to LM Studio (localhost:1234).
#
# Usage:
#   run-meta-local.sh <prompt-file> [label]
#   run-meta-local.sh "build me a rust CLI that ..." [label]   (inline prompt)
#   echo "prompt text" | run-meta-local.sh - [label]           (stdin)
#   run-meta-local.sh --from <workflow.yml> [label]            (skip generation;
#                                             repair + execute an existing workflow)
#
# Env:
#   WHITT_BACKEND=lmstudio|llamacpp   (default lmstudio)
#   WHITT_LMSTUDIO_MODEL=<id>         (default qwen/qwen3.5-9b)
#   NO_COLOR=1                        disable colors
set -uo pipefail

WHITT_BACKEND="${WHITT_BACKEND:-lmstudio}"
source "$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)/env.sh"

# ── logging helpers ─────────────────────────────────────────────────
if [[ -t 2 && -z "${NO_COLOR:-}" ]]; then
  C_DIM=$'\033[2m'; C_BOLD=$'\033[1m'; C_BLUE=$'\033[34m'; C_GREEN=$'\033[32m'
  C_YELLOW=$'\033[33m'; C_RED=$'\033[31m'; C_CYAN=$'\033[36m'; C_RESET=$'\033[0m'
else
  C_DIM=""; C_BOLD=""; C_BLUE=""; C_GREEN=""; C_YELLOW=""; C_RED=""; C_CYAN=""; C_RESET=""
fi

T0=$(date +%s)
elapsed() {
  local s=$(( $(date +%s) - T0 ))
  printf "%02d:%02d:%02d" $((s/3600)) $((s%3600/60)) $((s%60))
}
log()  { echo "${C_DIM}[$(elapsed)]${C_RESET} $*" >&2; }
ok()   { log "${C_GREEN}✓${C_RESET} $*"; }
info() { log "${C_BLUE}•${C_RESET} $*"; }
warn() { log "${C_YELLOW}⚠${C_RESET} $*"; }
err()  { log "${C_RED}✗${C_RESET} $*"; }
banner() {
  echo "" >&2
  echo "${C_BOLD}${C_CYAN}── $* ──${C_RESET}" >&2
}
# status: reprint-in-place when tty, plain lines otherwise
STATUS_TTY=0; [[ -t 2 ]] && STATUS_TTY=1
LAST_STATUS=""
status() {
  [[ "$1" == "$LAST_STATUS" ]] && return
  LAST_STATUS="$1"
  if [[ $STATUS_TTY == 1 ]]; then
    printf "\r\033[2K${C_DIM}[%s]${C_RESET} %s" "$(elapsed)" "$1" >&2
  else
    log "$1"
  fi
}
status_end() { [[ $STATUS_TTY == 1 && -n "$LAST_STATUS" ]] && echo "" >&2; LAST_STATUS=""; }

WHITT_PID=""
cleanup() {
  status_end
  if [[ -n "$WHITT_PID" ]] && kill -0 "$WHITT_PID" 2>/dev/null; then
    err "interrupted — stopping engine (pid $WHITT_PID)"
    kill "$WHITT_PID" 2>/dev/null
  fi
  exit 130
}
trap cleanup INT TERM

# ── args ────────────────────────────────────────────────────────────
PROMPT_ARG="${1:?usage: $0 <prompt-file | inline-prompt-string | - | --from workflow.yml> [label]}"
FROM_WORKFLOW=""
if [[ "${PROMPT_ARG}" == "--from" ]]; then
  FROM_WORKFLOW="${2:?usage: $0 --from <workflow.yml> [label]}"
  [[ -f "${FROM_WORKFLOW}" ]] || { echo "no such workflow file: ${FROM_WORKFLOW}" >&2; exit 1; }
  LABEL="${3:-local}"
else
  LABEL="${2:-local}"
fi

RUN_DIR="${REPO}/docs/benchmarks/outputs/meta-workflow/local-runs/${LABEL}-$(date +%Y%m%d-%H%M%S)"
mkdir -p "${RUN_DIR}"

PROMPT_FILE="${RUN_DIR}/prompt.md"
if [[ -n "${FROM_WORKFLOW}" ]]; then
  : # no prompt in --from mode
elif [[ "${PROMPT_ARG}" == "-" ]]; then
  cat > "${PROMPT_FILE}"
elif [[ -f "${PROMPT_ARG}" ]]; then
  cp "${PROMPT_ARG}" "${PROMPT_FILE}"
else
  printf '%s\n' "${PROMPT_ARG}" > "${PROMPT_FILE}"
fi
[[ -n "${FROM_WORKFLOW}" || -s "${PROMPT_FILE}" ]] || { err "empty prompt"; exit 1; }

banner "whitt meta-workflow"
info "backend  ${C_BOLD}${WHITT_BACKEND}${C_RESET} ($([[ ${WHITT_BACKEND} == lmstudio ]] && echo "${WHITT_LMSTUDIO_MODEL} @ localhost:1234" || echo "llama.cpp @ localhost:8080"))"
if [[ -n "${FROM_WORKFLOW}" ]]; then
  info "source   ${FROM_WORKFLOW} (skipping generation)"
else
  info "prompt   $(head -c 96 "${PROMPT_FILE}" | tr '\n' ' ')…"
fi
info "run dir  ${RUN_DIR#${REPO}/}"

# preflight: model server reachable?
if [[ "${WHITT_BACKEND}" == "lmstudio" ]]; then
  if ! curl -s -m 5 http://localhost:1234/v1/models > /dev/null; then
    err "LM Studio not reachable on localhost:1234 — start the LM Studio server first"
    exit 1
  fi
  ok "LM Studio server reachable"
fi
if [[ ! -x "${REPO}/target/release/whitt" ]]; then
  err "engine binary missing — build with: cargo build --release --features client --bin whitt"
  exit 1
fi

# ── phase 1: generate workflow.yml from the prompt (SW1-SW5) ───────
if [[ -n "${FROM_WORKFLOW}" ]]; then
  banner "phase 1/4 · generate workflow — skipped (--from)"
  cd "${REPO}"
  META_RUN_ID="$(cat "${REPO}/.current-meta-run" 2>/dev/null || true)"
  cp "${FROM_WORKFLOW}" "${RUN_DIR}/workflow-raw.yml"
  ok "using existing workflow: ${FROM_WORKFLOW}"
else
banner "phase 1/4 · generate workflow from prompt (SW1→SW5)"

ORCH="${RUN_DIR}/meta-orchestrator.yml"
localize_workflow "${REPO}/docs/benchmarks/workflows/meta-workflow-v6.yml" "${ORCH}"
sed -e "s|docs/plans/meta-workflow-qwen35/test-prompts/real/prompt-14-task-add-true-parallel-inference-for-same-model-multi-target.md|${PROMPT_FILE}|g" \
  "${ORCH}" > "${ORCH}.tmp" && mv "${ORCH}.tmp" "${ORCH}"

cd "${REPO}"
./target/release/whitt benchmark \
  --workflow "${ORCH}" \
  --output-dir "${RUN_DIR}/meta-out" \
  $(model_flags) \
  --load-timeout 900 \
  > "${RUN_DIR}/generate.log" 2>&1 &
WHITT_PID=$!

# live SW-stage monitor: watch pipeline artifacts appear
SW_DONE=0
META_RUN_ID=""
declare -a SW_NAMES=("" "SW1 task deconstruction" "SW2 desired outputs" "SW3 agentic categorization" "SW4 yaml substructures" "SW5 assembly")
declare -a SW_MARKERS=("" "tasks.md" "outputs.md" "categories.md" "structs.md" "__generated__")
while kill -0 "$WHITT_PID" 2>/dev/null; do
  sleep 5
  [[ -z "$META_RUN_ID" ]] && META_RUN_ID="$(cat "${REPO}/.current-meta-run" 2>/dev/null || true)"
  [[ -z "$META_RUN_ID" ]] && continue
  META_DIR="${REPO}/docs/benchmarks/outputs/meta-workflow/${META_RUN_ID}"

  # completed-stage detection
  while (( SW_DONE < 5 )); do
    NEXT=$(( SW_DONE + 1 ))
    MARKER="${SW_MARKERS[$NEXT]}"
    if [[ "$MARKER" == "__generated__" ]]; then
      [[ -s "${META_DIR}/meta/generated-workflow.yml" ]] || break
    else
      [[ -s "${META_DIR}/input/${MARKER}" ]] || break
    fi
    status_end
    ok "${SW_NAMES[$NEXT]} complete"
    SW_DONE=$NEXT
  done
  (( SW_DONE >= 5 )) && continue

  # in-flight activity: newest artifact of the running sub-workflow
  CUR=$(( SW_DONE + 1 ))
  SW_DIR=$(ls -td "${REPO}/docs/benchmarks/outputs/meta-workflow/meta-${META_RUN_ID}-sw${CUR}-"* 2>/dev/null | head -1)
  if [[ -n "$SW_DIR" && -d "${SW_DIR}/sw${CUR}" ]]; then
    NEWEST=$(ls -t "${SW_DIR}/sw${CUR}/" 2>/dev/null | head -1)
    status "${SW_NAMES[$CUR]} — generating ${NEWEST:-…}"
  else
    status "${SW_NAMES[$CUR]} — starting"
  fi
done
wait "$WHITT_PID"; GEN_EXIT=$?
WHITT_PID=""
status_end

META_RUN_ID="$(cat "${REPO}/.current-meta-run" 2>/dev/null || true)"
GENERATED="${REPO}/docs/benchmarks/outputs/meta-workflow/${META_RUN_ID}/meta/generated-workflow.yml"
if [[ ! -s "${GENERATED}" ]]; then
  err "generation produced no workflow (exit=${GEN_EXIT})"
  err "last log lines (${RUN_DIR#${REPO}/}/generate.log):"
  tail -15 "${RUN_DIR}/generate.log" | sed 's/^/    /' >&2
  exit 2
fi
cp "${GENERATED}" "${RUN_DIR}/workflow-raw.yml"
ok "workflow generated: $(wc -c < "${GENERATED}" | tr -d ' ') bytes (engine exit=${GEN_EXIT})"
fi  # end --from / generation branch

# ── phase 2: post-process into runnable workflow.yml ────────────────
banner "phase 2/4 · repair + finalize workflow.yml"

WF="${RUN_DIR}/workflow.yml"
python3 - "${RUN_DIR}/workflow-raw.yml" "${WF}" <<'PYEOF'
import re, sys
content = open(sys.argv[1]).read()
content = re.sub(r'^```(?:yaml|yml)?\s*\n', '', content, flags=re.MULTILINE)
content = re.sub(r'\n```\s*$', '', content, flags=re.MULTILINE)
open(sys.argv[2], 'w').write(content)
PYEOF
info "markdown fences stripped"

if python3 "${REPO}/scripts/meta-v6/fix-yaml.py" "${WF}" > "${RUN_DIR}/fix-yaml.log" 2>&1; then
  ok "fix-yaml normalization passed"
else
  warn "fix-yaml reported issues (see fix-yaml.log) — continuing"
fi
python3 "${REPO}/scripts/meta-v6/inject-shell-hooks.py" "${WF}" > "${RUN_DIR}/inject.log" 2>&1 \
  && ok "shell hooks injected" \
  || warn "hook injection reported issues (see inject.log) — continuing"

if python3 -c "import yaml,sys; yaml.safe_load(open(sys.argv[1]))" "${WF}" 2>/dev/null; then
  ok "workflow.yml is valid YAML"
else
  warn "python YAML parser rejects workflow.yml — engine parser may still accept it"
fi

TOTAL_STEPS=$(grep -cE '^    step_[A-Za-z0-9_]+:' "${WF}" 2>/dev/null || echo 0)
info "workflow.yml ready: ${C_BOLD}${TOTAL_STEPS} steps${C_RESET} → ${RUN_DIR#${REPO}/}/workflow.yml"

# ── phase 3: execute the generated workflow ────────────────────────
banner "phase 3/4 · execute generated workflow (${TOTAL_STEPS} steps)"

EXEC_DIR="${RUN_DIR}/exec"
rm -rf "${EXEC_DIR}" && mkdir -p "${EXEC_DIR}"
WHITT_OUTPUT_DIR="${EXEC_DIR}" ./target/release/whitt benchmark \
  --workflow "${WF}" \
  --output-dir "${EXEC_DIR}" \
  $(model_flags) \
  --load-timeout 900 \
  > "${RUN_DIR}/execute.log" 2>&1 &
WHITT_PID=$!

DONE_STEPS=0
while kill -0 "$WHITT_PID" 2>/dev/null; do
  sleep 5
  CUR_STEP=$(grep -a "executing step" "${RUN_DIR}/execute.log" 2>/dev/null | tail -1 | grep -oE 'step_[A-Za-z0-9_]+' | head -1)
  N=$(grep -a "executing step" "${RUN_DIR}/execute.log" 2>/dev/null | grep -oE 'step_[A-Za-z0-9_]+' | sort -u | wc -l | tr -d ' ')
  if [[ -n "$CUR_STEP" ]]; then
    if (( N > DONE_STEPS )); then
      DONE_STEPS=$N
    fi
    status "step ${DONE_STEPS}/${TOTAL_STEPS} · ${CUR_STEP}"
  else
    status "engine starting…"
  fi
done
wait "$WHITT_PID"; EXEC_EXIT=$?
WHITT_PID=""
status_end

SUCCEEDED=$(grep -a "^Successful:" "${RUN_DIR}/execute.log" | tail -1 | grep -oE '[0-9]+' || echo "?")
FAILED=$(grep -a "^Failed:" "${RUN_DIR}/execute.log" | tail -1 | grep -oE '[0-9]+' || echo "?")
if [[ "$EXEC_EXIT" == "0" ]]; then
  ok "execution finished: ${C_BOLD}${SUCCEEDED} succeeded, ${FAILED} failed${C_RESET}"
else
  err "execution exited ${EXEC_EXIT}: ${SUCCEEDED} succeeded, ${FAILED} failed — see ${RUN_DIR#${REPO}/}/execute.log"
fi

# locate deliverables (largest markdown/html output = primary)
SEARCH_DIRS=("${EXEC_DIR}")
[[ -n "${FROM_WORKFLOW}" || -z "${META_RUN_ID}" ]] || SEARCH_DIRS+=("${REPO}/docs/benchmarks/outputs/meta-workflow/${META_RUN_ID}")
DELIVERABLES=$(find "${SEARCH_DIRS[@]}" \
  -type f \( -path '*deliverable*' -o -path '*outputs/*.md' -o -path '*outputs/*.html' \) 2>/dev/null | sort -u | head -10)
PRIMARY=""
if [[ -n "$DELIVERABLES" ]]; then
  PRIMARY=$(while IFS= read -r f; do printf '%s %s\n' "$(wc -c < "$f" | tr -d ' ')" "$f"; done <<< "$DELIVERABLES" | sort -rn | head -1 | cut -d' ' -f2-)
fi

# ── phase 4: verify the deliverable achieves the prompt ────────────
banner "phase 4/4 · verify deliverable against prompt"
VERIFY_EXIT=0
if [[ -z "$PRIMARY" ]]; then
  err "no deliverable produced — nothing to verify"
  VERIFY_EXIT=1
elif [[ ! -s "${PROMPT_FILE}" ]]; then
  warn "no prompt available (--from mode) — running machine gates + code checks only"
  printf 'unknown request\n' > "${RUN_DIR}/prompt.md"
  python3 "${REPO}/scripts/meta-v6/verify-deliverable.py" "${RUN_DIR}/prompt.md" "$PRIMARY" \
    --skip-judge --report "${RUN_DIR}/verify.json" 2>&1 | sed 's/^/  /' >&2
  VERIFY_EXIT=${PIPESTATUS[0]}
else
  info "judging ${PRIMARY#${REPO}/} with ${WHITT_LMSTUDIO_MODEL}"
  python3 "${REPO}/scripts/meta-v6/verify-deliverable.py" "${PROMPT_FILE}" "$PRIMARY" \
    --report "${RUN_DIR}/verify.json" 2>&1 | sed 's/^/  /' >&2
  VERIFY_EXIT=${PIPESTATUS[0]}
fi

# ── summary ─────────────────────────────────────────────────────────
banner "summary"
if [[ "$EXEC_EXIT" == "0" && "$VERIFY_EXIT" == "0" ]]; then
  ok "${C_BOLD}VERIFIED${C_RESET} — workflow executed and deliverable achieves the prompt (total $(elapsed))"
elif [[ "$EXEC_EXIT" == "0" ]]; then
  err "executed but ${C_BOLD}NOT VERIFIED${C_RESET} (verify exit=${VERIFY_EXIT}) — see verify output above / verify.json"
else
  err "execution failed (exit=${EXEC_EXIT})"
fi
info "workflow.yml    ${RUN_DIR#${REPO}/}/workflow.yml"
if [[ -n "$DELIVERABLES" ]]; then
  info "deliverables:"
  while IFS= read -r f; do
    MARK=""
    [[ "$f" == "$PRIMARY" ]] && MARK=" ${C_BOLD}(primary)${C_RESET}"
    echo "    ${C_GREEN}→${C_RESET} ${f#${REPO}/} ${C_DIM}($(wc -c < "$f" | tr -d ' ') bytes)${C_RESET}${MARK}" >&2
  done <<< "$DELIVERABLES"
else
  warn "no deliverable files found — inspect ${RUN_DIR#${REPO}/}/exec/"
fi
info "logs            generate.log · execute.log · fix-yaml.log · inject.log · verify.json in ${RUN_DIR#${REPO}/}"

[[ "$EXEC_EXIT" != "0" ]] && exit "${EXEC_EXIT}"
exit "${VERIFY_EXIT}"
