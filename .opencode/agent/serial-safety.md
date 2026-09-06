---
description: Primary crash-safe agent for this repository. Use for all work in nimble-harbor.
mode: primary
model: openai/gpt-5.6-terra
permission:
  task: deny
---

Operate serially. Make one direct tool call per response. Never call parallel
tool wrappers, background agents, delegation, or subagents.

Read known files directly. Search only explicit repository directories with a
specific file glob. Never recursively scan $HOME, ~/.cache, ~/.local/share,
~/.config, node_modules, SQLite databases, or generated outputs. Never use a
pipeline to pretend an unbounded producer is bounded.

Before local LLM or Docker work, check available RAM and server health. Require
at least 6 GiB available RAM, exactly one whitt process, and no OOM, DeviceLost,
or Vulkan errors. Stop immediately on a safety failure. Run one live case at a
time; inspect its result before starting another.

Treat tool interruption as unsafe state. Inspect process status before retrying.
Do not resume experiments until requested safeguards validate cleanly.

These limits govern OpenCode orchestration only. Do not disable whitt execution
engine tools, YAML workflow actions, or their scripts. Keep scripts available;
make them resource-bounded by using known inputs, bounded reads, sequential
execution, and the preflight gates above.

Use OpenAI only for OpenCode agent work. This does not alter whitt workflow
providers: their current POC provider remains `llama_cpp_with_vulkan`.

Every new executable workflow YAML needs a strict
`workflow_execution_strategy.resource_admission` contract: `block` policy,
positive available RAM/VRAM/swap minima, positive KV/compute/host/runtime
estimates, and `telemetry.write_profile: true`. Estimate before writing from
model weights, load parameters, context, hardware, and expected duration. Read
the run-local `resource-admission-profile.json` after every live run; manually
calibrate source YAML estimates from measured minima and duration. Runtime may
write telemetry only; it must never mutate source YAML.

Every resource-admission model needs absolute `models.<model>.source_path`,
whose basename equals `name`. Verify actual dereferenced GGUF size with `stat`
before estimating; engine telemetry must record that nonzero stat value. Never
invent model weights, use a container-only path, or treat missing model source
metadata as an admitted workflow.
