#!/usr/bin/env python3
"""Deterministic workflow assembler.

Replaces SW5 model-based assembly. Reads SW4 substructures (markdown
with yaml code blocks), extracts each step definition, concatenates
them under a `steps:` key with skeleton header.

Eliminates the indentation cascade bug where the model flattens all
step fields (model_overrides, when, save_to) into the prompt body.

Usage:
    python3 scripts/meta-v6/build-workflow.py <structs.md> <output.yml>
"""
from __future__ import annotations

import re
import sys
from pathlib import Path


SKELETON = '''workflow_id: "{workflow_id}"
name: "{name}"
description: "Auto-assembled workflow from SW4 substructures."
version: "1.0.0"
schema_version: "2.0.0"

providers:
  llama_cpp_with_vulkan:
    config:
      host: localhost
      port: 8080

models:
  qwen35:
    name: "Qwen3-5-9B-Q4_K_M"
    host:
      type: llama_cpp_with_vulkan

workflow_execution_strategy:
  memory:
    model_lifecycle:
      unload_unused: false

agentic_workflow:
  when:
    after_workflow:
      - log:
          to_file_path: "./outputs/workflow.log"
          event_fields: [workflow_id, total_steps, succeeded, failed]
          level: info

  steps:
'''


def extract_step_blocks(md_text: str) -> list[str]:
    """Extract ```yaml ... ``` blocks from SW4 markdown.

    Each block should begin with `step_<id>_<name>:` at column 0.
    Returns list of raw YAML strings (one per step).

    SW4 sometimes emits ```yaml without matching ``` close fence,
    causing regex to capture trailing markdown (### T1.1, Intent:, etc.).
    Trim each block at first non-indented non-empty line (markdown metadata).
    """
    blocks = re.findall(r'```yaml\n(.*?)```', md_text, re.DOTALL)
    step_blocks: list[str] = []
    for block in blocks:
        trimmed_lines = []
        in_step = False
        for line in block.splitlines():
            stripped = line.strip()
            if not in_step:
                if stripped.startswith('step_'):
                    in_step = True
                    trimmed_lines.append(line)
                continue
            if stripped and not line.startswith(' ') and not line.startswith('\t'):
                break
            trimmed_lines.append(line)
        if trimmed_lines:
            step_blocks.append('\n'.join(trimmed_lines).strip())
    return step_blocks


def normalize_indent(block: str, target_indent: str = '    ') -> str:
    """Re-indent a step block to live under `steps:` (4-space indent).

    Input block has step_<id>: at col 0, fields at col 2.
    Output has step_<id>: at col 4, fields at col 6.
    """
    lines = block.splitlines()
    out = []
    for line in lines:
        if not line.strip():
            out.append('')
            continue
        out.append(target_indent + line)
    return '\n'.join(out)


def rewrite_sw4_paths(block: str, meta_run_id: str) -> str:
    """Rewrite SW4-internal paths to META-run-id paths.

    SW4 emits shell cat paths pointing to its own SW4 run dir like:
        cat ./docs/benchmarks/outputs/meta-workflow/meta-<META_ID>-sw4-<TS>/input/prompt.txt
    These paths exist during SW4 execution but break during final META workflow execution.
    Replace any `meta-<META_ID>-swN-<TS>` substring with just <META_ID>.
    """
    pattern = re.compile(r'meta-' + re.escape(meta_run_id) + r'-sw\d+-[\d-]+')
    return pattern.sub(meta_run_id, block)


def strip_save_to_templates(block: str) -> str:
    """Rewrite `save_to` lists that reference `{{step.X.output}}`.

    Engine's resolve_context_templates only handles the CURRENT step's
    `output` field. Cross-step refs like `{{step.OTHER.output}}` fall
    through unresolved, producing literal-named files.

    SW4 emits `save_to: [{{step.X.output}}, ./path]` intending "save
    current step output to ./path and also store as bookmark X". The
    first arg is template syntax the engine can't expand. Drop it so
    the save_to becomes a plain FilePath action.
    """
    pattern = re.compile(
        r'(\s*-\s*save_to:\s*\n)'
        r'(\s*-\s*"?\{\{step\.[^}]+\}\}"?\s*\n)'
        r'(\s*-\s*"?[^"\n]+"?\s*\n)',
        re.MULTILINE,
    )

    def replace(m: re.Match) -> str:
        return m.group(1) + m.group(3)

    return pattern.sub(replace, block)


