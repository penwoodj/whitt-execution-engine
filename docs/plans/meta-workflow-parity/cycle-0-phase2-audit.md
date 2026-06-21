# Phase 2 — Current State Audit (BRUTALLY HONEST)

**Audit date:** 2026-06-21
**Source:** b3 SW5 outputs from 2026-06-19 run

## SUMMARY

11/11 SW5 workflows generate valid YAML. 0/11 produce useful output when executed. Quality bar: ZERO.

## PER-PROMPT STRUCTURAL AUDIT

| # | YAML Valid | Lines | "Read X" instructions | Quality Notes |
|---|-----------|-------|----------------------|---------------|
| 05 | ✅ | 251 | 5 | Workflows reference nonexistent steps |
| 06 | ✅ | 302 | 1 | Asks model to write PDF (no tool) |
| 07 | ✅ | 268 | 12 | Heavy on "Read src/X" |
| 08 | ✅ | 251 | 7 | Debug workflow but model has no diagnostic tools |
| 09 | ✅ | 285 | 0 | Asks for UI component updates |
| 10 | ✅ | 282 | 3 | Research synthesis (most viable) |
| 11 | ✅ | 224 | 6 | Same as P10 |
| 12 | ✅ | 270 | 8 | Skill instruction generation |
| 13 | ✅ | 282 | 1 | Binary creation |
| 14 | ✅ | 120 | 0 | Tiny workflow, basic |
| 15 | ✅ | 365 | 4 | 7-file generation (impossible without tools) |

## DEEP DIVE — P14 SAMPLE

Sample step from P14 workflow:
```yaml
step_t1_verify_llama_http_client_clone:
  prompt: |
    Read {{step.t2_identify_request_type_imports.output}} and locate the struct
    definition of LlamaHttpClient in src/client/http_client.rs. Add #[derive(Clone)]
    above the struct if missing. Verify compilation succeeds with cargo check.
    Save updated file to ./outputs/http_client_updated.rs.
```

### Problems Found:

1. **NO `tools:` field** — model has zero tool access. Pure chat_completion.
2. **Forward reference** — step_t1 references {{step.t2.output}} but t2 runs AFTER t1. Will produce empty interpolation.
3. **"Save updated file"** — model cannot save files. save_to hook captures model's text output, NOT a file the model writes.
4. **"Verify compilation succeeds with cargo check"** — model cannot run cargo. No bash tool.
5. **"Add #[derive(Clone)]"** — model cannot edit files. Can only output text describing what should be added.

### What Model Will Actually Output

Trained Qwen3.5-9B receives this prompt → recognizes asks for filesystem actions → outputs:
> "I cannot directly access your local filesystem to modify files. However, I can describe what you should do:
> 1. Open src/client/http_client.rs
> 2. Add #[derive(Clone)] above the struct
> 3. Run cargo check
> If you share the file contents, I can provide exact line numbers..."

This text gets `save_to`'d to `./outputs/http_client_updated.rs`. File exists, contains refusal.

## ROOT CAUSES CONFIRMED

### RC1: No tool access from workflow steps (CRITICAL, blocks all code prompts)

Engine has 6 tools registered in `src/agent/tools.rs`:
- model_list, model_load, model_unload — lifecycle
- chat — text-in/text-out
- file_read — EXISTS but only for ReAct agent (src/agent/react.rs:45-65), NOT wired to workflow steps
- final_answer — ReAct termination

**Schema (docs/schema/unified-workflow-schema.yml:368-388) declares:**
- `tool: file_read`, `tool: shell_exec`, etc on steps
- `default_permissions` block

**But engine execution path:**
- Workflow steps call `chat_completion` only
- No code path passes step.tool to ToolRegistry
- shell_exec/file_write/web_fetch have NO implementation in tools.rs

### RC2: SW4/SW5 templates assume tool access (CRITICAL)

Generated prompts use phrasing:
- "Read src/foo.rs and..."
- "Modify the function at..."
- "Save updated file to..."
- "Run cargo check"

All require tool access model doesn't have.

### RC3: Workflow structural bugs (MEDIUM)

- Forward references (step_t1 → {{step.t2.output}}) 
- Step ordering doesn't match data flow
- Loop variables not properly scoped

### RC4: Workflow doesn't use available hook capabilities (MEDIUM)

Engine HAS shell hook action (b3 confirmed). Workflows don't use it to:
- Pre-read files into bookmarks (`cat src/foo.rs > bookmark_foo`)
- Run verification commands (`cargo check > bookmark_check_result`)

## EVALUATION SCORE

Per 02-VALIDATION-CRITERIA, scoring 11 prompts:

| Criterion | Pass Rate | Reason |
|-----------|-----------|--------|
| 1. Workflow Parses | 11/11 ✅ | All valid YAML |
| 2. Workflow Executes | 11/11 ✅ | All complete exit 0 (per b3) |
| 3. Output Real (not refusals) | 0/11 ❌ | All contain refusal text |
| 4. Objective Addressed | 0/11 ❌ | None accomplish prompt goal |
| 5. Quality vs Baseline | 0/11 ❌ | All far below baseline |

**TOTAL PARITY: 0/11 prompts pass.**

## HONEST ADMISSION

Previous b3 claim "11/11 prompts execute" was TECHNICALLY TRUE but QUALITATIVELY FALSE:
- ✅ Workflows ran to completion
- ✅ Output files were created
- ❌ Output content was LLM refusals, not real artifacts
- ❌ Prompt objectives NOT met
- ❌ Quality nowhere near OpenCode baseline

**Current parity: 0/11. Distance to parity: significant engine + template work needed.**

## WHAT WOULD FIX THIS

### Path A: Engine Tools (correct long-term fix)
Implement in `src/agent/tools.rs`:
1. `ShellTool` — runs bash commands, captures stdout/stderr
2. `FileWriteTool` — writes content to filesystem
3. `FileReadTool` for steps (currently only ReAct) — wire into workflow step execution
4. `GrepTool` — searches files
5. `WebFetchTool` — HTTP GET

Wire into workflow step execution in `src/benchmark/runner.rs`:
- Pass `tool:` declarations to tool registry
- Execute tools, capture results, inject into prompt context
- Or use ReAct-style loop within step

### Path B: Template Rewrite + Shell Hooks (workaround)
Modify SW4/SW5 prompt templates to:
1. Replace "Read src/X" with shell hook that pre-reads into bookmark
2. Replace "Save updated file" with "Output the new file content as text"
3. Replace "Run cargo check" with shell hook that runs + captures
4. Templates use {{bookmarks.X}} interpolation to inject pre-loaded content

Path B is faster but limits workflows to text analysis, not file modification.

## NEXT STEPS

Phase 3: Root cause analysis (done above — RC1, RC2, RC3, RC4)
Phase 4: Implementation choice — Path A vs Path B vs hybrid
