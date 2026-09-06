# AGENTIC-100 Manifest

100 cases, 25 engines x 4 variants. Difficulty H engines: sched, quota, counterfactual, epistemic, paging, residency, cascade.

| case | engine | difficulty | words | keys |
|---|---|---|---|---|
| a1-backoff-01 | backoff | L | 143 | attempts, wait_s, cap_hit |
| a1-backoff-02 | backoff | L | 145 | attempts, wait_s, cap_hit |
| a1-backoff-03 | backoff | L | 144 | attempts, wait_s, cap_hit |
| a1-backoff-04 | backoff | L | 148 | attempts, wait_s, cap_hit |
| a1-ratelimit-01 | ratelimit | L | 119 | allowed, rejected, active |
| a1-ratelimit-02 | ratelimit | L | 121 | allowed, rejected, active |
| a1-ratelimit-03 | ratelimit | L | 120 | allowed, rejected, active |
| a1-ratelimit-04 | ratelimit | L | 120 | allowed, rejected, active |
| a1-canary-01 | canary | L | 138 | rollout_pct, stages_done, status |
| a1-canary-02 | canary | L | 140 | rollout_pct, stages_done, status |
| a1-canary-03 | canary | L | 139 | rollout_pct, stages_done, status |
| a1-canary-04 | canary | L | 139 | rollout_pct, stages_done, status |
| a1-cache-01 | cache | L | 135 | hits, misses |
| a1-cache-02 | cache | L | 139 | hits, misses |
| a1-cache-03 | cache | L | 131 | hits, misses |
| a1-cache-04 | cache | L | 136 | hits, misses |
| a1-sched-01 | sched | H | 172 | order, met, rejected |
| a1-sched-02 | sched | H | 174 | order, met, rejected |
| a1-sched-03 | sched | H | 173 | order, met, rejected |
| a1-sched-04 | sched | H | 173 | order, met, rejected |
| a1-resume-01 | resume | L | 152 | executed, flagged, finished |
| a1-resume-02 | resume | L | 154 | executed, flagged, finished |
| a1-resume-03 | resume | L | 153 | executed, flagged, finished |
| a1-resume-04 | resume | L | 153 | executed, flagged, finished |
| a1-flag-01 | flag | L | 116 | kill, enabled_orgs |
| a1-flag-02 | flag | L | 118 | kill, enabled_orgs |
| a1-flag-03 | flag | L | 117 | kill, enabled_orgs |
| a1-flag-04 | flag | L | 117 | kill, enabled_orgs |
| a1-budget-01 | budget | L | 133 | a_docs, b_docs, c_docs, spend_k |
| a1-budget-02 | budget | L | 135 | a_docs, b_docs, c_docs, spend_k |
| a1-budget-03 | budget | L | 134 | a_docs, b_docs, c_docs, spend_k |
| a1-budget-04 | budget | L | 134 | a_docs, b_docs, c_docs, spend_k |
| a1-timeline-01 | timeline | L | 117 | detect_min, mttr_min, impact_min |
| a1-timeline-02 | timeline | L | 119 | detect_min, mttr_min, impact_min |
| a1-timeline-03 | timeline | L | 118 | detect_min, mttr_min, impact_min |
| a1-timeline-04 | timeline | L | 118 | detect_min, mttr_min, impact_min |
| a1-semver-01 | semver | L | 121 | safe, broken |
| a1-semver-02 | semver | L | 123 | safe, broken |
| a1-semver-03 | semver | L | 122 | safe, broken |
| a1-semver-04 | semver | L | 122 | safe, broken |
| a1-idem-01 | idem | L | 125 | charged, deduped |
| a1-idem-02 | idem | L | 127 | charged, deduped |
| a1-idem-03 | idem | L | 126 | charged, deduped |
| a1-idem-04 | idem | H | 126 | charged, deduped |
| a1-migration-01 | migration | L | 128 | processed, deadletter, retries |
| a1-migration-02 | migration | L | 127 | processed, deadletter, retries |
| a1-migration-03 | migration | L | 131 | processed, deadletter, retries |
| a1-migration-04 | migration | L | 127 | processed, deadletter, retries |
| a1-cascade-01 | cascade | H | 142 | b_timeouts, a_timeouts, latency_s, suppressed |
| a1-cascade-02 | cascade | H | 144 | b_timeouts, a_timeouts, latency_s, suppressed |
| a1-cascade-03 | cascade | H | 143 | b_timeouts, a_timeouts, latency_s, suppressed |
| a1-cascade-04 | cascade | H | 143 | b_timeouts, a_timeouts, latency_s, suppressed |
| a1-quota-01 | quota | H | 192 | atl_cap, bor_cap, cas_cap, pool_left, denied |
| a1-quota-02 | quota | H | 190 | atl_cap, bor_cap, cas_cap, pool_left, denied |
| a1-quota-03 | quota | H | 193 | atl_cap, bor_cap, cas_cap, pool_left, denied |
| a1-quota-04 | quota | H | 189 | atl_cap, bor_cap, cas_cap, pool_left, denied |
| a1-counterfactual-01 | counterfactual | H | 205 | elevated_min, m1_runs, rollbacks, deploys_total |
| a1-counterfactual-02 | counterfactual | H | 207 | elevated_min, m1_runs, rollbacks, deploys_total |
| a1-counterfactual-03 | counterfactual | H | 206 | elevated_min, m1_runs, rollbacks, deploys_total |
| a1-counterfactual-04 | counterfactual | H | 206 | elevated_min, m1_runs, rollbacks, deploys_total |
| a1-epistemic-01 | epistemic | H | 233 | true, false, unverified |
| a1-epistemic-02 | epistemic | H | 235 | true, false, unverified |
| a1-epistemic-03 | epistemic | H | 230 | true, false, unverified |
| a1-epistemic-04 | epistemic | H | 234 | true, false, unverified |
| a1-paging-01 | paging | H | 186 | pages_issued, slots_in_use, escalations, deploys_blocked |
| a1-paging-02 | paging | H | 184 | pages_issued, slots_in_use, escalations, deploys_blocked |
| a1-paging-03 | paging | H | 185 | pages_issued, slots_in_use, escalations, deploys_blocked |
| a1-paging-04 | paging | H | 183 | pages_issued, slots_in_use, escalations, deploys_blocked |
| a1-freeze-01 | freeze | L | 132 | shipped, pending, finished_by |
| a1-freeze-02 | freeze | L | 134 | shipped, pending, finished_by |
| a1-freeze-03 | freeze | L | 134 | shipped, pending, finished_by |
| a1-freeze-04 | freeze | L | 138 | shipped, pending, finished_by |
| a1-checkpoint-01 | checkpoint | L | 122 | wall_min, stages_rerun |
| a1-checkpoint-02 | checkpoint | H | 124 | wall_min, stages_rerun |
| a1-checkpoint-03 | checkpoint | H | 123 | wall_min, stages_rerun |
| a1-checkpoint-04 | checkpoint | L | 123 | wall_min, stages_rerun |
| a1-residency-01 | residency | H | 134 | loads, load_s, unload_s |
| a1-residency-02 | residency | H | 138 | loads, load_s, unload_s |
| a1-residency-03 | residency | H | 137 | loads, load_s, unload_s |
| a1-residency-04 | residency | H | 141 | loads, load_s, unload_s |
| a1-weighted-01 | weighted | L | 99 | alpha, beta, gamma, remainder_added_to |
| a1-weighted-02 | weighted | L | 101 | alpha, beta, gamma, remainder_added_to |
| a1-weighted-03 | weighted | L | 100 | alpha, beta, gamma, remainder_added_to |
| a1-weighted-04 | weighted | L | 100 | alpha, beta, gamma, remainder_added_to |
| a1-cron-01 | cron | L | 119 | overlaps_per_hour, job1_fires, job2_fires |
| a1-cron-02 | cron | L | 121 | overlaps_per_hour, job1_fires, job2_fires |
| a1-cron-03 | cron | L | 120 | overlaps_per_hour, job1_fires, job2_fires |
| a1-cron-04 | cron | L | 120 | overlaps_per_hour, job1_fires, job2_fires |
| a1-logs-01 | logs | L | 116 | infos, warns, errors, total_nonheartbeat |
| a1-logs-02 | logs | L | 118 | infos, warns, errors, total_nonheartbeat |
| a1-logs-03 | logs | L | 117 | infos, warns, errors, total_nonheartbeat |
| a1-logs-04 | logs | L | 117 | infos, warns, errors, total_nonheartbeat |
| a1-invalidation-01 | invalidation | L | 133 | rerun, first_invalid, untouched |
| a1-invalidation-02 | invalidation | L | 134 | rerun, first_invalid, untouched |
| a1-invalidation-03 | invalidation | L | 132 | rerun, first_invalid, untouched |
| a1-invalidation-04 | invalidation | L | 146 | rerun, first_invalid, untouched |
| a1-queue-01 | queue | L | 106 | processed_high, processed_med, processed_low, left_total |
| a1-queue-02 | queue | L | 108 | processed_high, processed_med, processed_low, left_total |
| a1-queue-03 | queue | L | 107 | processed_high, processed_med, processed_low, left_total |
| a1-queue-04 | queue | L | 107 | processed_high, processed_med, processed_low, left_total |
