#!/usr/bin/env python3
"""gen-cases-v2.py — compose the 90 matrix cases for self-healing v2.

Parses the authoritative 100-row table in docs/v2/01-CASE-MATRIX-100.md,
skips the 10 flagship rows (hand-written), and emits one YAML case per
remaining row with a composed 1000-1500 word prompt.

Composition model (deterministic, seeded by row number):
  prompt = ROLE + DOMAIN_CONTEXT + ARCHETYPE_SPEC + [COMPLEXITY_EXTRAS]
         + DATA_BLOCK + FILLER_POOL picks + OUTPUT_CONTRACT + QUALITY_BAR

Failure profiles, expected paths, oracle call counts derive from the
matrix's class/attempts columns. Exit contract mirrors flagship cases.

Usage: python3 gen-cases-v2.py [--out-dir DIR] [--row N]
"""
from __future__ import annotations

import argparse
import random
import re
import sys
from pathlib import Path

import yaml

EXP = Path(__file__).resolve().parents[1]          # experiments/self-healing
REPO = EXP.parent.parent                            # repo root
MATRIX = EXP / "docs" / "v2" / "01-CASE-MATRIX-100.md"
DEFAULT_OUT = EXP / "cases" / "v2" / "matrix"

FLAGSHIP_ROWS = {1, 6, 13, 14, 22, 35, 64, 76, 82, 90}

WORD_TARGET = (1050, 1400)

