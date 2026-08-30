# 01-typed-envelope — Research & Reasoning

## Why this atom exists
Inter-stage free text is where small models lose structure: JSON keys drift,
prose leaks in, error signals vanish. The typed-envelope atom forces every
stage boundary to declare its output shape + error classes so downstream
gates route on structure, not parsing luck.

## Sources
- **S41 Schema-First Tool APIs (arXiv 2603.13404):** schemas cut *interface*
  misuse, not semantic errors; recovery-conditioned analysis shows the payoff
  comes from pairing the schema with recovery paths → our condition C
  (error-class menu) tests exactly this pairing.
- **S38 ToolMisuseBench (2604.01508):** fault taxonomy (schema-drift,
  rate-limit, timeout, auth, adversarial) proves deterministic fault classes
  are enumerable → error-class menus are finite and checkable.
- **Constraint-tax finding (fusion FUSION-SOURCES, arXiv 2605.26128):** hard
  schemas on sub-3B raise validity but tank accuracy (19.7→11.0) when applied
  *before reasoning*. Our envelope declares shape but solve stays free-form;
  only extract packages (N1 reason-free-constrain-late). H2 guards the tax.
- **XGrammar-2 (2601.04426):** constrained 3B beats unconstrained 70B on
  tool-calling format — format reliability is scaffold territory.
- **P7 / N2 (fusion, proven):** deterministic checks make format failures
  impossible at the gate; harvester does check-before-echo packaging.

## Reasoning chain
1. Format failures are the cheapest failure class to eliminate (deterministic).
2. But eliminating them at the wrong time (before reasoning) taxes content.
3. Therefore: declare the envelope early (prompt), enforce it late (extract),
   classify failures (error-class menu) so retry stages get signal.
4. Conditions A/B/C isolate: no envelope / envelope / envelope+error-classes.
   Same truths across conditions → presentation-only manipulation.

## What would change our mind
- If C beats B only on format but not recovery speed, error-class menus are
  decoration → drop to B.
- If B/C show content tax on any engine, restrict envelope to extract stage
  (N1 purity) and re-run.

## Relation to opencode parity
opencode's tool-call loop is schema-driven (typed tool interfaces). Matching
it with 4B models requires the envelope at stage boundaries — the analog of
its structured tool I/O.
