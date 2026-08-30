# Gap-band matrix — final 17-case band suite

Specialists: Qwen3-4B-Instruct-2507-Q4_K_M, Qwen3-4B-Thinking-2507-Q4_K_M, Falcon-H1-7B-Instruct-Q4_K_M

| case | old fails | 4B-I | 4B-T | Falc | verdict |
|---|---|---|---|---|---|
| s2-env-01 | YES | fail | PASS | fail | VALID |
| s2-env-03 | YES | fail | PASS | PASS | VALID |
| s2-env-05 | YES | fail | PASS | fail | VALID |
| s2-env-06 | YES | PASS | fail | PASS | VALID |
| s2-env-08 | YES | PASS | PASS | fail | VALID |
| s2-env-09 | YES | fail | PASS | fail | VALID |
| s2-env-10 | YES | fail | PASS | fail | VALID |
| s2-env-11 | YES | fail | fail | fail | PENDING |
| s2-env-12 | YES | fail | PASS | fail | VALID |
| s2-env-13 | YES | fail | fail | fail | PENDING |
| s2-env-14 | YES | fail | PASS | fail | VALID |
| s2-fmt-01 | YES | fail | fail | PASS | VALID |
| s2-fmt-05 | YES | fail | PASS | fail | VALID |
| s2-fmt-07 | YES | fail | fail | PASS | VALID |
| s2-log-02 | YES | fail | PASS | fail | VALID |
| s2-log-06 | YES | PASS | fail | fail | VALID |
| s2-str-05 | YES | fail | PASS | PASS | VALID |

**15 VALID / 17** (2 PENDING = old fails, no single specialist passes — cascade candidates)