# --------------------------------------------------------------------------
# domain banks: org, mission, systems, constraints
# --------------------------------------------------------------------------
DOMAINS: dict[str, dict] = {
    "software-eng": {
        "org": "a platform engineering group at a logistics SaaS company",
        "system": "the \"Iridium\" shipment orchestration platform",
        "context": [
            "The platform moves roughly forty thousand shipment records a day through ingest queues, a normalization stage, a rules engine, and a notification fan-out layer.",
            "A recent migration replaced the legacy Python consumer tier with Rust workers; the remaining operational scripts are being consolidated into owned, scheduled tools.",
            "The team runs a six-week release train, requires conventional commits, and denies unwraps and missing docs at the workspace lint level.",
            "On-call engineers interact with the system through a runbook wiki and expect fail-fast behavior with actionable error output over internal retry loops.",
        ],
        "constraints": [
            "All storage access goes through the team's storage facade trait; no direct object-store clients.",
            "Public API items carry explicit stability annotations; the v0 series must not break surface without a deprecation note.",
            "Configuration resolves through the layered platform loader: defaults, file, environment, then CLI overrides.",
            "Metrics are emitted through the existing prometheus-compatible facade; new telemetry dependencies are rejected in review.",
        ],
        "data": "Interface inventory: 6 upstream endpoints, 4 event kinds, 3 facade traits, error envelope with 8 machine codes, rate ceilings of 100 writes and 1000 reads per minute.",
    },
    "data-analysis": {
        "org": "the analytics group at a consumer mobile app company",
        "system": "the \"Aurora\" growth analytics pipeline",
        "context": [
            "Weekly extracts feed a legacy Perl reporting pipeline that is being decommissioned after repeated format incidents that paged on-call engineers.",
            "The growth team trusts numbers only when every rate is recomputable from raw counts and every judgement cites an instrumentation note or the absence of one.",
            "Cohort metrics cover onboarding completion, activation, push enablement, and day-7 return, with known Android double-fire and ingest-outage quirks.",
            "The dashboard parser hard-fails on missing keys, percentage-versus-decimal confusion, and silent zero-fills for unmeasured cells.",
        ],
        "constraints": [
            "Rates are decimals to four places; deltas are signed percentages computed from summed numerators, never from averaging daily rates.",
            "Anomalies are deviations beyond 1.5 percentage points from the weekly mean, each with a cause or the literal string none-known.",
            "Unmeasured cells are null with an explanatory note field; zero-filling is a reportable defect.",
            "The data quality section must enumerate every known instrumentation issue with its affected scope.",
        ],
        "data": "Latest weekly extract: 7 daily cohorts between 16k and 25k registrations, onboarding rates near 0.86, activation near 0.50, push near 0.22, prior-week totals for deltas, two documented instrumentation quirks.",
    },
    "research-synthesis": {
        "org": "the research staff supporting a platform engineering team",
        "system": "the quarterly architecture decision process",
        "context": [
            "Leadership requires evidence matrices rather than opinion memos: every claim must trace to a citation key from a fixed source pack, audited by exact string match.",
            "The team's standing worry is that demos look great until domain shift bites, so contradictions in the evidence must be surfaced, not averaged away.",
            "Synthesis paragraphs follow a fixed micro-structure: claim, supporting keys, strongest counter-evidence, and one sentence on what the pack cannot resolve.",
            "Confidence values must be justified by evidence spread, evidence volume, or transfer distance — never by intuition.",
        ],
        "constraints": [
            "Nothing outside the source pack is citable; invented, merged, or abbreviated keys invalidate the artifact.",
            "Numbers cited must match the pack exactly; attribution of a number to the wrong source is a hard failure.",
            "Themes must not share identical support sets; duplicate support means merge or differentiate.",
            "The sensitivity sub-object maps decision levers to the theme whose confidence would move most, qualitatively only.",
        ],
        "data": "Source pack: six items spanning a classical baseline, a dense-retrieval result, a zero-shot benchmark, a hybrid routing study, an engineering cost study, and a reranking economics note.",
    },
    "ops-infra": {
        "org": "the incident and reliability team for an internal deployment platform",
        "system": "the \"Cartographer\" deploy orchestration fleet",
        "context": [
            "The fleet spans 14 regions; deploy jobs run on staggered regional schedules and verify artifact signatures against internal certificate bundles.",
            "Postmortem analyses feed a review board that rejects documents blaming symptoms, ignoring coincident window events, or lacking evidence citations per timeline entry.",
            "The platform VP reads these analyses to allocate budget; a wrong-layer fix last quarter caused a repeat incident within three weeks.",
            "Severity classification follows a published rubric: customer impact first, then internal blast radius, then mitigation time.",
        ],
        "constraints": [
            "Every timeline entry carries an ISO 8601 UTC timestamp, a description, and its evidence source string.",
            "The causal chain must run through primary evidence; temporal correlation alone must be labeled as such.",
            "Every coincident event in the window must be explicitly ruled in or out with the excluding evidence named.",
            "Remediation items carry fix, owner team, priority, and a verification step; at least one item must address process, not only technology.",
        ],
        "data": "Evidence pack: 62-minute incident window, 6 of 14 regions affected, 41 failed jobs, cert rotation at window start, an object-store lifecycle purge event, a 600-second bundle cache, and two unrelated coincident changes to exclude.",
    },
    "finance-risk": {
        "org": "the risk analytics desk at a mid-sized payments company",
        "system": "the quarterly exposure review for card portfolio risk",
        "context": [
            "The review reconciles chargeback ratios, authorization decline curves, and reserve adequacy against scheme network rules and internal appetite statements.",
            "Regulators and scheme networks require that every ratio in the review be recomputable from the raw settlement extracts, and that overrides carry documented rationale.",
            "Last cycle's review was returned for a unit error that mixed basis points with percentages in the reserve table.",
            "The desk publishes to both the CFO's staff and the scheme compliance liaison, each of whom spot-checks different cells.",
        ],
        "constraints": [
            "Ratios carry explicit units; basis-point and percentage values must never appear in the same column without labels.",
            "Every override line references the appetite paragraph it invokes.",
            "Trend extrapolations state their arithmetic; no unstated compounding.",
            "The verdict vocabulary is fixed: within-appetite, watch, breach-imminent, breach.",
        ],
        "data": "Quarterly extracts: 9 portfolio segments, chargeback ratios from 0.31 to 1.02 percent against a 0.90 scheme threshold, decline rates by bin range, reserve balances versus required floors, and a 60-day holiday-season projection window.",
    },
    "healthcare-info": {
        "org": "the clinical informatics team at a regional hospital network",
        "system": "the patient-communication scheduling service",
        "context": [
            "The network runs appointment reminder and result-notification workflows across 14 clinics, with message templates governed by a plain-language review board.",
            "Privacy constraints require that message content carry no diagnosis information and that logs carry no identifiers; violations are reportable events.",
            "The scheduling service reconciles clinic calendars nightly and must explain every skipped or merged slot in an exceptions report the clinics sign off on.",
            "Clinic staff audit the exceptions report weekly; unexplained slots generate support tickets that cost more than the automation saves.",
        ],
        "constraints": [
            "Messages pass the plain-language check at an eighth-grade reading level before template approval.",
            "Every exception entry names its rule and the calendar evidence for it.",
            "Report sections separate clinical content from transport telemetry; mixed sections are returned.",
            "Identifier scrubbing is verified by a fixed pattern suite before any artifact leaves the network boundary.",
        ],
        "data": "Nightly reconciliation: 14 clinics, roughly 4,100 appointments, 6 template families, 3 channel types with different failure semantics, 2.3 percent calendar conflict rate, and a known double-booking defect in one clinic's practice-management export.",
    },
    "legal-compliance": {
        "org": "the compliance automation team at a financial services firm",
        "system": "the multi-jurisdiction data protection program",
        "context": [
            "The firm is expanding its payments product into new markets, each with its own data protection framework, transfer mechanisms, and representative appointment rules.",
            "Program documents follow a RACI discipline with exactly one accountable owner per artifact, and effort figures use defined bands rather than raw hours.",
            "The group program provides a generic records inventory, an incident runbook, a web-era consent manager, and vendor due diligence questionnaires; each must be gap-assessed per market.",
            "Where the current state is unknown, plans must include discovery tasks rather than assumptions; silently inventing group policy is a review-board failure.",
        ],
        "constraints": [
            "Launch windows are fixed by commercial commitments; derived dates move, launch dates do not.",
            "Every workstream that cannot complete before launch carries a documented compensating control.",
            "Risk ratings use the defined three-point scale with operational definitions stated before use.",
            "The decision log pre-seeds open decisions with escalation path and review cadence.",
        ],
        "data": "Program inputs: 3 target jurisdictions, 6 existing program assets to gap-assess, a shared engineering capacity constrained by a platform migration, two approved analyst requisitions, and 90-day spaced launch milestones.",
    },
    "marketing-content": {
        "org": "the content strategy team at a B2B developer-tools company",
        "system": "the launch campaign for a new observability product",
        "context": [
            "The brand is technically precise, allergic to hype, and fond of dry humor; the audience distrusts superlatives and converts on documents that respect their intelligence.",
            "A banned-term style guide bars a fixed list of hype words; legal and references review run through inline markers rather than email threads.",
            "Claims discipline: every number carries a verification marker unless it appears in the approved claims register, which currently holds only the three pillar claims.",
            "Campaign assets ship against a dependency-locked publication order with pre-written rollback notes for announcement posts.",
        ],
        "constraints": [
            "Positioning states category, buyer, pain, differentiated claim, and reason to believe in under seventy-five words.",
            "Competitive contrast names the incumbent's genuine strength before its structural weakness.",
            "Success metrics come from the standard funnel vocabulary with day-7 and day-30 revision thresholds.",
            "No invented customer names, metrics, or quotes; anecdote placeholders are marked for the references team.",
        ],
        "data": "Campaign inputs: 3 proof pillars with quantified claims, 3 audience segments, 2 incumbent categories to contrast, a six-week launch runway, and a fourteen-asset minimum content matrix.",
    },
    "scientific-compute": {
        "org": "the research computing group supporting an observational science collaboration",
        "system": "the field-station data pipeline and the shared cluster it runs on",
        "context": [
            "Eighteen field stations log messy daily telemetry that a specification-driven pipeline converts into analysis-ready monthly tables with deterministic bytes.",
            "Firmware heterogeneity means the same physical quantity arrives under three field-naming schemes and two unit systems; unknown variants must fail loudly, never guess.",
            "The cluster scheduler, scratch storage, and MPI runtime all changed in one maintenance window, and post-incident analyses must separate causal evidence from coincidence.",
            "Domain constants must appear with values and one-line justifications because reviewers know data engineering, not the science.",
        ],
        "constraints": [
            "Cleaning rules are flag-not-delete: physically implausible values are marked suspect, connector spikes corrected via rolling median, storm events kept.",
            "Interpolation is forbidden across gaps longer than forty minutes; shorter gaps interpolate with corrected flags.",
            "Reprocessing is idempotent by construction, with stages marked skip-versus-recompute and reasons stated.",
            "Property-based tests inject exactly the anomalies the detector must find, and nothing else.",
        ],
        "data": "Pipeline inputs: 3 firmware variants across 18 stations, ten-minute observation windows, a six-value tide-stage table, quality flags from a closed four-value set, plus a cluster evidence pack with six diagnostic tests.",
    },
    "product-ux": {
        "org": "the product design research team at a productivity software company",
        "system": "the onboarding redesign program for the core workspace editor",
        "context": [
            "The editor's onboarding funnel loses most new users before first value; the redesign program pairs telemetry with moderated study findings.",
            "Findings documents must separate observed behavior from participant verbatims from facilitator interpretation, each with its own label.",
            "Telemetry events for the funnel were instrumented two quarters ago and carry a known double-count defect on one step that analyses must discount explicitly.",
            "Design reviews reject recommendation sections that cite no evidence and reject evidence sections with no traceable tie to a decision.",
        ],
        "constraints": [
            "Every recommendation names the evidence line that motivates it and the risk if the assumption behind it is wrong.",
            "Sample sizes and segment definitions appear before any percentages derived from them.",
            "Known instrumentation defects are carried as first-class caveats in every affected chart description.",
            "Competitive references are clearly marked as contextual, never as evidence.",
        ],
        "data": "Study inputs: funnel telemetry across 7 steps with step-level drop-off, 12 moderated sessions across 3 personas, a 340-response survey with known skew, and a competitive scan of 5 adjacent products.",
    },
}

