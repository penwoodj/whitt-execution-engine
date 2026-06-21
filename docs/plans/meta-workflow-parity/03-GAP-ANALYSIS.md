# 03 - GAP ANALYSIS (BRUTALLY HONEST)

## STATEMENT OF REALITY

Meta-workflow-v6 CANNOT match opencode baseline on prompts requiring file modification. Why: **engine lacks tool access from workflow steps**.

## EVIDENCE

### Engine Tool Inventory (verified bg_dd1babd5)

Implemented (6):
- model_list, model_load, model_unload — model lifecycle
- chat — text-in/text-out LLM call
- file_read — EXISTS but only for ReAct agent, NOT for workflow steps
- final_answer — ReAct loop termination

**Declared in schema but NEVER IMPLEMENTED:**
- shell_exec (no ShellTool in tools.rs)
- file_write (only file_read exists)
- web_fetch (no WebFetchTool)
- grep (no GrepTool)

### What Workflow Steps Actually Do

```yaml
step_foo:
  generative_entity: "${models.qwen35}"
  prompt: |
    Read src/foo.rs and extract pattern...
```

→ Engine calls chat_completion(prompt) → text out.
→ Model trained as assistant: "I cannot access filesystem..."
→ save_to writes refusal text → output artifact is garbage.

### Generated Workflow Examples (from b3 run)

Sample prompts from SW5 outputs:
- "Read src/benchmark/runner.rs and locate pattern at line 1858" → refusal
- "Read input list from previous step output" → works (uses {{step.X.output}})
- "Output a YAML summary describing code structure" → outputs YAML-shaped refusal text

## ROOT CAUSES (RANKED)

### RC1: NO tool access from workflow steps (CRITICAL)
- Workflows are text-in/text-out
- file_read tool exists but only wired to ReAct agent, not steps
- No shell_exec, file_write, web_fetch tools at all

### RC2: Meta-v6 SW4/SW5 prompt templates ASSUME tool access
- "Read src/X and..." — model can't
- "Modify the function at line Y" — model can't
- Templates need rewriting: use shell hooks to pre-load content via bookmarks

### RC3: Workflow structure doesn't expose hooks for content injection
- Hooks exist (save_to, log, etc) but no "load_file_to_bookmark" action
- Would need new HookAction variant or shell hook usage

### RC4: Model (Qwen3.5-9B) trained to refuse without tools
- Even with file content in prompt, model may refuse to "modify" code
- Smaller model = worse at meta-reasoning about file edits

## GAP CLOSURE OPTIONS

### Option A: Engine Tools (MULTI-DAY)
- Implement ShellTool, FileWriteTool, WebFetchTool in src/agent/tools.rs
- Wire tool calls into workflow step execution (not just ReAct)
- Update schema to match implementation
- **Pros:** Real agentic execution
- **Cons:** Days of engine work, scope creep

### Option B: Shell Hooks for Content Injection (HOURS)
- Use existing Shell hook action to pre-read files into bookmarks
- SW4/SW5 templates rewrite prompts:
  - BEFORE: "Read src/foo.rs and extract X"
  - AFTER: "Analyze this content: {{bookmarks.foo_content}}"
- **Pros:** Works with current engine, hours not days
- **Cons:** Read-only (no file modification), still need shell for writes

### Option C: Accept Limitations (HOURS)
- Acknowledge: meta-v6 produces ANALYSIS not IMPLEMENTATION
- Re-scope parity claim: "produce useful analysis of prompt"
- Drop file-modification prompts (14, 15) from parity target set
- **Pros:** Honest, fast
- **Cons:** Doesn't fully meet user's "match opencode" goal

## RECOMMENDATION

**Hybrid: Option B + Limitations Declared.**

1. Implement Option B (shell hooks for file content pre-load)
2. Iterate SW4/SW5 templates to use it
3. Re-run 11 prompts
4. Honestly evaluate: did prompts get answered?
5. For prompts requiring file modification: declare partial parity

## HONEST EVALUATION OF CURRENT STATE

Per b3 results, current "11/11 prompts execute" claim is TECHNICALLY TRUE but PRACTICALLY MISLEADING:
- ✅ Workflows run to completion
- ✅ Output files exist
- ❌ Output content = refusals / hallucinations
- ❌ Prompt objectives NOT met
- ❌ Quality far below opencode baseline

**Parity score (current): 0/11 prompts achieve objective.**

## ESTIMATED GAP TO PARITY

With Option B (shell hooks + template rewrite):
- Analysis prompts (research, doc gen): LIKELY achievable (6-8/11)
- Code modification prompts: UNLIKELY (would need engine tools)
- Total achievable: 6-8/11 = "acceptable parity" per criteria 02

With Option A (engine tools):
- 10-11/11 achievable
- But requires multi-day engine work