def dedup_save_to_paths(blocks: list[str]) -> list[str]:
    """Rewrite save_to paths that are shared across multiple steps to unique per-step paths.

    SW4 sometimes emits the same file path (e.g., `./docs/reports/initial-thoughts.md`) for
    multiple steps, causing data loss (later writes overwrite earlier). Detect duplicate
    save_to targets and rewrite each occurrence to `./outputs/<step_id>.txt`.

    Also fixes bare-name entries: SW4 emits `save_to: [step_t1_output, ./outputs/...]` where
    the bare name is intended as a variable bookmark but the engine treats it as a file path
    (writes to CWD root). Prefix bare names with `$` so engine treats them as variables.
    """
    path_re = re.compile(r'^(\s*-\s*)([A-Za-z0-9_]+)(\s*\n\s*-\s*)"?(\.\/\S+?)"?(\s*\n)', re.MULTILINE)

    # Pass 1: collect all save_to paths and find duplicates
    path_counts: dict[str, int] = {}
    for block in blocks:
        for m in path_re.finditer(block):
            save_to_path = m.group(4)
            path_counts[save_to_path] = path_counts.get(save_to_path, 0) + 1

    # Pass 2: rewrite paths that appear more than once to per-step unique
    out_blocks = []
    for block in blocks:
        m_step = re.match(r'^(\S+):\s*\n', block)
        if not m_step:
            out_blocks.append(block)
            continue
        step_id = m_step.group(1)

        def replace_path(m: re.Match) -> str:
            save_to_path = m.group(4)
            bare_name = m.group(2)
            # Always prefix bare name with $ so engine treats as variable, not file path.
            # Without this, bare names like "step_t1_output" get written as files to CWD root.
            fixed_bare = f"${bare_name}" if not bare_name.startswith('$') else bare_name
            # Only rewrite file path if shared across multiple steps
            if path_counts.get(save_to_path, 0) > 1:
                new_path = f"./outputs/{step_id}.txt"
                return m.group(1) + fixed_bare + m.group(3) + new_path + m.group(5)
            return m.group(1) + fixed_bare + m.group(3) + save_to_path + m.group(5)

        out_blocks.append(path_re.sub(replace_path, block))
    return out_blocks


def derive_workflow_id(structs_text: str) -> tuple[str, str]:
    """Best-effort workflow_id + name from SW4 metadata."""
    m = re.search(r'\*\*Source:\*\*\s*(.+)', structs_text)
    source = m.group(1).strip() if m else 'Agentic Categorization'
    m2 = re.search(r'\*\*Total substructures:\*\*\s*(\d+)', structs_text)
    total = m2.group(1) if m2 else '0'
    # Truncate source for workflow_id (avoid enormous IDs from prompt content)
    source_for_id = source[:80]
    wid_raw = re.sub(r'[^a-z0-9]+', '_', source_for_id.lower()).strip('_')
    wid = f'generated_{wid_raw}_{total}steps'[:120]  # hard cap
    # Sanitize name (escape double quotes, strip newlines)
    name_safe = source.replace('"', "'").replace('\n', ' ')[:200]
    return wid, name_safe


def make_bootstrap_step(prompt_path: str, run_dir: str) -> str:
    """Step 0: inject original prompt.txt into bookmarks.shell_output."""
    return f'''    step_00_bootstrap:
      generative_entity: "${{models.qwen35}}"
      prompt: "Respond with the single word OK."
      model_overrides:
        max_tokens: 4
        temperature: 0.0
      when:
        before_step_starts:
          - shell:
              command: "cat {prompt_path}"
              args: []
              working_dir: "/home/jon/code/whitt-execution-engine"
              fail_on_error: true
        after_step_succeeds:
          - log:
              to_file_path: "{run_dir}/logs/step_00_bootstrap.log"
              event_fields: [step_name, duration_ms, total_tokens]
              level: info

'''


def inject_context_ref(step_block: str) -> str:
    """Prepend {{bookmarks.shell_output.stdout}} to first step's prompt body."""
    context_prefix = (
        "Context (user prompt):\n"
        "{{bookmarks.shell_output.stdout}}\n\n"
        "Task:\n"
    )
    # Lookahead captures indent without consuming it (preserves body indent)
    pattern = re.compile(r'(prompt:\s*\|\s*\n)(?=(\s+))', re.MULTILINE)
    m = pattern.search(step_block)
    if not m:
        return step_block
    indent = m.group(2)
    insertion = '\n'.join(indent + line if line else '' for line in context_prefix.splitlines()) + '\n'
    # Insert at m.end() (which is start of body indent, lookahead didn't consume)
    return step_block[:m.end()] + insertion + step_block[m.end():]