# --------------------------------------------------------------------------
# archetype banks: deliverable shape + section requirements
# --------------------------------------------------------------------------
ARCHETYPES: dict[str, dict] = {
    "generate-artifact": {
        "framing": "produce a complete, self-contained artifact that downstream consumers use directly, without an intermediate translation step",
        "sections": [
            ("structure", "the artifact opens with a framing section, then one section per required component, each component complete enough to act on alone"),
            ("completeness", "every required component is present; absence of any component is a hard failure, as is placeholder text standing in for required substance"),
            ("specificity", "names, limits, enums, and numeric thresholds appear explicitly rather than paraphrased"),
        ],
        "schema_keys": ["framing", "components", "specificity_notes", "acceptance_checks"],
    },
    "transform-refine": {
        "framing": "produce a transformation specification that converts a described messy input into a fixed target shape, precise enough to implement without clarification meetings",
        "sections": [
            ("mapping", "field-by-field mapping from every input variant to the target shape, with resolution rules for renames, retypes, and absences"),
            ("rules", "every filter, clamp, normalization, and interpolation rule with parameters and rationale"),
            ("invariants", "the validation invariants the output must satisfy and how each is checked"),
        ],
        "schema_keys": ["mapping", "rules", "gap_handling", "invariants", "reproducibility"],
    },
    "analyze-evaluate": {
        "framing": "produce an evaluation artifact whose every verdict is recomputable from the raw measurements supplied, with cross-validation between sections",
        "sections": [
            ("tables", "computed statistics and derived rates with numerators and denominators shown"),
            ("judgements", "every judgement cites the specific evidence or note it rests on, or the literal absence marker"),
            ("consistency", "no statement anywhere may conflict with any numeric table in the same artifact"),
        ],
        "schema_keys": ["computed_stats", "judgements", "consistency_checks", "verdict"],
    },
    "plan-decompose": {
        "framing": "produce a phased, resourced, sequenced plan that a program manager could execute against without further meetings",
        "sections": [
            ("phases", "phases with entry and exit criteria, workstreams with owners, artifacts, dependencies, and coverage"),
            ("critical_path", "hard versus soft dependencies called out, with the critical path explicit"),
            ("risks", "a risk register with defined scales, owners, mitigations, and early-warning indicators"),
        ],
        "schema_keys": ["phases", "critical_path", "risk_register", "decision_log"],
    },
    "troubleshoot-diagnose": {
        "framing": "produce a diagnostic analysis that isolates root cause from coincidental correlation and prescribes verified remediation",
        "sections": [
            ("timeline", "chronological events with timestamps and evidence citations, including contributing changes and coincident events ruled in or out"),
            ("causal_chain", "the causal chain ordered earliest-first, with the single preventing control and the missing alerts named"),
            ("remediation", "fixes with owners, priorities, and verification steps, plus residual risk and follow-up diagnostics clearly marked as not yet run"),
        ],
        "schema_keys": ["timeline", "causal_chain", "blast_radius", "remediation"],
    },
}

