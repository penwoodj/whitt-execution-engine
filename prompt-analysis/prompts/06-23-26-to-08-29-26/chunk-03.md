# chunk-03 — prompts 101-150 of 1000
# span 06-23-26-to-08-29-26

## [0101] 06-28 18:52

(2026-06-28T18:52:15)
Orange Pi 4 Pro 12GB is $275

## [0102] 06-28 19:17

(2026-06-28T19:17:44)
Ive decided to start by purchasing Orange Pi Zero 3W 12GB LPDDR5 Allwinner A733 Octa Core CPU 3 Tops Single Board Computer, 2.0GHz Frequency WiFi 6 & Bluetooth 5.4 (BLE) Onboard Antenna Support 4K Display Run Android Ubuntu 

tell me what I will need to be able to use it.  Can I use it for a TV?

## [0103] 06-28 19:18

(2026-06-28T19:18:21)
[CONTEXT]: User just bought Orange Pi Zero 3W 12GB LPDDR5 (Allwinner A733). Wants to use as TV/media streamer and general SBC. Need to know: required accessories, available OS images, Android TV feasibility, Widevine DRM status, power supply spec.

[GOAL]: Concrete setup checklist — what to buy, what OS to flash, what works for TV use, what doesn't.

[DOWNSTREAM]: User will purchase accessories and flash OS based on this answer. Must be accurate — no guessing.

[REQUEST]:
1. **Power supply spec**: USB-C PD? Voltage/amps required? Does it support PD negotiation or fixed 5V?
2. **What's in box**: board only? Fan? Case? Antenna?
3. **uSD card requirements**: min class/speed, recommended size for Android + Linux dual
4. **Available OS images** (Orange Pi official download page): list exact images, Android version, Ubuntu/Debian versions, build dates
5. **Android TV vs Android Phone build**: is there an Android TV / Google TV interface build, or just standard Android? Does it have Leanback launcher?
6. **Widevine DRM certification level**: L1 or L2? This determines whether Netflix/Disney+ stream HD/4K. Check Orange Pi forums, XDA, 4PDA.
7. **HDCP support over HDMI**: required for commercial streaming services
8. **Kodi availability**: any A733-optimized build? Rockchip RK3588 has good Kodi support, what about A733?
9. **Community setup guides**: link best 2-3 actual user guides for Zero 3W (not generic Allwinner)
10. **Known issues / gotchas**: thermal throttling, WiFi stability, sleep/wake, audio over HDMI

Skip generic SBC tutorials. Need A733 / Zero 3W specific data. If info doesn't exist yet (board is new), say so explicitly — do not fabricate.

Return: actionable setup checklist + honest TV feasibility verdict.
<!-- OMO_INTERNAL_INITIATOR -->

## [0104] 06-28 19:38

(2026-06-28T19:38:05)
[search-mode]
MAXIMIZE SEARCH EFFORT. Launch multiple background agents IN PARALLEL:
- explore agents (codebase patterns, file structures, ast-grep)
- librarian agents (remote repos, official docs, GitHub examples)
Plus direct tools: Grep, ripgrep (rg), ast-grep (sg)
NEVER stop at first result - be exhaustive.

---

what about a 5v 4a 20w power supply? would that be risky only quick answer after web search

## [0105] 06-28 19:38

(2026-06-28T19:38:24)
[CONTEXT]: Orange Pi Zero 3W has USB-C 5V/3A DC-IN (NOT PD-negotiable, fixed 5V). User has a 5V 4A 20W USB-C power supply and asks if it's safe to use.

[GOAL]: Confirm whether 5V 4A supply is safe or risky for Zero 3W. Specifically check: overvoltage risk, current draw behavior, any known power issues, community reports of damage from higher-amp supplies.

[DOWNSTREAM]: Quick safety answer — is 5V 4A fine or will it fry the board?

[REQUEST]:
1. Zero 3W power input circuit: linear regulator or DC-DC? What's the input voltage tolerance?
2. Any community reports of power supply damage on Zero 3W or similar A733 boards
3. USB spec: can a 5V/4A supply harm a 5V/3A device? (devices draw what they need, not forced — confirm)
4. Any Zero 3W power consumption benchmarks under load (idle, CPU stress, WiFi active)
5. Any warnings about specific charger types (PD, QC, fixed 5V)

Return: 2-3 sentence verdict. Not a tutorial.
<!-- OMO_INTERNAL_INITIATOR -->

## [0106] 06-28 19:41

(2026-06-28T19:41:15)
can I use a non powered usb hub with it with the data usbc port?

## [0107] 06-28 19:41

(2026-06-28T19:41:52)
I have 20w I thought?

## [0108] 06-28 19:43

(2026-06-28T19:43:05)
so keyboard and mouse through bluetooth and a usb would work on a usb hub attachted to the data usbc?

## [0109] 06-28 19:45

(2026-06-28T19:45:51)
could hdmi and keyboard mouse and a usb 32gb drive all work through the data port from a non-powered hub? just the hdmi even?

## [0110] 06-28 23:43

(2026-06-28T23:43:56)
<auto-slash-command>
# /ralph-loop Command

**Description**: (builtin) Start self-referential development loop until completion

**User Arguments**: start all this by committing and pushing then cleaning up the top level of the repository then fixing any .yml workflows so if they are the cause this doesn't keep hapening and if you are the cause in the chat session just by directly working in there change the AGENTS.md file so you always work in the right folders with a map of what to put there and strict rules to follow that map and keep things organized in the repo without losing anything then after all of that commit and push again then 


we want to start to do some research and aggregation on where we are missing on score most commonly and what we can do to fix it and just start by documenting and answering and then document and answer what is missing with p15 and 



then using the new docs and chat context I want you to iterate with live system testing on the meta workflow generator .yml file and its sub workflows, working and iterating with live system testing on the smallest elements first before continuing to iterate on the aggregations of the sub workflow and sub workflow elements and don't stop until everything is fix and use plan files to fully document implementation and validation intent then use that to stay on track while you iterate and update the plans with live system testing validated progress.  don't stop iterating until we've validated at least 20 complex prompts agentically perform better through out meta workflow generator .yml workflow running with real execution in live system testing and with real read observed reality results analyzed critically and don't be nice with your results but be critical.  don't stop until everything is fully completed and verified.

**Scope**: builtin

---

## Command Instructions

<command-instruction>
You are starting a Ralph Loop - a self-referential development loop that runs until task completion.

## How Ralph Loop Works

1. You will work on the task continuously
2. When you believe the task is FULLY complete, output: `<promise>{{COMPLETION_PROMISE}}</promise>`
3. If you don't output the promise, the loop will automatically inject another prompt to continue
4. Maximum iterations: Configurable (default 100)

## Rules

- Focus on completing the task fully, not partially
- Don't output the completion promise until the task is truly done
- Each iteration should make meaningful progress toward the goal
- If stuck, try different approaches
- Use todos to track your progress

## Exit Conditions

1. **Completion**: Output your completion promise tag when fully complete
2. **Max Iterations**: Loop stops automatically at limit
3. **Cancel**: User runs `/cancel-ralph` command

## Your Task

Parse the arguments below and begin working on the task. The format is:
`"task description" [--completion-promise=TEXT] [--max-iterations=N] [--strategy=reset|continue]`

Default completion promise is "DONE" and default max iterations is 100.
</command-instruction>

<user-task>
start all this by committing and pushing then cleaning up the top level of the repository then fixing any .yml workflows so if they are the cause this doesn't keep hapening and if you are the cause in the chat session just by directly working in there change the AGENTS.md file so you always work in the right folders with a map of what to put there and strict rules to follow that map and keep things organized in the repo without losing anything then after all of that commit and push again then 


we want to start to do some research and aggregation on where we are missing on score most commonly and what we can do to fix it and just start by documenting and answering and then document and answer what is missing with p15 and 



then using the new docs and chat context I want you to iterate with live system testing on the meta workflow generator .yml file and its sub workflows, working and iterating with live system testing on the smallest elements first before continuing to iterate on the aggregations of the sub workflow and sub workflow elements and don't stop until everything is fix and use plan files to fully document implementation and validation intent then use that to stay on track while you iterate and update the plans with live system testing validated progress.  don't stop iterating until we've validated at least 20 complex prompts agentically perform better through out meta workflow generator .yml workflow running with real execution in live system testing and with real read observed reality results analyzed critically and don't be nice with your results but be critical.  don't stop until everything is fully completed and verified.
</user-task>


---

## User Request

start all this by committing and pushing then cleaning up the top level of the repository then fixing any .yml workflows so if they are the cause this doesn't keep hapening and if you are the cause in the chat session just by directly working in there change the AGENTS.md file so you always work in the right folders with a map of what to put there and strict rules to follow that map and keep things organized in the repo without losing anything then after all of that commit and push again then 


we want to start to do some research and aggregation on where we are missing on score most commonly and what we can do to fix it and just start by documenting and answering and then document and answer what is missing with p15 and 



then using the new docs and chat context I want you to iterate with live system testing on the meta workflow generator .yml file and its sub workflows, working and iterating with live system testing on the smallest elements first before continuing to iterate on the aggregations of the sub workflow and sub workflow elements and don't stop until everything is fix and use plan files to fully document implementation and validation intent then use that to stay on track while you iterate and update the plans with live system testing validated progress.  don't stop iterating until we've validated at least 20 complex prompts agentically perform better through out meta workflow generator .yml workflow running with real execution in live system testing and with real read observed reality results analyzed critically and don't be nice with your results but be critical.  don't stop until everything is fully completed and verified.
</auto-slash-command>

## [0111] 06-28 23:43

<command-instruction>
You are starting a Ralph Loop - a self-referential development loop that runs until task completion.

## How Ralph Loop Works

1. You will work on the task continuously
2. When you believe the task is FULLY complete, output: `<promise>{{COMPLETION_PROMISE}}</promise>`
3. If you don't output the promise, the loop will automatically inject another prompt to continue
4. Maximum iterations: Configurable (default 100)

## Rules

- Focus on completing the task fully, not partially
- Don't output the completion promise until the task is truly done
- Each iteration should make meaningful progress toward the goal
- If stuck, try different approaches
- Use todos to track your progress

## Exit Conditions

1. **Completion**: Output your completion promise tag when fully complete
2. **Max Iterations**: Loop stops automatically at limit
3. **Cancel**: User runs `/cancel-ralph` command

## Your Task

Parse the arguments below and begin working on the task. The format is:
`"task description" [--completion-promise=TEXT] [--max-iterations=N] [--strategy=reset|continue]`

Default completion promise is "DONE" and default max iterations is 100.
</command-instruction>

<user-task>
start all this by committing and pushing then cleaning up the top level of the repository then fixing any .yml workflows so if they are the cause this doesn't keep hapening and if you are the cause in the chat session just by directly working in there change the AGENTS.md file so you always work in the right folders with a map of what to put there and strict rules to follow that map and keep things organized in the repo without losing anything then after all of that commit and push again then 


we want to start to do some research and aggregation on where we are missing on score most commonly and what we can do to fix it and just start by documenting and answering and then document and answer what is missing with p15 and 



then using the new docs and chat context I want you to iterate with live system testing on the meta workflow generator .yml file and its sub workflows, working and iterating with live system testing on the smallest elements first before continuing to iterate on the aggregations of the sub workflow and sub workflow elements and don't stop until everything is fix and use plan files to fully document implementation and validation intent then use that to stay on track while you iterate and update the plans with live system testing validated progress.  don't stop iterating until we've validated at least 20 complex prompts agentically perform better through out meta workflow generator .yml workflow running with real execution in live system testing and with real read observed reality results analyzed critically and don't be nice with your results but be critical.  don't stop until everything is fully completed and verified.
</user-task>

## [0112] 06-29 11:05

(2026-06-29T11:05:51)
 continue

## [0113] 06-29 15:09

(2026-06-29T15:09:41)
continue

## [0114] 06-29 16:29

(2026-06-29T16:29:41)
continue

## [0115] 06-29 18:10

(2026-06-29T18:10:27)
[restore checkpointed session agent configuration after compaction]
<!-- OMO_INTERNAL_INITIATOR -->

## [0116] 06-29 18:51

(2026-06-29T18:51:18)
 (2026-06-29T17:24:14) P10 SW2 started (step 01-task-index). tasks.md 17712 bytes (substantial decomposition). Pipeline healthy.

Current ALL20 progress:

    P05: ✅ 32494B SW_WINS (22 vs 19)
    P06: ❌ (P06/P07 re-run queued)
    P07: ❌ (P06/P07 re-run queued)
    P08: ✅ 14998B SW_WINS (22 vs 21)
    P09: ✅ 19504B SW_WINS (22 vs 7)
    P10: 🔄 SW2 running (SW1 done 1269s, tasks.md 17712B)
    P11-P24: ⏳ Queued (~14 hours)

Proven win rate: 6/6 (100%) on tested prompts. Average SW=21.8 vs BASE=16.0.

Ralph Loop continuing. Next check: P10 SW2 completion.

## [0117] 06-30 16:57

(2026-06-30T16:57:22)
So I've purchase it along with everything I need. when I get it what steps will I need to follow to get a linux os on it.  I have a 128gb sd card

## [0118] 06-30 16:57

(2026-06-30T16:57:56)
can it support cachyos?

## [0119] 06-30 16:59

(2026-06-30T16:59:37)
so I'll want to run my ~/code/whitt-execution-engine/ on this os so I want as light weight of a weight of a linux os as possible

## [0120] 06-30 16:59

(2026-06-30T16:59:45)
[CONTEXT]: User wants to run ~/code/whitt-execution-engine/ on an Orange Pi Zero 3W (Allwinner A733, ARMv9, 12GB RAM, no GPU/VPU userspace on stock images). Wants the lightest possible Linux OS.

[GOAL]: Determine runtime requirements for whitt-execution-engine — what language, build tools, runtime dependencies, minimum OS libraries needed.

[DOWNSTREAM]: Will recommend lightest ARM Linux distro that supports these requirements AND the A733 BSP kernel.

[REQUEST]:
1. Check ~/code/whitt-execution-engine/ — what language is it written in? (check Cargo.toml, package.json, go.mod, requirements.txt, Makefile, pyproject.toml, etc.)
2. What are the build dependencies?
3. What are the runtime dependencies? (e.g., needs Python 3.11+, Node.js, Rust toolchain, etc.)
4. Does it need GUI/display? Or headless OK?
5. Does it need specific system libraries (libssl, systemd, docker, etc.)?
6. Any architecture-specific requirements? (x86-only binaries, etc.)
7. How large is the codebase / how much disk space needed?

Return: concise requirements list. Skip implementation details — only need to know what the runtime needs to pick an OS.
<!-- OMO_INTERNAL_INITIATOR -->

## [0121] 06-30 16:59

(2026-06-30T16:59:48)
[CONTEXT]: Orange Pi Zero 3W (Allwinner A733, ARMv9). Official images: Ubuntu 22.04 and Debian 11 (Bullseye), both with BSP kernel 6.6.98-sun60iw2. Mainline Linux support is incomplete — vendor BSP blobs required for GPU/VPU/WiFi. User wants the lightest possible Linux for headless server use.

[GOAL]: Find what lightweight ARM Linux distros can run on A733 with BSP kernel, ranked by RAM/disk footprint. Specifically investigate:
1. Alpine Linux ARM — does anyone have it running on A733? Any community builds?
2. Arch Linux ARM — any A733 support?
3. Debian minimal (netinst) — can it use the Orange Pi BSP kernel?
4. Ubuntu minimal (server) — already available, but can it be trimmed?
5. Buildroot / OpenWrt — any A733 images?
6. DietPi — does it support A733 or Zero 3W?

[DOWNSTREAM]: User wants headless server for running code projects. Every MB of RAM saved = more for the actual workload. Need to know realistic options given BSP kernel constraint.

[REQUEST]:
- Search for Alpine Linux A733 / Allwinner A733 images or community ports
- Search for Arch Linux ARM A733 support
- Check DietPi supported devices list for A733
- Check OpenWrt for A733 support
- Check if Debian netinst can boot with Orange Pi BSP kernel (swap kernel + dtb)
- Measure RAM usage of Ubuntu 22.04 server vs Debian 11 server on similar ARM SBCs (as proxy)

Return: ranked list from lightest to heaviest, with estimated RAM usage and feasibility notes.
<!-- OMO_INTERNAL_INITIATOR -->

## [0122] 06-30 17:17

(2026-06-30T17:17:38)
TASK: Fix 6 VM bugs in `compiler/crates/lr-vm/src/vm.rs` based on Batch 13 user answers. Run tests.

WORKDIR: /home/jon/code/left-right

CONTEXT: Left-Right VM. 465+ tests pass. Live testing + user Q&A revealed 6 divergences from documented spec.

BUG 1: Deprecated type-check operators `?!` and `?//` must HARD ERROR. Currently still work.

Spec (verified Batch 13): `?!` (legacy isBoolean) and `?//` (legacy isList) are HARD ERRORS. Users must use `??` (canonical isBoolean) and `?%` (canonical isList) instead.

Current state: `?!` appears at lines 225, 547, 616, 648, 684, 1183, 1262 (works as isBoolean). `?//` appears at lines 226, 548, 617, 618, 649, 685, 686, 1186, 1263 (works as isList).

Fix: Replace EVERY `"?!"` match arm with an ERROR return. Use:
`"?!" => return Err(VMError::Runtime("`?!` is deprecated. Use `??` for isBoolean.".to_string()))`

Same for `"?//"`:
`"?//" => return Err(VMError::Runtime("`?//` is deprecated. Use `?%` for isList.".to_string()))`

IMPORTANT: The diadic forms `(left, _, "?!")` and `(left, _, "?//")` at lines ~1183/1186 also need the same error treatment.

DO NOT touch `??` or `?%` — those are canonical and correct.

BUG 2: `<` `>` `<=` `>=` on String × String = LENGTH COMPARE (not lexicographic).

Spec (verified Batch 13): `` `apple` < `banana` `` → true (5 < 6, length-based). NOT lexicographic.

Current state: comparison operators on String likely throw or fall to default error.

Fix: Add String×String cases for `<`, `>`, `<=`, `>=` in the diadic dispatch. Compare `ls.len()` vs `rs.len()`:
```rust
(Value::String(ls), Value::String(rs), "<") => Value::boolean(mc, ls.len() < rs.len()),
(Value::String(ls), Value::String(rs), ">") => Value::boolean(mc, ls.len() > rs.len()),
(Value::String(ls), Value::String(rs), "<=") => Value::boolean(mc, ls.len() <= rs.len()),
(Value::String(ls), Value::String(rs), ">=") => Value::boolean(mc, ls.len() >= rs.len()),
```
Place these near the existing comparison dispatch (around line 740-743 for Number comparisons).

BUG 3: `/` String × Number = CHUNK SPLIT.

Spec (verified Batch 13): `` `abcdef` / 2 `` → `` [`ab`, `cd`, `ef`] ``. Split string into chunks of size N.

Fix: Add diadic arm `(Value::String(s), Value::Number(n), "/")`:
```rust
(Value::String(s), Value::Number(n), "/") => {
    let chunk_size = *n as usize;
    if chunk_size == 0 {
        return Err(VMError::Runtime("String chunk size must be > 0".to_string()));
    }
    let chunks: Vec<Value> = s.as_str()
        .chars()
        .collect::<Vec<_>>()
        .chunks(chunk_size)
        .map(|c| Value::string(mc, c.iter().collect::<String>()))
        .collect();
    Value::list(mc, chunks)
}
```
Edge cases: n=0 → error. n > len → one chunk (whole string). n non-integer → truncate.

BUG 4: `-` String × Number = COERCE right to String, then remove ALL occurrences.

Spec (verified Batch 13): `` `hello` - 2 `` → `` `hello` `` (no `2` substring, unchanged). `` `a2b2` - 2 `` → `` `ab` `` (all `2`s removed). Number coerced to string first.

Fix: Add diadic arm `(Value::String(s), Value::Number(n), "-")`:
```rust
(Value::String(s), Value::Number(n), "-") => {
    let remove_str = Value::Number(*n).to_string();
    Value::string(mc, s.as_str().replace(&remove_str, ""))
}
```
Note: Check how Value::Number stringifies — use the same path as `+` concatenation (probably `format!("{}", n)` then strip trailing `.0`).

BUG 5: `%` String × Number = THROWS.

Spec (verified Batch 13): `` `abcdef` % 2 `` throws. Numeric only.

Verify current behavior: if `%` on String already throws, no change needed. If it silently does something, make it throw with: `return Err(VMError::Runtime("`%` requires Number operands".to_string()))`.

BUG 6: `!!!` thrown value structure = MAP `{stack, error, ...}`.

Spec (verified Batch 13): `!!!` creates error map with `{stack: <error stack map>, error: <thrown value, same type as thrown>, foo: <other useful keys>}`.

Investigate current `!!!` implementation (search for `"!!!"` in vm.rs, around line 301). Check what value structure is thrown. If it's already a map with these fields, no change. If it throws raw value, wrap it:
```rust
// When throwing value v:
let err_map = Value::Map(map with entries:
    "error" -> v (the thrown value),
    "stack" -> stack trace map/string,
    // other context
);
```

If uncertain about exact additional fields, document current structure and flag for user clarification.

MUST DO:
1. Read vm.rs around all affected lines before editing
2. Fix all 6 bugs
3. Run `cargo test` from `compiler/` — ALL pass (update old tests that asserted deprecated ops work)
4. Run `cd compiler/crates/lr-cli && cargo run --release -- test` — all pass
5. Add NEW tests:
   - `true ??` → true
   - `true ?!` → ERROR (deprecated)
   - `[1,2] ?%` → true
   - `[1,2] ?//` → ERROR (deprecated)
   - `` `apple` < `banana` `` → true (5<6)
   - `` `ab` > `abc` `` → false (2<3)
   - `` `abcdef` / 2 `` → `[ab, cd, ef]`
   - `` `a2b2` - 2 `` → `ab`
   - `` `hello` - 2 `` → `hello` (no 2 in it)
   - `` `abc` % 2 `` → throws
6. `cargo check -p lr-vm` — zero warnings
7. Report `!!!` structure findings

MUST NOT DO:
- Do NOT remove `??` or `?%` (canonical, keep working)
- Do NOT touch qa-history, matrix, impl-corrections
- Do NOT suppress warnings
- Do NOT delete tests — update assertions for deprecated ops (expect errors now)
- Do NOT use `as any`-style hacks

VERIFY before reporting:
- `cargo test` — show pass count
- `cargo run --release -- test` — show pass count
- `cargo check -p lr-vm` — zero warnings
- Demonstrate each fix with a sample run
- Show exact lines changed

Report: bugs fixed, line numbers, test counts, `!!!` structure.
<!-- OMO_INTERNAL_INITIATOR -->

## [0123] 06-30 17:22

(2026-06-30T17:22:15)
give me an extremely exhaustive prompt for the whitt execution engine opencode workspace to implement what is needed for an orange pi build to get it running and workable on that specific hardware.  be exhaustive in your context and details and instructions on what it needs to do in that workspace only to make what is currently latest on the initial-creation branch.

## [0124] 06-30 17:35

(2026-06-30T17:35:46)
[search-mode]
MAXIMIZE SEARCH EFFORT. Launch multiple background agents IN PARALLEL:
- explore agents (codebase patterns, file structures, ast-grep)
- librarian agents (remote repos, official docs, GitHub examples)
Plus direct tools: Grep, ripgrep (rg), ast-grep (sg)
NEVER stop at first result - be exhaustive.

[analyze-mode]
ANALYSIS MODE. Gather context before diving deep:
CONTEXT GATHERING (parallel):
- 1-2 explore agents (codebase patterns, implementations)
- 1-2 librarian agents (if external library involved)
- Direct tools: Grep, AST-grep, LSP for targeted searches

IF COMPLEX - DO NOT STRUGGLE ALONE. Consult specialists:
- **Oracle**: Conventional problems (architecture, debugging, complex logic)
- **Artistry**: Non-conventional problems (different approach needed)

SYNTHESIZE findings before proceeding.
---
MANDATORY delegate_task params: ALWAYS include load_skills=[] and run_in_background when calling delegate_task.
Example: delegate_task(subagent_type="explore", prompt="...", run_in_background=true, load_skills=[])

---

<auto-slash-command>
# /ralph-loop Command

**Description**: (builtin) Start self-referential development loop until completion

**User Arguments**: so on this git workspace I'm wanting to get a new command with the cli that will install onto an orangepi 3w 12gb lpddr5 ram allwinner A733 to run locally on that board and to be installable from scripts and a new cli set of commands onto the device with a good .md doc explaining how to use the CLI and a well formatted easily skimmable way.  here is some info you need to be aware of as you go.  don't run anything outside of unit tests while you work.  on the other worktree im local system stress testing so the resources don't exist for you to live system test so make sure you only do local test runs of things as you write them and do so iteratively and critically review and fix and re evaluate before continuing on to the next task so you don't drift or think you've implemented something when in reality it won't work in real life when I get the device in the mail in a few weeks.

# Orange Pi Zero 3W (A733) Build — Exhaustive Implementation Prompt

**Target workspace**: `~/code/whitt-execution-engine/` (branch: `initial-creation`)
**Target hardware**: Orange Pi Zero 3W, Allwinner A733 SoC, 12GB LPDDR5, ARM64 (aarch64)
**Target OS**: Debian Bookworm minimal server, BSP kernel `6.6.98-sun60iw2` (NO Vulkan/GPU userspace)

---

## CONTEXT

### What this project is
whitt-execution-engine is a Rust CLI workflow engine that orchestrates LLM-powered YAML workflows. It talks to a llama.cpp HTTP server via OpenAI-compatible API. The engine itself is headless (no GUI). Currently hardcoded for x86_64 + AMD Vulkan GPU (RX 580). Needs to become portable to ARM64 CPU-only on an Allwinner A733 SBC with 12GB RAM.