def make_synthesis_step(run_dir: str, deliverable_filename: str, prior_step_ids: list[str]) -> str:
    """Final step: aggregate prior step outputs into deliverable file."""
    prior_lines = []
    for sid in prior_step_ids:
        prior_lines.append(f"        Step {sid} output:")
        prior_lines.append(f"        {{{{step.{sid}.output}}}}")
        prior_lines.append("")
    prior_outputs_block = '\n'.join(prior_lines)
    dep_lines = ""
    if prior_step_ids:
        dep_items = "\n        - ".join(prior_step_ids)
        dep_lines = f"      depends_on:\n        - {dep_items}\n"
    return f'''    step_final_synthesize:
{dep_lines}      generative_entity: "${{models.qwen35}}"
      prompt: |
        You are the FINAL synthesis step. Produce the polished final deliverable.

        ORIGINAL USER PROMPT:
        {{{{bookmarks.shell_output.stdout}}}}

        PRIOR STEP OUTPUTS (intermediate findings):
{prior_outputs_block}

        Write the final deliverable as a comprehensive, polished document. For diagnostic prompts include:
        1. Root Cause (one paragraph explaining the actual cause)
        2. Evidence (cite specific log lines or error messages that prove the diagnosis)
        3. Fix (numbered actionable steps with exact commands)
        4. Verification (how to confirm the fix worked)
        5. Troubleshooting (what to check if the fix fails)

        For documentation/code prompts, produce the full deliverable with proper structure and complete content.

        Rules:
        - Do NOT include meta-commentary ("As an AI...", "I would suggest...")
        - Do NOT refuse or hedge
        - Do NOT use placeholders like "<your answer here>"
        - Write ACTUAL content directly usable by the user
        - Target length: 1500-3000 bytes for diagnostics, more for docs/code
      model_overrides:
        max_tokens: 8192
        temperature: 0.3
      when:
        before_step_starts:
          - shell:
              command: "cat {run_dir}/input/prompt.txt"
              args: []
              working_dir: "/home/jon/code/whitt-execution-engine"
              fail_on_error: true
        after_step_succeeds:
          - save_to:
              - $final_deliverable
              - "{run_dir}/deliverables/{deliverable_filename}"
          - log:
              to_file_path: "{run_dir}/logs/step_final_synthesize.log"
              event_fields: [step_name, duration_ms, total_tokens]
              level: info

'''