COMPLEXITY_EXTRAS = {
    "A": [
        "Scope discipline: this is a focused task; do not extend the deliverable with adjacent-but-unrequested sections. Reviewers treat scope creep as a defect because it dilutes the required sections.",
        "One primary audience: write for the practitioner who will use the artifact directly. Executive framing beyond the required opening summary is out of scope.",
    ],
    "B": [
        "Dual audience: the artifact is read by a specialist reviewer and by an operator or executive; each needs its own entry point, and where their needs conflict, prefer the reader who acts on the artifact.",
        "Cross-cutting consistency: any figure, verdict, or commitment appearing in more than one section must agree exactly across sections; drift between sections is treated as a factual error, not a style issue.",
        "Assumption discipline: state every assumption explicitly; where the current state is unknown, include a discovery item rather than an assumption, and never invent organizational policy or history.",
        "Traceability: wherever the artifact makes a claim, it must be traceable to supplied evidence, a stated assumption, or an explicitly marked gap — never to unstated intuition.",
    ],
}

ARCHETYPE_WORKLOAD = {
    "generate-artifact": "WORKLOAD EXPECTATIONS. The artifact is expected to run between eight hundred and fifteen hundred words of substantive content once complete, structured so that each component could be extracted and used in isolation. Components that merely gesture at content — a section that says what it would contain rather than containing it — are returned as incomplete. Where the artifact includes enumerated items, each item carries the same field set, in the same order, so downstream tooling can iterate them uniformly.",
    "transform-refine": "WORKLOAD EXPECTATIONS. The specification is expected to cover every input variant listed in the supplied material, not a representative subset; a mapping section that handles the common case and waves at the rest is a hard failure. Where the transformation branches, each branch states its trigger condition, its action, and its effect on the output flags, so that a reviewer can trace any input record through the branch lattice by hand.",
    "analyze-evaluate": "WORKLOAD EXPECTATIONS. Every statistic in the artifact shows its derivation: the raw inputs, the arithmetic applied, and the result, presented so a reviewer can repeat the computation with a calculator and obtain the same figure. Judgement sections must be outnumbered by their citations; a judgement paragraph with no traceable basis is deleted in review, not softened.",
    "plan-decompose": "WORKLOAD EXPECTATIONS. The plan is expected to decompose to the level of workstreams with single owners — not phases alone, and not individual tasks. Each workstream's artifact list names documents or systems, not effort. Sequencing must be expressed as explicit dependencies the reader can diagram without interpretation, and the plan must survive a reviewer asking, for any workstream, what blocks it and what it blocks.",
    "troubleshoot-diagnose": "WORKLOAD EXPECTATIONS. The analysis must account for every supplied diagnostic test, using it, superseding it, or explicitly ruling it out; cherry-picking the tests that fit a preferred narrative is the failure mode this artifact exists to prevent. The causal chain must be falsifiable: for each link, the analysis states what evidence would have shown that link to be false.",
}