### What the A733 hardware CAN and CANNOT do
- **CAN run**: ARM64 (aarch64) Linux with BSP kernel, headless workloads, CPU inference via llama.cpp
- **CANNOT do**: No Vulkan userspace on stock images (PowerVR BXM-4-64 has no open-source driver, no ICD). No GPU device nodes (`/dev/dri` doesn't exist). No X11/Wayland display server (headless only). No Docker with GPU passthrough.
- **Performance expectation**: llama.cpp CPU-only on 8 ARM cores (2×A76@2.0G + 6×A55@1.8G) with 12GB LPDDR5 @2400MHz 32-bit (~19.2 GB/s bandwidth). Expect ~1-4 tok/s on 3B models, ~0.5-1.5 tok/s on 7B-9B models. NOT fast. This is a Tier A/B micro-orchestrator, not a Tier C speed rig.

### Key constraint: CROSS-COMPILE
391 crates with LTO + codegen-units=1. Compiling on the 12GB ARM SBC would take hours and may OOM during linking. ALL Rust compilation must happen on the x86_64 host, then scp the binary to the SBC. llama.cpp server should be compiled natively on the SBC OR cross-compiled.

---

## EXHAUSTIVE FILE-BY-FILE CHANGES REQUIRED

Every file listed below needs specific modifications. DO NOT skip any file. DO NOT add new dependencies unless absolutely necessary. DO NOT restructure the crate — make surgical, minimal changes.

---

### 1. `Cargo.toml` (line 63)

**PROBLEM**: `llama-cpp-2 = { version = "0.1.138", features = ["vulkan"] }` — the `vulkan` feature is hardcoded, not optional. This forces Vulkan ICD detection at build time and runtime. On A733, Vulkan doesn't exist, so this will fail to build (no `libvulkan-dev` headers) or fail at runtime (no ICD JSON).

**REQUIRED CHANGES**:
1. Add a `vulkan` feature flag that gates the llama-cpp-2 vulkan feature:
   ```toml
   [features]
   default = []  # DO NOT make vulkan default anymore
   client = ["dep:reqwest", "dep:async-stream", "dep:futures", "dep:bytes"]
   sqlite = ["dep:rusqlite"]
   clipboard = ["dep:arboard"]
   vulkan = []  # NEW: enables Vulkan GPU acceleration
   ```

2. Change the llama-cpp-2 dependency to conditionally include vulkan:
   ```toml
   llama-cpp-2 = { version = "0.1.138", features = ["vulkan"], optional = true }
   ```

3. Add a non-optional CPU-only llama-cpp-2 dependency:
   ```toml
   [target.'cfg(not(feature = "vulkan"))'.dependencies]
   llama-cpp-2-cpu = { package = "llama-cpp-2", version = "0.1.138" }

   [target.'cfg(feature = "vulkan")'.dependencies]
   llama-cpp-2 = { version = "0.1.138", features = ["vulkan"] }
   ```

   **IMPORTANT ALTERNATIVE** (if the above target-cfg approach causes duplicate crate conflicts):
   Simply make the dependency conditional on the feature:
   ```toml
   llama-cpp-2 = { version = "0.1.138" }
   ```

   And use a cfg attribute in the Rust code to conditionally pass the vulkan init flag to llama-cpp-2. Check llama-cpp-2's API — the `vulkan` feature likely controls a build-time flag. If it can be set at runtime instead, just drop the feature entirely and handle Vulkan init in application code. **READ llama-cpp-2's docs/source to determine the correct approach before changing.**

**CRITICAL**: After this change, `cargo build` (no features) must produce a CPU-only binary. `cargo build --features vulkan` must produce a Vulkan-capable binary. Both must compile on x86_64. `cargo build --target aarch64-unknown-linux-gnu` (no features) must cross-compile for ARM64.

---

### 2. `src/backend/llama_vulkan.rs`

**PROBLEM**: Struct is named `LlamaCppVulkanBackend` and docs reference "Vulkan GPU acceleration" throughout. However, examining the actual code reveals this is just an HTTP client that talks to a llama.cpp server over localhost HTTP. It does NOT link Vulkan directly — it uses `llama-cpp-2` Rust bindings which handle the GPU connection internally. The naming is misleading.

**REQUIRED CHANGES**:
1. Rename the struct from `LlamaCppVulkanBackend` to `LlamaCppBackend` (or `LlamaCppCpuBackend` for the non-vulkan variant). **HOWEVER**: this is a PUBLIC API change that affects all imports. Check if this struct is used outside this crate (it isn't — it's a binary crate, not a library published on crates.io). So renaming is safe.

2. **SAFER APPROACH** (preserving API compat): Keep `LlamaCppVulkanBackend` as a type alias for backward compat:
   ```rust
   /// Llama.cpp backend with Vulkan support.
   #[derive(Debug, Clone)]
   pub struct LlamaCppBackend { /* same fields */ }

   /// Backward-compatible alias.
   #[deprecated(note = "Use LlamaCppBackend instead. This works with both CPU and Vulkan llama.cpp servers.")]
   pub type LlamaCppVulkanBackend = LlamaCppBackend;
   ```

3. Add conditional compilation for any llama-cpp-2 Vulkan-specific initialization. If the file uses `#[cfg(feature = "vulkan")]` to gate any code, add the appropriate `#[cfg(not(feature = "vulkan"))]` fallbacks.

4. Update all doc comments that say "Vulkan GPU acceleration" to say "llama.cpp backend (CPU or GPU)".

5. The HTTP client approach (talking to llama-server over HTTP) is correct for both CPU and Vulkan. No changes needed to the HTTP logic itself — llama-server handles backend selection internally.

---

### 3. `src/backend/mod.rs`

**PROBLEM**: Hardcodes `LlamaCppVulkanBackend` re-export.

**REQUIRED CHANGES**:
1. If you renamed the struct, update the re-export:
   ```rust
   pub use llama_vulkan::LlamaCppBackend;
   #[allow(deprecated)]
   pub use llama_vulkan::LlamaCppVulkanBackend;  // backward compat
   ```

2. Consider renaming `llama_vulkan.rs` to `llama_cpp.rs` to reflect the backend-agnostic nature. If you do, update the `pub mod` declaration. **Check all imports across the entire codebase first** using grep/lsp_find_references before renaming.

---

### 4. `config.yml` (root config file)

**PROBLEM**: Multiple settings are tuned for x86_64 + AMD Vulkan GPU with 16GB+ RAM:
- `context.size: 100000` — 100K context on 12GB LPDDR5 with a 9B model WILL OOM. 9B Q4_K_M ≈ 6GB model + KV cache. At 100K ctx with q8_0 cache, KV alone could be 3-4GB. 6GB + 4GB = 10GB, leaving 2GB for OS + everything else. Tight and likely OOM.
- `hardware.gpu_layers: 99` — No GPU on A733, this is meaningless.
- `hardware.threads: 6` — Should be 8 for the 8-core A733 (2×A76 + 6×A55).
- `hardware.flash_attn: true` — May or may not work in CPU mode. Check llama.cpp docs. If unsupported in CPU-only, set false.
- `vulkan.visible_devices: "0"` — No Vulkan on A733.

**REQUIRED CHANGES**:
1. Reduce `context.size` to `4096` or `8192` (safe for 12GB with 9B Q4; 16384 possible but risky with large models).
2. Set `hardware.gpu_layers: 0` (explicitly CPU-only).
3. Set `hardware.threads: 8` (use all A733 cores).
4. Remove or conditionally ignore the `vulkan:` section.
5. Consider reducing `cache.cache_type_k` and `cache.cache_type_v` from `q8_0` to `q4_0` to save memory on 12GB.

**HOWEVER**: Do NOT break the x86_64 Vulkan desktop config. The approach should be:
- Either create a separate `config.a733.yml` file with ARM-optimized settings
- Or make the config values overridable via environment variables (they already are in `.env.example`)
- Or add profile sections to config.yml: `profiles: { desktop: { ... }, a733: { ... } }`

**RECOMMENDED**: Create `configs/profiles/a733.yml` with:
```yaml
context:
  size: 8192
  batch_size: 512
  ubatch_size: 128
hardware:
  threads: 8
  gpu_layers: 0
  use_mmap: true
  flash_attn: false  # safer for CPU-only
cache:
  cache_type_k: q4_0  # save memory vs q8_0
  cache_type_v: q4_0
vulkan:
  visible_devices: ""  # disabled
```

---

### 5. `.env.example`

**PROBLEM**: `LLAMACPP_BACKEND=vulkan` and `LLAMACPP_GPU_LAYERS=99` are Vulkan-desktop defaults.

**REQUIRED CHANGES**:
1. Change `LLAMACPP_BACKEND=vulkan` to `LLAMACPP_BACKEND=cpu` as default (desktop users can override).
   - **OR**: Add comments showing both:
     ```
     # llama.cpp Backend
     # Options: cpu, vulkan, cuda, metal
     # Default: cpu (use vulkan for GPU-accelerated desktops)
     LLAMACPP_BACKEND=cpu
     ```
2. Change `LLAMACPP_GPU_LAYERS=99` to `LLAMACPP_GPU_LAYERS=0` as default.
3. Add `MAX_ALLOCATED_MEMORY_MB=8192` comment about reducing for SBCs:
   ```
   # For 12GB ARM SBCs: reduce to 8192 to leave room for OS
   ```

---

### 6. `docker/Dockerfile`

**PROBLEM**: Hardcoded for x86_64 + Vulkan:
- `FROM ghcr.io/ggml-org/llama.cpp:server-vulkan` — amd64-only image, won't build on ARM
- Line 58: `yq_linux_amd64` — wrong arch for A733
- Installs `libvulkan1`, `mesa-vulkan-drivers`, `vulkan-tools` — useless on A733
- `ENV AMD_VULKAN_ICD=RADV` — meaningless on ARM
- `GGML_VK_FORCE_MAX_ALLOCATION_SIZE` env var — Vulkan-only
- Validates `radeon_icd.json` exists — will fail on ARM

**REQUIRED CHANGES — TWO-PHASE APPROACH**:

**Phase A**: Create a new `docker/Dockerfile.aarch64` (DO NOT modify the existing Dockerfile — it works for desktop):
```dockerfile
# Stage 1: Build llama.cpp from source for aarch64 (CPU-only)
FROM ubuntu:24.04 AS builder

ARG LLAMA_CPP_VERSION=b4450

ENV DEBIAN_FRONTEND=noninteractive

RUN apt-get update && apt-get install -y --no-install-recommends \
    build-essential cmake git wget ca-certificates \
    && rm -rf /var/lib/apt/lists/*

# Build llama.cpp with CPU-only support (no Vulkan, no CUDA)
RUN git clone --depth 1 --branch master https://github.com/ggerganov/llama.cpp /tmp/llama.cpp \
    && cd /tmp/llama.cpp \
    && cmake -B build -DLLAMA_CURL=ON -DLLAMA_NATIVE=OFF -DLLAMA_ACCELERATE=OFF \
       -DLLAMA_BLAS=OFF -DLLAMA_VULKAN=OFF -DLLAMA_CUDA=OFF \
       -DLLAMA_METAL=OFF -DLLAMA_OPENMP=ON \
       -DCMAKE_BUILD_TYPE=Release \
    && cmake --build build --config Release -j$(nproc)

# Stage 2: Minimal runtime image
FROM ubuntu:24.04

ENV DEBIAN_FRONTEND=noninteractive
ENV TZ=UTC
ENV PYTHONUNBUFFERED=1
ENV HF_HUB_DISABLE_TELEMETRY=1

# Minimal runtime deps — NO Vulkan libs
RUN apt-get update && apt-get install -y --no-install-recommends \
    python3 \
    python3-pip \
    curl \
    tini \
    ca-certificates \
    libgomp1 \
    && rm -rf /var/lib/apt/lists/*

RUN pip3 install --no-cache-dir --break-system-packages huggingface-hub

# Install yq for aarch64 (NOT amd64)
RUN curl -fsSL https://github.com/mikefarah/yq/releases/latest/download/yq_linux_arm64 -o /usr/local/bin/yq && \
    chmod +x /usr/local/bin/yq

COPY --from=builder /tmp/llama.cpp/build/bin/llama-server /usr/local/bin/llama-server

RUN mkdir -p /models /config /var/log/llama-server /var/run/llama-server

WORKDIR /app
COPY docker/entrypoint.sh /entrypoint.sh
RUN chmod +x /entrypoint.sh

EXPOSE 8080

HEALTHCHECK --interval=30s --timeout=10s --start-period=60s --retries=3 \
    CMD curl -f http://localhost:8080/props || exit 1

VOLUME ["/models", "/config", "/var/log/llama-server"]

RUN useradd -m -u 1001 -s /bin/bash llama && \
    chown -R llama:llama /models /config /var/log/llama-server /app
USER llama

ENTRYPOINT ["tini", "--"]
CMD ["/entrypoint.sh"]
```

**Phase B**: In the existing `docker/Dockerfile`, add a multi-arch `ARG` for yq:
```dockerfile
ARG TARGETARCH
RUN if [ "$TARGETARCH" = "arm64" ]; then \
        curl -fsSL https://github.com/mikefarah/yq/releases/latest/download/yq_linux_arm64 -o /usr/local/bin/yq; \
    else \
        curl -fsSL https://github.com/mikefarah/yq/releases/latest/download/yq_linux_amd64 -o /usr/local/bin/yq; \
    fi && chmod +x /usr/local/bin/yq
```

---

### 7. `docker/docker-compose.yml`

**PROBLEM**: Hardcoded for desktop Vulkan GPU:
- `devices: /dev/dri:/dev/dri` — GPU passthrough, doesn't exist on A733
- `group_add: "${RENDER_GID:-987}"`, `"${VIDEO_GID:-983}"` — GPU group IDs
- `shm_size: 8g` — excessive for 12GB SBC
- `AMD_VULKAN_ICD: RADV` — Vulkan env var
- `GGML_VK_DISABLE_DEBUG: "1"` — Vulkan env var
- `RADV_PERFTEST: "nogttspill"` — AMD GPU performance tuning

**REQUIRED CHANGES**: Create `docker/docker-compose.aarch64.yml` (DO NOT modify the desktop compose file):
```yaml
services:
  llama-server:
    build:
      context: ..
      dockerfile: docker/Dockerfile.aarch64
    container_name: whitt-llama-server
    restart: unless-stopped
    volumes:
      - ../configs/profiles/a733.yml:/config/config.yml:ro
      - ../configs/models:/config/models:ro
      - ./entrypoint.sh:/entrypoint.sh:ro
      - ${MODEL_HOST_PATH:-../models}:/models
      - ../logs:/var/log/llama-server
    # NO devices: — no GPU on A733
    network_mode: host
    shm_size: 2g  # reduced for 12GB SBC
    user: "${DOCKER_UID:-1000}:${DOCKER_GID:-1000}"
    environment:
      HUGGING_FACE_HUB_TOKEN: ${HF_TOKEN:-}
      HF_HOME: /tmp/hf
      LLAMA_ARG_N_GPU_LAYERS: "0"  # explicit CPU-only
    healthcheck:
      test: ["CMD", "curl", "-f", "http://localhost:8080/props"]
      interval: 30s
      timeout: 10s
      retries: 3
      start_period: 120s  # longer startup for CPU inference
    logging:
      driver: "json-file"
      options:
        max-size: "10m"
        max-file: "3"

volumes:
  models:
    driver: local
  logs:
    driver: local

networks:
  default:
    name: whitt-network
```

---

### 8. `docker/entrypoint.sh`

**PROBLEM**: `validate_vulkan()` function (lines 400-460) checks for `/dev/dri`, radeon ICD, vulkaninfo — all of which fail on A733. The function does gracefully fall back to CPU mode (`export LLAMA_ARG_N_GPU_LAYERS="0"`), so it won't crash. But it prints misleading error messages.

**REQUIRED CHANGES**:
1. Add early detection for non-GPU environments. Before `validate_vulkan()`:
   ```bash
   # Detect non-GPU environment (ARM SBC without Vulkan)
   detect_hardware_profile() {
       local arch=$(uname -m)
       if [ "$arch" = "aarch64" ] || [ "$arch" = "arm64" ]; then
           if [ ! -d /dev/dri ]; then
               log_info "Detected ARM64 SBC without GPU device nodes — CPU-only mode."
               export LLAMA_ARG_N_GPU_LAYERS="0"
               return 0
           fi
       fi
       return 1
   }
   ```

2. In `main()`, call `detect_hardware_profile` BEFORE `validate_vulkan`. If it returns 0, skip `validate_vulkan` entirely.

3. Make `validate_vulkan` warnings non-fatal on aarch64 (they already are — it sets `N_GPU_LAYERS=0` and returns 0).

---

### 9. `.github/workflows/ci.yml`

**PROBLEM**: CI only runs on x86_64 (`ubuntu-latest`, `macos-latest`, `windows-latest`). No ARM64 target testing.

**REQUIRED CHANGES**:
1. Add an ARM64 cross-compile job to verify the code compiles for aarch64:
   ```yaml
   cross-compile-arm64:
     name: Cross-compile ARM64
     runs-on: ubuntu-latest
     steps:
       - uses: actions/checkout@v4
       - uses: dtolnay/rust-toolchain@stable
         with:
           targets: aarch64-unknown-linux-gnu
       - run: sudo apt-get update && sudo apt-get install -y gcc-aarch64-linux-gnu
       - run: cargo build --release --target aarch64-unknown-linux-gnu
       - run: cargo build --release --target aarch64-unknown-linux-gnu --features client
   ```

2. Add the `vulkan` feature build as a separate step (to ensure the feature gate works):
   ```yaml
   build-vulkan:
     name: Build (Vulkan)
     runs-on: ubuntu-latest
     steps:
       - uses: actions/checkout@v4
       - uses: dtolnay/rust-toolchain@stable
       - run: cargo build --release --features vulkan
   ```

3. Ensure clippy/check tests pass WITHOUT the vulkan feature (the default build should be CPU-only after your changes).

---

### 10. `configs/models/` (all 14 model config files)

**PROBLEM**: These are per-model configs with context sizes that may be too large for 12GB RAM. Some set `context.size: 8192` which is OK, but any that set larger need review.

**REQUIRED CHANGES**: 
- Review all 14 configs. Flag any with `context.size` > 8192 as "may OOM on 12GB ARM".
- No need to modify these files — they are per-model configs that users override at runtime. Just ensure the a733 profile in `configs/profiles/a733.yml` takes precedence.

---

### 11. `README.md`

**PROBLEM**: Quick Start only shows Vulkan desktop path. Prerequisites say "Docker with GPU support (Vulkan compatible)".

**REQUIRED CHANGES**:
1. Add ARM64 / A733 section to README:
   ```markdown
   ## ARM64 / SBC (CPU-only)

   # Cross-compile from x86_64 host
   cargo build --release --target aarch64-unknown-linux-gnu

   # Copy binary to SBC
   scp target/aarch64-unknown-linux-gnu/release/whitt user@opi-zero3w:~/

   # On the SBC: install runtime deps
   sudo apt install libstdc++ libgomp1 ca-certificates

   # Build llama.cpp natively (CPU-only)
   git clone https://github.com/ggerganov/llama.cpp && cd llama.cpp
   cmake -B build -DLLAMA_CURL=ON -DLLAMA_OPENMP=ON -DCMAKE_BUILD_TYPE=Release
   cmake --build build --config Release -j$(nproc)

   # Start server
   ./build/bin/llama-server -m /path/to/model.gguf -c 8192 -t 8 -ngl 0

   # Or use Docker (aarch64 image)
   docker compose -f docker/docker-compose.aarch64.yml up -d
   ```

2. Update Prerequisites to not require GPU:
   ```markdown
   ### Prerequisites
   - Rust 1.70+ (edition 2021)
   - Docker (optional, for llama.cpp server)
   - Linux or macOS
   - For GPU: Vulkan-compatible GPU with drivers (optional — CPU mode works on all platforms)
   ```

---

### 12. `src/config/provider.rs`

**PROBLEM**: Contains `LlamaCppVulkanProvider` struct and provider config. References Vulkan throughout doc comments.

**REQUIRED CHANGES**:
1. If struct was renamed in `llama_vulkan.rs`, update references here.
2. If keeping `LlamaCppVulkanBackend` as a type alias, add deprecation notice to `LlamaCppVulkanProvider` too:
   ```rust
   #[deprecated(note = "Use LlamaCppProvider instead. Works with CPU and GPU llama.cpp servers.")]
   pub type LlamaCppVulkanProvider = LlamaCppProvider;
   ```
3. Update doc comments to reflect "llama.cpp backend (CPU or GPU)".
4. **IMPORTANT**: Use `lsp_find_references` on `LlamaCppVulkanProvider` to find ALL usages before renaming. The workflow schema YAML uses `llama_cpp_with_vulkan` as a provider name — that string is user-facing config, not a Rust type. Keep backward compat.

---

## FILES YOU MUST READ BEFORE STARTING

Read these files to understand the full codebase before making changes:
1. `Cargo.toml` — full file
2. `src/backend/mod.rs` — full file (12 lines)
3. `src/backend/llama_vulkan.rs` — full file (647 lines)
4. `src/backend/llm_backend.rs` — full file (149 lines)
5. `src/config/provider.rs` — full file (489 lines)
6. `docker/entrypoint.sh` — full file (747 lines)
7. `docker/Dockerfile` — full file (101 lines)
8. `docker/docker-compose.yml` — full file (51 lines)
9. `.env.example` — full file (203 lines)
10. `config.yml` — full file (48 lines)
11. `.github/workflows/ci.yml` — full file (242 lines)
12. `README.md` — full file (174 lines)
13. ALL 14 files in `configs/models/` — to verify context sizes
14. `docs/contributing/environment-variables.md` — if it exists, may reference Vulkan env vars

Use `grep -r "vulkan" src/` to find ALL Vulkan references in Rust code.
Use `grep -r "vulkan" docker/` to find ALL Vulkan references in Docker files.
Use `grep -r "VULKAN\|vulkan" . --include="*.yml" --include="*.yaml"` to find ALL YAML references.
Use `grep -r "LlamaCppVulkan" src/` to find all struct/type references.
Use `grep -r "llama_cpp_with_vulkan" . --include="*.yml" --include="*.rs"` to find all provider name strings.

---

## VERIFICATION CHECKLIST

After ALL changes, verify:

1. **`cargo check`** — compiles without errors (CPU-only, no features)
2. **`cargo check --features vulkan`** — compiles with Vulkan feature enabled
3. **`cargo check --features client`** — compiles with client feature (HTTP client)
4. **`cargo check --features "client,vulkan"`** — compiles with both
5. **`cargo check --features "client"` with `--target aarch64-unknown-linux-gnu`** — cross-compile check (requires `gcc-aarch64-linux-gnu` installed)
6. **`cargo clippy --all-targets -- -D warnings`** — no clippy warnings
7. **`cargo fmt -- --check`** — formatting OK
8. **`cargo test`** — all tests pass (637 tests, 0 failures — do not introduce regressions)
9. **`cargo test --features client`** — client tests pass
10. **`lsp_diagnostics`** on every changed file — zero errors
11. **`docker build -f docker/Dockerfile.aarch64 .`** — Dockerfile builds (if Docker available)
12. **README links are valid** — no broken references

---

## THINGS YOU MUST NOT DO

1. **DO NOT** break the existing x86_64 Vulkan desktop workflow. All existing functionality must continue to work when `--features vulkan` is passed.
2. **DO NOT** change the llama.cpp HTTP API protocol. The engine talks HTTP to llama-server — this doesn't change between CPU and GPU.
3. **DO NOT** add new crate dependencies unless absolutely necessary for the CPU/GPU feature split.
4. **DO NOT** modify any test files. Tests mock the backend and don't care about Vulkan vs CPU.
5. **DO NOT** delete `docker/Dockerfile` or `docker/docker-compose.yml`. Create new `*.aarch64*` variants alongside them.
6. **DO NOT** change the default Rust edition, MSRV, or profile settings.
7. **DO NOT** use `as any`, `@ts-ignore`, or similar suppression. If something doesn't compile, fix it properly.
8. **DO NOT** commit anything. Just make the changes.
9. **DO NOT** build or run the project — just ensure it compiles and tests pass.

---

## BUILD CROSS-COMPILE TOOLCHAIN SETUP (for reference, not a task)

On the x86_64 host, to cross-compile for aarch64:
```bash
# Install cross-compile toolchain
sudo apt install gcc-aarch64-linux-gnu g++-aarch64-linux-gnu

# Add Rust target
rustup target add aarch64-unknown-linux-gnu

# Configure cargo for cross-compilation (in .cargo/config.toml)
# [target.aarch64-unknown-linux-gnu]
# linker = "aarch64-linux-gnu-gcc"

# Build CPU-only binary for ARM64
cargo build --release --target aarch64-unknown-linux-gnu

# Build with client features
cargo build --release --target aarch64-unknown-linux-gnu --features client
```

---

## ON THE SBC (reference, not a task)

Once the binary is deployed to the Orange Pi Zero 3W:
```bash
# Install runtime dependencies
sudo apt update
sudo apt install -y libstdc++6 libgomp1 ca-certificates

# Verify binary runs
./whitt --version || ./whitt --help

# Build llama.cpp natively (CPU-only, with OpenMP for multi-core)
sudo apt install -y build-essential cmake git wget
git clone --depth 1 https://github.com/ggerganov/llama.cpp
cd llama.cpp
cmake -B build \
  -DLLAMA_CURL=ON \
  -DLLAMA_OPENMP=ON \
  -DLLAMA_NATIVE=OFF \
  -DCMAKE_BUILD_TYPE=Release
cmake --build build --config Release -j$(nproc)

# Download a small model (SmolLM3 ~2GB Q4_K_M, fits easily in 12GB)
wget -O models/SmolLM3-Q4_K_M.gguf \
  "https://huggingface.co/./resolve/main/SmolLM3-Q4_K_M.gguf"

# Start server (CPU-only, 8 threads, 8K context)
./build/bin/llama-server \
  -m models/SmolLM3-Q4_K_M.gguf \
  -c 8192 \
  -t 8 \
  -ngl 0 \
  --host 127.0.0.1 \
  --port 8080

# In another terminal: test the engine
./whitt chat --model SmolLM3
```

---

## SUMMARY OF ALL FILES TO CREATE OR MODIFY

| File | Action | Priority |
|------|--------|----------|
| `Cargo.toml` | MODIFY — make vulkan feature optional, CPU default | P0 (blocks everything) |
| `src/backend/llama_vulkan.rs` | MODIFY — rename struct, update docs, add cfg gates | P0 |
| `src/backend/mod.rs` | MODIFY — update re-exports for renamed struct | P0 |
| `src/config/provider.rs` | MODIFY — rename provider struct, add type alias | P1 |
| `.env.example` | MODIFY — change defaults to CPU | P1 |
| `config.yml` | MODIFY — reduce context, set gpu_layers=0, threads=8 | P1 |
| `configs/profiles/a733.yml` | CREATE — ARM-optimized config profile | P1 |
| `docker/Dockerfile.aarch64` | CREATE — CPU-only ARM64 Docker image | P2 |
| `docker/docker-compose.aarch64.yml` | CREATE — ARM64 compose without GPU | P2 |
| `docker/entrypoint.sh` | MODIFY — add ARM detection, skip Vulkan validate | P2 |
| `.github/workflows/ci.yml` | MODIFY — add ARM64 cross-compile job | P2 |
| `README.md` | MODIFY — add ARM64/SBC section, update prerequisites | P3 |
| `docker/Dockerfile` | MODIFY — add TARGETARCH arg for yq download | P3 |
| `.cargo/config.toml` | CREATE — cross-compile linker config for aarch64 | P2 |

**Scope**: builtin

---

## Command Instructions

<command-instruction>
You are starting a Ralph Loop - a self-referential development loop that runs until task completion.

## How Ralph Loop Works

1. You will work on the task continuously
2. When you believe the task is FULLY complete, output: `<promise>{{COMPLETION_PROMISE}}</promise>`
3. If you don't output the promise, the loop will automatically inject another prompt to continue
4. Maximum iterations: Configurable (default 100)

## Rules

- Focus on completing the task fully, not partially
- Don't output the completion promise until the task is truly done
- Each iteration should make meaningful progress toward the goal
- If stuck, try different approaches
- Use todos to track your progress

## Exit Conditions

1. **Completion**: Output your completion promise tag when fully complete
2. **Max Iterations**: Loop stops automatically at limit
3. **Cancel**: User runs `/cancel-ralph` command

## Your Task

Parse the arguments below and begin working on the task. The format is:
`"task description" [--completion-promise=TEXT] [--max-iterations=N] [--strategy=reset|continue]`

Default completion promise is "DONE" and default max iterations is 100.
</command-instruction>

<user-task>
so on this git workspace I'm wanting to get a new command with the cli that will install onto an orangepi 3w 12gb lpddr5 ram allwinner A733 to run locally on that board and to be installable from scripts and a new cli set of commands onto the device with a good .md doc explaining how to use the CLI and a well formatted easily skimmable way.  here is some info you need to be aware of as you go.  don't run anything outside of unit tests while you work.  on the other worktree im local system stress testing so the resources don't exist for you to live system test so make sure you only do local test runs of things as you write them and do so iteratively and critically review and fix and re evaluate before continuing on to the next task so you don't drift or think you've implemented something when in reality it won't work in real life when I get the device in the mail in a few weeks.

# Orange Pi Zero 3W (A733) Build — Exhaustive Implementation Prompt

**Target workspace**: `~/code/whitt-execution-engine/` (branch: `initial-creation`)
**Target hardware**: Orange Pi Zero 3W, Allwinner A733 SoC, 12GB LPDDR5, ARM64 (aarch64)
**Target OS**: Debian Bookworm minimal server, BSP kernel `6.6.98-sun60iw2` (NO Vulkan/GPU userspace)

---

## CONTEXT

### What this project is
whitt-execution-engine is a Rust CLI workflow engine that orchestrates LLM-powered YAML workflows. It talks to a llama.cpp HTTP server via OpenAI-compatible API. The engine itself is headless (no GUI). Currently hardcoded for x86_64 + AMD Vulkan GPU (RX 580). Needs to become portable to ARM64 CPU-only on an Allwinner A733 SBC with 12GB RAM.

### What the A733 hardware CAN and CANNOT do
- **CAN run**: ARM64 (aarch64) Linux with BSP kernel, headless workloads, CPU inference via llama.cpp
- **CANNOT do**: No Vulkan userspace on stock images (PowerVR BXM-4-64 has no open-source driver, no ICD). No GPU device nodes (`/dev/dri` doesn't exist). No X11/Wayland display server (headless only). No Docker with GPU passthrough.
- **Performance expectation**: llama.cpp CPU-only on 8 ARM cores (2×A76@2.0G + 6×A55@1.8G) with 12GB LPDDR5 @2400MHz 32-bit (~19.2 GB/s bandwidth). Expect ~1-4 tok/s on 3B models, ~0.5-1.5 tok/s on 7B-9B models. NOT fast. This is a Tier A/B micro-orchestrator, not a Tier C speed rig.

### Key constraint: CROSS-COMPILE
391 crates with LTO + codegen-units=1. Compiling on the 12GB ARM SBC would take hours and may OOM during linking. ALL Rust compilation must happen on the x86_64 host, then scp the binary to the SBC. llama.cpp server should be compiled natively on the SBC OR cross-compiled.

---

## EXHAUSTIVE FILE-BY-FILE CHANGES REQUIRED

Every file listed below needs specific modifications. DO NOT skip any file. DO NOT add new dependencies unless absolutely necessary. DO NOT restructure the crate — make surgical, minimal changes.

---

### 1. `Cargo.toml` (line 63)

**PROBLEM**: `llama-cpp-2 = { version = "0.1.138", features = ["vulkan"] }` — the `vulkan` feature is hardcoded, not optional. This forces Vulkan ICD detection at build time and runtime. On A733, Vulkan doesn't exist, so this will fail to build (no `libvulkan-dev` headers) or fail at runtime (no ICD JSON).

**REQUIRED CHANGES**:
1. Add a `vulkan` feature flag that gates the llama-cpp-2 vulkan feature:
   ```toml
   [features]
   default = []  # DO NOT make vulkan default anymore
   client = ["dep:reqwest", "dep:async-stream", "dep:futures", "dep:bytes"]
   sqlite = ["dep:rusqlite"]
   clipboard = ["dep:arboard"]
   vulkan = []  # NEW: enables Vulkan GPU acceleration
   ```

2. Change the llama-cpp-2 dependency to conditionally include vulkan:
   ```toml
   llama-cpp-2 = { version = "0.1.138", features = ["vulkan"], optional = true }
   ```

3. Add a non-optional CPU-only llama-cpp-2 dependency:
   ```toml
   [target.'cfg(not(feature = "vulkan"))'.dependencies]
   llama-cpp-2-cpu = { package = "llama-cpp-2", version = "0.1.138" }

   [target.'cfg(feature = "vulkan")'.dependencies]
   llama-cpp-2 = { version = "0.1.138", features = ["vulkan"] }
   ```

   **IMPORTANT ALTERNATIVE** (if the above target-cfg approach causes duplicate crate conflicts):
   Simply make the dependency conditional on the feature:
   ```toml
   llama-cpp-2 = { version = "0.1.138" }
   ```

   And use a cfg attribute in the Rust code to conditionally pass the vulkan init flag to llama-cpp-2. Check llama-cpp-2's API — the `vulkan` feature likely controls a build-time flag. If it can be set at runtime instead, just drop the feature entirely and handle Vulkan init in application code. **READ llama-cpp-2's docs/source to determine the correct approach before changing.**

**CRITICAL**: After this change, `cargo build` (no features) must produce a CPU-only binary. `cargo build --features vulkan` must produce a Vulkan-capable binary. Both must compile on x86_64. `cargo build --target aarch64-unknown-linux-gnu` (no features) must cross-compile for ARM64.

---

### 2. `src/backend/llama_vulkan.rs`

**PROBLEM**: Struct is named `LlamaCppVulkanBackend` and docs reference "Vulkan GPU acceleration" throughout. However, examining the actual code reveals this is just an HTTP client that talks to a llama.cpp server over localhost HTTP. It does NOT link Vulkan directly — it uses `llama-cpp-2` Rust bindings which handle the GPU connection internally. The naming is misleading.

**REQUIRED CHANGES**:
1. Rename the struct from `LlamaCppVulkanBackend` to `LlamaCppBackend` (or `LlamaCppCpuBackend` for the non-vulkan variant). **HOWEVER**: this is a PUBLIC API change that affects all imports. Check if this struct is used outside this crate (it isn't — it's a binary crate, not a library published on crates.io). So renaming is safe.

2. **SAFER APPROACH** (preserving API compat): Keep `LlamaCppVulkanBackend` as a type alias for backward compat:
   ```rust
   /// Llama.cpp backend with Vulkan support.
   #[derive(Debug, Clone)]
   pub struct LlamaCppBackend { /* same fields */ }

   /// Backward-compatible alias.
   #[deprecated(note = "Use LlamaCppBackend instead. This works with both CPU and Vulkan llama.cpp servers.")]
   pub type LlamaCppVulkanBackend = LlamaCppBackend;
   ```

3. Add conditional compilation for any llama-cpp-2 Vulkan-specific initialization. If the file uses `#[cfg(feature = "vulkan")]` to gate any code, add the appropriate `#[cfg(not(feature = "vulkan"))]` fallbacks.

4. Update all doc comments that say "Vulkan GPU acceleration" to say "llama.cpp backend (CPU or GPU)".

5. The HTTP client approach (talking to llama-server over HTTP) is correct for both CPU and Vulkan. No changes needed to the HTTP logic itself — llama-server handles backend selection internally.

---

### 3. `src/backend/mod.rs`

**PROBLEM**: Hardcodes `LlamaCppVulkanBackend` re-export.

**REQUIRED CHANGES**:
1. If you renamed the struct, update the re-export:
   ```rust
   pub use llama_vulkan::LlamaCppBackend;
   #[allow(deprecated)]
   pub use llama_vulkan::LlamaCppVulkanBackend;  // backward compat
   ```

2. Consider renaming `llama_vulkan.rs` to `llama_cpp.rs` to reflect the backend-agnostic nature. If you do, update the `pub mod` declaration. **Check all imports across the entire codebase first** using grep/lsp_find_references before renaming.

---

### 4. `config.yml` (root config file)

**PROBLEM**: Multiple settings are tuned for x86_64 + AMD Vulkan GPU with 16GB+ RAM:
- `context.size: 100000` — 100K context on 12GB LPDDR5 with a 9B model WILL OOM. 9B Q4_K_M ≈ 6GB model + KV cache. At 100K ctx with q8_0 cache, KV alone could be 3-4GB. 6GB + 4GB = 10GB, leaving 2GB for OS + everything else. Tight and likely OOM.
- `hardware.gpu_layers: 99` — No GPU on A733, this is meaningless.
- `hardware.threads: 6` — Should be 8 for the 8-core A733 (2×A76 + 6×A55).
- `hardware.flash_attn: true` — May or may not work in CPU mode. Check llama.cpp docs. If unsupported in CPU-only, set false.
- `vulkan.visible_devices: "0"` — No Vulkan on A733.

**REQUIRED CHANGES**:
1. Reduce `context.size` to `4096` or `8192` (safe for 12GB with 9B Q4; 16384 possible but risky with large models).
2. Set `hardware.gpu_layers: 0` (explicitly CPU-only).
3. Set `hardware.threads: 8` (use all A733 cores).
4. Remove or conditionally ignore the `vulkan:` section.
5. Consider reducing `cache.cache_type_k` and `cache.cache_type_v` from `q8_0` to `q4_0` to save memory on 12GB.

**HOWEVER**: Do NOT break the x86_64 Vulkan desktop config. The approach should be:
- Either create a separate `config.a733.yml` file with ARM-optimized settings
- Or make the config values overridable via environment variables (they already are in `.env.example`)
- Or add profile sections to config.yml: `profiles: { desktop: { ... }, a733: { ... } }`

**RECOMMENDED**: Create `configs/profiles/a733.yml` with:
```yaml
context:
  size: 8192
  batch_size: 512
  ubatch_size: 128
hardware:
  threads: 8
  gpu_layers: 0
  use_mmap: true
  flash_attn: false  # safer for CPU-only
cache:
  cache_type_k: q4_0  # save memory vs q8_0
  cache_type_v: q4_0
vulkan:
  visible_devices: ""  # disabled
```

---

### 5. `.env.example`

**PROBLEM**: `LLAMACPP_BACKEND=vulkan` and `LLAMACPP_GPU_LAYERS=99` are Vulkan-desktop defaults.

**REQUIRED CHANGES**:
1. Change `LLAMACPP_BACKEND=vulkan` to `LLAMACPP_BACKEND=cpu` as default (desktop users can override).
   - **OR**: Add comments showing both:
     ```
     # llama.cpp Backend
     # Options: cpu, vulkan, cuda, metal
     # Default: cpu (use vulkan for GPU-accelerated desktops)
     LLAMACPP_BACKEND=cpu
     ```
2. Change `LLAMACPP_GPU_LAYERS=99` to `LLAMACPP_GPU_LAYERS=0` as default.
3. Add `MAX_ALLOCATED_MEMORY_MB=8192` comment about reducing for SBCs:
   ```
   # For 12GB ARM SBCs: reduce to 8192 to leave room for OS
   ```

---

### 6. `docker/Dockerfile`

**PROBLEM**: Hardcoded for x86_64 + Vulkan:
- `FROM ghcr.io/ggml-org/llama.cpp:server-vulkan` — amd64-only image, won't build on ARM
- Line 58: `yq_linux_amd64` — wrong arch for A733
- Installs `libvulkan1`, `mesa-vulkan-drivers`, `vulkan-tools` — useless on A733
- `ENV AMD_VULKAN_ICD=RADV` — meaningless on ARM
- `GGML_VK_FORCE_MAX_ALLOCATION_SIZE` env var — Vulkan-only
- Validates `radeon_icd.json` exists — will fail on ARM

**REQUIRED CHANGES — TWO-PHASE APPROACH**:

**Phase A**: Create a new `docker/Dockerfile.aarch64` (DO NOT modify the existing Dockerfile — it works for desktop):
```dockerfile
# Stage 1: Build llama.cpp from source for aarch64 (CPU-only)
FROM ubuntu:24.04 AS builder

ARG LLAMA_CPP_VERSION=b4450

ENV DEBIAN_FRONTEND=noninteractive

RUN apt-get update && apt-get install -y --no-install-recommends \
    build-essential cmake git wget ca-certificates \
    && rm -rf /var/lib/apt/lists/*

# Build llama.cpp with CPU-only support (no Vulkan, no CUDA)
RUN git clone --depth 1 --branch master https://github.com/ggerganov/llama.cpp /tmp/llama.cpp \
    && cd /tmp/llama.cpp \
    && cmake -B build -DLLAMA_CURL=ON -DLLAMA_NATIVE=OFF -DLLAMA_ACCELERATE=OFF \
       -DLLAMA_BLAS=OFF -DLLAMA_VULKAN=OFF -DLLAMA_CUDA=OFF \
       -DLLAMA_METAL=OFF -DLLAMA_OPENMP=ON \
       -DCMAKE_BUILD_TYPE=Release \
    && cmake --build build --config Release -j$(nproc)

# Stage 2: Minimal runtime image
FROM ubuntu:24.04

ENV DEBIAN_FRONTEND=noninteractive
ENV TZ=UTC
ENV PYTHONUNBUFFERED=1
ENV HF_HUB_DISABLE_TELEMETRY=1

# Minimal runtime deps — NO Vulkan libs
RUN apt-get update && apt-get install -y --no-install-recommends \
    python3 \
    python3-pip \
    curl \
    tini \
    ca-certificates \
    libgomp1 \
    && rm -rf /var/lib/apt/lists/*

RUN pip3 install --no-cache-dir --break-system-packages huggingface-hub

# Install yq for aarch64 (NOT amd64)
RUN curl -fsSL https://github.com/mikefarah/yq/releases/latest/download/yq_linux_arm64 -o /usr/local/bin/yq && \
    chmod +x /usr/local/bin/yq

COPY --from=builder /tmp/llama.cpp/build/bin/llama-server /usr/local/bin/llama-server

RUN mkdir -p /models /config /var/log/llama-server /var/run/llama-server

WORKDIR /app
COPY docker/entrypoint.sh /entrypoint.sh
RUN chmod +x /entrypoint.sh

EXPOSE 8080

HEALTHCHECK --interval=30s --timeout=10s --start-period=60s --retries=3 \
    CMD curl -f http://localhost:8080/props || exit 1

VOLUME ["/models", "/config", "/var/log/llama-server"]

RUN useradd -m -u 1001 -s /bin/bash llama && \
    chown -R llama:llama /models /config /var/log/llama-server /app
USER llama

ENTRYPOINT ["tini", "--"]
CMD ["/entrypoint.sh"]
```

**Phase B**: In the existing `docker/Dockerfile`, add a multi-arch `ARG` for yq:
```dockerfile
ARG TARGETARCH
RUN if [ "$TARGETARCH" = "arm64" ]; then \
        curl -fsSL https://github.com/mikefarah/yq/releases/latest/download/yq_linux_arm64 -o /usr/local/bin/yq; \
    else \
        curl -fsSL https://github.com/mikefarah/yq/releases/latest/download/yq_linux_amd64 -o /usr/local/bin/yq; \
    fi && chmod +x /usr/local/bin/yq
```

---

### 7. `docker/docker-compose.yml`

**PROBLEM**: Hardcoded for desktop Vulkan GPU:
- `devices: /dev/dri:/dev/dri` — GPU passthrough, doesn't exist on A733
- `group_add: "${RENDER_GID:-987}"`, `"${VIDEO_GID:-983}"` — GPU group IDs
- `shm_size: 8g` — excessive for 12GB SBC
- `AMD_VULKAN_ICD: RADV` — Vulkan env var
- `GGML_VK_DISABLE_DEBUG: "1"` — Vulkan env var
- `RADV_PERFTEST: "nogttspill"` — AMD GPU performance tuning

**REQUIRED CHANGES**: Create `docker/docker-compose.aarch64.yml` (DO NOT modify the desktop compose file):
```yaml
services:
  llama-server:
    build:
      context: ..
      dockerfile: docker/Dockerfile.aarch64
    container_name: whitt-llama-server
    restart: unless-stopped
    volumes:
      - ../configs/profiles/a733.yml:/config/config.yml:ro
      - ../configs/models:/config/models:ro
      - ./entrypoint.sh:/entrypoint.sh:ro
      - ${MODEL_HOST_PATH:-../models}:/models
      - ../logs:/var/log/llama-server
    # NO devices: — no GPU on A733
    network_mode: host
    shm_size: 2g  # reduced for 12GB SBC
    user: "${DOCKER_UID:-1000}:${DOCKER_GID:-1000}"
    environment:
      HUGGING_FACE_HUB_TOKEN: ${HF_TOKEN:-}
      HF_HOME: /tmp/hf
      LLAMA_ARG_N_GPU_LAYERS: "0"  # explicit CPU-only
    healthcheck:
      test: ["CMD", "curl", "-f", "http://localhost:8080/props"]
      interval: 30s
      timeout: 10s
      retries: 3
      start_period: 120s  # longer startup for CPU inference
    logging:
      driver: "json-file"
      options:
        max-size: "10m"
        max-file: "3"

volumes:
  models:
    driver: local
  logs:
    driver: local

networks:
  default:
    name: whitt-network
```

---

### 8. `docker/entrypoint.sh`

**PROBLEM**: `validate_vulkan()` function (lines 400-460) checks for `/dev/dri`, radeon ICD, vulkaninfo — all of which fail on A733. The function does gracefully fall back to CPU mode (`export LLAMA_ARG_N_GPU_LAYERS="0"`), so it won't crash. But it prints misleading error messages.

**REQUIRED CHANGES**:
1. Add early detection for non-GPU environments. Before `validate_vulkan()`:
   ```bash
   # Detect non-GPU environment (ARM SBC without Vulkan)
   detect_hardware_profile() {
       local arch=$(uname -m)
       if [ "$arch" = "aarch64" ] || [ "$arch" = "arm64" ]; then
           if [ ! -d /dev/dri ]; then
               log_info "Detected ARM64 SBC without GPU device nodes — CPU-only mode."
               export LLAMA_ARG_N_GPU_LAYERS="0"
               return 0
           fi
       fi
       return 1
   }
   ```

2. In `main()`, call `detect_hardware_profile` BEFORE `validate_vulkan`. If it returns 0, skip `validate_vulkan` entirely.

3. Make `validate_vulkan` warnings non-fatal on aarch64 (they already are — it sets `N_GPU_LAYERS=0` and returns 0).

---

### 9. `.github/workflows/ci.yml`

**PROBLEM**: CI only runs on x86_64 (`ubuntu-latest`, `macos-latest`, `windows-latest`). No ARM64 target testing.

**REQUIRED CHANGES**:
1. Add an ARM64 cross-compile job to verify the code compiles for aarch64:
   ```yaml
   cross-compile-arm64:
     name: Cross-compile ARM64
     runs-on: ubuntu-latest
     steps:
       - uses: actions/checkout@v4
       - uses: dtolnay/rust-toolchain@stable
         with:
           targets: aarch64-unknown-linux-gnu
       - run: sudo apt-get update && sudo apt-get install -y gcc-aarch64-linux-gnu
       - run: cargo build --release --target aarch64-unknown-linux-gnu
       - run: cargo build --release --target aarch64-unknown-linux-gnu --features client
   ```

2. Add the `vulkan` feature build as a separate step (to ensure the feature gate works):
   ```yaml
   build-vulkan:
     name: Build (Vulkan)
     runs-on: ubuntu-latest
     steps:
       - uses: actions/checkout@v4
       - uses: dtolnay/rust-toolchain@stable
       - run: cargo build --release --features vulkan
   ```

3. Ensure clippy/check tests pass WITHOUT the vulkan feature (the default build should be CPU-only after your changes).

---

### 10. `configs/models/` (all 14 model config files)

**PROBLEM**: These are per-model configs with context sizes that may be too large for 12GB RAM. Some set `context.size: 8192` which is OK, but any that set larger need review.

**REQUIRED CHANGES**: 
- Review all 14 configs. Flag any with `context.size` > 8192 as "may OOM on 12GB ARM".
- No need to modify these files — they are per-model configs that users override at runtime. Just ensure the a733 profile in `configs/profiles/a733.yml` takes precedence.

---

### 11. `README.md`

**PROBLEM**: Quick Start only shows Vulkan desktop path. Prerequisites say "Docker with GPU support (Vulkan compatible)".

**REQUIRED CHANGES**:
1. Add ARM64 / A733 section to README:
   ```markdown
   ## ARM64 / SBC (CPU-only)

   # Cross-compile from x86_64 host
   cargo build --release --target aarch64-unknown-linux-gnu

   # Copy binary to SBC
   scp target/aarch64-unknown-linux-gnu/release/whitt user@opi-zero3w:~/

   # On the SBC: install runtime deps
   sudo apt install libstdc++ libgomp1 ca-certificates

   # Build llama.cpp natively (CPU-only)
   git clone https://github.com/ggerganov/llama.cpp && cd llama.cpp
   cmake -B build -DLLAMA_CURL=ON -DLLAMA_OPENMP=ON -DCMAKE_BUILD_TYPE=Release
   cmake --build build --config Release -j$(nproc)

   # Start server
   ./build/bin/llama-server -m /path/to/model.gguf -c 8192 -t 8 -ngl 0

   # Or use Docker (aarch64 image)
   docker compose -f docker/docker-compose.aarch64.yml up -d
   ```

2. Update Prerequisites to not require GPU:
   ```markdown
   ### Prerequisites
   - Rust 1.70+ (edition 2021)
   - Docker (optional, for llama.cpp server)
   - Linux or macOS
   - For GPU: Vulkan-compatible GPU with drivers (optional — CPU mode works on all platforms)
   ```

---

### 12. `src/config/provider.rs`

**PROBLEM**: Contains `LlamaCppVulkanProvider` struct and provider config. References Vulkan throughout doc comments.

**REQUIRED CHANGES**:
1. If struct was renamed in `llama_vulkan.rs`, update references here.
2. If keeping `LlamaCppVulkanBackend` as a type alias, add deprecation notice to `LlamaCppVulkanProvider` too:
   ```rust
   #[deprecated(note = "Use LlamaCppProvider instead. Works with CPU and GPU llama.cpp servers.")]
   pub type LlamaCppVulkanProvider = LlamaCppProvider;
   ```
3. Update doc comments to reflect "llama.cpp backend (CPU or GPU)".
4. **IMPORTANT**: Use `lsp_find_references` on `LlamaCppVulkanProvider` to find ALL usages before renaming. The workflow schema YAML uses `llama_cpp_with_vulkan` as a provider name — that string is user-facing config, not a Rust type. Keep backward compat.

---

## FILES YOU MUST READ BEFORE STARTING

Read these files to understand the full codebase before making changes:
1. `Cargo.toml` — full file
2. `src/backend/mod.rs` — full file (12 lines)
3. `src/backend/llama_vulkan.rs` — full file (647 lines)
4. `src/backend/llm_backend.rs` — full file (149 lines)
5. `src/config/provider.rs` — full file (489 lines)
6. `docker/entrypoint.sh` — full file (747 lines)
7. `docker/Dockerfile` — full file (101 lines)
8. `docker/docker-compose.yml` — full file (51 lines)
9. `.env.example` — full file (203 lines)
10. `config.yml` — full file (48 lines)
11. `.github/workflows/ci.yml` — full file (242 lines)
12. `README.md` — full file (174 lines)
13. ALL 14 files in `configs/models/` — to verify context sizes
14. `docs/contributing/environment-variables.md` — if it exists, may reference Vulkan env vars

Use `grep -r "vulkan" src/` to find ALL Vulkan references in Rust code.
Use `grep -r "vulkan" docker/` to find ALL Vulkan references in Docker files.
Use `grep -r "VULKAN\|vulkan" . --include="*.yml" --include="*.yaml"` to find ALL YAML references.
Use `grep -r "LlamaCppVulkan" src/` to find all struct/type references.
Use `grep -r "llama_cpp_with_vulkan" . --include="*.yml" --include="*.rs"` to find all provider name strings.

---

## VERIFICATION CHECKLIST

After ALL changes, verify:

1. **`cargo check`** — compiles without errors (CPU-only, no features)
2. **`cargo check --features vulkan`** — compiles with Vulkan feature enabled
3. **`cargo check --features client`** — compiles with client feature (HTTP client)
4. **`cargo check --features "client,vulkan"`** — compiles with both
5. **`cargo check --features "client"` with `--target aarch64-unknown-linux-gnu`** — cross-compile check (requires `gcc-aarch64-linux-gnu` installed)
6. **`cargo clippy --all-targets -- -D warnings`** — no clippy warnings
7. **`cargo fmt -- --check`** — formatting OK
8. **`cargo test`** — all tests pass (637 tests, 0 failures — do not introduce regressions)
9. **`cargo test --features client`** — client tests pass
10. **`lsp_diagnostics`** on every changed file — zero errors
11. **`docker build -f docker/Dockerfile.aarch64 .`** — Dockerfile builds (if Docker available)
12. **README links are valid** — no broken references

---

## THINGS YOU MUST NOT DO

1. **DO NOT** break the existing x86_64 Vulkan desktop workflow. All existing functionality must continue to work when `--features vulkan` is passed.
2. **DO NOT** change the llama.cpp HTTP API protocol. The engine talks HTTP to llama-server — this doesn't change between CPU and GPU.
3. **DO NOT** add new crate dependencies unless absolutely necessary for the CPU/GPU feature split.
4. **DO NOT** modify any test files. Tests mock the backend and don't care about Vulkan vs CPU.
5. **DO NOT** delete `docker/Dockerfile` or `docker/docker-compose.yml`. Create new `*.aarch64*` variants alongside them.
6. **DO NOT** change the default Rust edition, MSRV, or profile settings.
7. **DO NOT** use `as any`, `@ts-ignore`, or similar suppression. If something doesn't compile, fix it properly.
8. **DO NOT** commit anything. Just make the changes.
9. **DO NOT** build or run the project — just ensure it compiles and tests pass.

---

## BUILD CROSS-COMPILE TOOLCHAIN SETUP (for reference, not a task)

On the x86_64 host, to cross-compile for aarch64:
```bash
# Install cross-compile toolchain
sudo apt install gcc-aarch64-linux-gnu g++-aarch64-linux-gnu

# Add Rust target
rustup target add aarch64-unknown-linux-gnu

# Configure cargo for cross-compilation (in .cargo/config.toml)
# [target.aarch64-unknown-linux-gnu]
# linker = "aarch64-linux-gnu-gcc"

# Build CPU-only binary for ARM64
cargo build --release --target aarch64-unknown-linux-gnu

# Build with client features
cargo build --release --target aarch64-unknown-linux-gnu --features client
```

---

## ON THE SBC (reference, not a task)

Once the binary is deployed to the Orange Pi Zero 3W:
```bash
# Install runtime dependencies
sudo apt update
sudo apt install -y libstdc++6 libgomp1 ca-certificates

# Verify binary runs
./whitt --version || ./whitt --help

# Build llama.cpp natively (CPU-only, with OpenMP for multi-core)
sudo apt install -y build-essential cmake git wget
git clone --depth 1 https://github.com/ggerganov/llama.cpp
cd llama.cpp
cmake -B build \
  -DLLAMA_CURL=ON \
  -DLLAMA_OPENMP=ON \
  -DLLAMA_NATIVE=OFF \
  -DCMAKE_BUILD_TYPE=Release
cmake --build build --config Release -j$(nproc)

# Download a small model (SmolLM3 ~2GB Q4_K_M, fits easily in 12GB)
wget -O models/SmolLM3-Q4_K_M.gguf \
  "https://huggingface.co/./resolve/main/SmolLM3-Q4_K_M.gguf"

# Start server (CPU-only, 8 threads, 8K context)
./build/bin/llama-server \
  -m models/SmolLM3-Q4_K_M.gguf \
  -c 8192 \
  -t 8 \
  -ngl 0 \
  --host 127.0.0.1 \
  --port 8080

# In another terminal: test the engine
./whitt chat --model SmolLM3
```

---

## SUMMARY OF ALL FILES TO CREATE OR MODIFY

| File | Action | Priority |
|------|--------|----------|
| `Cargo.toml` | MODIFY — make vulkan feature optional, CPU default | P0 (blocks everything) |
| `src/backend/llama_vulkan.rs` | MODIFY — rename struct, update docs, add cfg gates | P0 |
| `src/backend/mod.rs` | MODIFY — update re-exports for renamed struct | P0 |
| `src/config/provider.rs` | MODIFY — rename provider struct, add type alias | P1 |
| `.env.example` | MODIFY — change defaults to CPU | P1 |
| `config.yml` | MODIFY — reduce context, set gpu_layers=0, threads=8 | P1 |
| `configs/profiles/a733.yml` | CREATE — ARM-optimized config profile | P1 |
| `docker/Dockerfile.aarch64` | CREATE — CPU-only ARM64 Docker image | P2 |
| `docker/docker-compose.aarch64.yml` | CREATE — ARM64 compose without GPU | P2 |
| `docker/entrypoint.sh` | MODIFY — add ARM detection, skip Vulkan validate | P2 |
| `.github/workflows/ci.yml` | MODIFY — add ARM64 cross-compile job | P2 |
| `README.md` | MODIFY — add ARM64/SBC section, update prerequisites | P3 |
| `docker/Dockerfile` | MODIFY — add TARGETARCH arg for yq download | P3 |
| `.cargo/config.toml` | CREATE — cross-compile linker config for aarch64 | P2 |
</user-task>


---

## User Request

so on this git workspace I'm wanting to get a new command with the cli that will install onto an orangepi 3w 12gb lpddr5 ram allwinner A733 to run locally on that board and to be installable from scripts and a new cli set of commands onto the device with a good .md doc explaining how to use the CLI and a well formatted easily skimmable way.  here is some info you need to be aware of as you go.  don't run anything outside of unit tests while you work.  on the other worktree im local system stress testing so the resources don't exist for you to live system test so make sure you only do local test runs of things as you write them and do so iteratively and critically review and fix and re evaluate before continuing on to the next task so you don't drift or think you've implemented something when in reality it won't work in real life when I get the device in the mail in a few weeks.

# Orange Pi Zero 3W (A733) Build — Exhaustive Implementation Prompt

**Target workspace**: `~/code/whitt-execution-engine/` (branch: `initial-creation`)
**Target hardware**: Orange Pi Zero 3W, Allwinner A733 SoC, 12GB LPDDR5, ARM64 (aarch64)
**Target OS**: Debian Bookworm minimal server, BSP kernel `6.6.98-sun60iw2` (NO Vulkan/GPU userspace)

---

## CONTEXT

### What this project is
whitt-execution-engine is a Rust CLI workflow engine that orchestrates LLM-powered YAML workflows. It talks to a llama.cpp HTTP server via OpenAI-compatible API. The engine itself is headless (no GUI). Currently hardcoded for x86_64 + AMD Vulkan GPU (RX 580). Needs to become portable to ARM64 CPU-only on an Allwinner A733 SBC with 12GB RAM.

### What the A733 hardware CAN and CANNOT do
- **CAN run**: ARM64 (aarch64) Linux with BSP kernel, headless workloads, CPU inference via llama.cpp
- **CANNOT do**: No Vulkan userspace on stock images (PowerVR BXM-4-64 has no open-source driver, no ICD). No GPU device nodes (`/dev/dri` doesn't exist). No X11/Wayland display server (headless only). No Docker with GPU passthrough.
- **Performance expectation**: llama.cpp CPU-only on 8 ARM cores (2×A76@2.0G + 6×A55@1.8G) with 12GB LPDDR5 @2400MHz 32-bit (~19.2 GB/s bandwidth). Expect ~1-4 tok/s on 3B models, ~0.5-1.5 tok/s on 7B-9B models. NOT fast. This is a Tier A/B micro-orchestrator, not a Tier C speed rig.

### Key constraint: CROSS-COMPILE
391 crates with LTO + codegen-units=1. Compiling on the 12GB ARM SBC would take hours and may OOM during linking. ALL Rust compilation must happen on the x86_64 host, then scp the binary to the SBC. llama.cpp server should be compiled natively on the SBC OR cross-compiled.

---

## EXHAUSTIVE FILE-BY-FILE CHANGES REQUIRED

Every file listed below needs specific modifications. DO NOT skip any file. DO NOT add new dependencies unless absolutely necessary. DO NOT restructure the crate — make surgical, minimal changes.

---

### 1. `Cargo.toml` (line 63)

**PROBLEM**: `llama-cpp-2 = { version = "0.1.138", features = ["vulkan"] }` — the `vulkan` feature is hardcoded, not optional. This forces Vulkan ICD detection at build time and runtime. On A733, Vulkan doesn't exist, so this will fail to build (no `libvulkan-dev` headers) or fail at runtime (no ICD JSON).

**REQUIRED CHANGES**:
1. Add a `vulkan` feature flag that gates the llama-cpp-2 vulkan feature:
   ```toml
   [features]
   default = []  # DO NOT make vulkan default anymore
   client = ["dep:reqwest", "dep:async-stream", "dep:futures", "dep:bytes"]
   sqlite = ["dep:rusqlite"]
   clipboard = ["dep:arboard"]
   vulkan = []  # NEW: enables Vulkan GPU acceleration
   ```

2. Change the llama-cpp-2 dependency to conditionally include vulkan:
   ```toml
   llama-cpp-2 = { version = "0.1.138", features = ["vulkan"], optional = true }
   ```

3. Add a non-optional CPU-only llama-cpp-2 dependency:
   ```toml
   [target.'cfg(not(feature = "vulkan"))'.dependencies]
   llama-cpp-2-cpu = { package = "llama-cpp-2", version = "0.1.138" }

   [target.'cfg(feature = "vulkan")'.dependencies]
   llama-cpp-2 = { version = "0.1.138", features = ["vulkan"] }
   ```

   **IMPORTANT ALTERNATIVE** (if the above target-cfg approach causes duplicate crate conflicts):
   Simply make the dependency conditional on the feature:
   ```toml
   llama-cpp-2 = { version = "0.1.138" }
   ```

   And use a cfg attribute in the Rust code to conditionally pass the vulkan init flag to llama-cpp-2. Check llama-cpp-2's API — the `vulkan` feature likely controls a build-time flag. If it can be set at runtime instead, just drop the feature entirely and handle Vulkan init in application code. **READ llama-cpp-2's docs/source to determine the correct approach before changing.**

**CRITICAL**: After this change, `cargo build` (no features) must produce a CPU-only binary. `cargo build --features vulkan` must produce a Vulkan-capable binary. Both must compile on x86_64. `cargo build --target aarch64-unknown-linux-gnu` (no features) must cross-compile for ARM64.

---

### 2. `src/backend/llama_vulkan.rs`

**PROBLEM**: Struct is named `LlamaCppVulkanBackend` and docs reference "Vulkan GPU acceleration" throughout. However, examining the actual code reveals this is just an HTTP client that talks to a llama.cpp server over localhost HTTP. It does NOT link Vulkan directly — it uses `llama-cpp-2` Rust bindings which handle the GPU connection internally. The naming is misleading.

**REQUIRED CHANGES**:
1. Rename the struct from `LlamaCppVulkanBackend` to `LlamaCppBackend` (or `LlamaCppCpuBackend` for the non-vulkan variant). **HOWEVER**: this is a PUBLIC API change that affects all imports. Check if this struct is used outside this crate (it isn't — it's a binary crate, not a library published on crates.io). So renaming is safe.

2. **SAFER APPROACH** (preserving API compat): Keep `LlamaCppVulkanBackend` as a type alias for backward compat:
   ```rust
   /// Llama.cpp backend with Vulkan support.
   #[derive(Debug, Clone)]
   pub struct LlamaCppBackend { /* same fields */ }

   /// Backward-compatible alias.
   #[deprecated(note = "Use LlamaCppBackend instead. This works with both CPU and Vulkan llama.cpp servers.")]
   pub type LlamaCppVulkanBackend = LlamaCppBackend;
   ```

3. Add conditional compilation for any llama-cpp-2 Vulkan-specific initialization. If the file uses `#[cfg(feature = "vulkan")]` to gate any code, add the appropriate `#[cfg(not(feature = "vulkan"))]` fallbacks.

4. Update all doc comments that say "Vulkan GPU acceleration" to say "llama.cpp backend (CPU or GPU)".

5. The HTTP client approach (talking to llama-server over HTTP) is correct for both CPU and Vulkan. No changes needed to the HTTP logic itself — llama-server handles backend selection internally.

---

### 3. `src/backend/mod.rs`

**PROBLEM**: Hardcodes `LlamaCppVulkanBackend` re-export.

**REQUIRED CHANGES**:
1. If you renamed the struct, update the re-export:
   ```rust
   pub use llama_vulkan::LlamaCppBackend;
   #[allow(deprecated)]
   pub use llama_vulkan::LlamaCppVulkanBackend;  // backward compat
   ```

2. Consider renaming `llama_vulkan.rs` to `llama_cpp.rs` to reflect the backend-agnostic nature. If you do, update the `pub mod` declaration. **Check all imports across the entire codebase first** using grep/lsp_find_references before renaming.

---

### 4. `config.yml` (root config file)

**PROBLEM**: Multiple settings are tuned for x86_64 + AMD Vulkan GPU with 16GB+ RAM:
- `context.size: 100000` — 100K context on 12GB LPDDR5 with a 9B model WILL OOM. 9B Q4_K_M ≈ 6GB model + KV cache. At 100K ctx with q8_0 cache, KV alone could be 3-4GB. 6GB + 4GB = 10GB, leaving 2GB for OS + everything else. Tight and likely OOM.
- `hardware.gpu_layers: 99` — No GPU on A733, this is meaningless.
- `hardware.threads: 6` — Should be 8 for the 8-core A733 (2×A76 + 6×A55).
- `hardware.flash_attn: true` — May or may not work in CPU mode. Check llama.cpp docs. If unsupported in CPU-only, set false.
- `vulkan.visible_devices: "0"` — No Vulkan on A733.

**REQUIRED CHANGES**:
1. Reduce `context.size` to `4096` or `8192` (safe for 12GB with 9B Q4; 16384 possible but risky with large models).
2. Set `hardware.gpu_layers: 0` (explicitly CPU-only).
3. Set `hardware.threads: 8` (use all A733 cores).
4. Remove or conditionally ignore the `vulkan:` section.
5. Consider reducing `cache.cache_type_k` and `cache.cache_type_v` from `q8_0` to `q4_0` to save memory on 12GB.

**HOWEVER**: Do NOT break the x86_64 Vulkan desktop config. The approach should be:
- Either create a separate `config.a733.yml` file with ARM-optimized settings
- Or make the config values overridable via environment variables (they already are in `.env.example`)
- Or add profile sections to config.yml: `profiles: { desktop: { ... }, a733: { ... } }`

**RECOMMENDED**: Create `configs/profiles/a733.yml` with:
```yaml
context:
  size: 8192
  batch_size: 512
  ubatch_size: 128
hardware:
  threads: 8
  gpu_layers: 0
  use_mmap: true
  flash_attn: false  # safer for CPU-only
cache:
  cache_type_k: q4_0  # save memory vs q8_0
  cache_type_v: q4_0
vulkan:
  visible_devices: ""  # disabled
```

---

### 5. `.env.example`

**PROBLEM**: `LLAMACPP_BACKEND=vulkan` and `LLAMACPP_GPU_LAYERS=99` are Vulkan-desktop defaults.

**REQUIRED CHANGES**:
1. Change `LLAMACPP_BACKEND=vulkan` to `LLAMACPP_BACKEND=cpu` as default (desktop users can override).
   - **OR**: Add comments showing both:
     ```
     # llama.cpp Backend
     # Options: cpu, vulkan, cuda, metal
     # Default: cpu (use vulkan for GPU-accelerated desktops)
     LLAMACPP_BACKEND=cpu
     ```
2. Change `LLAMACPP_GPU_LAYERS=99` to `LLAMACPP_GPU_LAYERS=0` as default.
3. Add `MAX_ALLOCATED_MEMORY_MB=8192` comment about reducing for SBCs:
   ```
   # For 12GB ARM SBCs: reduce to 8192 to leave room for OS
   ```

---

### 6. `docker/Dockerfile`

**PROBLEM**: Hardcoded for x86_64 + Vulkan:
- `FROM ghcr.io/ggml-org/llama.cpp:server-vulkan` — amd64-only image, won't build on ARM
- Line 58: `yq_linux_amd64` — wrong arch for A733
- Installs `libvulkan1`, `mesa-vulkan-drivers`, `vulkan-tools` — useless on A733
- `ENV AMD_VULKAN_ICD=RADV` — meaningless on ARM
- `GGML_VK_FORCE_MAX_ALLOCATION_SIZE` env var — Vulkan-only
- Validates `radeon_icd.json` exists — will fail on ARM

**REQUIRED CHANGES — TWO-PHASE APPROACH**:

**Phase A**: Create a new `docker/Dockerfile.aarch64` (DO NOT modify the existing Dockerfile — it works for desktop):
```dockerfile
# Stage 1: Build llama.cpp from source for aarch64 (CPU-only)
FROM ubuntu:24.04 AS builder

ARG LLAMA_CPP_VERSION=b4450

ENV DEBIAN_FRONTEND=noninteractive

RUN apt-get update && apt-get install -y --no-install-recommends \
    build-essential cmake git wget ca-certificates \
    && rm -rf /var/lib/apt/lists/*

# Build llama.cpp with CPU-only support (no Vulkan, no CUDA)
RUN git clone --depth 1 --branch master https://github.com/ggerganov/llama.cpp /tmp/llama.cpp \
    && cd /tmp/llama.cpp \
    && cmake -B build -DLLAMA_CURL=ON -DLLAMA_NATIVE=OFF -DLLAMA_ACCELERATE=OFF \
       -DLLAMA_BLAS=OFF -DLLAMA_VULKAN=OFF -DLLAMA_CUDA=OFF \
       -DLLAMA_METAL=OFF -DLLAMA_OPENMP=ON \
       -DCMAKE_BUILD_TYPE=Release \
    && cmake --build build --config Release -j$(nproc)

# Stage 2: Minimal runtime image
FROM ubuntu:24.04

ENV DEBIAN_FRONTEND=noninteractive
ENV TZ=UTC
ENV PYTHONUNBUFFERED=1
ENV HF_HUB_DISABLE_TELEMETRY=1

# Minimal runtime deps — NO Vulkan libs
RUN apt-get update && apt-get install -y --no-install-recommends \
    python3 \
    python3-pip \
    curl \
    tini \
    ca-certificates \
    libgomp1 \
    && rm -rf /var/lib/apt/lists/*

RUN pip3 install --no-cache-dir --break-system-packages huggingface-hub

# Install yq for aarch64 (NOT amd64)
RUN curl -fsSL https://github.com/mikefarah/yq/releases/latest/download/yq_linux_arm64 -o /usr/local/bin/yq && \
    chmod +x /usr/local/bin/yq

COPY --from=builder /tmp/llama.cpp/build/bin/llama-server /usr/local/bin/llama-server

RUN mkdir -p /models /config /var/log/llama-server /var/run/llama-server

WORKDIR /app
COPY docker/entrypoint.sh /entrypoint.sh
RUN chmod +x /entrypoint.sh

EXPOSE 8080

HEALTHCHECK --interval=30s --timeout=10s --start-period=60s --retries=3 \
    CMD curl -f http://localhost:8080/props || exit 1

VOLUME ["/models", "/config", "/var/log/llama-server"]

RUN useradd -m -u 1001 -s /bin/bash llama && \
    chown -R llama:llama /models /config /var/log/llama-server /app
USER llama

ENTRYPOINT ["tini", "--"]
CMD ["/entrypoint.sh"]
```

**Phase B**: In the existing `docker/Dockerfile`, add a multi-arch `ARG` for yq:
```dockerfile
ARG TARGETARCH
RUN if [ "$TARGETARCH" = "arm64" ]; then \
        curl -fsSL https://github.com/mikefarah/yq/releases/latest/download/yq_linux_arm64 -o /usr/local/bin/yq; \
    else \
        curl -fsSL https://github.com/mikefarah/yq/releases/latest/download/yq_linux_amd64 -o /usr/local/bin/yq; \
    fi && chmod +x /usr/local/bin/yq
```

---

### 7. `docker/docker-compose.yml`

**PROBLEM**: Hardcoded for desktop Vulkan GPU:
- `devices: /dev/dri:/dev/dri` — GPU passthrough, doesn't exist on A733
- `group_add: "${RENDER_GID:-987}"`, `"${VIDEO_GID:-983}"` — GPU group IDs
- `shm_size: 8g` — excessive for 12GB SBC
- `AMD_VULKAN_ICD: RADV` — Vulkan env var
- `GGML_VK_DISABLE_DEBUG: "1"` — Vulkan env var
- `RADV_PERFTEST: "nogttspill"` — AMD GPU performance tuning

**REQUIRED CHANGES**: Create `docker/docker-compose.aarch64.yml` (DO NOT modify the desktop compose file):
```yaml
services:
  llama-server:
    build:
      context: ..
      dockerfile: docker/Dockerfile.aarch64
    container_name: whitt-llama-server
    restart: unless-stopped
    volumes:
      - ../configs/profiles/a733.yml:/config/config.yml:ro
      - ../configs/models:/config/models:ro
      - ./entrypoint.sh:/entrypoint.sh:ro
      - ${MODEL_HOST_PATH:-../models}:/models
      - ../logs:/var/log/llama-server
    # NO devices: — no GPU on A733
    network_mode: host
    shm_size: 2g  # reduced for 12GB SBC
    user: "${DOCKER_UID:-1000}:${DOCKER_GID:-1000}"
    environment:
      HUGGING_FACE_HUB_TOKEN: ${HF_TOKEN:-}
      HF_HOME: /tmp/hf
      LLAMA_ARG_N_GPU_LAYERS: "0"  # explicit CPU-only
    healthcheck:
      test: ["CMD", "curl", "-f", "http://localhost:8080/props"]
      interval: 30s
      timeout: 10s
      retries: 3
      start_period: 120s  # longer startup for CPU inference
    logging:
      driver: "json-file"
      options:
        max-size: "10m"
        max-file: "3"

volumes:
  models:
    driver: local
  logs:
    driver: local

networks:
  default:
    name: whitt-network
```

---

### 8. `docker/entrypoint.sh`

**PROBLEM**: `validate_vulkan()` function (lines 400-460) checks for `/dev/dri`, radeon ICD, vulkaninfo — all of which fail on A733. The function does gracefully fall back to CPU mode (`export LLAMA_ARG_N_GPU_LAYERS="0"`), so it won't crash. But it prints misleading error messages.

**REQUIRED CHANGES**:
1. Add early detection for non-GPU environments. Before `validate_vulkan()`:
   ```bash
   # Detect non-GPU environment (ARM SBC without Vulkan)
   detect_hardware_profile() {
       local arch=$(uname -m)
       if [ "$arch" = "aarch64" ] || [ "$arch" = "arm64" ]; then
           if [ ! -d /dev/dri ]; then
               log_info "Detected ARM64 SBC without GPU device nodes — CPU-only mode."
               export LLAMA_ARG_N_GPU_LAYERS="0"
               return 0
           fi
       fi
       return 1
   }
   ```

2. In `main()`, call `detect_hardware_profile` BEFORE `validate_vulkan`. If it returns 0, skip `validate_vulkan` entirely.

3. Make `validate_vulkan` warnings non-fatal on aarch64 (they already are — it sets `N_GPU_LAYERS=0` and returns 0).

---

### 9. `.github/workflows/ci.yml`

**PROBLEM**: CI only runs on x86_64 (`ubuntu-latest`, `macos-latest`, `windows-latest`). No ARM64 target testing.

**REQUIRED CHANGES**:
1. Add an ARM64 cross-compile job to verify the code compiles for aarch64:
   ```yaml
   cross-compile-arm64:
     name: Cross-compile ARM64
     runs-on: ubuntu-latest
     steps:
       - uses: actions/checkout@v4
       - uses: dtolnay/rust-toolchain@stable
         with:
           targets: aarch64-unknown-linux-gnu
       - run: sudo apt-get update && sudo apt-get install -y gcc-aarch64-linux-gnu
       - run: cargo build --release --target aarch64-unknown-linux-gnu
       - run: cargo build --release --target aarch64-unknown-linux-gnu --features client
   ```

2. Add the `vulkan` feature build as a separate step (to ensure the feature gate works):
   ```yaml
   build-vulkan:
     name: Build (Vulkan)
     runs-on: ubuntu-latest
     steps:
       - uses: actions/checkout@v4
       - uses: dtolnay/rust-toolchain@stable
       - run: cargo build --release --features vulkan
   ```

3. Ensure clippy/check tests pass WITHOUT the vulkan feature (the default build should be CPU-only after your changes).

---

### 10. `configs/models/` (all 14 model config files)

**PROBLEM**: These are per-model configs with context sizes that may be too large for 12GB RAM. Some set `context.size: 8192` which is OK, but any that set larger need review.

**REQUIRED CHANGES**: 
- Review all 14 configs. Flag any with `context.size` > 8192 as "may OOM on 12GB ARM".
- No need to modify these files — they are per-model configs that users override at runtime. Just ensure the a733 profile in `configs/profiles/a733.yml` takes precedence.

---

### 11. `README.md`

**PROBLEM**: Quick Start only shows Vulkan desktop path. Prerequisites say "Docker with GPU support (Vulkan compatible)".

**REQUIRED CHANGES**:
1. Add ARM64 / A733 section to README:
   ```markdown
   ## ARM64 / SBC (CPU-only)

   # Cross-compile from x86_64 host
   cargo build --release --target aarch64-unknown-linux-gnu

   # Copy binary to SBC
   scp target/aarch64-unknown-linux-gnu/release/whitt user@opi-zero3w:~/

   # On the SBC: install runtime deps
   sudo apt install libstdc++ libgomp1 ca-certificates

   # Build llama.cpp natively (CPU-only)
   git clone https://github.com/ggerganov/llama.cpp && cd llama.cpp
   cmake -B build -DLLAMA_CURL=ON -DLLAMA_OPENMP=ON -DCMAKE_BUILD_TYPE=Release
   cmake --build build --config Release -j$(nproc)

   # Start server
   ./build/bin/llama-server -m /path/to/model.gguf -c 8192 -t 8 -ngl 0

   # Or use Docker (aarch64 image)
   docker compose -f docker/docker-compose.aarch64.yml up -d
   ```

2. Update Prerequisites to not require GPU:
   ```markdown
   ### Prerequisites
   - Rust 1.70+ (edition 2021)
   - Docker (optional, for llama.cpp server)
   - Linux or macOS
   - For GPU: Vulkan-compatible GPU with drivers (optional — CPU mode works on all platforms)
   ```

---

### 12. `src/config/provider.rs`

**PROBLEM**: Contains `LlamaCppVulkanProvider` struct and provider config. References Vulkan throughout doc comments.

**REQUIRED CHANGES**:
1. If struct was renamed in `llama_vulkan.rs`, update references here.
2. If keeping `LlamaCppVulkanBackend` as a type alias, add deprecation notice to `LlamaCppVulkanProvider` too:
   ```rust
   #[deprecated(note = "Use LlamaCppProvider instead. Works with CPU and GPU llama.cpp servers.")]
   pub type LlamaCppVulkanProvider = LlamaCppProvider;
   ```
3. Update doc comments to reflect "llama.cpp backend (CPU or GPU)".
4. **IMPORTANT**: Use `lsp_find_references` on `LlamaCppVulkanProvider` to find ALL usages before renaming. The workflow schema YAML uses `llama_cpp_with_vulkan` as a provider name — that string is user-facing config, not a Rust type. Keep backward compat.

---

## FILES YOU MUST READ BEFORE STARTING

Read these files to understand the full codebase before making changes:
1. `Cargo.toml` — full file
2. `src/backend/mod.rs` — full file (12 lines)
3. `src/backend/llama_vulkan.rs` — full file (647 lines)
4. `src/backend/llm_backend.rs` — full file (149 lines)
5. `src/config/provider.rs` — full file (489 lines)
6. `docker/entrypoint.sh` — full file (747 lines)
7. `docker/Dockerfile` — full file (101 lines)
8. `docker/docker-compose.yml` — full file (51 lines)
9. `.env.example` — full file (203 lines)
10. `config.yml` — full file (48 lines)
11. `.github/workflows/ci.yml` — full file (242 lines)
12. `README.md` — full file (174 lines)
13. ALL 14 files in `configs/models/` — to verify context sizes
14. `docs/contributing/environment-variables.md` — if it exists, may reference Vulkan env vars

Use `grep -r "vulkan" src/` to find ALL Vulkan references in Rust code.
Use `grep -r "vulkan" docker/` to find ALL Vulkan references in Docker files.
Use `grep -r "VULKAN\|vulkan" . --include="*.yml" --include="*.yaml"` to find ALL YAML references.
Use `grep -r "LlamaCppVulkan" src/` to find all struct/type references.
Use `grep -r "llama_cpp_with_vulkan" . --include="*.yml" --include="*.rs"` to find all provider name strings.

---

## VERIFICATION CHECKLIST

After ALL changes, verify:

1. **`cargo check`** — compiles without errors (CPU-only, no features)
2. **`cargo check --features vulkan`** — compiles with Vulkan feature enabled
3. **`cargo check --features client`** — compiles with client feature (HTTP client)
4. **`cargo check --features "client,vulkan"`** — compiles with both
5. **`cargo check --features "client"` with `--target aarch64-unknown-linux-gnu`** — cross-compile check (requires `gcc-aarch64-linux-gnu` installed)
6. **`cargo clippy --all-targets -- -D warnings`** — no clippy warnings
7. **`cargo fmt -- --check`** — formatting OK
8. **`cargo test`** — all tests pass (637 tests, 0 failures — do not introduce regressions)
9. **`cargo test --features client`** — client tests pass
10. **`lsp_diagnostics`** on every changed file — zero errors
11. **`docker build -f docker/Dockerfile.aarch64 .`** — Dockerfile builds (if Docker available)
12. **README links are valid** — no broken references

---

## THINGS YOU MUST NOT DO

1. **DO NOT** break the existing x86_64 Vulkan desktop workflow. All existing functionality must continue to work when `--features vulkan` is passed.
2. **DO NOT** change the llama.cpp HTTP API protocol. The engine talks HTTP to llama-server — this doesn't change between CPU and GPU.
3. **DO NOT** add new crate dependencies unless absolutely necessary for the CPU/GPU feature split.
4. **DO NOT** modify any test files. Tests mock the backend and don't care about Vulkan vs CPU.
5. **DO NOT** delete `docker/Dockerfile` or `docker/docker-compose.yml`. Create new `*.aarch64*` variants alongside them.
6. **DO NOT** change the default Rust edition, MSRV, or profile settings.
7. **DO NOT** use `as any`, `@ts-ignore`, or similar suppression. If something doesn't compile, fix it properly.
8. **DO NOT** commit anything. Just make the changes.
9. **DO NOT** build or run the project — just ensure it compiles and tests pass.

---

## BUILD CROSS-COMPILE TOOLCHAIN SETUP (for reference, not a task)

On the x86_64 host, to cross-compile for aarch64:
```bash
# Install cross-compile toolchain
sudo apt install gcc-aarch64-linux-gnu g++-aarch64-linux-gnu

# Add Rust target
rustup target add aarch64-unknown-linux-gnu

# Configure cargo for cross-compilation (in .cargo/config.toml)
# [target.aarch64-unknown-linux-gnu]
# linker = "aarch64-linux-gnu-gcc"

# Build CPU-only binary for ARM64
cargo build --release --target aarch64-unknown-linux-gnu

# Build with client features
cargo build --release --target aarch64-unknown-linux-gnu --features client
```

---

## ON THE SBC (reference, not a task)

Once the binary is deployed to the Orange Pi Zero 3W:
```bash
# Install runtime dependencies
sudo apt update
sudo apt install -y libstdc++6 libgomp1 ca-certificates

# Verify binary runs
./whitt --version || ./whitt --help

# Build llama.cpp natively (CPU-only, with OpenMP for multi-core)
sudo apt install -y build-essential cmake git wget
git clone --depth 1 https://github.com/ggerganov/llama.cpp
cd llama.cpp
cmake -B build \
  -DLLAMA_CURL=ON \
  -DLLAMA_OPENMP=ON \
  -DLLAMA_NATIVE=OFF \
  -DCMAKE_BUILD_TYPE=Release
cmake --build build --config Release -j$(nproc)

# Download a small model (SmolLM3 ~2GB Q4_K_M, fits easily in 12GB)
wget -O models/SmolLM3-Q4_K_M.gguf \
  "https://huggingface.co/./resolve/main/SmolLM3-Q4_K_M.gguf"

# Start server (CPU-only, 8 threads, 8K context)
./build/bin/llama-server \
  -m models/SmolLM3-Q4_K_M.gguf \
  -c 8192 \
  -t 8 \
  -ngl 0 \
  --host 127.0.0.1 \
  --port 8080

# In another terminal: test the engine
./whitt chat --model SmolLM3
```

---

## SUMMARY OF ALL FILES TO CREATE OR MODIFY

| File | Action | Priority |
|------|--------|----------|
| `Cargo.toml` | MODIFY — make vulkan feature optional, CPU default | P0 (blocks everything) |
| `src/backend/llama_vulkan.rs` | MODIFY — rename struct, update docs, add cfg gates | P0 |
| `src/backend/mod.rs` | MODIFY — update re-exports for renamed struct | P0 |
| `src/config/provider.rs` | MODIFY — rename provider struct, add type alias | P1 |
| `.env.example` | MODIFY — change defaults to CPU | P1 |
| `config.yml` | MODIFY — reduce context, set gpu_layers=0, threads=8 | P1 |
| `configs/profiles/a733.yml` | CREATE — ARM-optimized config profile | P1 |
| `docker/Dockerfile.aarch64` | CREATE — CPU-only ARM64 Docker image | P2 |
| `docker/docker-compose.aarch64.yml` | CREATE — ARM64 compose without GPU | P2 |
| `docker/entrypoint.sh` | MODIFY — add ARM detection, skip Vulkan validate | P2 |
| `.github/workflows/ci.yml` | MODIFY — add ARM64 cross-compile job | P2 |
| `README.md` | MODIFY — add ARM64/SBC section, update prerequisites | P3 |
| `docker/Dockerfile` | MODIFY — add TARGETARCH arg for yq download | P3 |
| `.cargo/config.toml` | CREATE — cross-compile linker config for aarch64 | P2 |
</auto-slash-command>

## [0125] 06-30 17:35

<command-instruction>
You are starting a Ralph Loop - a self-referential development loop that runs until task completion.

## How Ralph Loop Works

1. You will work on the task continuously
2. When you believe the task is FULLY complete, output: `<promise>{{COMPLETION_PROMISE}}</promise>`
3. If you don't output the promise, the loop will automatically inject another prompt to continue
4. Maximum iterations: Configurable (default 100)

## Rules

- Focus on completing the task fully, not partially
- Don't output the completion promise until the task is truly done
- Each iteration should make meaningful progress toward the goal
- If stuck, try different approaches
- Use todos to track your progress

## Exit Conditions

1. **Completion**: Output your completion promise tag when fully complete
2. **Max Iterations**: Loop stops automatically at limit
3. **Cancel**: User runs `/cancel-ralph` command

## Your Task

Parse the arguments below and begin working on the task. The format is:
`"task description" [--completion-promise=TEXT] [--max-iterations=N] [--strategy=reset|continue]`

Default completion promise is "DONE" and default max iterations is 100.
</command-instruction>

<user-task>
so on this git workspace I'm wanting to get a new command with the cli that will install onto an orangepi 3w 12gb lpddr5 ram allwinner A733 to run locally on that board and to be installable from scripts and a new cli set of commands onto the device with a good .md doc explaining how to use the CLI and a well formatted easily skimmable way.  here is some info you need to be aware of as you go.  don't run anything outside of unit tests while you work.  on the other worktree im local system stress testing so the resources don't exist for you to live system test so make sure you only do local test runs of things as you write them and do so iteratively and critically review and fix and re evaluate before continuing on to the next task so you don't drift or think you've implemented something when in reality it won't work in real life when I get the device in the mail in a few weeks.

# Orange Pi Zero 3W (A733) Build — Exhaustive Implementation Prompt

**Target workspace**: `~/code/whitt-execution-engine/` (branch: `initial-creation`)
**Target hardware**: Orange Pi Zero 3W, Allwinner A733 SoC, 12GB LPDDR5, ARM64 (aarch64)
**Target OS**: Debian Bookworm minimal server, BSP kernel `6.6.98-sun60iw2` (NO Vulkan/GPU userspace)

---

## CONTEXT

### What this project is
whitt-execution-engine is a Rust CLI workflow engine that orchestrates LLM-powered YAML workflows. It talks to a llama.cpp HTTP server via OpenAI-compatible API. The engine itself is headless (no GUI). Currently hardcoded for x86_64 + AMD Vulkan GPU (RX 580). Needs to become portable to ARM64 CPU-only on an Allwinner A733 SBC with 12GB RAM.

### What the A733 hardware CAN and CANNOT do
- **CAN run**: ARM64 (aarch64) Linux with BSP kernel, headless workloads, CPU inference via llama.cpp
- **CANNOT do**: No Vulkan userspace on stock images (PowerVR BXM-4-64 has no open-source driver, no ICD). No GPU device nodes (`/dev/dri` doesn't exist). No X11/Wayland display server (headless only). No Docker with GPU passthrough.
- **Performance expectation**: llama.cpp CPU-only on 8 ARM cores (2×A76@2.0G + 6×A55@1.8G) with 12GB LPDDR5 @2400MHz 32-bit (~19.2 GB/s bandwidth). Expect ~1-4 tok/s on 3B models, ~0.5-1.5 tok/s on 7B-9B models. NOT fast. This is a Tier A/B micro-orchestrator, not a Tier C speed rig.

### Key constraint: CROSS-COMPILE
391 crates with LTO + codegen-units=1. Compiling on the 12GB ARM SBC would take hours and may OOM during linking. ALL Rust compilation must happen on the x86_64 host, then scp the binary to the SBC. llama.cpp server should be compiled natively on the SBC OR cross-compiled.

---

## EXHAUSTIVE FILE-BY-FILE CHANGES REQUIRED

Every file listed below needs specific modifications. DO NOT skip any file. DO NOT add new dependencies unless absolutely necessary. DO NOT restructure the crate — make surgical, minimal changes.

---

### 1. `Cargo.toml` (line 63)

**PROBLEM**: `llama-cpp-2 = { version = "0.1.138", features = ["vulkan"] }` — the `vulkan` feature is hardcoded, not optional. This forces Vulkan ICD detection at build time and runtime. On A733, Vulkan doesn't exist, so this will fail to build (no `libvulkan-dev` headers) or fail at runtime (no ICD JSON).

**REQUIRED CHANGES**:
1. Add a `vulkan` feature flag that gates the llama-cpp-2 vulkan feature:
   ```toml
   [features]
   default = []  # DO NOT make vulkan default anymore
   client = ["dep:reqwest", "dep:async-stream", "dep:futures", "dep:bytes"]
   sqlite = ["dep:rusqlite"]
   clipboard = ["dep:arboard"]
   vulkan = []  # NEW: enables Vulkan GPU acceleration
   ```

2. Change the llama-cpp-2 dependency to conditionally include vulkan:
   ```toml
   llama-cpp-2 = { version = "0.1.138", features = ["vulkan"], optional = true }
   ```

3. Add a non-optional CPU-only llama-cpp-2 dependency:
   ```toml
   [target.'cfg(not(feature = "vulkan"))'.dependencies]
   llama-cpp-2-cpu = { package = "llama-cpp-2", version = "0.1.138" }

   [target.'cfg(feature = "vulkan")'.dependencies]
   llama-cpp-2 = { version = "0.1.138", features = ["vulkan"] }
   ```

   **IMPORTANT ALTERNATIVE** (if the above target-cfg approach causes duplicate crate conflicts):
   Simply make the dependency conditional on the feature:
   ```toml
   llama-cpp-2 = { version = "0.1.138" }
   ```

   And use a cfg attribute in the Rust code to conditionally pass the vulkan init flag to llama-cpp-2. Check llama-cpp-2's API — the `vulkan` feature likely controls a build-time flag. If it can be set at runtime instead, just drop the feature entirely and handle Vulkan init in application code. **READ llama-cpp-2's docs/source to determine the correct approach before changing.**

**CRITICAL**: After this change, `cargo build` (no features) must produce a CPU-only binary. `cargo build --features vulkan` must produce a Vulkan-capable binary. Both must compile on x86_64. `cargo build --target aarch64-unknown-linux-gnu` (no features) must cross-compile for ARM64.

---

### 2. `src/backend/llama_vulkan.rs`

**PROBLEM**: Struct is named `LlamaCppVulkanBackend` and docs reference "Vulkan GPU acceleration" throughout. However, examining the actual code reveals this is just an HTTP client that talks to a llama.cpp server over localhost HTTP. It does NOT link Vulkan directly — it uses `llama-cpp-2` Rust bindings which handle the GPU connection internally. The naming is misleading.

**REQUIRED CHANGES**:
1. Rename the struct from `LlamaCppVulkanBackend` to `LlamaCppBackend` (or `LlamaCppCpuBackend` for the non-vulkan variant). **HOWEVER**: this is a PUBLIC API change that affects all imports. Check if this struct is used outside this crate (it isn't — it's a binary crate, not a library published on crates.io). So renaming is safe.

2. **SAFER APPROACH** (preserving API compat): Keep `LlamaCppVulkanBackend` as a type alias for backward compat:
   ```rust
   /// Llama.cpp backend with Vulkan support.
   #[derive(Debug, Clone)]
   pub struct LlamaCppBackend { /* same fields */ }

   /// Backward-compatible alias.
   #[deprecated(note = "Use LlamaCppBackend instead. This works with both CPU and Vulkan llama.cpp servers.")]
   pub type LlamaCppVulkanBackend = LlamaCppBackend;
   ```

3. Add conditional compilation for any llama-cpp-2 Vulkan-specific initialization. If the file uses `#[cfg(feature = "vulkan")]` to gate any code, add the appropriate `#[cfg(not(feature = "vulkan"))]` fallbacks.

4. Update all doc comments that say "Vulkan GPU acceleration" to say "llama.cpp backend (CPU or GPU)".

5. The HTTP client approach (talking to llama-server over HTTP) is correct for both CPU and Vulkan. No changes needed to the HTTP logic itself — llama-server handles backend selection internally.

---

### 3. `src/backend/mod.rs`

**PROBLEM**: Hardcodes `LlamaCppVulkanBackend` re-export.

**REQUIRED CHANGES**:
1. If you renamed the struct, update the re-export:
   ```rust
   pub use llama_vulkan::LlamaCppBackend;
   #[allow(deprecated)]
   pub use llama_vulkan::LlamaCppVulkanBackend;  // backward compat
   ```

2. Consider renaming `llama_vulkan.rs` to `llama_cpp.rs` to reflect the backend-agnostic nature. If you do, update the `pub mod` declaration. **Check all imports across the entire codebase first** using grep/lsp_find_references before renaming.

---

### 4. `config.yml` (root config file)

**PROBLEM**: Multiple settings are tuned for x86_64 + AMD Vulkan GPU with 16GB+ RAM:
- `context.size: 100000` — 100K context on 12GB LPDDR5 with a 9B model WILL OOM. 9B Q4_K_M ≈ 6GB model + KV cache. At 100K ctx with q8_0 cache, KV alone could be 3-4GB. 6GB + 4GB = 10GB, leaving 2GB for OS + everything else. Tight and likely OOM.
- `hardware.gpu_layers: 99` — No GPU on A733, this is meaningless.
- `hardware.threads: 6` — Should be 8 for the 8-core A733 (2×A76 + 6×A55).
- `hardware.flash_attn: true` — May or may not work in CPU mode. Check llama.cpp docs. If unsupported in CPU-only, set false.
- `vulkan.visible_devices: "0"` — No Vulkan on A733.

**REQUIRED CHANGES**:
1. Reduce `context.size` to `4096` or `8192` (safe for 12GB with 9B Q4; 16384 possible but risky with large models).
2. Set `hardware.gpu_layers: 0` (explicitly CPU-only).
3. Set `hardware.threads: 8` (use all A733 cores).
4. Remove or conditionally ignore the `vulkan:` section.
5. Consider reducing `cache.cache_type_k` and `cache.cache_type_v` from `q8_0` to `q4_0` to save memory on 12GB.

**HOWEVER**: Do NOT break the x86_64 Vulkan desktop config. The approach should be:
- Either create a separate `config.a733.yml` file with ARM-optimized settings
- Or make the config values overridable via environment variables (they already are in `.env.example`)
- Or add profile sections to config.yml: `profiles: { desktop: { ... }, a733: { ... } }`

**RECOMMENDED**: Create `configs/profiles/a733.yml` with:
```yaml
context:
  size: 8192
  batch_size: 512
  ubatch_size: 128
hardware:
  threads: 8
  gpu_layers: 0
  use_mmap: true
  flash_attn: false  # safer for CPU-only
cache:
  cache_type_k: q4_0  # save memory vs q8_0
  cache_type_v: q4_0
vulkan:
  visible_devices: ""  # disabled
```

---

### 5. `.env.example`

**PROBLEM**: `LLAMACPP_BACKEND=vulkan` and `LLAMACPP_GPU_LAYERS=99` are Vulkan-desktop defaults.

**REQUIRED CHANGES**:
1. Change `LLAMACPP_BACKEND=vulkan` to `LLAMACPP_BACKEND=cpu` as default (desktop users can override).
   - **OR**: Add comments showing both:
     ```
     # llama.cpp Backend
     # Options: cpu, vulkan, cuda, metal
     # Default: cpu (use vulkan for GPU-accelerated desktops)
     LLAMACPP_BACKEND=cpu
     ```
2. Change `LLAMACPP_GPU_LAYERS=99` to `LLAMACPP_GPU_LAYERS=0` as default.
3. Add `MAX_ALLOCATED_MEMORY_MB=8192` comment about reducing for SBCs:
   ```
   # For 12GB ARM SBCs: reduce to 8192 to leave room for OS
   ```

---

### 6. `docker/Dockerfile`

**PROBLEM**: Hardcoded for x86_64 + Vulkan:
- `FROM ghcr.io/ggml-org/llama.cpp:server-vulkan` — amd64-only image, won't build on ARM
- Line 58: `yq_linux_amd64` — wrong arch for A733
- Installs `libvulkan1`, `mesa-vulkan-drivers`, `vulkan-tools` — useless on A733
- `ENV AMD_VULKAN_ICD=RADV` — meaningless on ARM
- `GGML_VK_FORCE_MAX_ALLOCATION_SIZE` env var — Vulkan-only
- Validates `radeon_icd.json` exists — will fail on ARM

**REQUIRED CHANGES — TWO-PHASE APPROACH**:

**Phase A**: Create a new `docker/Dockerfile.aarch64` (DO NOT modify the existing Dockerfile — it works for desktop):
```dockerfile
# Stage 1: Build llama.cpp from source for aarch64 (CPU-only)
FROM ubuntu:24.04 AS builder

ARG LLAMA_CPP_VERSION=b4450

ENV DEBIAN_FRONTEND=noninteractive

RUN apt-get update && apt-get install -y --no-install-recommends \
    build-essential cmake git wget ca-certificates \
    && rm -rf /var/lib/apt/lists/*

# Build llama.cpp with CPU-only support (no Vulkan, no CUDA)
RUN git clone --depth 1 --branch master https://github.com/ggerganov/llama.cpp /tmp/llama.cpp \
    && cd /tmp/llama.cpp \
    && cmake -B build -DLLAMA_CURL=ON -DLLAMA_NATIVE=OFF -DLLAMA_ACCELERATE=OFF \
       -DLLAMA_BLAS=OFF -DLLAMA_VULKAN=OFF -DLLAMA_CUDA=OFF \
       -DLLAMA_METAL=OFF -DLLAMA_OPENMP=ON \
       -DCMAKE_BUILD_TYPE=Release \
    && cmake --build build --config Release -j$(nproc)

# Stage 2: Minimal runtime image
FROM ubuntu:24.04

ENV DEBIAN_FRONTEND=noninteractive
ENV TZ=UTC
ENV PYTHONUNBUFFERED=1
ENV HF_HUB_DISABLE_TELEMETRY=1

# Minimal runtime deps — NO Vulkan libs
RUN apt-get update && apt-get install -y --no-install-recommends \
    python3 \
    python3-pip \
    curl \
    tini \
    ca-certificates \
    libgomp1 \
    && rm -rf /var/lib/apt/lists/*

RUN pip3 install --no-cache-dir --break-system-packages huggingface-hub

# Install yq for aarch64 (NOT amd64)
RUN curl -fsSL https://github.com/mikefarah/yq/releases/latest/download/yq_linux_arm64 -o /usr/local/bin/yq && \
    chmod +x /usr/local/bin/yq

COPY --from=builder /tmp/llama.cpp/build/bin/llama-server /usr/local/bin/llama-server

RUN mkdir -p /models /config /var/log/llama-server /var/run/llama-server

WORKDIR /app
COPY docker/entrypoint.sh /entrypoint.sh
RUN chmod +x /entrypoint.sh

EXPOSE 8080

HEALTHCHECK --interval=30s --timeout=10s --start-period=60s --retries=3 \
    CMD curl -f http://localhost:8080/props || exit 1

VOLUME ["/models", "/config", "/var/log/llama-server"]

RUN useradd -m -u 1001 -s /bin/bash llama && \
    chown -R llama:llama /models /config /var/log/llama-server /app
USER llama

ENTRYPOINT ["tini", "--"]
CMD ["/entrypoint.sh"]
```

**Phase B**: In the existing `docker/Dockerfile`, add a multi-arch `ARG` for yq:
```dockerfile
ARG TARGETARCH
RUN if [ "$TARGETARCH" = "arm64" ]; then \
        curl -fsSL https://github.com/mikefarah/yq/releases/latest/download/yq_linux_arm64 -o /usr/local/bin/yq; \
    else \
        curl -fsSL https://github.com/mikefarah/yq/releases/latest/download/yq_linux_amd64 -o /usr/local/bin/yq; \
    fi && chmod +x /usr/local/bin/yq
```

---

### 7. `docker/docker-compose.yml`

**PROBLEM**: Hardcoded for desktop Vulkan GPU:
- `devices: /dev/dri:/dev/dri` — GPU passthrough, doesn't exist on A733
- `group_add: "${RENDER_GID:-987}"`, `"${VIDEO_GID:-983}"` — GPU group IDs
- `shm_size: 8g` — excessive for 12GB SBC
- `AMD_VULKAN_ICD: RADV` — Vulkan env var
- `GGML_VK_DISABLE_DEBUG: "1"` — Vulkan env var
- `RADV_PERFTEST: "nogttspill"` — AMD GPU performance tuning

**REQUIRED CHANGES**: Create `docker/docker-compose.aarch64.yml` (DO NOT modify the desktop compose file):
```yaml
services:
  llama-server:
    build:
      context: ..
      dockerfile: docker/Dockerfile.aarch64
    container_name: whitt-llama-server
    restart: unless-stopped
    volumes:
      - ../configs/profiles/a733.yml:/config/config.yml:ro
      - ../configs/models:/config/models:ro
      - ./entrypoint.sh:/entrypoint.sh:ro
      - ${MODEL_HOST_PATH:-../models}:/models
      - ../logs:/var/log/llama-server
    # NO devices: — no GPU on A733
    network_mode: host
    shm_size: 2g  # reduced for 12GB SBC
    user: "${DOCKER_UID:-1000}:${DOCKER_GID:-1000}"
    environment:
      HUGGING_FACE_HUB_TOKEN: ${HF_TOKEN:-}
      HF_HOME: /tmp/hf
      LLAMA_ARG_N_GPU_LAYERS: "0"  # explicit CPU-only
    healthcheck:
      test: ["CMD", "curl", "-f", "http://localhost:8080/props"]
      interval: 30s
      timeout: 10s
      retries: 3
      start_period: 120s  # longer startup for CPU inference
    logging:
      driver: "json-file"
      options:
        max-size: "10m"
        max-file: "3"

volumes:
  models:
    driver: local
  logs:
    driver: local

networks:
  default:
    name: whitt-network
```

---

### 8. `docker/entrypoint.sh`

**PROBLEM**: `validate_vulkan()` function (lines 400-460) checks for `/dev/dri`, radeon ICD, vulkaninfo — all of which fail on A733. The function does gracefully fall back to CPU mode (`export LLAMA_ARG_N_GPU_LAYERS="0"`), so it won't crash. But it prints misleading error messages.

**REQUIRED CHANGES**:
1. Add early detection for non-GPU environments. Before `validate_vulkan()`:
   ```bash
   # Detect non-GPU environment (ARM SBC without Vulkan)
   detect_hardware_profile() {
       local arch=$(uname -m)
       if [ "$arch" = "aarch64" ] || [ "$arch" = "arm64" ]; then
           if [ ! -d /dev/dri ]; then
               log_info "Detected ARM64 SBC without GPU device nodes — CPU-only mode."
               export LLAMA_ARG_N_GPU_LAYERS="0"
               return 0
           fi
       fi
       return 1
   }
   ```

2. In `main()`, call `detect_hardware_profile` BEFORE `validate_vulkan`. If it returns 0, skip `validate_vulkan` entirely.

3. Make `validate_vulkan` warnings non-fatal on aarch64 (they already are — it sets `N_GPU_LAYERS=0` and returns 0).

---

### 9. `.github/workflows/ci.yml`

**PROBLEM**: CI only runs on x86_64 (`ubuntu-latest`, `macos-latest`, `windows-latest`). No ARM64 target testing.

**REQUIRED CHANGES**:
1. Add an ARM64 cross-compile job to verify the code compiles for aarch64:
   ```yaml
   cross-compile-arm64:
     name: Cross-compile ARM64
     runs-on: ubuntu-latest
     steps:
       - uses: actions/checkout@v4
       - uses: dtolnay/rust-toolchain@stable
         with:
           targets: aarch64-unknown-linux-gnu
       - run: sudo apt-get update && sudo apt-get install -y gcc-aarch64-linux-gnu
       - run: cargo build --release --target aarch64-unknown-linux-gnu
       - run: cargo build --release --target aarch64-unknown-linux-gnu --features client
   ```

2. Add the `vulkan` feature build as a separate step (to ensure the feature gate works):
   ```yaml
   build-vulkan:
     name: Build (Vulkan)
     runs-on: ubuntu-latest
     steps:
       - uses: actions/checkout@v4
       - uses: dtolnay/rust-toolchain@stable
       - run: cargo build --release --features vulkan
   ```

3. Ensure clippy/check tests pass WITHOUT the vulkan feature (the default build should be CPU-only after your changes).

---

### 10. `configs/models/` (all 14 model config files)

**PROBLEM**: These are per-model configs with context sizes that may be too large for 12GB RAM. Some set `context.size: 8192` which is OK, but any that set larger need review.

**REQUIRED CHANGES**: 
- Review all 14 configs. Flag any with `context.size` > 8192 as "may OOM on 12GB ARM".
- No need to modify these files — they are per-model configs that users override at runtime. Just ensure the a733 profile in `configs/profiles/a733.yml` takes precedence.

---

### 11. `README.md`

**PROBLEM**: Quick Start only shows Vulkan desktop path. Prerequisites say "Docker with GPU support (Vulkan compatible)".

**REQUIRED CHANGES**:
1. Add ARM64 / A733 section to README:
   ```markdown
   ## ARM64 / SBC (CPU-only)

   # Cross-compile from x86_64 host
   cargo build --release --target aarch64-unknown-linux-gnu

   # Copy binary to SBC
   scp target/aarch64-unknown-linux-gnu/release/whitt user@opi-zero3w:~/

   # On the SBC: install runtime deps
   sudo apt install libstdc++ libgomp1 ca-certificates

   # Build llama.cpp natively (CPU-only)
   git clone https://github.com/ggerganov/llama.cpp && cd llama.cpp
   cmake -B build -DLLAMA_CURL=ON -DLLAMA_OPENMP=ON -DCMAKE_BUILD_TYPE=Release
   cmake --build build --config Release -j$(nproc)

   # Start server
   ./build/bin/llama-server -m /path/to/model.gguf -c 8192 -t 8 -ngl 0

   # Or use Docker (aarch64 image)
   docker compose -f docker/docker-compose.aarch64.yml up -d
   ```

2. Update Prerequisites to not require GPU:
   ```markdown
   ### Prerequisites
   - Rust 1.70+ (edition 2021)
   - Docker (optional, for llama.cpp server)
   - Linux or macOS
   - For GPU: Vulkan-compatible GPU with drivers (optional — CPU mode works on all platforms)
   ```

---

### 12. `src/config/provider.rs`

**PROBLEM**: Contains `LlamaCppVulkanProvider` struct and provider config. References Vulkan throughout doc comments.

**REQUIRED CHANGES**:
1. If struct was renamed in `llama_vulkan.rs`, update references here.
2. If keeping `LlamaCppVulkanBackend` as a type alias, add deprecation notice to `LlamaCppVulkanProvider` too:
   ```rust
   #[deprecated(note = "Use LlamaCppProvider instead. Works with CPU and GPU llama.cpp servers.")]
   pub type LlamaCppVulkanProvider = LlamaCppProvider;
   ```
3. Update doc comments to reflect "llama.cpp backend (CPU or GPU)".
4. **IMPORTANT**: Use `lsp_find_references` on `LlamaCppVulkanProvider` to find ALL usages before renaming. The workflow schema YAML uses `llama_cpp_with_vulkan` as a provider name — that string is user-facing config, not a Rust type. Keep backward compat.

---

## FILES YOU MUST READ BEFORE STARTING

Read these files to understand the full codebase before making changes:
1. `Cargo.toml` — full file
2. `src/backend/mod.rs` — full file (12 lines)
3. `src/backend/llama_vulkan.rs` — full file (647 lines)
4. `src/backend/llm_backend.rs` — full file (149 lines)
5. `src/config/provider.rs` — full file (489 lines)
6. `docker/entrypoint.sh` — full file (747 lines)
7. `docker/Dockerfile` — full file (101 lines)
8. `docker/docker-compose.yml` — full file (51 lines)
9. `.env.example` — full file (203 lines)
10. `config.yml` — full file (48 lines)
11. `.github/workflows/ci.yml` — full file (242 lines)
12. `README.md` — full file (174 lines)
13. ALL 14 files in `configs/models/` — to verify context sizes
14. `docs/contributing/environment-variables.md` — if it exists, may reference Vulkan env vars

Use `grep -r "vulkan" src/` to find ALL Vulkan references in Rust code.
Use `grep -r "vulkan" docker/` to find ALL Vulkan references in Docker files.
Use `grep -r "VULKAN\|vulkan" . --include="*.yml" --include="*.yaml"` to find ALL YAML references.
Use `grep -r "LlamaCppVulkan" src/` to find all struct/type references.
Use `grep -r "llama_cpp_with_vulkan" . --include="*.yml" --include="*.rs"` to find all provider name strings.

---

## VERIFICATION CHECKLIST

After ALL changes, verify:

1. **`cargo check`** — compiles without errors (CPU-only, no features)
2. **`cargo check --features vulkan`** — compiles with Vulkan feature enabled
3. **`cargo check --features client`** — compiles with client feature (HTTP client)
4. **`cargo check --features "client,vulkan"`** — compiles with both
5. **`cargo check --features "client"` with `--target aarch64-unknown-linux-gnu`** — cross-compile check (requires `gcc-aarch64-linux-gnu` installed)
6. **`cargo clippy --all-targets -- -D warnings`** — no clippy warnings
7. **`cargo fmt -- --check`** — formatting OK
8. **`cargo test`** — all tests pass (637 tests, 0 failures — do not introduce regressions)
9. **`cargo test --features client`** — client tests pass
10. **`lsp_diagnostics`** on every changed file — zero errors
11. **`docker build -f docker/Dockerfile.aarch64 .`** — Dockerfile builds (if Docker available)
12. **README links are valid** — no broken references

---

## THINGS YOU MUST NOT DO

1. **DO NOT** break the existing x86_64 Vulkan desktop workflow. All existing functionality must continue to work when `--features vulkan` is passed.
2. **DO NOT** change the llama.cpp HTTP API protocol. The engine talks HTTP to llama-server — this doesn't change between CPU and GPU.
3. **DO NOT** add new crate dependencies unless absolutely necessary for the CPU/GPU feature split.
4. **DO NOT** modify any test files. Tests mock the backend and don't care about Vulkan vs CPU.
5. **DO NOT** delete `docker/Dockerfile` or `docker/docker-compose.yml`. Create new `*.aarch64*` variants alongside them.
6. **DO NOT** change the default Rust edition, MSRV, or profile settings.
7. **DO NOT** use `as any`, `@ts-ignore`, or similar suppression. If something doesn't compile, fix it properly.
8. **DO NOT** commit anything. Just make the changes.
9. **DO NOT** build or run the project — just ensure it compiles and tests pass.

---

## BUILD CROSS-COMPILE TOOLCHAIN SETUP (for reference, not a task)

On the x86_64 host, to cross-compile for aarch64:
```bash
# Install cross-compile toolchain
sudo apt install gcc-aarch64-linux-gnu g++-aarch64-linux-gnu

# Add Rust target
rustup target add aarch64-unknown-linux-gnu

# Configure cargo for cross-compilation (in .cargo/config.toml)
# [target.aarch64-unknown-linux-gnu]
# linker = "aarch64-linux-gnu-gcc"

# Build CPU-only binary for ARM64
cargo build --release --target aarch64-unknown-linux-gnu

# Build with client features
cargo build --release --target aarch64-unknown-linux-gnu --features client
```

---

## ON THE SBC (reference, not a task)

Once the binary is deployed to the Orange Pi Zero 3W:
```bash
# Install runtime dependencies
sudo apt update
sudo apt install -y libstdc++6 libgomp1 ca-certificates

# Verify binary runs
./whitt --version || ./whitt --help

# Build llama.cpp natively (CPU-only, with OpenMP for multi-core)
sudo apt install -y build-essential cmake git wget
git clone --depth 1 https://github.com/ggerganov/llama.cpp
cd llama.cpp
cmake -B build \
  -DLLAMA_CURL=ON \
  -DLLAMA_OPENMP=ON \
  -DLLAMA_NATIVE=OFF \
  -DCMAKE_BUILD_TYPE=Release
cmake --build build --config Release -j$(nproc)

# Download a small model (SmolLM3 ~2GB Q4_K_M, fits easily in 12GB)
wget -O models/SmolLM3-Q4_K_M.gguf \
  "https://huggingface.co/./resolve/main/SmolLM3-Q4_K_M.gguf"

# Start server (CPU-only, 8 threads, 8K context)
./build/bin/llama-server \
  -m models/SmolLM3-Q4_K_M.gguf \
  -c 8192 \
  -t 8 \
  -ngl 0 \
  --host 127.0.0.1 \
  --port 8080

# In another terminal: test the engine
./whitt chat --model SmolLM3
```

---

## SUMMARY OF ALL FILES TO CREATE OR MODIFY

| File | Action | Priority |
|------|--------|----------|
| `Cargo.toml` | MODIFY — make vulkan feature optional, CPU default | P0 (blocks everything) |
| `src/backend/llama_vulkan.rs` | MODIFY — rename struct, update docs, add cfg gates | P0 |
| `src/backend/mod.rs` | MODIFY — update re-exports for renamed struct | P0 |
| `src/config/provider.rs` | MODIFY — rename provider struct, add type alias | P1 |
| `.env.example` | MODIFY — change defaults to CPU | P1 |
| `config.yml` | MODIFY — reduce context, set gpu_layers=0, threads=8 | P1 |
| `configs/profiles/a733.yml` | CREATE — ARM-optimized config profile | P1 |
| `docker/Dockerfile.aarch64` | CREATE — CPU-only ARM64 Docker image | P2 |
| `docker/docker-compose.aarch64.yml` | CREATE — ARM64 compose without GPU | P2 |
| `docker/entrypoint.sh` | MODIFY — add ARM detection, skip Vulkan validate | P2 |
| `.github/workflows/ci.yml` | MODIFY — add ARM64 cross-compile job | P2 |
| `README.md` | MODIFY — add ARM64/SBC section, update prerequisites | P3 |
| `docker/Dockerfile` | MODIFY — add TARGETARCH arg for yq download | P3 |
| `.cargo/config.toml` | CREATE — cross-compile linker config for aarch64 | P2 |
</user-task>

## [0126] 06-30 18:02

(2026-06-30T18:02:40)
[search-mode]
MAXIMIZE SEARCH EFFORT. Launch multiple background agents IN PARALLEL:
- explore agents (codebase patterns, file structures, ast-grep)
- librarian agents (remote repos, official docs, GitHub examples)
Plus direct tools: Grep, ripgrep (rg), ast-grep (sg)
NEVER stop at first result - be exhaustive.

[analyze-mode]
ANALYSIS MODE. Gather context before diving deep:
CONTEXT GATHERING (parallel):
- 1-2 explore agents (codebase patterns, implementations)
- 1-2 librarian agents (if external library involved)
- Direct tools: Grep, AST-grep, LSP for targeted searches

IF COMPLEX - DO NOT STRUGGLE ALONE. Consult specialists:
- **Oracle**: Conventional problems (architecture, debugging, complex logic)
- **Artistry**: Non-conventional problems (different approach needed)

SYNTHESIZE findings before proceeding.
---
MANDATORY delegate_task params: ALWAYS include load_skills=[] and run_in_background when calling delegate_task.
Example: delegate_task(subagent_type="explore", prompt="...", run_in_background=true, load_skills=[])

---

What remains un unit tested or unimplemented from this prompt?  No changes just answer exhaustively:

so on this git workspace I'm wanting to get a new command with the cli that will install onto an orangepi 3w 12gb lpddr5 ram allwinner A733 to run locally on that board and to be installable from scripts and a new cli set of commands onto the device with a good .md doc explaining how to use the CLI and a well formatted easily skimmable way.  here is some info you need to be aware of as you go.  don't run anything outside of unit tests while you work.  on the other worktree im local system stress testing so the resources don't exist for you to live system test so make sure you only do local test runs of things as you write them and do so iteratively and critically review and fix and re evaluate before continuing on to the next task so you don't drift or think you've implemented something when in reality it won't work in real life when I get the device in the mail in a few weeks.

# Orange Pi Zero 3W (A733) Build — Exhaustive Implementation Prompt

**Target workspace**: `~/code/whitt-execution-engine/` (branch: `initial-creation`)
**Target hardware**: Orange Pi Zero 3W, Allwinner A733 SoC, 12GB LPDDR5, ARM64 (aarch64)
**Target OS**: Debian Bookworm minimal server, BSP kernel `6.6.98-sun60iw2` (NO Vulkan/GPU userspace)

---

## CONTEXT

### What this project is
whitt-execution-engine is a Rust CLI workflow engine that orchestrates LLM-powered YAML workflows. It talks to a llama.cpp HTTP server via OpenAI-compatible API. The engine itself is headless (no GUI). Currently hardcoded for x86_64 + AMD Vulkan GPU (RX 580). Needs to become portable to ARM64 CPU-only on an Allwinner A733 SBC with 12GB RAM.

### What the A733 hardware CAN and CANNOT do
- **CAN run**: ARM64 (aarch64) Linux with BSP kernel, headless workloads, CPU inference via llama.cpp
- **CANNOT do**: No Vulkan userspace on stock images (PowerVR BXM-4-64 has no open-source driver, no ICD). No GPU device nodes (`/dev/dri` doesn't exist). No X11/Wayland display server (headless only). No Docker with GPU passthrough.
- **Performance expectation**: llama.cpp CPU-only on 8 ARM cores (2×A76@2.0G + 6×A55@1.8G) with 12GB LPDDR5 @2400MHz 32-bit (~19.2 GB/s bandwidth). Expect ~1-4 tok/s on 3B models, ~0.5-1.5 tok/s on 7B-9B models. NOT fast. This is a Tier A/B micro-orchestrator, not a Tier C speed rig.

### Key constraint: CROSS-COMPILE
391 crates with LTO + codegen-units=1. Compiling on the 12GB ARM SBC would take hours and may OOM during linking. ALL Rust compilation must happen on the x86_64 host, then scp the binary to the SBC. llama.cpp server should be compiled natively on the SBC OR cross-compiled.

---

## EXHAUSTIVE FILE-BY-FILE CHANGES REQUIRED

Every file listed below needs specific modifications. DO NOT skip any file. DO NOT add new dependencies unless absolutely necessary. DO NOT restructure the crate — make surgical, minimal changes.

---

### 1. `Cargo.toml` (line 63)

**PROBLEM**: `llama-cpp-2 = { version = "0.1.138", features = ["vulkan"] }` — the `vulkan` feature is hardcoded, not optional. This forces Vulkan ICD detection at build time and runtime. On A733, Vulkan doesn't exist, so this will fail to build (no `libvulkan-dev` headers) or fail at runtime (no ICD JSON).

**REQUIRED CHANGES**:
1. Add a `vulkan` feature flag that gates the llama-cpp-2 vulkan feature:
   ```toml
   [features]
   default = []  # DO NOT make vulkan default anymore
   client = ["dep:reqwest", "dep:async-stream", "dep:futures", "dep:bytes"]
   sqlite = ["dep:rusqlite"]
   clipboard = ["dep:arboard"]
   vulkan = []  # NEW: enables Vulkan GPU acceleration
   ```

2. Change the llama-cpp-2 dependency to conditionally include vulkan:
   ```toml
   llama-cpp-2 = { version = "0.1.138", features = ["vulkan"], optional = true }
   ```

3. Add a non-optional CPU-only llama-cpp-2 dependency:
   ```toml
   [target.'cfg(not(feature = "vulkan"))'.dependencies]
   llama-cpp-2-cpu = { package = "llama-cpp-2", version = "0.1.138" }

   [target.'cfg(feature = "vulkan")'.dependencies]
   llama-cpp-2 = { version = "0.1.138", features = ["vulkan"] }
   ```

   **IMPORTANT ALTERNATIVE** (if the above target-cfg approach causes duplicate crate conflicts):
   Simply make the dependency conditional on the feature:
   ```toml
   llama-cpp-2 = { version = "0.1.138" }
   ```

   And use a cfg attribute in the Rust code to conditionally pass the vulkan init flag to llama-cpp-2. Check llama-cpp-2's API — the `vulkan` feature likely controls a build-time flag. If it can be set at runtime instead, just drop the feature entirely and handle Vulkan init in application code. **READ llama-cpp-2's docs/source to determine the correct approach before changing.**

**CRITICAL**: After this change, `cargo build` (no features) must produce a CPU-only binary. `cargo build --features vulkan` must produce a Vulkan-capable binary. Both must compile on x86_64. `cargo build --target aarch64-unknown-linux-gnu` (no features) must cross-compile for ARM64.

---

### 2. `src/backend/llama_vulkan.rs`

**PROBLEM**: Struct is named `LlamaCppVulkanBackend` and docs reference "Vulkan GPU acceleration" throughout. However, examining the actual code reveals this is just an HTTP client that talks to a llama.cpp server over localhost HTTP. It does NOT link Vulkan directly — it uses `llama-cpp-2` Rust bindings which handle the GPU connection internally. The naming is misleading.

**REQUIRED CHANGES**:
1. Rename the struct from `LlamaCppVulkanBackend` to `LlamaCppBackend` (or `LlamaCppCpuBackend` for the non-vulkan variant). **HOWEVER**: this is a PUBLIC API change that affects all imports. Check if this struct is used outside this crate (it isn't — it's a binary crate, not a library published on crates.io). So renaming is safe.

2. **SAFER APPROACH** (preserving API compat): Keep `LlamaCppVulkanBackend` as a type alias for backward compat:
   ```rust
   /// Llama.cpp backend with Vulkan support.
   #[derive(Debug, Clone)]
   pub struct LlamaCppBackend { /* same fields */ }

   /// Backward-compatible alias.
   #[deprecated(note = "Use LlamaCppBackend instead. This works with both CPU and Vulkan llama.cpp servers.")]
   pub type LlamaCppVulkanBackend = LlamaCppBackend;
   ```

3. Add conditional compilation for any llama-cpp-2 Vulkan-specific initialization. If the file uses `#[cfg(feature = "vulkan")]` to gate any code, add the appropriate `#[cfg(not(feature = "vulkan"))]` fallbacks.

4. Update all doc comments that say "Vulkan GPU acceleration" to say "llama.cpp backend (CPU or GPU)".

5. The HTTP client approach (talking to llama-server over HTTP) is correct for both CPU and Vulkan. No changes needed to the HTTP logic itself — llama-server handles backend selection internally.

---

### 3. `src/backend/mod.rs`

**PROBLEM**: Hardcodes `LlamaCppVulkanBackend` re-export.

**REQUIRED CHANGES**:
1. If you renamed the struct, update the re-export:
   ```rust
   pub use llama_vulkan::LlamaCppBackend;
   #[allow(deprecated)]
   pub use llama_vulkan::LlamaCppVulkanBackend;  // backward compat
   ```

2. Consider renaming `llama_vulkan.rs` to `llama_cpp.rs` to reflect the backend-agnostic nature. If you do, update the `pub mod` declaration. **Check all imports across the entire codebase first** using grep/lsp_find_references before renaming.

---

### 4. `config.yml` (root config file)

**PROBLEM**: Multiple settings are tuned for x86_64 + AMD Vulkan GPU with 16GB+ RAM:
- `context.size: 100000` — 100K context on 12GB LPDDR5 with a 9B model WILL OOM. 9B Q4_K_M ≈ 6GB model + KV cache. At 100K ctx with q8_0 cache, KV alone could be 3-4GB. 6GB + 4GB = 10GB, leaving 2GB for OS + everything else. Tight and likely OOM.
- `hardware.gpu_layers: 99` — No GPU on A733, this is meaningless.
- `hardware.threads: 6` — Should be 8 for the 8-core A733 (2×A76 + 6×A55).
- `hardware.flash_attn: true` — May or may not work in CPU mode. Check llama.cpp docs. If unsupported in CPU-only, set false.
- `vulkan.visible_devices: "0"` — No Vulkan on A733.

**REQUIRED CHANGES**:
1. Reduce `context.size` to `4096` or `8192` (safe for 12GB with 9B Q4; 16384 possible but risky with large models).
2. Set `hardware.gpu_layers: 0` (explicitly CPU-only).
3. Set `hardware.threads: 8` (use all A733 cores).
4. Remove or conditionally ignore the `vulkan:` section.
5. Consider reducing `cache.cache_type_k` and `cache.cache_type_v` from `q8_0` to `q4_0` to save memory on 12GB.

**HOWEVER**: Do NOT break the x86_64 Vulkan desktop config. The approach should be:
- Either create a separate `config.a733.yml` file with ARM-optimized settings
- Or make the config values overridable via environment variables (they already are in `.env.example`)
- Or add profile sections to config.yml: `profiles: { desktop: { ... }, a733: { ... } }`

**RECOMMENDED**: Create `configs/profiles/a733.yml` with:
```yaml
context:
  size: 8192
  batch_size: 512
  ubatch_size: 128
hardware:
  threads: 8
  gpu_layers: 0
  use_mmap: true
  flash_attn: false  # safer for CPU-only
cache:
  cache_type_k: q4_0  # save memory vs q8_0
  cache_type_v: q4_0
vulkan:
  visible_devices: ""  # disabled
```

---

### 5. `.env.example`

**PROBLEM**: `LLAMACPP_BACKEND=vulkan` and `LLAMACPP_GPU_LAYERS=99` are Vulkan-desktop defaults.

**REQUIRED CHANGES**:
1. Change `LLAMACPP_BACKEND=vulkan` to `LLAMACPP_BACKEND=cpu` as default (desktop users can override).
   - **OR**: Add comments showing both:
     ```
     # llama.cpp Backend
     # Options: cpu, vulkan, cuda, metal
     # Default: cpu (use vulkan for GPU-accelerated desktops)
     LLAMACPP_BACKEND=cpu
     ```
2. Change `LLAMACPP_GPU_LAYERS=99` to `LLAMACPP_GPU_LAYERS=0` as default.
3. Add `MAX_ALLOCATED_MEMORY_MB=8192` comment about reducing for SBCs:
   ```
   # For 12GB ARM SBCs: reduce to 8192 to leave room for OS
   ```

---

### 6. `docker/Dockerfile`

**PROBLEM**: Hardcoded for x86_64 + Vulkan:
- `FROM ghcr.io/ggml-org/llama.cpp:server-vulkan` — amd64-only image, won't build on ARM
- Line 58: `yq_linux_amd64` — wrong arch for A733
- Installs `libvulkan1`, `mesa-vulkan-drivers`, `vulkan-tools` — useless on A733
- `ENV AMD_VULKAN_ICD=RADV` — meaningless on ARM
- `GGML_VK_FORCE_MAX_ALLOCATION_SIZE` env var — Vulkan-only
- Validates `radeon_icd.json` exists — will fail on ARM

**REQUIRED CHANGES — TWO-PHASE APPROACH**:

**Phase A**: Create a new `docker/Dockerfile.aarch64` (DO NOT modify the existing Dockerfile — it works for desktop):
```dockerfile
# Stage 1: Build llama.cpp from source for aarch64 (CPU-only)
FROM ubuntu:24.04 AS builder

ARG LLAMA_CPP_VERSION=b4450

ENV DEBIAN_FRONTEND=noninteractive

RUN apt-get update && apt-get install -y --no-install-recommends \
    build-essential cmake git wget ca-certificates \
    && rm -rf /var/lib/apt/lists/*

# Build llama.cpp with CPU-only support (no Vulkan, no CUDA)
RUN git clone --depth 1 --branch master https://github.com/ggerganov/llama.cpp /tmp/llama.cpp \
    && cd /tmp/llama.cpp \
    && cmake -B build -DLLAMA_CURL=ON -DLLAMA_NATIVE=OFF -DLLAMA_ACCELERATE=OFF \
       -DLLAMA_BLAS=OFF -DLLAMA_VULKAN=OFF -DLLAMA_CUDA=OFF \
       -DLLAMA_METAL=OFF -DLLAMA_OPENMP=ON \
       -DCMAKE_BUILD_TYPE=Release \
    && cmake --build build --config Release -j$(nproc)

# Stage 2: Minimal runtime image
FROM ubuntu:24.04

ENV DEBIAN_FRONTEND=noninteractive
ENV TZ=UTC
ENV PYTHONUNBUFFERED=1
ENV HF_HUB_DISABLE_TELEMETRY=1

# Minimal runtime deps — NO Vulkan libs
RUN apt-get update && apt-get install -y --no-install-recommends \
    python3 \
    python3-pip \
    curl \
    tini \
    ca-certificates \
    libgomp1 \
    && rm -rf /var/lib/apt/lists/*

RUN pip3 install --no-cache-dir --break-system-packages huggingface-hub

# Install yq for aarch64 (NOT amd64)
RUN curl -fsSL https://github.com/mikefarah/yq/releases/latest/download/yq_linux_arm64 -o /usr/local/bin/yq && \
    chmod +x /usr/local/bin/yq

COPY --from=builder /tmp/llama.cpp/build/bin/llama-server /usr/local/bin/llama-server

RUN mkdir -p /models /config /var/log/llama-server /var/run/llama-server

WORKDIR /app
COPY docker/entrypoint.sh /entrypoint.sh
RUN chmod +x /entrypoint.sh

EXPOSE 8080

HEALTHCHECK --interval=30s --timeout=10s --start-period=60s --retries=3 \
    CMD curl -f http://localhost:8080/props || exit 1

VOLUME ["/models", "/config", "/var/log/llama-server"]

RUN useradd -m -u 1001 -s /bin/bash llama && \
    chown -R llama:llama /models /config /var/log/llama-server /app
USER llama

ENTRYPOINT ["tini", "--"]
CMD ["/entrypoint.sh"]
```

**Phase B**: In the existing `docker/Dockerfile`, add a multi-arch `ARG` for yq:
```dockerfile
ARG TARGETARCH
RUN if [ "$TARGETARCH" = "arm64" ]; then \
        curl -fsSL https://github.com/mikefarah/yq/releases/latest/download/yq_linux_arm64 -o /usr/local/bin/yq; \
    else \
        curl -fsSL https://github.com/mikefarah/yq/releases/latest/download/yq_linux_amd64 -o /usr/local/bin/yq; \
    fi && chmod +x /usr/local/bin/yq
```

---

### 7. `docker/docker-compose.yml`

**PROBLEM**: Hardcoded for desktop Vulkan GPU:
- `devices: /dev/dri:/dev/dri` — GPU passthrough, doesn't exist on A733
- `group_add: "${RENDER_GID:-987}"`, `"${VIDEO_GID:-983}"` — GPU group IDs
- `shm_size: 8g` — excessive for 12GB SBC
- `AMD_VULKAN_ICD: RADV` — Vulkan env var
- `GGML_VK_DISABLE_DEBUG: "1"` — Vulkan env var
- `RADV_PERFTEST: "nogttspill"` — AMD GPU performance tuning

**REQUIRED CHANGES**: Create `docker/docker-compose.aarch64.yml` (DO NOT modify the desktop compose file):
```yaml
services:
  llama-server:
    build:
      context: ..
      dockerfile: docker/Dockerfile.aarch64
    container_name: whitt-llama-server
    restart: unless-stopped
    volumes:
      - ../configs/profiles/a733.yml:/config/config.yml:ro
      - ../configs/models:/config/models:ro
      - ./entrypoint.sh:/entrypoint.sh:ro
      - ${MODEL_HOST_PATH:-../models}:/models
      - ../logs:/var/log/llama-server
    # NO devices: — no GPU on A733
    network_mode: host
    shm_size: 2g  # reduced for 12GB SBC
    user: "${DOCKER_UID:-1000}:${DOCKER_GID:-1000}"
    environment:
      HUGGING_FACE_HUB_TOKEN: ${HF_TOKEN:-}
      HF_HOME: /tmp/hf
      LLAMA_ARG_N_GPU_LAYERS: "0"  # explicit CPU-only
    healthcheck:
      test: ["CMD", "curl", "-f", "http://localhost:8080/props"]
      interval: 30s
      timeout: 10s
      retries: 3
      start_period: 120s  # longer startup for CPU inference
    logging:
      driver: "json-file"
      options:
        max-size: "10m"
        max-file: "3"

volumes:
  models:
    driver: local
  logs:
    driver: local

networks:
  default:
    name: whitt-network
```

---

### 8. `docker/entrypoint.sh`

**PROBLEM**: `validate_vulkan()` function (lines 400-460) checks for `/dev/dri`, radeon ICD, vulkaninfo — all of which fail on A733. The function does gracefully fall back to CPU mode (`export LLAMA_ARG_N_GPU_LAYERS="0"`), so it won't crash. But it prints misleading error messages.

**REQUIRED CHANGES**:
1. Add early detection for non-GPU environments. Before `validate_vulkan()`:
   ```bash
   # Detect non-GPU environment (ARM SBC without Vulkan)
   detect_hardware_profile() {
       local arch=$(uname -m)
       if [ "$arch" = "aarch64" ] || [ "$arch" = "arm64" ]; then
           if [ ! -d /dev/dri ]; then
               log_info "Detected ARM64 SBC without GPU device nodes — CPU-only mode."
               export LLAMA_ARG_N_GPU_LAYERS="0"
               return 0
           fi
       fi
       return 1
   }
   ```

2. In `main()`, call `detect_hardware_profile` BEFORE `validate_vulkan`. If it returns 0, skip `validate_vulkan` entirely.

3. Make `validate_vulkan` warnings non-fatal on aarch64 (they already are — it sets `N_GPU_LAYERS=0` and returns 0).

---

### 9. `.github/workflows/ci.yml`

**PROBLEM**: CI only runs on x86_64 (`ubuntu-latest`, `macos-latest`, `windows-latest`). No ARM64 target testing.

**REQUIRED CHANGES**:
1. Add an ARM64 cross-compile job to verify the code compiles for aarch64:
   ```yaml
   cross-compile-arm64:
     name: Cross-compile ARM64
     runs-on: ubuntu-latest
     steps:
       - uses: actions/checkout@v4
       - uses: dtolnay/rust-toolchain@stable
         with:
           targets: aarch64-unknown-linux-gnu
       - run: sudo apt-get update && sudo apt-get install -y gcc-aarch64-linux-gnu
       - run: cargo build --release --target aarch64-unknown-linux-gnu
       - run: cargo build --release --target aarch64-unknown-linux-gnu --features client
   ```

2. Add the `vulkan` feature build as a separate step (to ensure the feature gate works):
   ```yaml
   build-vulkan:
     name: Build (Vulkan)
     runs-on: ubuntu-latest
     steps:
       - uses: actions/checkout@v4
       - uses: dtolnay/rust-toolchain@stable
       - run: cargo build --release --features vulkan
   ```

3. Ensure clippy/check tests pass WITHOUT the vulkan feature (the default build should be CPU-only after your changes).

---

### 10. `configs/models/` (all 14 model config files)

**PROBLEM**: These are per-model configs with context sizes that may be too large for 12GB RAM. Some set `context.size: 8192` which is OK, but any that set larger need review.

**REQUIRED CHANGES**: 
- Review all 14 configs. Flag any with `context.size` > 8192 as "may OOM on 12GB ARM".
- No need to modify these files — they are per-model configs that users override at runtime. Just ensure the a733 profile in `configs/profiles/a733.yml` takes precedence.

---

### 11. `README.md`

**PROBLEM**: Quick Start only shows Vulkan desktop path. Prerequisites say "Docker with GPU support (Vulkan compatible)".

**REQUIRED CHANGES**:
1. Add ARM64 / A733 section to README:
   ```markdown
   ## ARM64 / SBC (CPU-only)

   # Cross-compile from x86_64 host
   cargo build --release --target aarch64-unknown-linux-gnu

   # Copy binary to SBC
   scp target/aarch64-unknown-linux-gnu/release/whitt user@opi-zero3w:~/

   # On the SBC: install runtime deps
   sudo apt install libstdc++ libgomp1 ca-certificates

   # Build llama.cpp natively (CPU-only)
   git clone https://github.com/ggerganov/llama.cpp && cd llama.cpp
   cmake -B build -DLLAMA_CURL=ON -DLLAMA_OPENMP=ON -DCMAKE_BUILD_TYPE=Release
   cmake --build build --config Release -j$(nproc)

   # Start server
   ./build/bin/llama-server -m /path/to/model.gguf -c 8192 -t 8 -ngl 0

   # Or use Docker (aarch64 image)
   docker compose -f docker/docker-compose.aarch64.yml up -d
   ```

2. Update Prerequisites to not require GPU:
   ```markdown
   ### Prerequisites
   - Rust 1.70+ (edition 2021)
   - Docker (optional, for llama.cpp server)
   - Linux or macOS
   - For GPU: Vulkan-compatible GPU with drivers (optional — CPU mode works on all platforms)
   ```

---

### 12. `src/config/provider.rs`

**PROBLEM**: Contains `LlamaCppVulkanProvider` struct and provider config. References Vulkan throughout doc comments.

**REQUIRED CHANGES**:
1. If struct was renamed in `llama_vulkan.rs`, update references here.
2. If keeping `LlamaCppVulkanBackend` as a type alias, add deprecation notice to `LlamaCppVulkanProvider` too:
   ```rust
   #[deprecated(note = "Use LlamaCppProvider instead. Works with CPU and GPU llama.cpp servers.")]
   pub type LlamaCppVulkanProvider = LlamaCppProvider;
   ```
3. Update doc comments to reflect "llama.cpp backend (CPU or GPU)".
4. **IMPORTANT**: Use `lsp_find_references` on `LlamaCppVulkanProvider` to find ALL usages before renaming. The workflow schema YAML uses `llama_cpp_with_vulkan` as a provider name — that string is user-facing config, not a Rust type. Keep backward compat.

---

## FILES YOU MUST READ BEFORE STARTING

Read these files to understand the full codebase before making changes:
1. `Cargo.toml` — full file
2. `src/backend/mod.rs` — full file (12 lines)
3. `src/backend/llama_vulkan.rs` — full file (647 lines)
4. `src/backend/llm_backend.rs` — full file (149 lines)
5. `src/config/provider.rs` — full file (489 lines)
6. `docker/entrypoint.sh` — full file (747 lines)
7. `docker/Dockerfile` — full file (101 lines)
8. `docker/docker-compose.yml` — full file (51 lines)
9. `.env.example` — full file (203 lines)
10. `config.yml` — full file (48 lines)
11. `.github/workflows/ci.yml` — full file (242 lines)
12. `README.md` — full file (174 lines)
13. ALL 14 files in `configs/models/` — to verify context sizes
14. `docs/contributing/environment-variables.md` — if it exists, may reference Vulkan env vars

Use `grep -r "vulkan" src/` to find ALL Vulkan references in Rust code.
Use `grep -r "vulkan" docker/` to find ALL Vulkan references in Docker files.
Use `grep -r "VULKAN\|vulkan" . --include="*.yml" --include="*.yaml"` to find ALL YAML references.
Use `grep -r "LlamaCppVulkan" src/` to find all struct/type references.
Use `grep -r "llama_cpp_with_vulkan" . --include="*.yml" --include="*.rs"` to find all provider name strings.

---

## VERIFICATION CHECKLIST

After ALL changes, verify:

1. **`cargo check`** — compiles without errors (CPU-only, no features)
2. **`cargo check --features vulkan`** — compiles with Vulkan feature enabled
3. **`cargo check --features client`** — compiles with client feature (HTTP client)
4. **`cargo check --features "client,vulkan"`** — compiles with both
5. **`cargo check --features "client"` with `--target aarch64-unknown-linux-gnu`** — cross-compile check (requires `gcc-aarch64-linux-gnu` installed)
6. **`cargo clippy --all-targets -- -D warnings`** — no clippy warnings
7. **`cargo fmt -- --check`** — formatting OK
8. **`cargo test`** — all tests pass (637 tests, 0 failures — do not introduce regressions)
9. **`cargo test --features client`** — client tests pass
10. **`lsp_diagnostics`** on every changed file — zero errors
11. **`docker build -f docker/Dockerfile.aarch64 .`** — Dockerfile builds (if Docker available)
12. **README links are valid** — no broken references

---

## THINGS YOU MUST NOT DO

1. **DO NOT** break the existing x86_64 Vulkan desktop workflow. All existing functionality must continue to work when `--features vulkan` is passed.
2. **DO NOT** change the llama.cpp HTTP API protocol. The engine talks HTTP to llama-server — this doesn't change between CPU and GPU.
3. **DO NOT** add new crate dependencies unless absolutely necessary for the CPU/GPU feature split.
4. **DO NOT** modify any test files. Tests mock the backend and don't care about Vulkan vs CPU.
5. **DO NOT** delete `docker/Dockerfile` or `docker/docker-compose.yml`. Create new `*.aarch64*` variants alongside them.
6. **DO NOT** change the default Rust edition, MSRV, or profile settings.
7. **DO NOT** use `as any`, `@ts-ignore`, or similar suppression. If something doesn't compile, fix it properly.
8. **DO NOT** commit anything. Just make the changes.
9. **DO NOT** build or run the project — just ensure it compiles and tests pass.

---

## BUILD CROSS-COMPILE TOOLCHAIN SETUP (for reference, not a task)

On the x86_64 host, to cross-compile for aarch64:
```bash
# Install cross-compile toolchain
sudo apt install gcc-aarch64-linux-gnu g++-aarch64-linux-gnu

# Add Rust target
rustup target add aarch64-unknown-linux-gnu

# Configure cargo for cross-compilation (in .cargo/config.toml)
# [target.aarch64-unknown-linux-gnu]
# linker = "aarch64-linux-gnu-gcc"

# Build CPU-only binary for ARM64
cargo build --release --target aarch64-unknown-linux-gnu

# Build with client features
cargo build --release --target aarch64-unknown-linux-gnu --features client
```

---

## ON THE SBC (reference, not a task)

Once the binary is deployed to the Orange Pi Zero 3W:
```bash
# Install runtime dependencies
sudo apt update
sudo apt install -y libstdc++6 libgomp1 ca-certificates

# Verify binary runs
./whitt --version || ./whitt --help

# Build llama.cpp natively (CPU-only, with OpenMP for multi-core)
sudo apt install -y build-essential cmake git wget
git clone --depth 1 https://github.com/ggerganov/llama.cpp
cd llama.cpp
cmake -B build \
  -DLLAMA_CURL=ON \
  -DLLAMA_OPENMP=ON \
  -DLLAMA_NATIVE=OFF \
  -DCMAKE_BUILD_TYPE=Release
cmake --build build --config Release -j$(nproc)

# Download a small model (SmolLM3 ~2GB Q4_K_M, fits easily in 12GB)
wget -O models/SmolLM3-Q4_K_M.gguf \
  "https://huggingface.co/./resolve/main/SmolLM3-Q4_K_M.gguf"

# Start server (CPU-only, 8 threads, 8K context)
./build/bin/llama-server \
  -m models/SmolLM3-Q4_K_M.gguf \
  -c 8192 \
  -t 8 \
  -ngl 0 \
  --host 127.0.0.1 \
  --port 8080

# In another terminal: test the engine
./whitt chat --model SmolLM3
```

---

## SUMMARY OF ALL FILES TO CREATE OR MODIFY

| File | Action | Priority |
|------|--------|----------|
| `Cargo.toml` | MODIFY — make vulkan feature optional, CPU default | P0 (blocks everything) |
| `src/backend/llama_vulkan.rs` | MODIFY — rename struct, update docs, add cfg gates | P0 |
| `src/backend/mod.rs` | MODIFY — update re-exports for renamed struct | P0 |
| `src/config/provider.rs` | MODIFY — rename provider struct, add type alias | P1 |
| `.env.example` | MODIFY — change defaults to CPU | P1 |
| `config.yml` | MODIFY — reduce context, set gpu_layers=0, threads=8 | P1 |
| `configs/profiles/a733.yml` | CREATE — ARM-optimized config profile | P1 |
| `docker/Dockerfile.aarch64` | CREATE — CPU-only ARM64 Docker image | P2 |
| `docker/docker-compose.aarch64.yml` | CREATE — ARM64 compose without GPU | P2 |
| `docker/entrypoint.sh` | MODIFY — add ARM detection, skip Vulkan validate | P2 |
| `.github/workflows/ci.yml` | MODIFY — add ARM64 cross-compile job | P2 |
| `README.md` | MODIFY — add ARM64/SBC section, update prerequisites | P3 |
| `docker/Dockerfile` | MODIFY — add TARGETARCH arg for yq download | P3 |
| `.cargo/config.toml` | CREATE — cross-compile linker config for aarch64 | P2 |

## [0127] 06-30 18:12

(2026-06-30T18:12:48)
implement and verify all missing items and gaps then build integration and e2e testing that does not actually run a model but runs through local testing of the code and scripts.  I want 100% coverage of real user flows with the requirements and changes for the orange pi, and I want it all separate from the linux install stuff.

## [0128] 06-30 18:34

(2026-06-30T18:34:57)
[search-mode]
MAXIMIZE SEARCH EFFORT. Launch multiple background agents IN PARALLEL:
- explore agents (codebase patterns, file structures, ast-grep)
- librarian agents (remote repos, official docs, GitHub examples)
Plus direct tools: Grep, ripgrep (rg), ast-grep (sg)
NEVER stop at first result - be exhaustive.

[analyze-mode]
ANALYSIS MODE. Gather context before diving deep:
CONTEXT GATHERING (parallel):
- 1-2 explore agents (codebase patterns, implementations)
- 1-2 librarian agents (if external library involved)
- Direct tools: Grep, AST-grep, LSP for targeted searches

IF COMPLEX - DO NOT STRUGGLE ALONE. Consult specialists:
- **Oracle**: Conventional problems (architecture, debugging, complex logic)
- **Artistry**: Non-conventional problems (different approach needed)

SYNTHESIZE findings before proceeding.
---
MANDATORY delegate_task params: ALWAYS include load_skills=[] and run_in_background when calling delegate_task.
Example: delegate_task(subagent_type="explore", prompt="...", run_in_background=true, load_skills=[])

---

What remains un-unit tested or unimplemented from this prompt?  No changes just answer exhaustively:

so on this git workspace I'm wanting to get a new command with the cli that will install onto an orangepi 3w 12gb lpddr5 ram allwinner A733 to run locally on that board and to be installable from scripts and a new cli set of commands onto the device with a good .md doc explaining how to use the CLI and a well formatted easily skimmable way.  here is some info you need to be aware of as you go.  don't run anything outside of unit tests while you work.  on the other worktree im local system stress testing so the resources don't exist for you to live system test so make sure you only do local test runs of things as you write them and do so iteratively and critically review and fix and re evaluate before continuing on to the next task so you don't drift or think you've implemented something when in reality it won't work in real life when I get the device in the mail in a few weeks.

# Orange Pi Zero 3W (A733) Build — Exhaustive Implementation Prompt

**Target workspace**: `~/code/whitt-execution-engine/` (branch: `initial-creation`)
**Target hardware**: Orange Pi Zero 3W, Allwinner A733 SoC, 12GB LPDDR5, ARM64 (aarch64)
**Target OS**: Debian Bookworm minimal server, BSP kernel `6.6.98-sun60iw2` (NO Vulkan/GPU userspace)

---

## CONTEXT

### What this project is
whitt-execution-engine is a Rust CLI workflow engine that orchestrates LLM-powered YAML workflows. It talks to a llama.cpp HTTP server via OpenAI-compatible API. The engine itself is headless (no GUI). Currently hardcoded for x86_64 + AMD Vulkan GPU (RX 580). Needs to become portable to ARM64 CPU-only on an Allwinner A733 SBC with 12GB RAM.

### What the A733 hardware CAN and CANNOT do
- **CAN run**: ARM64 (aarch64) Linux with BSP kernel, headless workloads, CPU inference via llama.cpp
- **CANNOT do**: No Vulkan userspace on stock images (PowerVR BXM-4-64 has no open-source driver, no ICD). No GPU device nodes (`/dev/dri` doesn't exist). No X11/Wayland display server (headless only). No Docker with GPU passthrough.
- **Performance expectation**: llama.cpp CPU-only on 8 ARM cores (2×A76@2.0G + 6×A55@1.8G) with 12GB LPDDR5 @2400MHz 32-bit (~19.2 GB/s bandwidth). Expect ~1-4 tok/s on 3B models, ~0.5-1.5 tok/s on 7B-9B models. NOT fast. This is a Tier A/B micro-orchestrator, not a Tier C speed rig.

### Key constraint: CROSS-COMPILE
391 crates with LTO + codegen-units=1. Compiling on the 12GB ARM SBC would take hours and may OOM during linking. ALL Rust compilation must happen on the x86_64 host, then scp the binary to the SBC. llama.cpp server should be compiled natively on the SBC OR cross-compiled.

---

## EXHAUSTIVE FILE-BY-FILE CHANGES REQUIRED

Every file listed below needs specific modifications. DO NOT skip any file. DO NOT add new dependencies unless absolutely necessary. DO NOT restructure the crate — make surgical, minimal changes.

---

### 1. `Cargo.toml` (line 63)

**PROBLEM**: `llama-cpp-2 = { version = "0.1.138", features = ["vulkan"] }` — the `vulkan` feature is hardcoded, not optional. This forces Vulkan ICD detection at build time and runtime. On A733, Vulkan doesn't exist, so this will fail to build (no `libvulkan-dev` headers) or fail at runtime (no ICD JSON).

**REQUIRED CHANGES**:
1. Add a `vulkan` feature flag that gates the llama-cpp-2 vulkan feature:
   ```toml
   [features]
   default = []  # DO NOT make vulkan default anymore
   client = ["dep:reqwest", "dep:async-stream", "dep:futures", "dep:bytes"]
   sqlite = ["dep:rusqlite"]
   clipboard = ["dep:arboard"]
   vulkan = []  # NEW: enables Vulkan GPU acceleration
   ```

2. Change the llama-cpp-2 dependency to conditionally include vulkan:
   ```toml
   llama-cpp-2 = { version = "0.1.138", features = ["vulkan"], optional = true }
   ```

3. Add a non-optional CPU-only llama-cpp-2 dependency:
   ```toml
   [target.'cfg(not(feature = "vulkan"))'.dependencies]
   llama-cpp-2-cpu = { package = "llama-cpp-2", version = "0.1.138" }

   [target.'cfg(feature = "vulkan")'.dependencies]
   llama-cpp-2 = { version = "0.1.138", features = ["vulkan"] }
   ```

   **IMPORTANT ALTERNATIVE** (if the above target-cfg approach causes duplicate crate conflicts):
   Simply make the dependency conditional on the feature:
   ```toml
   llama-cpp-2 = { version = "0.1.138" }
   ```

   And use a cfg attribute in the Rust code to conditionally pass the vulkan init flag to llama-cpp-2. Check llama-cpp-2's API — the `vulkan` feature likely controls a build-time flag. If it can be set at runtime instead, just drop the feature entirely and handle Vulkan init in application code. **READ llama-cpp-2's docs/source to determine the correct approach before changing.**

**CRITICAL**: After this change, `cargo build` (no features) must produce a CPU-only binary. `cargo build --features vulkan` must produce a Vulkan-capable binary. Both must compile on x86_64. `cargo build --target aarch64-unknown-linux-gnu` (no features) must cross-compile for ARM64.

---

### 2. `src/backend/llama_vulkan.rs`

**PROBLEM**: Struct is named `LlamaCppVulkanBackend` and docs reference "Vulkan GPU acceleration" throughout. However, examining the actual code reveals this is just an HTTP client that talks to a llama.cpp server over localhost HTTP. It does NOT link Vulkan directly — it uses `llama-cpp-2` Rust bindings which handle the GPU connection internally. The naming is misleading.

**REQUIRED CHANGES**:
1. Rename the struct from `LlamaCppVulkanBackend` to `LlamaCppBackend` (or `LlamaCppCpuBackend` for the non-vulkan variant). **HOWEVER**: this is a PUBLIC API change that affects all imports. Check if this struct is used outside this crate (it isn't — it's a binary crate, not a library published on crates.io). So renaming is safe.

2. **SAFER APPROACH** (preserving API compat): Keep `LlamaCppVulkanBackend` as a type alias for backward compat:
   ```rust
   /// Llama.cpp backend with Vulkan support.
   #[derive(Debug, Clone)]
   pub struct LlamaCppBackend { /* same fields */ }

   /// Backward-compatible alias.
   #[deprecated(note = "Use LlamaCppBackend instead. This works with both CPU and Vulkan llama.cpp servers.")]
   pub type LlamaCppVulkanBackend = LlamaCppBackend;
   ```

3. Add conditional compilation for any llama-cpp-2 Vulkan-specific initialization. If the file uses `#[cfg(feature = "vulkan")]` to gate any code, add the appropriate `#[cfg(not(feature = "vulkan"))]` fallbacks.

4. Update all doc comments that say "Vulkan GPU acceleration" to say "llama.cpp backend (CPU or GPU)".

5. The HTTP client approach (talking to llama-server over HTTP) is correct for both CPU and Vulkan. No changes needed to the HTTP logic itself — llama-server handles backend selection internally.

---

### 3. `src/backend/mod.rs`

**PROBLEM**: Hardcodes `LlamaCppVulkanBackend` re-export.

**REQUIRED CHANGES**:
1. If you renamed the struct, update the re-export:
   ```rust
   pub use llama_vulkan::LlamaCppBackend;
   #[allow(deprecated)]
   pub use llama_vulkan::LlamaCppVulkanBackend;  // backward compat
   ```

2. Consider renaming `llama_vulkan.rs` to `llama_cpp.rs` to reflect the backend-agnostic nature. If you do, update the `pub mod` declaration. **Check all imports across the entire codebase first** using grep/lsp_find_references before renaming.

---

### 4. `config.yml` (root config file)

**PROBLEM**: Multiple settings are tuned for x86_64 + AMD Vulkan GPU with 16GB+ RAM:
- `context.size: 100000` — 100K context on 12GB LPDDR5 with a 9B model WILL OOM. 9B Q4_K_M ≈ 6GB model + KV cache. At 100K ctx with q8_0 cache, KV alone could be 3-4GB. 6GB + 4GB = 10GB, leaving 2GB for OS + everything else. Tight and likely OOM.
- `hardware.gpu_layers: 99` — No GPU on A733, this is meaningless.
- `hardware.threads: 6` — Should be 8 for the 8-core A733 (2×A76 + 6×A55).
- `hardware.flash_attn: true` — May or may not work in CPU mode. Check llama.cpp docs. If unsupported in CPU-only, set false.
- `vulkan.visible_devices: "0"` — No Vulkan on A733.

**REQUIRED CHANGES**:
1. Reduce `context.size` to `4096` or `8192` (safe for 12GB with 9B Q4; 16384 possible but risky with large models).
2. Set `hardware.gpu_layers: 0` (explicitly CPU-only).
3. Set `hardware.threads: 8` (use all A733 cores).
4. Remove or conditionally ignore the `vulkan:` section.
5. Consider reducing `cache.cache_type_k` and `cache.cache_type_v` from `q8_0` to `q4_0` to save memory on 12GB.

**HOWEVER**: Do NOT break the x86_64 Vulkan desktop config. The approach should be:
- Either create a separate `config.a733.yml` file with ARM-optimized settings
- Or make the config values overridable via environment variables (they already are in `.env.example`)
- Or add profile sections to config.yml: `profiles: { desktop: { ... }, a733: { ... } }`

**RECOMMENDED**: Create `configs/profiles/a733.yml` with:
```yaml
context:
  size: 8192
  batch_size: 512
  ubatch_size: 128
hardware:
  threads: 8
  gpu_layers: 0
  use_mmap: true
  flash_attn: false  # safer for CPU-only
cache:
  cache_type_k: q4_0  # save memory vs q8_0
  cache_type_v: q4_0
vulkan:
  visible_devices: ""  # disabled
```

---

### 5. `.env.example`

**PROBLEM**: `LLAMACPP_BACKEND=vulkan` and `LLAMACPP_GPU_LAYERS=99` are Vulkan-desktop defaults.

**REQUIRED CHANGES**:
1. Change `LLAMACPP_BACKEND=vulkan` to `LLAMACPP_BACKEND=cpu` as default (desktop users can override).
   - **OR**: Add comments showing both:
     ```
     # llama.cpp Backend
     # Options: cpu, vulkan, cuda, metal
     # Default: cpu (use vulkan for GPU-accelerated desktops)
     LLAMACPP_BACKEND=cpu
     ```
2. Change `LLAMACPP_GPU_LAYERS=99` to `LLAMACPP_GPU_LAYERS=0` as default.
3. Add `MAX_ALLOCATED_MEMORY_MB=8192` comment about reducing for SBCs:
   ```
   # For 12GB ARM SBCs: reduce to 8192 to leave room for OS
   ```

---

### 6. `docker/Dockerfile`

**PROBLEM**: Hardcoded for x86_64 + Vulkan:
- `FROM ghcr.io/ggml-org/llama.cpp:server-vulkan` — amd64-only image, won't build on ARM
- Line 58: `yq_linux_amd64` — wrong arch for A733
- Installs `libvulkan1`, `mesa-vulkan-drivers`, `vulkan-tools` — useless on A733
- `ENV AMD_VULKAN_ICD=RADV` — meaningless on ARM
- `GGML_VK_FORCE_MAX_ALLOCATION_SIZE` env var — Vulkan-only
- Validates `radeon_icd.json` exists — will fail on ARM

**REQUIRED CHANGES — TWO-PHASE APPROACH**:

**Phase A**: Create a new `docker/Dockerfile.aarch64` (DO NOT modify the existing Dockerfile — it works for desktop):
```dockerfile
# Stage 1: Build llama.cpp from source for aarch64 (CPU-only)
FROM ubuntu:24.04 AS builder

ARG LLAMA_CPP_VERSION=b4450

ENV DEBIAN_FRONTEND=noninteractive

RUN apt-get update && apt-get install -y --no-install-recommends \
    build-essential cmake git wget ca-certificates \
    && rm -rf /var/lib/apt/lists/*

# Build llama.cpp with CPU-only support (no Vulkan, no CUDA)
RUN git clone --depth 1 --branch master https://github.com/ggerganov/llama.cpp /tmp/llama.cpp \
    && cd /tmp/llama.cpp \
    && cmake -B build -DLLAMA_CURL=ON -DLLAMA_NATIVE=OFF -DLLAMA_ACCELERATE=OFF \
       -DLLAMA_BLAS=OFF -DLLAMA_VULKAN=OFF -DLLAMA_CUDA=OFF \
       -DLLAMA_METAL=OFF -DLLAMA_OPENMP=ON \
       -DCMAKE_BUILD_TYPE=Release \
    && cmake --build build --config Release -j$(nproc)

# Stage 2: Minimal runtime image
FROM ubuntu:24.04

ENV DEBIAN_FRONTEND=noninteractive
ENV TZ=UTC
ENV PYTHONUNBUFFERED=1
ENV HF_HUB_DISABLE_TELEMETRY=1

# Minimal runtime deps — NO Vulkan libs
RUN apt-get update && apt-get install -y --no-install-recommends \
    python3 \
    python3-pip \
    curl \
    tini \
    ca-certificates \
    libgomp1 \
    && rm -rf /var/lib/apt/lists/*

RUN pip3 install --no-cache-dir --break-system-packages huggingface-hub

# Install yq for aarch64 (NOT amd64)
RUN curl -fsSL https://github.com/mikefarah/yq/releases/latest/download/yq_linux_arm64 -o /usr/local/bin/yq && \
    chmod +x /usr/local/bin/yq

COPY --from=builder /tmp/llama.cpp/build/bin/llama-server /usr/local/bin/llama-server

RUN mkdir -p /models /config /var/log/llama-server /var/run/llama-server

WORKDIR /app
COPY docker/entrypoint.sh /entrypoint.sh
RUN chmod +x /entrypoint.sh

EXPOSE 8080

HEALTHCHECK --interval=30s --timeout=10s --start-period=60s --retries=3 \
    CMD curl -f http://localhost:8080/props || exit 1

VOLUME ["/models", "/config", "/var/log/llama-server"]

RUN useradd -m -u 1001 -s /bin/bash llama && \
    chown -R llama:llama /models /config /var/log/llama-server /app
USER llama

ENTRYPOINT ["tini", "--"]
CMD ["/entrypoint.sh"]
```

**Phase B**: In the existing `docker/Dockerfile`, add a multi-arch `ARG` for yq:
```dockerfile
ARG TARGETARCH
RUN if [ "$TARGETARCH" = "arm64" ]; then \
        curl -fsSL https://github.com/mikefarah/yq/releases/latest/download/yq_linux_arm64 -o /usr/local/bin/yq; \
    else \
        curl -fsSL https://github.com/mikefarah/yq/releases/latest/download/yq_linux_amd64 -o /usr/local/bin/yq; \
    fi && chmod +x /usr/local/bin/yq
```

---

### 7. `docker/docker-compose.yml`

**PROBLEM**: Hardcoded for desktop Vulkan GPU:
- `devices: /dev/dri:/dev/dri` — GPU passthrough, doesn't exist on A733
- `group_add: "${RENDER_GID:-987}"`, `"${VIDEO_GID:-983}"` — GPU group IDs
- `shm_size: 8g` — excessive for 12GB SBC
- `AMD_VULKAN_ICD: RADV` — Vulkan env var
- `GGML_VK_DISABLE_DEBUG: "1"` — Vulkan env var
- `RADV_PERFTEST: "nogttspill"` — AMD GPU performance tuning

**REQUIRED CHANGES**: Create `docker/docker-compose.aarch64.yml` (DO NOT modify the desktop compose file):
```yaml
services:
  llama-server:
    build:
      context: ..
      dockerfile: docker/Dockerfile.aarch64
    container_name: whitt-llama-server
    restart: unless-stopped
    volumes:
      - ../configs/profiles/a733.yml:/config/config.yml:ro
      - ../configs/models:/config/models:ro
      - ./entrypoint.sh:/entrypoint.sh:ro
      - ${MODEL_HOST_PATH:-../models}:/models
      - ../logs:/var/log/llama-server
    # NO devices: — no GPU on A733
    network_mode: host
    shm_size: 2g  # reduced for 12GB SBC
    user: "${DOCKER_UID:-1000}:${DOCKER_GID:-1000}"
    environment:
      HUGGING_FACE_HUB_TOKEN: ${HF_TOKEN:-}
      HF_HOME: /tmp/hf
      LLAMA_ARG_N_GPU_LAYERS: "0"  # explicit CPU-only
    healthcheck:
      test: ["CMD", "curl", "-f", "http://localhost:8080/props"]
      interval: 30s
      timeout: 10s
      retries: 3
      start_period: 120s  # longer startup for CPU inference
    logging:
      driver: "json-file"
      options:
        max-size: "10m"
        max-file: "3"

volumes:
  models:
    driver: local
  logs:
    driver: local

networks:
  default:
    name: whitt-network
```

---

### 8. `docker/entrypoint.sh`

**PROBLEM**: `validate_vulkan()` function (lines 400-460) checks for `/dev/dri`, radeon ICD, vulkaninfo — all of which fail on A733. The function does gracefully fall back to CPU mode (`export LLAMA_ARG_N_GPU_LAYERS="0"`), so it won't crash. But it prints misleading error messages.

**REQUIRED CHANGES**:
1. Add early detection for non-GPU environments. Before `validate_vulkan()`:
   ```bash
   # Detect non-GPU environment (ARM SBC without Vulkan)
   detect_hardware_profile() {
       local arch=$(uname -m)
       if [ "$arch" = "aarch64" ] || [ "$arch" = "arm64" ]; then
           if [ ! -d /dev/dri ]; then
               log_info "Detected ARM64 SBC without GPU device nodes — CPU-only mode."
               export LLAMA_ARG_N_GPU_LAYERS="0"
               return 0
           fi
       fi
       return 1
   }
   ```

2. In `main()`, call `detect_hardware_profile` BEFORE `validate_vulkan`. If it returns 0, skip `validate_vulkan` entirely.

3. Make `validate_vulkan` warnings non-fatal on aarch64 (they already are — it sets `N_GPU_LAYERS=0` and returns 0).

---

### 9. `.github/workflows/ci.yml`

**PROBLEM**: CI only runs on x86_64 (`ubuntu-latest`, `macos-latest`, `windows-latest`). No ARM64 target testing.

**REQUIRED CHANGES**:
1. Add an ARM64 cross-compile job to verify the code compiles for aarch64:
   ```yaml
   cross-compile-arm64:
     name: Cross-compile ARM64
     runs-on: ubuntu-latest
     steps:
       - uses: actions/checkout@v4
       - uses: dtolnay/rust-toolchain@stable
         with:
           targets: aarch64-unknown-linux-gnu
       - run: sudo apt-get update && sudo apt-get install -y gcc-aarch64-linux-gnu
       - run: cargo build --release --target aarch64-unknown-linux-gnu
       - run: cargo build --release --target aarch64-unknown-linux-gnu --features client
   ```

2. Add the `vulkan` feature build as a separate step (to ensure the feature gate works):
   ```yaml
   build-vulkan:
     name: Build (Vulkan)
     runs-on: ubuntu-latest
     steps:
       - uses: actions/checkout@v4
       - uses: dtolnay/rust-toolchain@stable
       - run: cargo build --release --features vulkan
   ```

3. Ensure clippy/check tests pass WITHOUT the vulkan feature (the default build should be CPU-only after your changes).

---

### 10. `configs/models/` (all 14 model config files)

**PROBLEM**: These are per-model configs with context sizes that may be too large for 12GB RAM. Some set `context.size: 8192` which is OK, but any that set larger need review.

**REQUIRED CHANGES**: 
- Review all 14 configs. Flag any with `context.size` > 8192 as "may OOM on 12GB ARM".
- No need to modify these files — they are per-model configs that users override at runtime. Just ensure the a733 profile in `configs/profiles/a733.yml` takes precedence.

---

### 11. `README.md`

**PROBLEM**: Quick Start only shows Vulkan desktop path. Prerequisites say "Docker with GPU support (Vulkan compatible)".

**REQUIRED CHANGES**:
1. Add ARM64 / A733 section to README:
   ```markdown
   ## ARM64 / SBC (CPU-only)

   # Cross-compile from x86_64 host
   cargo build --release --target aarch64-unknown-linux-gnu

   # Copy binary to SBC
   scp target/aarch64-unknown-linux-gnu/release/whitt user@opi-zero3w:~/

   # On the SBC: install runtime deps
   sudo apt install libstdc++ libgomp1 ca-certificates

   # Build llama.cpp natively (CPU-only)
   git clone https://github.com/ggerganov/llama.cpp && cd llama.cpp
   cmake -B build -DLLAMA_CURL=ON -DLLAMA_OPENMP=ON -DCMAKE_BUILD_TYPE=Release
   cmake --build build --config Release -j$(nproc)

   # Start server
   ./build/bin/llama-server -m /path/to/model.gguf -c 8192 -t 8 -ngl 0

   # Or use Docker (aarch64 image)
   docker compose -f docker/docker-compose.aarch64.yml up -d
   ```

2. Update Prerequisites to not require GPU:
   ```markdown
   ### Prerequisites
   - Rust 1.70+ (edition 2021)
   - Docker (optional, for llama.cpp server)
   - Linux or macOS
   - For GPU: Vulkan-compatible GPU with drivers (optional — CPU mode works on all platforms)
   ```

---

### 12. `src/config/provider.rs`

**PROBLEM**: Contains `LlamaCppVulkanProvider` struct and provider config. References Vulkan throughout doc comments.

**REQUIRED CHANGES**:
1. If struct was renamed in `llama_vulkan.rs`, update references here.
2. If keeping `LlamaCppVulkanBackend` as a type alias, add deprecation notice to `LlamaCppVulkanProvider` too:
   ```rust
   #[deprecated(note = "Use LlamaCppProvider instead. Works with CPU and GPU llama.cpp servers.")]
   pub type LlamaCppVulkanProvider = LlamaCppProvider;
   ```
3. Update doc comments to reflect "llama.cpp backend (CPU or GPU)".
4. **IMPORTANT**: Use `lsp_find_references` on `LlamaCppVulkanProvider` to find ALL usages before renaming. The workflow schema YAML uses `llama_cpp_with_vulkan` as a provider name — that string is user-facing config, not a Rust type. Keep backward compat.

---

## FILES YOU MUST READ BEFORE STARTING

Read these files to understand the full codebase before making changes:
1. `Cargo.toml` — full file
2. `src/backend/mod.rs` — full file (12 lines)
3. `src/backend/llama_vulkan.rs` — full file (647 lines)
4. `src/backend/llm_backend.rs` — full file (149 lines)
5. `src/config/provider.rs` — full file (489 lines)
6. `docker/entrypoint.sh` — full file (747 lines)
7. `docker/Dockerfile` — full file (101 lines)
8. `docker/docker-compose.yml` — full file (51 lines)
9. `.env.example` — full file (203 lines)
10. `config.yml` — full file (48 lines)
11. `.github/workflows/ci.yml` — full file (242 lines)
12. `README.md` — full file (174 lines)
13. ALL 14 files in `configs/models/` — to verify context sizes
14. `docs/contributing/environment-variables.md` — if it exists, may reference Vulkan env vars

Use `grep -r "vulkan" src/` to find ALL Vulkan references in Rust code.
Use `grep -r "vulkan" docker/` to find ALL Vulkan references in Docker files.
Use `grep -r "VULKAN\|vulkan" . --include="*.yml" --include="*.yaml"` to find ALL YAML references.
Use `grep -r "LlamaCppVulkan" src/` to find all struct/type references.
Use `grep -r "llama_cpp_with_vulkan" . --include="*.yml" --include="*.rs"` to find all provider name strings.

---

## VERIFICATION CHECKLIST

After ALL changes, verify:

1. **`cargo check`** — compiles without errors (CPU-only, no features)
2. **`cargo check --features vulkan`** — compiles with Vulkan feature enabled
3. **`cargo check --features client`** — compiles with client feature (HTTP client)
4. **`cargo check --features "client,vulkan"`** — compiles with both
5. **`cargo check --features "client"` with `--target aarch64-unknown-linux-gnu`** — cross-compile check (requires `gcc-aarch64-linux-gnu` installed)
6. **`cargo clippy --all-targets -- -D warnings`** — no clippy warnings
7. **`cargo fmt -- --check`** — formatting OK
8. **`cargo test`** — all tests pass (637 tests, 0 failures — do not introduce regressions)
9. **`cargo test --features client`** — client tests pass
10. **`lsp_diagnostics`** on every changed file — zero errors
11. **`docker build -f docker/Dockerfile.aarch64 .`** — Dockerfile builds (if Docker available)
12. **README links are valid** — no broken references

---

## THINGS YOU MUST NOT DO

1. **DO NOT** break the existing x86_64 Vulkan desktop workflow. All existing functionality must continue to work when `--features vulkan` is passed.
2. **DO NOT** change the llama.cpp HTTP API protocol. The engine talks HTTP to llama-server — this doesn't change between CPU and GPU.
3. **DO NOT** add new crate dependencies unless absolutely necessary for the CPU/GPU feature split.
4. **DO NOT** modify any test files. Tests mock the backend and don't care about Vulkan vs CPU.
5. **DO NOT** delete `docker/Dockerfile` or `docker/docker-compose.yml`. Create new `*.aarch64*` variants alongside them.
6. **DO NOT** change the default Rust edition, MSRV, or profile settings.
7. **DO NOT** use `as any`, `@ts-ignore`, or similar suppression. If something doesn't compile, fix it properly.
8. **DO NOT** commit anything. Just make the changes.
9. **DO NOT** build or run the project — just ensure it compiles and tests pass.

---

## BUILD CROSS-COMPILE TOOLCHAIN SETUP (for reference, not a task)

On the x86_64 host, to cross-compile for aarch64:
```bash
# Install cross-compile toolchain
sudo apt install gcc-aarch64-linux-gnu g++-aarch64-linux-gnu

# Add Rust target
rustup target add aarch64-unknown-linux-gnu

# Configure cargo for cross-compilation (in .cargo/config.toml)
# [target.aarch64-unknown-linux-gnu]
# linker = "aarch64-linux-gnu-gcc"

# Build CPU-only binary for ARM64
cargo build --release --target aarch64-unknown-linux-gnu

# Build with client features
cargo build --release --target aarch64-unknown-linux-gnu --features client
```

---

## ON THE SBC (reference, not a task)

Once the binary is deployed to the Orange Pi Zero 3W:
```bash
# Install runtime dependencies
sudo apt update
sudo apt install -y libstdc++6 libgomp1 ca-certificates

# Verify binary runs
./whitt --version || ./whitt --help

# Build llama.cpp natively (CPU-only, with OpenMP for multi-core)
sudo apt install -y build-essential cmake git wget
git clone --depth 1 https://github.com/ggerganov/llama.cpp
cd llama.cpp
cmake -B build \
  -DLLAMA_CURL=ON \
  -DLLAMA_OPENMP=ON \
  -DLLAMA_NATIVE=OFF \
  -DCMAKE_BUILD_TYPE=Release
cmake --build build --config Release -j$(nproc)

# Download a small model (SmolLM3 ~2GB Q4_K_M, fits easily in 12GB)
wget -O models/SmolLM3-Q4_K_M.gguf \
  "https://huggingface.co/./resolve/main/SmolLM3-Q4_K_M.gguf"

# Start server (CPU-only, 8 threads, 8K context)
./build/bin/llama-server \
  -m models/SmolLM3-Q4_K_M.gguf \
  -c 8192 \
  -t 8 \
  -ngl 0 \
  --host 127.0.0.1 \
  --port 8080

# In another terminal: test the engine
./whitt chat --model SmolLM3
```

---

## SUMMARY OF ALL FILES TO CREATE OR MODIFY

| File | Action | Priority |
|------|--------|----------|
| `Cargo.toml` | MODIFY — make vulkan feature optional, CPU default | P0 (blocks everything) |
| `src/backend/llama_vulkan.rs` | MODIFY — rename struct, update docs, add cfg gates | P0 |
| `src/backend/mod.rs` | MODIFY — update re-exports for renamed struct | P0 |
| `src/config/provider.rs` | MODIFY — rename provider struct, add type alias | P1 |
| `.env.example` | MODIFY — change defaults to CPU | P1 |
| `config.yml` | MODIFY — reduce context, set gpu_layers=0, threads=8 | P1 |
| `configs/profiles/a733.yml` | CREATE — ARM-optimized config profile | P1 |
| `docker/Dockerfile.aarch64` | CREATE — CPU-only ARM64 Docker image | P2 |
| `docker/docker-compose.aarch64.yml` | CREATE — ARM64 compose without GPU | P2 |
| `docker/entrypoint.sh` | MODIFY — add ARM detection, skip Vulkan validate | P2 |
| `.github/workflows/ci.yml` | MODIFY — add ARM64 cross-compile job | P2 |
| `README.md` | MODIFY — add ARM64/SBC section, update prerequisites | P3 |
| `docker/Dockerfile` | MODIFY — add TARGETARCH arg for yq download | P3 |
| `.cargo/config.toml` | CREATE — cross-compile linker config for aarch64 | P2 |

## [0129] 06-30 18:35

(2026-06-30T18:35:51)
TASK: Convert deprecated type-check operators `?!` and `?//` to HARD ERRORS in `compiler/crates/lr-vm/src/vm.rs`.

WORKDIR: /home/jon/code/left-right

CONTEXT: Left-Right VM. `?!` (legacy isBoolean) and `?//` (legacy isList) must now HARD ERROR per user Batch 13. Canonical ops are `??` (isBoolean) and `?%` (isList).

CURRENT STATE (all these arms exist and return valid booleans — they must be changed to errors):

`?!` sites (grep shows these lines):
- Line 225: `"?!" => Ok(Value::boolean(mc, false))` (undefined postfix)
- Line 547: `"?!" => Value::boolean(mc, false)` (String postfix)
- Line 616: `"?!" => Value::boolean(mc, false)` (List postfix)
- Line 648: `"?!" => Value::boolean(mc, false)` (Number postfix)
- Line 684: `"?!" => Value::boolean(mc, true)` (Boolean postfix — returns TRUE)
- Line 1183: `(left, _, "?!") => { ... }` (diadic form)
- Line 1262: `"?!" => Value::boolean(mc, false)` (undefined postfix another site)

`?//` sites:
- Line 226: `"?//" => Ok(Value::boolean(mc, false))` (undefined postfix)
- Line 548: `"?//" => Value::boolean(mc, false)` (String postfix)
- Line 617: `"?//" => Value::boolean(mc, true)` (List postfix — returns TRUE)
- Line 649: `"?//" => Value::boolean(mc, false)` (Number postfix)
- Line 685: `"?//" => Value::boolean(mc, false)` (Boolean postfix)
- Line 1186: `(left, _, "?//") => { ... }` (diadic form)
- Line 1263: `"?//" => Value::boolean(mc, false)` (undefined postfix another site)

FIX: Replace each arm's body with a hard error. Pattern:

For unary postfix arms (returning `Ok(...)` or `Value::...`):
```
"?!" => return Err(VMError::Runtime("`?!` is deprecated. Use `??` for isBoolean.".to_string())),
```
```
"?//" => return Err(VMError::Runtime("`?//` is deprecated. Use `?%` for isList.".to_string())),
```

For the diadic form `(left, _, "?!") => { ... }` and `(left, _, "?//") => { ... }`:
Replace the entire arm body with the same error return.

IMPORTANT CONTEXT — match arm structure:
- Some arms are in `match` blocks that return `Result<Value, VMError>` (the `Ok(...)` ones use `return Err(...)` naturally)
- Some arms are in closures that return `Value` directly (no Result) — these CANNOT use `return Err(...)`. For those, you need to check the surrounding function signature. If the function returns `Result`, use `return Err(...)`. If it returns `Value` directly, you may need to find a different way (perhaps the outer caller handles errors).

READ each site's surrounding function signature BEFORE editing to determine if `return Err(...)` is valid there. The unary postfix operator dispatch at lines 208-228 returns `Result<Value, VMError>` (uses `Ok(...)`), so `return Err(...)` works there. The String/Number/Boolean postfix blocks at 526+, 613+, 675+ may return `Value` directly (no Result) — for those, check if there's an outer error handling layer.

If a site CANNOT return Err (because the function returns Value not Result), then:
- Return a Value::Error or similar error value if such exists
- OR leave that site working (backward compat) and only error in the Result-returning paths
- Document which sites couldn't be converted

ALTERNATIVE simpler approach: If converting all sites is too complex due to return type issues, just handle the MOST COMMON paths (unary postfix at lines 225-226, 547-548, 616-617, 648-649, 684-685) and leave diadic forms. Most users write `x ?!` not `x ?! y`.

MUST DO:
1. Read vm.rs around each site before editing
2. Convert as many `?!` and `?//` arms to errors as possible (prefer Result-returning paths)
3. Run `cargo test` from `compiler/` — update tests that expected old behavior
4. Add tests: `true ?!` errors, `[1] ?//` errors, `true ??` works, `[1] ?%` works
5. `cargo check -p lr-vm` — zero warnings

MUST NOT DO:
- Do NOT touch `??` or `?%` (canonical, keep working)
- Do NOT touch qa-history, matrix, source other than vm.rs
- Do NOT suppress warnings
- Do NOT delete tests

VERIFY:
- `cargo test` pass count
- `cargo check -p lr-vm` clean
- Show sample: `cargo run --release --bin lr -c 'true ?!'` errors with deprecation message

Report: sites converted, sites skipped (with reason), test count.
<!-- OMO_INTERNAL_INITIATOR -->

## [0130] 06-30 18:36

(2026-06-30T18:36:05)
TASK: Add 3 new String operator behaviors to `compiler/crates/lr-vm/src/vm.rs`.

WORKDIR: /home/jon/code/left-right

CONTEXT: Left-Right VM. Three String operations need implementation per user spec.

FIX 1: String × String comparison operators use LENGTH COMPARE.

Spec: `` `apple` < `banana` `` → true (5 < 6). Compare `ls.len()` vs `rs.len()`.

Add these 4 arms in the diadic dispatch (find the section near line 740 where Number `<` `>` `<=` `>=` are handled, add String cases nearby OR in a new String dispatch section):

```rust
(Value::String(ls), Value::String(rs), "<") => Value::boolean(mc, ls.len() < rs.len()),
(Value::String(ls), Value::String(rs), ">") => Value::boolean(mc, ls.len() > rs.len()),
(Value::String(ls), Value::String(rs), "<=") => Value::boolean(mc, ls.len() <= rs.len()),
(Value::String(ls), Value::String(rs), ">=") => Value::boolean(mc, ls.len() >= rs.len()),
```

Place them BEFORE any catch-all/fallback arm so they match first.

FIX 2: String `/` Number = CHUNK SPLIT.

Spec: `` `abcdef` / 2 `` → `` [`ab`, `cd`, `ef`] ``. Split into N-char chunks.

Add this arm in diadic dispatch:
```rust
(Value::String(s), Value::Number(n), "/") => {
    let chunk_size = *n as usize;
    if chunk_size == 0 {
        return Err(VMError::Runtime("String chunk size must be > 0".to_string()));
    }
    let chunks: Vec<Value> = s.as_str()
        .chars()
        .collect::<Vec<_>>()
        .chunks(chunk_size)
        .map(|c| Value::string(mc, c.iter().collect::<String>()))
        .collect();
    Value::list(mc, chunks)
}
```

Edge cases: n=0 → error. n > len → one chunk (whole string). Non-integer n → `as usize` truncates.

FIX 3: String `-` Number = COERCE Number to String, remove ALL occurrences.

Spec: `` `a2b2` - 2 `` → `` `ab` `` (all `2`s removed). `` `hello` - 2 `` → `` `hello` `` (no `2` substring, unchanged).

Add this arm in diadic dispatch:
```rust
(Value::String(s), Value::Number(n), "-") => {
    let n_str = if *n == n.trunc() {
        format!("{}", *n as i64)
    } else {
        format!("{}", n)
    };
    Value::string(mc, s.as_str().replace(&n_str, ""))
}
```

Note: The number formatting must strip trailing `.0` (e.g. `2.0` → `"2"` not `"2.0"`). Check how other Number→String coercion works in the file and mirror it. The code above uses `n.trunc()` check — adjust if the codebase has a different pattern.

MUST DO:
1. Read vm.rs diadic dispatch section to find correct insertion points
2. Read how existing Number→String coercion works (around line 1036, 608)
3. Add all 3 fixes (6 new arms total: 4 compare + 1 chunk + 1 coerce-remove)
4. Place new arms BEFORE fallback/catch-all arms
5. Run `cargo test` from `compiler/` — ALL pass
6. Add tests:
   - `` `apple` < `banana` `` → true
   - `` `ab` > `abc` `` → false (2 > 3 = false)
   - `` `abc` <= `abc` `` → true (equal length)
   - `` `abcdef` / 2 `` → `[ab, cd, ef]`
   - `` `abcdef` / 3 `` → `[abc, def]`
   - `` `abcdef` / 10 `` → `[abcdef]` (one chunk)
   - `` `a2b2` - 2 `` → `ab`
   - `` `hello` - 2 `` → `hello`
7. `cargo check -p lr-vm` — zero warnings

MUST NOT DO:
- Do NOT touch `?!` or `?//` (separate task handles those)
- Do NOT touch qa-history, matrix
- Do NOT change existing comparison behavior on Numbers
- Do NOT suppress warnings
- Do NOT delete tests

VERIFY:
- `cargo test` pass count
- `cargo check -p lr-vm` clean
- Show: `cargo run --release --bin lr -c '\`apple\` < \`banana\`'` → true
- Show: `cargo run --release --bin lr -c '\`abcdef\` / 2'` → `[ab, cd, ef]`

Report: arms added, line numbers, test count, sample outputs.
<!-- OMO_INTERNAL_INITIATOR -->

## [0131] 06-30 18:45

(2026-06-30T18:45:22)
ask me a series of 10 multiple choice questions in opencode iteratively making fixes and implementing things as I answer 10 question chunks.  make the questions are clear and direct and about what I want interface wise, and what capabilities I want to occure with those interfaces.  Make sure you never stop until I say everything is complete. continue iterating on the unit, integration, and e2e tests that don't actually run the llm as another workflow is using all system resources available for that so stick to just the local tests and fix everything you found before asking your batch of 10 questions and make sure you really iterate and verify you've impleemnted and documented everything in a well organized way before asking your next batch of questions.  don't stop until I answer in a question batch that you are done.

## [0132] 06-30 18:45

(2026-06-30T18:45:34)
[search-mode]
MAXIMIZE SEARCH EFFORT. Launch multiple background agents IN PARALLEL:
- explore agents (codebase patterns, file structures, ast-grep)
- librarian agents (remote repos, official docs, GitHub examples)
Plus direct tools: Grep, ripgrep (rg), ast-grep (sg)
NEVER stop at first result - be exhaustive.

---

<auto-slash-command>
# /ralph-loop Command

**Description**: (builtin) Start self-referential development loop until completion

**User Arguments**: ask me a series of 10 multiple choice questions in opencode iteratively making fixes and implementing things as I answer 10 question chunks.  make the questions are clear and direct and about what I want interface wise, and what capabilities I want to occure with those interfaces.  Make sure you never stop until I say everything is complete. continue iterating on the unit, integration, and e2e tests that don't actually run the llm as another workflow is using all system resources available for that so stick to just the local tests and fix everything you found before asking your batch of 10 questions and make sure you really iterate and verify you've impleemnted and documented everything in a well organized way before asking your next batch of questions.  don't stop until I answer in a question batch that you are done.

**Scope**: builtin

---

## Command Instructions

<command-instruction>
You are starting a Ralph Loop - a self-referential development loop that runs until task completion.

## How Ralph Loop Works

1. You will work on the task continuously
2. When you believe the task is FULLY complete, output: `<promise>{{COMPLETION_PROMISE}}</promise>`
3. If you don't output the promise, the loop will automatically inject another prompt to continue
4. Maximum iterations: Configurable (default 100)

## Rules

- Focus on completing the task fully, not partially
- Don't output the completion promise until the task is truly done
- Each iteration should make meaningful progress toward the goal
- If stuck, try different approaches
- Use todos to track your progress

## Exit Conditions

1. **Completion**: Output your completion promise tag when fully complete
2. **Max Iterations**: Loop stops automatically at limit
3. **Cancel**: User runs `/cancel-ralph` command

## Your Task

Parse the arguments below and begin working on the task. The format is:
`"task description" [--completion-promise=TEXT] [--max-iterations=N] [--strategy=reset|continue]`

Default completion promise is "DONE" and default max iterations is 100.
</command-instruction>

<user-task>
ask me a series of 10 multiple choice questions in opencode iteratively making fixes and implementing things as I answer 10 question chunks.  make the questions are clear and direct and about what I want interface wise, and what capabilities I want to occure with those interfaces.  Make sure you never stop until I say everything is complete. continue iterating on the unit, integration, and e2e tests that don't actually run the llm as another workflow is using all system resources available for that so stick to just the local tests and fix everything you found before asking your batch of 10 questions and make sure you really iterate and verify you've impleemnted and documented everything in a well organized way before asking your next batch of questions.  don't stop until I answer in a question batch that you are done.
</user-task>


---

## User Request

ask me a series of 10 multiple choice questions in opencode iteratively making fixes and implementing things as I answer 10 question chunks.  make the questions are clear and direct and about what I want interface wise, and what capabilities I want to occure with those interfaces.  Make sure you never stop until I say everything is complete. continue iterating on the unit, integration, and e2e tests that don't actually run the llm as another workflow is using all system resources available for that so stick to just the local tests and fix everything you found before asking your batch of 10 questions and make sure you really iterate and verify you've impleemnted and documented everything in a well organized way before asking your next batch of questions.  don't stop until I answer in a question batch that you are done.
</auto-slash-command>

## [0133] 06-30 18:45

<command-instruction>
You are starting a Ralph Loop - a self-referential development loop that runs until task completion.

## How Ralph Loop Works

1. You will work on the task continuously
2. When you believe the task is FULLY complete, output: `<promise>{{COMPLETION_PROMISE}}</promise>`
3. If you don't output the promise, the loop will automatically inject another prompt to continue
4. Maximum iterations: Configurable (default 100)

## Rules

- Focus on completing the task fully, not partially
- Don't output the completion promise until the task is truly done
- Each iteration should make meaningful progress toward the goal
- If stuck, try different approaches
- Use todos to track your progress

## Exit Conditions

1. **Completion**: Output your completion promise tag when fully complete
2. **Max Iterations**: Loop stops automatically at limit
3. **Cancel**: User runs `/cancel-ralph` command

## Your Task

Parse the arguments below and begin working on the task. The format is:
`"task description" [--completion-promise=TEXT] [--max-iterations=N] [--strategy=reset|continue]`

Default completion promise is "DONE" and default max iterations is 100.
</command-instruction>

<user-task>
ask me a series of 10 multiple choice questions in opencode iteratively making fixes and implementing things as I answer 10 question chunks.  make the questions are clear and direct and about what I want interface wise, and what capabilities I want to occure with those interfaces.  Make sure you never stop until I say everything is complete. continue iterating on the unit, integration, and e2e tests that don't actually run the llm as another workflow is using all system resources available for that so stick to just the local tests and fix everything you found before asking your batch of 10 questions and make sure you really iterate and verify you've impleemnted and documented everything in a well organized way before asking your next batch of questions.  don't stop until I answer in a question batch that you are done.
</user-task>

## [0134] 06-30 18:50

(2026-06-30T18:50:45)
read through both the output workflow and their actual live run results and make a detailed report in the same location as the workflows with exhaustive detail about the reality of where we are at and what kind of results we are producing then in the chat give me the path from this workspace to get to your report then give a summary in the chat.  don't stop until this is all completed and critically reviewed and fixed at least 3 times judged critically based on the reality of the agentic performance, if the output workflows from the meta workflow generator are actually better conceptually on what they complete and how reliably the output workflows actually complete the tasks by looking at the actual output results of the output workflows really actually live system running and executing on real files and read those files like a human would looking for issues as you iterate.

## [0135] 07-01 19:40

(2026-07-01T19:40:57)
What from this prompt remains incomplete no changes just exhaustively answer?

read through both the output workflow and their actual live run results and make a detailed report in the same location as the workflows with exhaustive detail about the reality of where we are at and what kind of results we are producing then in the chat give me the path from this workspace to get to your report then give a summary in the chat.  don't stop until this is all completed and critically reviewed and fixed at least 3 times judged critically based on the reality of the agentic performance, if the output workflows from the meta workflow generator are actually better conceptually on what they complete and how reliably the output workflows actually complete the tasks by looking at the actual output results of the output workflows really actually live system running and executing on real files and read those files like a human would looking for issues as you iterate.

## [0136] 07-01 20:41

(2026-07-01T20:41:05)
 continue and make it exhaustive then after its fully exhaustively documented with all the gaps try to iterate with as small as possible units or llm chains to iterate and fix all found gaps with as little as possible. don't do any integration testing but after you've fully iterated through and found and fixed every gap iteratively committing and pushing as you go I want you to ask me 10 multiptle choice opencode questions about what level and standards I want for integration testing and suggestions around options that involve time and focus on specific details in the questions and possible answers.  don't stop until all of this is completed and at the end of the 10 answers you need to continue and implement the integration testing fully and don't let yourself get stuck or stop for any reason.

## [0137] 07-01 20:41

(2026-07-01T20:41:20)
 continue and make it exhaustive then after its fully exhaustively documented with all the gaps try to iterate with as small as possible units or llm chains to iterate and fix all found gaps with as little as possible. don't do any integration testing but after you've fully iterated through and found and fixed every gap iteratively committing and pushing as you go I want you to ask me 10 multiptle choice opencode questions about what level and standards I want for integration testing and suggestions around options that involve time and focus on specific details in the questions and possible answers.  don't stop until all of this is completed and at the end of the 10 answers you need to continue and implement the integration testing fully and don't let yourself get stuck or stop for any reason.

## [0138] 07-01 20:43

(2026-07-01T20:43:12)
give me a context dump prompt to pass to a new workspace so nothing is lost when I go to a new empty folder

## [0139] 07-01 20:55

(2026-07-01T20:55:28)
[analyze-mode]
ANALYSIS MODE. Gather context before diving deep:
CONTEXT GATHERING (parallel):
- 1-2 explore agents (codebase patterns, implementations)
- 1-2 librarian agents (if external library involved)
- Direct tools: Grep, AST-grep, LSP for targeted searches

IF COMPLEX - DO NOT STRUGGLE ALONE. Consult specialists:
- **Oracle**: Conventional problems (architecture, debugging, complex logic)
- **Artistry**: Non-conventional problems (different approach needed)

SYNTHESIZE findings before proceeding.
---
MANDATORY delegate_task params: ALWAYS include load_skills=[] and run_in_background when calling delegate_task.
Example: delegate_task(subagent_type="explore", prompt="...", run_in_background=true, load_skills=[])

---

# PoE2 on CachyOS — Session Handoff

## GOAL
Get Path of Exile 2 running stably on CachyOS. Steam installed and launching. PoE2 launches but crashes after ~2 min. Need to stabilize.

## SYSTEM SPECS
- **OS:** CachyOS (Arch-based), kernel 6.19.9-1-cachyos (Clang/LTO, PREEMPT_DYNAMIC)
- **Default shell:** `/bin/fish` (CRITICAL — Steam must be launched via `bash -c 'steam &'`, NOT plain `steam &`. Fish breaks Steam's bash scripts due to unbound LD_LIBRARY_PATH variable)
- **GPU:** AMD Radeon RX 580 Series / RX 480 (Polaris10, same die), 8GB VRAM, RADV driver
- **Monitor:** DELL U2415, 1920x1200
- **Driver stack:** Mesa 26.0.3, vulkan-radeon, lib32-vulkan-radeon, lib32-gcc-libs 16.1.1+r12+g301eb08fa2c5-2
- **CPU governor:** schedutil (could switch to BORE kernel for gaming)
- **Drives:** 140GB free on /home. External ext4 at `/run/media/jon/data` (currently `nosuid,nodev` from udisks2 — **needs exec**)

## INSTALLED PACKAGES
- `cachyos-gaming-meta` (1.0.0-7)
- `cachyos-gaming-applications` (1.0.0-2) — brings Steam, Lutris, Heroic, Gamescope, MangoHud, GOverlay
- `proton-cachyos` (latest, bleeding-edge branch, 11.0-100)
- `steam` (latest)
- `lib32-gcc-libs` (upgraded to 16.1.1+r12 — fixed GLIBCXX_3.4.35 missing)
- Plus all optionals: gamescope, mangohud, goverlay, umu-launcher, wine-cachyos-opt, protontricks, winetricks

## CURRENT STEAM STATE
- ~/.steam and ~/.local/share/Steam freshly bootstrapped (full nuke + reset completed)
- Steam launches successfully via `bash -c 'steam &'`
- GPU correctly detected: `AMD Radeon RX 580 Series (RADV POLARIS10)` with 8GB VRAM
- Proton-CachyOS 11.0-100 is default compat tool (auto-selected, no manual VDF edit needed)
- `ntsync` running

## POE2 (AppID 2694490) STATE
- **Install path:** `/run/media/jon/data/SteamLibrary/steamapps/common/Path of Exile 2/` (external ext4 drive)
- **Compatdata:** `/run/media/jon/data/SteamLibrary/steamapps/compatdata/2694490/`
- **Config file:** `compatdata/2694490/pfx/drive_c/users/steamuser/Documents/My Games/Path of Exile 2/poe2_production_Config.ini`
- **Config key values:**
  - `renderer_type=Vulkan` (correct for AMD)
  - `framerate_limit=240` but `framerate_limit_enabled=false` (BAD — uncapped GPU runs wild)
  - `upscale=FSR`, `upscale_quality=Balanced`
  - `texture_quality=TextureQualityLow`
  - `engine_multithreading_mode=enabled`
  - `global_illumination_detail=0`
  - `shadow_type=High`
  - `dynamic_resolution_fps=60`, `use_dynamic_resolution=true`

## CRASH SYMPTOMS (last attempt)
- Game launches, runs ~2 min, dies
- Steam-side errors (NOT game-side):
  - `CSteamEngine::BMainLoop appears to have stalled > 15 seconds without event signalled`
  - `CFileWriter::Flush - Waited 6000 iterations`
  - `gameoverlayui` crash dumps (disconnected socket reads)
  - `Game Recording - game stopped [gameid=2694490]`
- Normal noise (NOT bugs): `gameoverlayrenderer.so wrong ELF class: ELFCLASS32/64: ignored` — expected, 32-bit overlay can't preload into 64-bit game
- No PoE2-specific crash dump found in compatdata

## PENDING FIXES TO TRY (in order)
1. **Remount drive with exec:** `sudo mount -o remount,exec /run/media/jon/data` — verify with `mount | grep data`
2. **Permanent fstab fix:** `sudo blkid /dev/sda1` → add `UUID=<uuid> /run/media/jon/data ext4 rw,relatime,exec 0 2`
3. **Cap framerate in config:** edit poe2_production_Config.ini → `framerate_limit_enabled=true`, `framerate_limit=60`
4. **Disable Steam Overlay for PoE2** (overlay crashed): Steam GUI → Right-click PoE2 → Properties → uncheck "Enable Steam Overlay while in-game"
5. **Clear shader cache:** `rm -rf "/run/media/jon/data/SteamLibrary/steamapps/shadercache/2694490"` AND `rm -rf ~/.local/share/Steam/shader_cache_temp_dir_d3d12_64`
6. **Relaunch via:** `bash -c 'steam &'`, then launch PoE2 from Steam GUI
7. **If still crashes:** switch compat tool to **Proton Experimental** (Steam GUI → Properties → Compatibility → Force "Proton Experimental")

## RECOMMENDED LAUNCH OPTIONS (add in Steam GUI Properties)
```
game-performance PROTON_ENABLE_WAYLAND=1 DXVK_ENABLE_NVAPI=1 PROTON_USE_NTSYNC=1 MANGOHUD=1 %command%
```
- `game-performance` — CachyOS wrapper sets CPU to performance governor
- `PROTON_ENABLE_WAYLAND=1` — Wayland via XWayland
- `PROTON_USE_NTSYNC=1` — better Wine threading
- `MANGOHUD=1` — FPS overlay (optional)

## KEY RESEARCH FINDINGS
- **ProtonDB:** PoE2 rated Platinum (574+ reports)
- **Anti-cheat:** Server-side only, NO kernel driver — Linux works clean
- **GGG fixes Proton compat within 24-48h of patches**
- **Proton Experimental got PoE2-specific fix June 11, 2026** (chat character crash)
- **Arch dropped `lib32-gtk2` + `steam-native-runtime` from multilib Oct 2025** — plain `steam` package uses bundled runtime
- **Steam-runtime fix pattern from Valve dev (kisak-valve):** delete `ubuntu12_32/steam-runtime` to force regenerate
- **CachyOS Gaming wiki:** https://wiki.cachyos.org/configuration/gaming/
- **PoE2 sysreq:** Min GTX 960 3GB / Rec RTX 2060 or RX 5600XT, 16GB RAM
- **VRAM usage:** 5-9GB at 4K Ultra, ~5-6GB at 1440p, ~4-5GB at 1080p
- **BORE kernel** (`linux-cachyos-bore`) is best for gaming; currently on `linux-cachyos` (EEVDF)
- **NVIDIA black screen fix** (N/A for AMD but documented): edit `poe2_production_Config.ini` → `renderer_type=DirectX12`

## THINGS THAT DIDN'T WORK
- Plain `steam &` from fish → crashes with `LD_LIBRARY_PATH: unbound variable`
- `steam --reset` from fish → same fish bug
- `lib32-gtk2` not in repos anymore — irrelevant, Steam bundles its own GTK2 in steam-runtime

## WHAT'S WORKING
- Steam install, bootstrap, launch
- Proton-CachyOS registration
- PoE2 download/install on external drive
- PoE2 initial launch (prefix creation, config generation, 2 min runtime)

## WHAT'S NOT WORKING
- PoE2 stability beyond ~2 minutes
- Steam Engine client UI hangs during gameplay
- Steam Overlay crashing in PoE2

## HOW TO VERIFY FIX WORKED
After applying fixes, launch PoE2 and check:
- Game stays running >5 min without crash
- Steam doesn't report `BMainLoop stalled`
- No crash dumps in `/tmp/dumps/`
- Frame rate visible in MangoHud stays at ~60fps (locked)

## [0140] 07-01 21:01

(2026-07-01T21:01:26)
I want to have my xbox controler work on this machine.  how can I do that? no changes just answer

## [0141] 07-01 21:03

(2026-07-01T21:03:26)
[search-mode]
MAXIMIZE SEARCH EFFORT. Launch multiple background agents IN PARALLEL:
- explore agents (codebase patterns, file structures, ast-grep)
- librarian agents (remote repos, official docs, GitHub examples)
Plus direct tools: Grep, ripgrep (rg), ast-grep (sg)
NEVER stop at first result - be exhaustive.

---

will this work? web search then implement everything needed so it will work on my current os

## [0142] 07-01 21:03

(2026-07-01T21:03:38)
with steam path of exiles 2

## [0143] 07-01 21:03

(2026-07-01T21:03:47)
[CONTEXT] User on CachyOS (Arch-based), kernel 6.19.9, has Xbox Wireless Adapter for Windows USB dongle (USB ID 045e:02e6). Wants Xbox controller working wirelessly.

[GOAL] Determine the current (2025-2026) recommended driver stack for this specific dongle on modern Linux kernels. Need authoritative answer on which driver works.

[DOWNSTREAM] Will use findings to install/configure on user's machine. Need exact package names, AUR names, commands, kernel module names, firmware sources.

[REQUEST] Research these specific points:
1. Is USB ID 045e:02e6 supported by mainline kernel xpad driver in 2025/2026? Check kernel.org, xpad source.
2. xone driver (medusalix/xone on GitHub) — current status, does it support 045e:02e6, install procedure on Arch, DKMS package name on AUR.
3. xpadneo (atar-axis/xpadneo) — does it work for the dongle (not just bluetooth)?
4. Any firmware requirements — xone needs firmware blobs, where from, legal status.
5. Known issues / kernel 6.x compatibility for xone and xpad.
6. Arch Wiki "Gamepad" / "Xbox 360 controller" / "Xbox One controller" pages — current recommendations.
7. Conflicts between xpad and xone — blacklist requirements.

Return: authoritative recommendation with exact commands. Cite sources (URLs). Skip tutorials — production guidance only.
<!-- OMO_INTERNAL_INITIATOR -->

## [0144] 07-01 21:25

(2026-07-01T21:25:49)
continue

## [0145] 07-01 23:02

(2026-07-01T23:02:42)
What from this prompt remains incomplete no changes just exhaustively answer?

read through both the output workflow and their actual live run results and make a detailed report in the same location as the workflows with exhaustive detail about the reality of where we are at and what kind of results we are producing then in the chat give me the path from this workspace to get to your report then give a summary in the chat.  don't stop until this is all completed and critically reviewed and fixed at least 3 times judged critically based on the reality of the agentic performance, if the output workflows from the meta workflow generator are actually better conceptually on what they complete and how reliably the output workflows actually complete the tasks by looking at the actual output results of the output workflows really actually live system running and executing on real files and read those files like a human would looking for issues as you iterate.

## [0146] 07-01 23:08

(2026-07-01T23:08:02)
<auto-slash-command>
# /ralph-loop Command

**Description**: (builtin) Start self-referential development loop until completion

**User Arguments**: run live system testing until the car is driven and you know in detail how it drives and fix what you find along the way.  don't stop until we surpass opencode fully with real agentic execution

**Scope**: builtin

---

## Command Instructions

<command-instruction>
You are starting a Ralph Loop - a self-referential development loop that runs until task completion.

## How Ralph Loop Works

1. You will work on the task continuously
2. When you believe the task is FULLY complete, output: `<promise>{{COMPLETION_PROMISE}}</promise>`
3. If you don't output the promise, the loop will automatically inject another prompt to continue
4. Maximum iterations: Configurable (default 100)

## Rules

- Focus on completing the task fully, not partially
- Don't output the completion promise until the task is truly done
- Each iteration should make meaningful progress toward the goal
- If stuck, try different approaches
- Use todos to track your progress

## Exit Conditions

1. **Completion**: Output your completion promise tag when fully complete
2. **Max Iterations**: Loop stops automatically at limit
3. **Cancel**: User runs `/cancel-ralph` command

## Your Task

Parse the arguments below and begin working on the task. The format is:
`"task description" [--completion-promise=TEXT] [--max-iterations=N] [--strategy=reset|continue]`

Default completion promise is "DONE" and default max iterations is 100.
</command-instruction>

<user-task>
run live system testing until the car is driven and you know in detail how it drives and fix what you find along the way.  don't stop until we surpass opencode fully with real agentic execution
</user-task>


---

## User Request

run live system testing until the car is driven and you know in detail how it drives and fix what you find along the way.  don't stop until we surpass opencode fully with real agentic execution
</auto-slash-command>

## [0147] 07-01 23:08

<command-instruction>
You are starting a Ralph Loop - a self-referential development loop that runs until task completion.

## How Ralph Loop Works

1. You will work on the task continuously
2. When you believe the task is FULLY complete, output: `<promise>{{COMPLETION_PROMISE}}</promise>`
3. If you don't output the promise, the loop will automatically inject another prompt to continue
4. Maximum iterations: Configurable (default 100)

## Rules

- Focus on completing the task fully, not partially
- Don't output the completion promise until the task is truly done
- Each iteration should make meaningful progress toward the goal
- If stuck, try different approaches
- Use todos to track your progress

## Exit Conditions

1. **Completion**: Output your completion promise tag when fully complete
2. **Max Iterations**: Loop stops automatically at limit
3. **Cancel**: User runs `/cancel-ralph` command

## Your Task

Parse the arguments below and begin working on the task. The format is:
`"task description" [--completion-promise=TEXT] [--max-iterations=N] [--strategy=reset|continue]`

Default completion promise is "DONE" and default max iterations is 100.
</command-instruction>

<user-task>
run live system testing until the car is driven and you know in detail how it drives and fix what you find along the way.  don't stop until we surpass opencode fully with real agentic execution
</user-task>

## [0148] 07-01 23:08

(2026-07-01T23:08:40)
did you get the controller stuff set up?

## [0149] 07-01 23:12

(2026-07-01T23:12:30)
i think i'm connected now

## [0150] 07-01 23:14

(2026-07-01T23:14:06)
did you fix it?

