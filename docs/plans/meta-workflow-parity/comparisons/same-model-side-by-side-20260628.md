==========================================
PROMPT: p06
==========================================
SW1-SW5 pipeline:
  Size:  17136 bytes (363 lines)
  Substantive (>500B): YES
  Refusal patterns:    0

opencode baseline (single-shot Qwen3-5-9B-Q4_K_M):
  Size:  27337 bytes (401 lines)
  Substantive (>500B): YES
  Refusal patterns:    0

==========================================
SCORECARD
==========================================
SW1-SW5:       2
opencode base: 4

VERDICT: BASELINE_WINS (SW=2 vs BASE=4)

==========================================
PROMPT: p07
==========================================
SW1-SW5 pipeline:
  Size:  16798 bytes (497 lines)
  Substantive (>500B): YES
  Refusal patterns:    0

opencode baseline (single-shot Qwen3-5-9B-Q4_K_M):
  Size:  33237 bytes (961 lines)
  Substantive (>500B): YES
  Refusal patterns:    0

==========================================
SCORECARD
==========================================
SW1-SW5:       2
opencode base: 4

VERDICT: BASELINE_WINS (SW=2 vs BASE=4)

==========================================
PROMPT: p09
==========================================
SW1-SW5 pipeline:
  Size:  12021 bytes (384 lines)
  Substantive (>500B): YES
  Refusal patterns:    0

opencode baseline (single-shot Qwen3-5-9B-Q4_K_M):
  Size:  887 bytes (20 lines)
  Substantive (>500B): YES
  Refusal patterns:    0

==========================================
SCORECARD
==========================================
SW1-SW5:       4
opencode base: 2

VERDICT: SW_WINS (SW=4 vs BASE=2)

==========================================
PROMPT: p10
==========================================
SW1-SW5 pipeline:
  Size:  4146 bytes (121 lines)
  Substantive (>500B): YES
  Refusal patterns:    0

opencode baseline (single-shot Qwen3-5-9B-Q4_K_M):
  Size:  10021 bytes (217 lines)
  Substantive (>500B): YES
  Refusal patterns:    0

==========================================
SCORECARD
==========================================
SW1-SW5:       2
opencode base: 4

VERDICT: BASELINE_WINS (SW=2 vs BASE=4)

==========================================
PROMPT: p12
==========================================
SW1-SW5 pipeline:
  Size:  10767 bytes (227 lines)
  Substantive (>500B): YES
  Refusal patterns:    0

opencode baseline (single-shot Qwen3-5-9B-Q4_K_M):
  Size:  6728 bytes (161 lines)
  Substantive (>500B): YES
  Refusal patterns:    0

==========================================
SCORECARD
==========================================
SW1-SW5:       4
opencode base: 2

VERDICT: SW_WINS (SW=4 vs BASE=2)

==========================================
PROMPT: p13
==========================================
SW1-SW5 pipeline:
  Size:  15359 bytes (419 lines)
  Substantive (>500B): YES
  Refusal patterns:    0

opencode baseline (single-shot Qwen3-5-9B-Q4_K_M):
  Size:  11446 bytes (333 lines)
  Substantive (>500B): YES
  Refusal patterns:    0

==========================================
SCORECARD
==========================================
SW1-SW5:       4
opencode base: 2

VERDICT: SW_WINS (SW=4 vs BASE=2)

==========================================
PROMPT: p14
==========================================
SW1-SW5 pipeline:
  Size:  10140 bytes (265 lines)
  Substantive (>500B): YES
  Refusal patterns:    0

opencode baseline (single-shot Qwen3-5-9B-Q4_K_M):
  Size:  23950 bytes (422 lines)
  Substantive (>500B): YES
  Refusal patterns:    5

==========================================
SCORECARD
==========================================
SW1-SW5:       2
opencode base: -11

VERDICT: SW_WINS (SW=2 vs BASE=-11)

==========================================
PER-PROMPT SUMMARY
==========================================
p06: SW=17136B vs BASE=27337B → BASELINE_WINS (SW=2 vs BASE=4)
p07: SW=16798B vs BASE=33237B → BASELINE_WINS (SW=2 vs BASE=4)
p08: SKIP (missing file)
p09: SW=12021B vs BASE=887B → SW_WINS (SW=4 vs BASE=2)
p10: SW=4146B vs BASE=10021B → BASELINE_WINS (SW=2 vs BASE=4)
p12: SW=10767B vs BASE=6728B → SW_WINS (SW=4 vs BASE=2)
p13: SW=15359B vs BASE=11446B → SW_WINS (SW=4 vs BASE=2)
p14: SW=10140B vs BASE=23950B → SW_WINS (SW=2 vs BASE=-11)

==========================================
AGGREGATE
==========================================
Total compared:  7
SW1-SW5 wins:    4
Baseline wins:   3
Ties:            0

OVERALL: SW1-SW5 SURPASSES opencode baseline (same model Qwen3-5-9B-Q4_K_M)