ARCHETYPE_DEPTH = {
    "generate-artifact": "DEPTH REQUIREMENTS. Where the artifact defines structure, it defines it completely: field names, value types, closed vocabularies, and the exact strings used as identifiers. Where it gives examples, the examples are valid instances of the structure, not sketches of instances. Where it states limits, it states the behavior at the boundary, not merely the boundary value.",
    "transform-refine": "DEPTH REQUIREMENTS. Edge cases are first-class citizens: empty inputs, single-record inputs, records matching multiple variants, and records matching none. The specification states, for each, whether the transformation halts, flags, or produces output, and what that output looks like. The reproducibility section pins down ordering, rounding, and serialization choices by name.",
    "analyze-evaluate": "DEPTH REQUIREMENTS. Uncertainty is stated, not hidden: where the supplied material supports multiple readings, the artifact presents the readings and names the one it adopts. Where a figure is sensitive to a parameter choice — window size, threshold, rounding rule — the artifact states the sensitivity rather than presenting a single number as settled.",
    "plan-decompose": "DEPTH REQUIREMENTS. The plan quantifies what it can and qualifies what it cannot: durations use the defined effort bands, dependencies use the hard-soft distinction, and risks use the defined scales. Any date that is derived rather than fixed is labeled derived, and the plan states the slip threshold that forces recomputation of the critical path.",
    "troubleshoot-diagnose": "DEPTH REQUIREMENTS. The determination section distinguishes what is established, what is most likely, and what remains unknown, and uses those exact labels. Remediation separates the immediate fix from the systemic prevention, and the verification protocol proves the fix at more than one timescale — immediately, and under the conditions that produced the original symptoms.",
}

