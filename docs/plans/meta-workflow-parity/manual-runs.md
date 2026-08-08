# Meta-Workflow Manual Run Prompts

**Purpose:** Test prompts for the meta-workflow generator (SW1-SW5 cascade). Written in natural language, the way you'd actually paste them into opencode.

---

## Prompt A — Outline → YAML Project Plan

```
transform this plain-text outline into a YAML project plan. structure: modules → tasks → deliverables, with est_hours and depends_on fields per task. fill in reasonable estimates if the outline doesn't say. don't invent tasks that aren't implied by the outline.

outline:
  Phase 1: foundation
  - set up CI pipeline
  - write initial data model
  Phase 2: build
  - implement CRUD endpoints (depends on data model)
  - wire auth middleware
  - add request validation (depends on endpoints)
  Phase 3: ship
  - load test
  - write docs (depends on endpoints)
  - deploy to staging
```

---

## Prompt B — Meeting Transcript → Action Items (YAML)

```
extract every action item from the meeting transcript below. output as a YAML list — each item has assignee, action, due_hint (specific date if mentioned, else "next sprint" / "this week" / "asap"), and blockers (empty list if none). don't paraphrase the action into vagueness — keep it concrete. skip items that are pure information sharing with no actual ask.

transcript:
  Priya: ok so the migration is scheduled for tuesday the 12th. Marcus can you make sure the runbook is reviewed by friday?
  Marcus: yeah I'll get it reviewed by EOD friday. who's actually executing on tuesday?
  Priya: David said he'd drive it but he's on call that week so we should get him backup. Jon can you be secondary?
  Jon: yeah I can be secondary. I need the rollback steps called out separately though, not buried in step 9.
  Marcus: noted, I'll pull them up to their own section. also we still need someone to write the customer comms — that's been open for two weeks.
  Priya: I'll take that, draft by monday. anything else?
  Jon: the dashboard alerting is still flapping, I'll look at it tomorrow morning.
```

---

## Prompt C — Feature Description → Gherkin Test Matrix

```
turn this feature description into gherkin scenarios. one Feature block, multiple Scenario blocks, grouped via tags (@happy, @edge, @failure, @security, @rate-limit). cover the happy path, edge cases, failure modes, and abuse vectors. every Then must be checkable — no "works correctly" hand-waves.

feature:
  Password reset via email link.
  - User clicks "forgot password", enters email.
  - If email exists in system, send reset link to that address. Link is single-use, expires after 1 hour.
  - If email doesn't exist, show the same success page anyway (don't leak which emails are registered).
  - User clicks link, lands on reset form. Form requires new password twice, must differ from old, must meet complexity rules (8+ chars, 1 number, 1 symbol).
  - On submit: invalidate link, update password, send confirmation email, log the IP/user-agent of the reset.
  - Rate limit: max 5 reset requests per email per hour.
```

<!-- NEXT STEPS (to be added after prompts are reviewed):
     - Step-by-step copy-paste blocks for running each prompt through the SW1-SW5 cascade
     - Conditional flow (how to retry / branch based on quality scores)
     - Inspection commands per step
-->

---

## SIDE NOTES (for step-generation phase — do not lose these requirements)

### 1. Tool calls must be simulated in the copy-paste prompts

When the meta-workflow generator runs end-to-end through the engine, the LLM has access to actual tools (`file_read`, `file_write`, `shell_exec`, `grep`, etc.) and the framework wires the outputs back into subsequent steps automatically.

When I run this **manually** by pasting prompts into LM Studio, there are no tools. The model just returns text.

**Therefore:** every copy-paste prompt I generate for the manual run MUST simulate the framework's tool calls by inlining the content the tool would have returned. Concretely:

- Where the engine would fire `file_read("src/foo.rs")` and inject the file contents into the next prompt via `{{bookmarks.shell_output}}`, my copy-paste prompt must literally contain:
  ```
  <file path="src/foo.rs">
  ...contents of src/foo.rs here, pre-read by me before pasting...
  </file>
  ```
- Where the engine would run a `shell_exec` hook (e.g., `cat ./outputs/sw1/tasks.md`) and feed the stdout into the next step, my copy-paste prompt must contain the actual stdout text inside a `<shell_output command="...">` block.
- Where the engine would call `grep` / `glob` and pass results forward, the prompt must include the pre-computed results.

The model's response at each step will be the workflow's intermediate artifact (tasks list, categorization, substructures, etc.). I will then copy that response into the next step's prompt as the "injected context" before pasting step N+1.

This means: **before I can paste step N+1, I must run step N, copy its output, paste it into the prepared template for step N+1, and only then send.** That is the manual loop.

### 2. Unique prompt per step must be highlighted

Each step in the SW1-SW5 cascade has its own prompt template. The template has two parts:

1. **Workflow scaffolding** — boilerplate instructions shared across all prompts in that SW (e.g., "You are SW1 task deconstruction. Decompose the prompt into atomic tasks. Use 1-3pts story scale. Output format: ...")
2. **Unique step prompt** — the specific instruction for THIS step (e.g., "Step 03 — Evaluate the task list you just produced. Score each criterion. Output VERDICT: PASS or FAIL.")

When I generate the copy-paste list, every block must visually distinguish these two parts so I can quickly eyeball what is being asked at that step without re-reading the scaffolding every time.

Suggested format for each step block:

```
[Step NN — <SW stage>: <one-line description>]
============================================================

--- WORKFLOW SCAFFOLDING (boilerplate, same for every prompt in this SW) ---
<the boilerplate prompt text>

--- UNIQUE STEP PROMPT (this is what changes per step) ---   <-- HIGHLIGHTED
> **<step_name>:**
>
> <the unique instruction for THIS step, called out in bold/quoted so it's scannable>

--- INJECTED CONTEXT (paste previous step's output here before sending) ---
<prompt_03_input>
...I will paste step NN-1's model response here before sending to LM Studio...
</prompt_03_input>
```

The `UNIQUE STEP PROMPT` block must always be visually distinct — bold + indented + surrounded by `---` rules — so when I'm working through the cascade manually I can:

- Scan the file vertically to find the next step
- Read only the unique part if I already know the scaffolding
- Skip to `INJECTED CONTEXT` to paste the prior output

Do NOT bury the unique instruction inside a wall of boilerplate. Do NOT omit the scaffolding either (model needs it). The structure above is what makes the file usable as a manual run script.

