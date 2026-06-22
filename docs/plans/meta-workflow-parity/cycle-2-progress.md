# Cycle 2 Progress Report

**Started:** 2026-06-22 01:31 CDT
**ETA:** ~14 hours (finish ~15:30 CDT)
**Status:** IN PROGRESS

## Cycle 2 Changes

### Engine: src/benchmark/runner.rs:1814-1835
Auto-inject `{{bookmarks.shell_output.stdout}}` content into prompt when:
1. Shell hook ran and produced `shell_output` bookmark
2. Step prompt references `shell_output` in prose (not template var)
3. Logs: `[benchmark] auto-injected shell_output (N bytes) into step X prompt`

### SW4/SW5 Templates
- FORBID: "Read src/X" phrasing
- REQUIRE: `{{bookmarks.shell_output.stdout}}` template var when shell hook present
- ADDED: FOR FILE MODIFICATIONS section with `cp` + cargo check pattern

### fix-yaml.py (10 rules)
1. strip_markdown_fences
2. fix_gwt_unquoted_equals
3. fix_inline_save_to
4. fix_save_to_map_to_list
5. fix_save_to_string_form
6. fix_save_to_outside_when
7. fix_log_after_save_to_indent
8. fix_steps_list_to_map
9. fix_save_to_null_map_pattern
10. fix_save_to_unwritable_paths

### parity-check.sh (10 criteria)
- C1-C5: Structural (YAML, refuses, Read X, shell hooks, well-formed)
- C6-C10: Execution (substantive outputs, no refuse outputs, avg size, code change hooks, logs)
- Pass threshold: 40/50

## Per-Prompt Scope Classification

Per user clarification (Q4: "Cross-repo prompts out of scope"):

| Prompt | Repo | Scope | Expected Output |
|--------|------|-------|-----------------|
| P05 | whitt-execution-engine | ✅ IN | Rust code (parallel inference) |
| P06 | AgentQueue/yaml-to-local-rust-agentsdk | 🔵 OUT | HTML+CSS+PDF |
| P07 | ? | ✅ IN | (read prompt) |
| P08 | AgentQueue/folder-summary-visualizer | 🔵 OUT | Debug vite error |
| P09 | human-file-cartographer | 🔵 OUT | React components |
| P10 | ? | ✅ IN | (read prompt) |
| P11 | ? | ✅ IN | (read prompt, same as P10) |
| P12 | ? | ✅ IN | Skill instruction |
| P13 | ? | ✅ IN | (read prompt) |
| P14 | whitt-execution-engine | ✅ IN | Parallel inference (PROVEN in Cycle 1) |
| P15 | whitt-execution-engine | ✅ IN | Agent ReAct layer |

**IN-SCOPE: 8 prompts (P05, P07, P10, P11, P12, P13, P14, P15)**
**OUT-OF-SCOPE: 3 prompts (P06, P08, P09) — will run but mark DEFERRED**

## Running

- PID: 3006494
- Started: 2026-06-22 01:31:32 CDT
- Expected finish: ~2026-06-22 15:30 CDT
- Log: /tmp/parity-batch-20260622-013132.log
- Results: docs/plans/meta-workflow-parity/cycle-2-results/

## Validation Gate (per 02-VALIDATION-CRITERIA.md)

- 8/8 in-scope prompts PASS (≥40/50) = ACCEPTABLE
- <8/8 PASS = iterate Cycle 3 (SW4/SW5 prompt tuning)
- Cycle 3 budget: 1 iteration only (per 04-ITERATION-STRATEGY.md max 3 cycles)

## Open Risks

1. Docker instability under sustained load (per b3 known limitations)
2. Model may still produce refusal text for prompts requiring tools we don't have (file_write, grep)
3. fix-yaml.py may not catch all malformed YAML patterns
4. Prompts P06/P08/P09 reference external repos — workflows will run but produce refuse outputs
