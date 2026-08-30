# Source: Guardrails AI + NeMo Guardrails — Output Schema Validation

- **URLs:**
  - https://github.com/guardrails-ai/guardrails
  - https://guardrails.elsaifoundry.ai/guardrails-process/output-rails.html
  - https://docs.nvidia.com/nemo/guardrails/configure-guardrails/yaml-schema/guardrails-configuration
- **Retrieved:** 2026-08-24 (websearch)

## Method Mechanics

**Guardrails AI:** JSON schema conformance + field validators (regex, length, format). On-fail actions: `reask | fix | filter | refrain | noop | exception`. Reask budget default 1–3 attempts. Optional LLM reask with validation errors appended.

**NeMo Guardrails:** multi-rail (input/dialog/exec/output), LLM-free regex/keyword rails + optional LLM-judge rails (`self_check_output`, `self_check_facts`). Parallel rails, configurable thresholds.

## Core Insight

Deterministic validators (schema, regex, keywords) are LLM-free, cheap, high-precision FIRST-LINE detectors. LLM-judges only as second line.

## What We Borrow — THE SPOOF-PHASE DETECTOR SUITE

detect.py implements exactly this first-line suite (all LLM-free):

| Detector | Signal | YAML/engine analogue |
|----------|--------|---------------------|
| json_parsable | JSON validity | engine's `AfterStepSucceedsContext.json_parsable` field exists natively |
| schema_ok | required fields present | Guardrails schema conformance |
| refusal_keywords | "cannot", "unable", "refuse" | NeMo input/output rails |
| length_bounds | min ≤ len ≤ max | Guardrails length validator |
| tool_error_count | failed tool calls | exec rail |
| hallucination_markers | unsupported-claim markers | NeMo fact-check (deterministic keyword form) |
| contradiction_markers | opposing assertions | self_check analogue |
| upstream_errors | dependency failure markers | — (workflow-specific) |

1. **On-fail → reask pattern = our heal loop** (detect fail → heal → retry, budget 3 = Guardrails reask 1–3 upper range)
2. **Validator severity levels** — detect.json separates hard signals (tool errors, refusal) from soft (length, confidence) — classification uses priority order, not raw vote

## Divergence

Guardrails wraps model calls in production. We embed detection as workflow steps + hooks (engine-native), same effect under whitt orchestration.