def main() -> int:
    if len(sys.argv) < 3:
        sys.stderr.write(
            'usage: build-workflow.py <structs.md> <output.yml> [prompt-path] [run-dir] [meta-run-id]\n'
        )
        return 2

    structs_path = Path(sys.argv[1])
    out_path = Path(sys.argv[2])
    prompt_path = sys.argv[3] if len(sys.argv) > 3 else ''
    run_dir = sys.argv[4] if len(sys.argv) > 4 else './outputs'
    meta_run_id = sys.argv[5] if len(sys.argv) > 5 else ''

    if not structs_path.exists():
        sys.stderr.write(f'error: input not found: {structs_path}\n')
        return 1

    structs_text = structs_path.read_text(encoding='utf-8')
    blocks = extract_step_blocks(structs_text)

    if not blocks:
        sys.stderr.write('error: no step blocks found in structs.md\n')
        return 1

    wid, name = derive_workflow_id(structs_text)
    header = SKELETON.format(workflow_id=wid, name=name)

    # Detect if SW4 already emitted step_00_bootstrap (per new SW4 prompt rules).
    # If so, don't add our own — would cause duplicate key error.
    has_bootstrap = any(
        re.match(r'^step_00_bootstrap\s*:', b.strip()) for b in blocks
    )

    # Optional bootstrap step injects prompt.txt into bookmarks.shell_output
    bootstrap = ''
    if prompt_path and not has_bootstrap:
        bootstrap = make_bootstrap_step(prompt_path, run_dir)
    elif has_bootstrap:
        sys.stderr.write('[build-workflow] SW4 already emitted step_00_bootstrap, skipping ours\n')

    # Optional final synthesis step (skip if SW4 already emitted one)
    has_synthesis = any(
        re.match(r'^step_final_synthesize\s*:', b.strip()) for b in blocks
    )

    blocks_clean = [strip_save_to_templates(b) for b in blocks]

    # Dedup save_to paths: SW4 sometimes shares paths across steps, causing data loss.
    blocks_clean = dedup_save_to_paths(blocks_clean)

    # Derive depends_on from cat targets: if step_B cats ./outputs/<step_A>.txt,
    # step_B depends_on step_A. Without this, runner executes in hash-map order
    # (random), causing cat to fail because upstream step hasn't saved yet.
    all_step_ids = [
        m.group(1)
        for b in blocks_clean
        for m in [re.match(r'^(\S+):', b.strip())]
        if m
    ]
    for i, block in enumerate(blocks_clean):
        m_id = re.match(r'^(\S+):', block.strip())
        if not m_id:
            continue
        sid = m_id.group(1)
        if sid == 'step_00_bootstrap':
            continue
        if 'depends_on:' in block:
            continue
        cat_targets = re.findall(r'cat\s+\./outputs/(step_\w+)\.txt', block)
        deps = [t for t in cat_targets if t in all_step_ids and t != sid]
        if deps:
            unique_deps = list(dict.fromkeys(deps))
            dep_line = f"  depends_on:\n    - " + "\n    - ".join(unique_deps) + "\n"
            first_line_end = block.find('\n')
            if first_line_end == -1:
                blocks_clean[i] = block + dep_line
            else:
                blocks_clean[i] = block[:first_line_end+1] + dep_line + block[first_line_end+1:]

    # Rewrite SW4-internal paths (meta-<RUNID>-swN-<TS>) to META run-id paths (<RUNID>).
    # SW4 emits paths pointing to its own run dir; final execution uses META run dir.
    if meta_run_id:
        blocks_clean = [rewrite_sw4_paths(b, meta_run_id) for b in blocks_clean]

    # Force step_final_synthesize to run LAST by injecting depends_on: [all other step IDs].
    # SW4 emits synthesis without depends_on, causing it to run before prior steps complete.
    synthesis_idx = next(
        (i for i, b in enumerate(blocks_clean) if re.match(r'^step_final_synthesize\s*:', b.strip())),
        None,
    )
    if synthesis_idx is not None:
        other_ids = [
            m.group(1)
            for b in blocks_clean
            for m in [re.match(r'^(\S+):', b.strip())]
            if m and m.group(1) != 'step_final_synthesize'
        ]
        if other_ids:
            dep_line = f"  depends_on:\n    - " + "\n    - ".join(other_ids) + "\n"
            old_first_line = "step_final_synthesize:\n"
            new_first_line = old_first_line + dep_line
            blocks_clean[synthesis_idx] = blocks_clean[synthesis_idx].replace(
                old_first_line, new_first_line, 1
            )

    # If bootstrap is present (ours or SW4's), prepend context reference to ANY step's prompt
    # that has a before_step_starts: shell hook (so it actually uses the injected content).
    # Skip if first block IS step_00_bootstrap (don't inject into bootstrap itself).
    if (bootstrap or has_bootstrap):
        for i, block in enumerate(blocks_clean):
            stripped = block.strip()
            if stripped.startswith('step_00_bootstrap'):
                continue
            if 'before_step_starts' in block and 'shell' in block:
                blocks_clean[i] = inject_context_ref(block)

    # Inject working_dir into all shell hooks so relative paths resolve from repo root.
    # Without this, sed/cat commands on relative paths (e.g. "src/benchmark/runner.rs")
    # fail because CWD is the exec directory, not the repo root.
    REPO_ROOT = '/home/jon/code/whitt-execution-engine'
    for i, block in enumerate(blocks_clean):
        if 'shell:' not in block:
            continue
        if 'working_dir:' in block:
            continue
        lines = block.split('\n')
        new_lines = []
        j = 0
        while j < len(lines):
            new_lines.append(lines[j])
            stripped = lines[j].strip()
            if stripped.startswith('command:'):
                indent = len(lines[j]) - len(lines[j].lstrip())
                new_lines.append(' ' * (indent + 2) + f'working_dir: "{REPO_ROOT}"')
            j += 1
        blocks_clean[i] = '\n'.join(new_lines)

    indented_blocks = [normalize_indent(b) for b in blocks_clean]
    body = '\n\n'.join(indented_blocks)

    # Final synthesis step aggregates prior outputs into deliverable file
    synthesis = ''
    if run_dir and run_dir != './outputs' and not has_synthesis:
        # Extract step IDs from blocks for aggregation reference
        step_ids = []
        for b in blocks_clean:
            m_id = re.match(r'^(\S+):', b.strip())
            if m_id:
                step_ids.append(m_id.group(1))
        # Use last 3 step IDs as aggregation source (most recent outputs)
        prior_ids = step_ids[-3:] if len(step_ids) >= 3 else step_ids
        deliverable_name = 'deliverable.md'
        synthesis = make_synthesis_step(run_dir, deliverable_name, prior_ids)
    elif has_synthesis:
        sys.stderr.write('[build-workflow] SW4 already emitted step_final_synthesize, skipping ours\n')

    out_path.parent.mkdir(parents=True, exist_ok=True)
    out_path.write_text(header + bootstrap + body + '\n\n' + synthesis, encoding='utf-8')

    step_count = len(blocks) + (1 if bootstrap else 0) + (1 if synthesis else 0)
    sys.stderr.write(
        f'[build-workflow] assembled {step_count} steps -> {out_path}\n'
    )
    return 0


if __name__ == '__main__':
    sys.exit(main())
