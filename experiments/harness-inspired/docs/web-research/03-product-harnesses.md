# Web Research: Product Harness Comparison (2025-2026)

> Compiled 2026-08-23. 10 products. Mechanics only.

## Batch 1: CLI/Cloud Coding Agents

### Claude Code
- Loop: ReAct `while(true)` generator, 9 continue points, 10-step pipeline
- Verify: 5-layer progressive compression (budget→snip→micro→collapse→auto), model summaries
- Repair: 7 recovery paths (413 reactive compact, max-output escalation, model fallback, retry+backoff)
- Terminate: no tool use, max turns, context overflow, hook interrupt, abort
- Trick: StreamingToolExecutor starts tools BEFORE model finishes; prompt caching w/ static/dynamic split (40% speedup)
- Pro: production error recovery, circuit breakers
- Con: 1,729-line monolithic loop; compaction degrades system instructions over time
- Src: harrisonsec.com/blog/claude-code-deep-dive-query-loop/

### OpenAI Codex CLI
- Loop: stateful turn loop, SSE streaming, pre/post-turn hooks, previous_response_id resume
- Verify: auto-compact at 90% context via /responses/compact or inline summarization
- Repair: 5x retry timeout/server, exp backoff on 429; ContextWindowExceeded → drop oldest, retry
- Terminate: assistant msg w/o tools, user abort, hard error, max tokens
- Trick: GhostSnapshot preserved through compaction → /undo rollback; environment_diff injects only changed fields
- Pro: stateless pause/resume/fork; auto-compact keeps long sessions alive
- Con: developer messages discarded on compaction; quadratic growth otherwise
- Src: openai.com/index/unrolling-the-codex-agent-loop/

### Cursor (cloud agents)
- Loop: Temporal workflow; agent/machine/conversation state decoupled; multi-worker activities
- Verify: Auto-review classifier (fast model) blocks ~4% of actions pre-execution w/ explanation
- Repair: Temporal retries/timeouts; autoinstall self-heals missing secrets
- Terminate: single task then exit; pod fail → Temporal reschedules
- Trick: Agent+ pattern — parent monitors, child gets FRESH CONTEXT + browser; append-only storage
- Pro: 99.99% reliability via Temporal; multi-day runs survive hibernation
- Con: no per-turn hook between tool request and execution
- Src: cursor.com/blog/cloud-agent-lessons/ + cursor.com/blog/agent-autonomy-auto-review

### Devin (Cognition)
- Loop: plan-gather-execute-observe in cloud microVM; parallel child Devins; ACU metering
- Verify: per-action testing (run→observe→repair); parent reviews child diffs pre-integration
- Repair: self-correction loop (error→open→edit→retest); Fusion routes frontier/sidekick mid-session
- Terminate: task verified, ACU budget out, sleep/wake, human takeover
- Trick: Devin Fusion — sidekick handles routine while frontier stays in cache; switches free during compaction
- Pro: 88% internal PRs driven by Fusion router
- Con: ACU metering unpredictable
- Src: cognition.com/blog/devin-fusion/

### Aider
- Loop: reflection loop (max 3) per message; auto-commit per turn; optional Architect+Editor 2-model pipeline
- Verify: auto-lint + auto-test after edits (fix-with-confirm); git diff review
- Repair: fuzzy match on search/replace miss; reflection feeds errors back w/o re-prompting
- Terminate: reflected_message exit; context overflow → continue; Ctrl-C
- Trick: repo map via tree-sitter + PageRank → codebase in few K tokens
- Pro: best git integration; zero compaction complexity
- Con: no subagents; context window = hard ceiling; reflection capped at 3
- Src: deepwiki.com/Aider-AI/aider/2-core-architecture

## Batch 2: Frameworks/Benchmarks

### SWE-agent / SWE-bench
- Loop: Docker sandbox → patch → test → grade
- Verify: repo test suite pass/fail
- Repair: none — single-shot patch
- Terminate: test pass or timeout
- Trick: three-layer Docker caching
- Pro: deterministic, reproducible
- Con: no self-repair after test failure
- Src: swebench.com/SWE-bench/reference/harness/

### OpenHands (ex-OpenDevin)
- Loop: event stream → LLM → action → observe → repeat
- Verify: LLM observes tool outputs, self-evaluates
- Repair: StuckDetector catches loops → retry or error
- Terminate: FinishAction, stuck, budget, pause
- Trick: stateless agent + append-only event log
- Pro: replayable, swappable workspace, debuggable
- Con: complex 5-phase step fn
- Src: deepwiki.com/All-Hands-AI/OpenHands/6.1

### LangGraph
- Loop: Pregel execution → router → node → observe → route
- Verify: tool results feed back to LLM
- Repair: loop guard on repeated calls, graceful exit
- Terminate: conditional edge → END, recursion limit
- Trick: managed RemainingSteps graceful degradation
- Pro: explicit routing, composable graph
- Con: wiring errors cause silent loops
- Src: infowok.com/langgraph-agent-looping-fix-2026

### AutoGen/AG2
- Loop: MemoryStream → Assembly → LLM → tools → observe
- Verify: LoopDetector observer watches patterns
- Repair: auto-retry middleware, HaltEvent on failure
- Terminate: HaltEvent, human interrupt, stop
- Trick: middleware stack at 4 call sites
- Pro: pluggable layers
- Con: explicit config needed for advanced features
- Src: docs.ag2.ai/docs/user-guide/agent_harness/

### GitHub Copilot agent mode
- Loop: plan → edit → run tests → observe → iterate
- Verify: test results read back, failures auto-fixed
- Repair: auto-fix test failures real-time
- Terminate: complete, interrupt, stuck
- Trick: synchronous pair-programming style
- Pro: tight feedback loop
- Con: needs continuous human oversight
- Src: github.blog (coding agent vs agent mode)

## Patterns We Steal for v3+

| Pattern | Source | v3 Implementation |
|---------|--------|-------------------|
| Append-only event log = replayable truth | OpenHands, AgentAssay | ha-trace.jsonl + collector re-score |
| Fresh-context child (Agent+) | Cursor, research P19 | fix_2 = history-free from-scratch solve |
| Reflection capped at 2-3 | Aider, FeedbackEval F9 | FIX x2 cap |
| Auto-review gate before expensive action | Cursor Auto-review | deterministic check before judge |
| Per-action metering | Devin ACU | stage/token counts in trace |
| Loop/stuck detection | OpenHands, LangGraph, AutoGen | GWT routing on exit_code (no silent loops) |
| Undo snapshots | Codex GhostSnapshot | artifacts saved per stage (future) |