FILLER_POOL = [
    "Definitions first: any scale, rating, or band used in the artifact must be operationally defined before its first use, so two independent readers score the same item the same way.",
    "Units discipline: every numeric column carries its unit in its label; unitless numbers in derived tables are treated as defects by the review tooling.",
    "Determinism: wherever ordering affects output, the ordering rule is stated; wherever rounding occurs, the rounding rule is stated; both are followed consistently.",
    "Marker conventions: inline markers for verification, legal, and references review use the bracketed vocabulary, and every number not in the approved register carries one.",
    "Idempotency: any repeated application of the process described must converge to the same artifact; steps that must be skipped on re-run are identified with reasons.",
    "Failure loudness: where the described process encounters input it cannot classify, it halts with a diagnostic naming the offending input; silent drops and guessed mappings are prohibited.",
    "Review cadence: named sections carry named owners and the checkpoints at which they are reviewed; unowned sections are returned by default.",
    "Hand-off completeness: the artifact closes with the checklist of downstream consumers and the specific sections each must sign off before the artifact's status advances.",
    "Versioning: the artifact carries a version field and a status vocabulary; supersession is explicit rather than implicit in filenames.",
    "Reconciliation: wherever totals and components both appear, the artifact states the reconciliation between them and treats mismatch as a hard error rather than a rounding note.",
    "Terminology control: the artifact fixes its vocabulary in a short glossary paragraph before first use, and does not alternate between synonyms for the same concept; reviewers track referential integrity line by line.",
    "Evidence recency: every supplied fact is treated as current as of this engagement; if the artifact depends on a fact changing, it marks that dependency explicitly rather than assuming staleness.",
    "Escalation path: wherever the artifact delegates a decision, it names the deciding role, the information that role needs, and the timeframe in which the decision blocks downstream work.",
    "Negative space: the artifact states at least once what is deliberately out of scope and why, so reviewers can distinguish omission from oversight.",
    "Audit hooks: wherever the artifact asserts a property that could silently regress, it names the check that would catch the regression and where that check lives.",
    "Precision over fluency: where a precise but awkward formulation and a fluent but ambiguous one compete, the artifact chooses precision; polish that costs determinism is rejected in review.",
]

OUTPUT_CONTRACT_TMPL = """OUTPUT CONTRACT: the deliverable is a single JSON object. First
character "{{", last character "}}". No markdown fences, no commentary
outside the object, all strings double-quoted, dates in ISO 8601 where
they appear. Required top-level keys: {keys}. The consuming pipeline
hard-fails on missing keys, extra keys beyond a single optional "notes"
field, wrong value types, or unparseable JSON. There is no partial
credit: the artifact either parses and validates or it is an incident."""

QUALITY_BAR_TMPL = """QUALITY BAR: a domain reviewer will spot-check three specific claims
or computations against the raw material supplied above; every number
must be exactly reproducible and every judgement must name its basis.
Reviewers have been asked to reject artifacts that hedge, that paper
over gaps in the supplied material, or that resolve ambiguity silently
instead of stating the interpretation they chose."""

ROLE_TMPL = "You are the senior specialist {role} at {org}. Your task is to {framing}."

# --------------------------------------------------------------------------
# failure profile generator (mirrors flagship semantics)
# --------------------------------------------------------------------------
CLASS_STRATEGY = {
    "F1": "corrective_prompt",
    "F2": "tool_reselect",
    "F3": "replan",
    "F4": "replan",
}

CLASS_SIGNALS = {
    "F1": ["hallucination_keys", "confidence_degraded"],
    "F2": ["schema_missing_keys", "schema_nested_wrong", "tool_error"],
    "F3": ["contradiction_markers", "cross_section_inconsistency"],
    "F4": ["truncated_output", "missing_contract_fields", "upstream_errors"],
    "F4-persist": ["truncated_output", "unsupported_claims", "missing_contract_fields"],
}


def failure_profile(cls: str, attempts: list[int]) -> dict:
    notes_map = {
        "clean": "Clean control: first-attempt output is schema-complete, "
                 "hallucination-free, adequately confident; R clears theta "
                 "outside the gray zone on attempt 1.",
        "F1": "Attempt failures carry the confidence/hallucination signature: "
              "invented keys, methods, or numbers alongside inflated stated "
              "confidence; the corrective-prompt heal scrubs them on the "
              "following attempt.",
        "F2": "Attempt failures carry the schema/format signature: wrapper-key "
              "nesting, missing required keys, wrong value types; the "
              "tool-reselect heal (schema-focused re-read) repairs the shape "
              "on the following attempt.",
        "F3": "Attempt failures carry the logical-consistency signature: "
              "verdicts contradicting the artifact's own tables and "
              "cross-section numeric drift; the replan heal (heavy model) "
              "recomputes verdicts from tables on the following attempt.",
        "F4": "Attempt failures carry the upstream/context signature: "
              "truncation against the length ceiling and dropped contract "
              "fields; each replan heal recovers completeness until the "
              "final attempt passes.",
        "F4-persist": "Failure signature persists across all three attempts, "
                      "each heal trading one defect for another; the attempt "
                      "budget exhausts and the workflow records final_fail "
                      "with an escalation note.",
    }
    key = cls if cls in notes_map else ("F4-persist" if cls.endswith("persist") else cls)
    return {"attempts_failed": attempts, "notes": notes_map.get(key, notes_map[cls.rstrip("0123456789")] if False else "Persistent failure signature across the attempt budget.")}


