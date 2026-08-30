# chunk-19 summary (prompts 0901-0950, 08-24 → 08-27)

## Self-healing experiment scoping
- New `self-healing` subfolder experiment; scoping to models recently in use (harness experiment + REA+); SPOOFED workflow YAML (no local model loads); primary paper arxiv 2605.06737 "A Self-Heal[ing...]" + related (0906-0910, 0912)

## Findings + bug hunts
- "Look at the experiments folders, tell me overall findings: what is and is not working in these workflow[s]" (0911)
- REA+ capped at 66-67/73 (90%) after full budget; 7 stubborn cases: char-precision, unit-con... (0916)
- Framework-level bug inventory in whitt-execution-engine Rust repo (worktree path) ×4 fan-out contexts (0918-0920, 0930-0931); "I fixed 15 engine bugs in worktree..." (0932)
- Core meta principles being investigated (0921); deep web research on those principles + others' progress (0922)
- Design next experiment folder + deep web research + document at top level of new folder (0923)

## Full-agentic-system genesis (current experiment)
- "My REAL GOAL: build subworkflows usable like ATOMS to create a working version of the meta-workflow [generator]" (0927)
- Make new experiment subfolder full-agentic-system + subfolders per experiment [needed] (0928)
- "If these experiment workflows were stitched together, what would the top-level control flow look like?" (0935)
- Per-prompt expected runtime average + variance range (0936)
- More web research for additional experiments/subworkflows to fully leverage all previous insights (0937)
- "Design 100 new EXTRA-HARD test cases in areas the current reasoning workflow is struggling" (0938)
- Copyable context dump w/ instructions to re-spoof-run the meta experiment + all sub-experiments (0939)
- 3 representative test cases (0925); avg test input length (0926)

## Control/steering meta
- "Anything remaining?" (0941); "ask me multiple choice questions in opencode to resolve the remaining issues" (0942)
- "Pause and checkpoint and stop ALL processes/workflows related to this session" ×2 (0943-0944)
- Expected avg runtime per subworkflow + meta workflow per case (0945)
- "Do NOT resume yet. Tell me current status, results, run times per case + all cases + by workflow" (0946)
- "YOU CAN NOW LOAD AND UNLOAD LLM MODELS AND MAKE CALLS. Run through all experiments sequentially" (0947)
- status? ×4; "status? check for hung background stuff" (0954); language status + how the language works (0951-0952)
- "Continue, limit has been reset" (0955)
- "Just tell me status + remaining todos THEN STOP. DO NOT CONTINUE EXECUTING, JUST READ AND ANSWER" (0957)
- "Clear the current todos and processes forcing you to keep going; use the compact..." (0958)
- "Make a context dump file in this experiment folder, then give me a prompt to start this chat fresh using the full exhau[stive dump]" (0959)
