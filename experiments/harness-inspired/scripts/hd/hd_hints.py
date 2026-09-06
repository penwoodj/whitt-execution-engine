#!/usr/bin/env python3
"""Per-archetype method hints for hd-* entity questions. Injected into v12
workflow ENTITY blocks so the solver gets the unit+method reminder next to
each question (design doc delta: exact-token+unit-spec wording).
Deterministic, zero LLM.

Usage (from gen-v12): enrich_question_hd(arch, eid, question)
"""

REWRITE_HD = {
    # A — boundary/envelope
    "quota_proration": "*: prorated charge is ceil(days*rate/30) computed in integer cents, round up only at the end.",
    "tiered_postage": "*: each tier's rate applies only to the slice of units inside its band, sum the slices, never re-rate the whole batch at one tier.",
    "ballot_quorum": "*: quorum counts at-or-over members exactly, margin is yes minus no after resolving the chair rule.",
    "disk_quota_conv": "*: convert gibibytes to mebibytes by multiplying 1024 before any comparison, report in mib.",
    # B — staged holds
    "escrow_ladder": "*: each tranche releases only after its own hold clears, sum released tranches only.",
    "per_diem_holds": "*: per-diem accrues rate*nights in whole cents, the hold subtracts from the total before payout.",
    "multi_wallet": "*: spend draws wallets in the stated order, stop at the first wallet that cannot cover its slice.",
    "rebate_tiers": "*: gross is units times unit price, the rebate percent comes from the tier the total units fall in, never a neighboring tier.",
    # C — rotation
    "ring_rotation_2cyc": "*: advance the ring a full two cycles, position arithmetic is modulo the cohort size.",
    "shift_carousel": "*: the carousel advances one slot per rotation, track each member's landing slot after the stated rotations.",
    "canary_percent": "*: canary percent is canary nodes over cohort total, integer percent, truncating division.",
    # D — queues
    "redrive_decay": "*: each redrive pass reaches only the surviving fraction, survivors multiply pass over pass.",
    "backoff_scale": "*: backoff doubles per attempt and is capped at the stated ceiling, report the capped wait.",
    "poison_mix": "*: poison and retry lanes count separately, never mix lane counts in one total.",
    # E — capacity
    "standby_cascade": "*: promotions cascade only while seats remain, the count stops at capacity even if the queue is longer.",
    "overbook_bump": "*: bumped is booked minus capacity when over, volunteers leave before involuntary bumps.",
    "room_split": "*: split the room before counting occupancy, a wall is not a shared seat.",
    # F — routing
    "triage_2hop": "*: follow the two-hop routing table in order, the final destination is the last hop that matches.",
    "cold_divert": "*: cold entries divert only past the stated threshold, count diversions, not entries.",
    "customs_tier": "*: the duty tier comes from the declared value band with strict edges, compute duty on the banded slice.",
    # G — clocks
    "sla_pause_clock": "*: paused minutes never count against the clock, subtract every pause span before judging breach.",
    "window_overlap_dedupe": "*: dedupe alerts inside the same window before counting, one window one alert.",
    "accrual_diff": "*: accrual is a per-month integer, the asked difference is plain integer subtraction.",
    # H — char-precision
    "label_surgery": "*: exact string surgery, splice at the stated indices, characters not bytes, preserve case.",
    "checksum_weighted": "*: weighted checksum is sum of digit times weight modulo the stated base.",
}


def enrich_question_hd(arch, eid, question):
    hint = REWRITE_HD.get(arch)
    if not hint:
        return question
    return f"{question}. hint: {hint}"