def expected_block(cls: str, attempts: list[int]) -> dict:
    n = len(attempts)
    if cls == "clean" or n == 0:
        return {
            "final": "accept",
            "path": ["attempt", "classify", "judge_gate", "accept", "report"],
            "attempts_to_success": 1,
        }
    base = cls.replace("-persist", "")
    heal = f"heal_{CLASS_STRATEGY[base]}"
    heals_used = n - 1  # a heal follows every failed attempt except the last
    path: list[str] = []
    for i in range(n):
        path += ["attempt", "classify"]
        if i < heals_used:
            path.append(heal)
    final = "final_fail" if n >= 3 else "accept"
    if final == "final_fail":
        path.append("final_fail")
    else:
        path += ["judge_gate", "accept"]
    path.append("report")
    return {"final": final, "path": path, "attempts_to_success": (n if final == "accept" else None)}


def oracle_block(cls: str, attempts: list[int]) -> dict:
    n = len(attempts)
    base = cls.replace("-persist", "")
    worker = n
    heavy = 0
    if base in ("F3", "F4"):
        heavy = n - 1
    if base in ("F1", "F2"):
        heavy = 0
    # worker: F3/F4 first attempt is worker then heavy handles replan; attempts after heal
    # are still worker generations (heal record feeds worker retry) except F4-persist
    # second/third attempts stay worker. Keep worker = n for all, heavy = replans.
    return {
        "llm_calls": {"worker": worker, "heavy": heavy, "judge": 0},
        "gate_flags": {
            "need_replan": base in ("F3", "F4"),
            "need_judge": False,
            "budget_exhausted": n >= 3,
        },
    }


# --------------------------------------------------------------------------
# matrix parsing
# --------------------------------------------------------------------------
ROW_RE = re.compile(
    r"^\|\s*(\d+)\s*\|\s*([a-z0-9-]+)\s*\|\s*([a-z-]+)\s*\|\s*([a-z-]+)\s*\|"
    r"\s*(\d+)\s*\|\s*([A-Za-z0-9-]+)(?:\(@([^)]*)\))?\s*\|\s*([A-Z]+@\d+)\s*\|\s*(\d+)\s*\|$"
)


def parse_matrix() -> list[dict]:
    rows = []
    for line in MATRIX.read_text().splitlines():
        m = ROW_RE.match(line)
        if not m:
            continue
        num, cid, dom, arc, cpx, cls, att, verdict, calls = m.groups()
        attempts = [int(a.lstrip("@")) for a in att.split(",")] if att else []
        rows.append(
            {
                "num": int(num),
                "cid": cid,
                "domain": dom,
                "archetype": arc,
                "complexity": "A" if int(cpx) <= 3 else "B",
                "class": cls,
                "attempts": attempts,
                "verdict": verdict,
                "calls": int(calls),
            }
        )
    # positional dedupe (doc has a duplicated row number)
    seen_nums = [r["num"] for r in rows]
    if len(rows) != 100:
        sys.exit(f"matrix parse error: expected 100 rows, got {len(rows)}")
    return rows


