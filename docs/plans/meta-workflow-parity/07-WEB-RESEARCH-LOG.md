# 07 - WEB RESEARCH LOG

Per user request: "use web research pauses regularly in plan files to test assumptions and adjust implementation direction"

## ASSUMPTIONS TO TEST

### A1: Qwen3.5-9B can do tool calling
- **Status:** UNVERIFIED
- **Test:** Search Qwen3.5 tool-calling capability, function-calling support
- **If false:** Engine must wrap tool calls as text parsing, not native function calling

### A2: llama.cpp with Vulkan supports tool calling
- **Status:** UNVERIFIED
- **Test:** Search llama.cpp server tool/function calling API, Vulkan backend support
- **If false:** Must implement tool protocol at engine layer, not model layer

### A3: Workflows can use shell hooks for arbitrary file I/O
- **Status:** LIKELY TRUE (b3 confirmed shell action exists)
- **Test:** Verify shell action can: `cat file > bookmark`, `cmd > output`
- **If true:** Track B (template rewrite) is viable

### A4: Other workflow engines (n8n, temporal, airflow) solved similar problem
- **Status:** UNVERIFIED
- **Test:** Search "LLM workflow engine tool calling", "agentic workflow YAML"
- **If found:** Borrow patterns

### A5: OpenCode agent loop is reproducible as YAML workflow
- **Status:** UNVERIFIED
- **Test:** Map opencode's perceive→plan→act→verify loop to workflow steps
- **If false:** Need engine changes to support ReAct-style loops

## RESEARCH PAUSES SCHEDULED

| When | Topic | Method |
|------|-------|--------|
| End of Phase 2 (audit) | A1, A2 — model tool calling | webfetch Qwen docs, llama.cpp docs |
| Start of Cycle 2 (engine tools) | A3 — shell hook capabilities | test in current engine |
| Start of Cycle 3 (if needed) | A4 — other workflow engines | librarian agent search |
| End of Cycle 3 | A5 — opencode loop reproducibility | design exercise |

## RESEARCH LOG ENTRIES

### 2026-06-21 — Initial Plan Creation
- Did high-level audit via explore agents (b_dd1babd5, b_4d0915a6)
- Confirmed: engine has 6 tools, none accessible from workflow steps
- Confirmed: shell_exec/file_write/web_fetch declared in schema but UNIMPLEMENTED
- Next research pause: after Phase 1 baselines

## OPEN QUESTIONS FOR USER

(Body of questions to resolve if encountered)

1. Is implementing engine tools (Track A) in scope, or workflow-only (Track B)?
2. If Track A: any constraint on adding dependencies (tokio::process, etc.)?
3. If prompts require internet (research-style), is web_fetch in scope?
4. Time budget: how many days max for this iteration cycle?
5. Acceptable partial parity threshold (6/11? 8/11?) before declaring done?