# --------------------------------------------------------------------------
# prompt composition
# --------------------------------------------------------------------------
def compose_prompt(row: dict, rng: random.Random) -> str:
    dom = DOMAINS[row["domain"]]
    arc = ARCHETYPES[row["archetype"]]
    tier = row["complexity"]

    paras: list[str] = []

    paras.append(
        ROLE_TMPL.format(
            role=f"for {dom['org']}",
            org=dom["org"],
            framing=arc["framing"],
        ).replace("You are the senior specialist for ", "You are the senior specialist at ")
        + f" The work concerns {dom['system']}."
    )

    # domain context (3-4 paragraphs)
    ctx = list(dom["context"])
    rng.shuffle(ctx)
    paras.append("BACKGROUND. " + " ".join(ctx[:3]))
    if tier == "B" and len(ctx) > 3:
        paras.append("ADDITIONAL CONTEXT. " + ctx[3])

    # archetype spec
    sec_lines = [f"- {name}: {req}" for name, req in arc["sections"]]
    paras.append(
        "DELIVERABLE REQUIREMENTS. The artifact must contain, in order, the "
        "sections implied by the following requirements:\n"
        + "\n".join(sec_lines)
        + "\nEach section must stand alone: a reader who jumps directly to any "
        "one section must be able to act on it without reading the others."
    )
    paras.append(ARCHETYPE_WORKLOAD[row["archetype"]])
    paras.append(ARCHETYPE_DEPTH[row["archetype"]])

    # domain constraints
    cons = list(dom["constraints"])
    rng.shuffle(cons)
    pick = cons[:3] if tier == "B" else cons[:2]
    paras.append(
        "BINDING CONSTRAINTS (from the governing team, non-negotiable):\n"
        + "\n".join(f"- {c}" for c in pick)
    )

    # data block
    paras.append("SUPPLIED MATERIAL (the only factual basis for the artifact). " + dom["data"])

    # complexity extras
    paras.append("SCOPE AND RIGOR. " + " ".join(COMPLEXITY_EXTRAS[tier]))

    # filler to reach word target
    filler = list(FILLER_POOL)
    rng.shuffle(filler)
    base_words = sum(len(p.split()) for p in paras)
    lo, hi = WORD_TARGET
    i = 0
    while base_words < lo and i < len(filler):
        paras.append("PROCESS DISCIPLINE. " + filler[i])
        base_words = sum(len(p.split()) for p in paras)
        i += 1
    if base_words > hi:
        # drop last filler paragraph if over
        while base_words > hi and len(paras) > 6:
            removed = paras.pop()
            base_words -= len(removed.split())

    keys = ", ".join(arc["schema_keys"])
    paras.append(OUTPUT_CONTRACT_TMPL.format(keys=keys))
    paras.append(QUALITY_BAR_TMPL)
    return "\n\n".join(p for p in paras if p)


def schema_block(row: dict) -> dict:
    arc = ARCHETYPES[row["archetype"]]
    props = {k: {"type": "string"} for k in arc["schema_keys"]}
    return {
        "type": "object",
        "required": arc["schema_keys"],
        "properties": props,
        "optional": ["notes"],
    }


def build_case(row: dict) -> dict:
    rng = random.Random(20260824 + row["num"])
    cls = row["class"] if row["attempts"] else "clean"
    if row["attempts"] and len(row["attempts"]) >= 3 and row["verdict"] == "FA@3":
        cls_disp = f"{cls}-persist"
    else:
        cls_disp = cls
    prompt = compose_prompt(row, rng)
    words = len(prompt.split())
    if not (1000 <= words <= 1500):
        raise SystemExit(f"row {row['num']} word count {words} out of range")
    case = {
        "case_id": row["cid"],
        "meta": {
            "domain": row["domain"],
            "archetype": row["archetype"],
            "complexity": row["complexity"],
            "class": cls_disp,
            "source": f"case-matrix row {row['num']} ({row['cid']})",
            "words_prompt": f"~{words}",
        },
        "task": {
            "prompt": prompt,
            "expected_schema": schema_block(row),
            "output_contract": (
                "Single JSON object, no fences or commentary. Required "
                "top-level keys per contract section; parser hard-fails on "
                "missing keys, wrong types, or extra keys beyond the "
                "optional notes field."
            ),
        },
        "failure": {
            "class": cls_disp,
            "injection_profile": failure_profile(cls_disp, row["attempts"]),
            "signals": CLASS_SIGNALS.get(
                cls_disp if cls_disp.endswith("persist") else cls, []
            ),
        },
        "expected": expected_block(cls_disp, row["attempts"]),
        "oracle": oracle_block(cls_disp, row["attempts"]),
        "budget": {"llm_call_ceiling": 6},
    }
    return case


def main() -> int:
    ap = argparse.ArgumentParser()
    ap.add_argument("--out-dir", type=Path, default=DEFAULT_OUT)
    ap.add_argument("--row", type=int, help="compose a single matrix row (debug)")
    args = ap.parse_args()

    rows = parse_matrix()
    if args.row:
        rows = [r for r in rows if r["num"] == args.row]

    args.out_dir.mkdir(parents=True, exist_ok=True)
    written = 0
    for row in rows:
        if row["num"] in FLAGSHIP_ROWS and not args.row:
            continue
        case = build_case(row)
        out = args.out_dir / f"{row['cid']}.yml"
        with out.open("w") as fh:
            yaml.safe_dump(case, fh, sort_keys=False, width=100, allow_unicode=True)
        written += 1
    print(f"wrote {written} cases to {args.out_dir}")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
