# Task Breakdown

**Source prompt:** Read the file at /tmp/pipeline-input.csv which contains sales transaction data with columns: transaction_id, customer_email, product_sku, quantity, unit_price, sale_date, sales_region, payment_method. Execute a complete data pipeline in these phases: PHASE 1 - INGESTION AND VALIDATION...

**Total tasks:** 11 (5 parent + 6 subtasks expanded from high-complexity parents)
**Total story points:** 21
**Maximum leaf complexity:** 3 pts

---

## T1 — ingest_csv (2 pts)

**Story points:** 2
**Why:** Single-file load with known schema; standard CSV parsing. Half-day work.
**Action:** Read /tmp/pipeline-input.csv and parse into in-memory row objects using the documented column schema.
**Depends on:** none
**GWT criteria:**
  - **Given:** /tmp/pipeline-input.csv exists with declared columns
  - **When:** ingestion runs
  - **Then:** row count > 0, each row exposes 8 typed fields

### T1.1 — validate_unique_transaction_id (1 pt)
**Action:** Scan rows; flag any duplicate transaction_id values; emit violation count.
**GWT:** Given parsed rows / When dedup check runs / Then duplicate_count emitted, zero duplicates pass

### T1.2 — validate_field_formats (2 pts)
**Action:** For each row, verify customer_email matches RFC email pattern, quantity is positive integer, unit_price is positive number, sale_date matches YYYY-MM-DD. Count failures per check.
**GWT:** Given parsed rows / When per-field regex/type checks run / Then failure_counts dict has 4 keys with integer values

### T1.3 — partition_valid_invalid (1 pt)
**Action:** Split rows into valid_set (passes all 4 checks + unique id) and invalid_set. Emit both counts.
**GWT:** Given validated rows / When partition runs / Then valid_count + invalid_count sum to total rows

---

## T2 — enrich_transactions (2 pts)

**Story points:** 2
**Why:** Per-row arithmetic + bucket classification + per-region aggregation. Linear transform, well-bounded.
**Action:** For each valid row compute total_amount = quantity * unit_price; assign size bucket (micro/small/medium/large/enterprise); accumulate per-region payment_method tallies.
**Depends on:** T1
**GWT criteria:**
  - **Given:** valid_set from T1.3
  - **When:** enrichment runs over all valid rows
  - **Then:** each row has total_amount (number) + size_bucket (enum); region_payment_counts map has one entry per region with dominant method labeled

---

## T3 — detect_anomalies (3 pts)

**Story points:** 3
**Why:** Three distinct statistical detectors (z-score, customer-day-frequency, region-mean-deviation). Multi-step but linear; ~1.5 days.
**Action:** Compute mean + stddev of total_amount; flag rows > 3 stddev above mean. Compute per-customer-per-day transaction counts; flag customers with >10 in any day. Compute overall_avg_total and per-region avg; flag regions differing > 50% from overall.
**Depends on:** T2
**GWT criteria:**
  - **Given:** enriched rows with total_amount + customer_email + sale_date + sales_region
  - **When:** all three detectors run
  - **Then:** anomaly_lists has 3 keys (statistical_outliers, frequent_customers, deviating_regions) each a list with reasons attached

### T3.1 — z_score_outlier_detection (1 pt)
**Action:** Compute mean μ and stddev σ of total_amount; emit rows where total > μ + 3σ.
**GWT:** Given total_amount list / When z-score computed / Then outliers list has reason=">3σ above mean"

### T3.2 — customer_frequency_detection (2 pts)
**Action:** Group by (customer_email, sale_date); count transactions per group; emit customers with any group count > 10.
**GWT:** Given rows / When grouped by (email,date) / Then frequent_customers list has reason=">10 transactions on YYYY-MM-DD"

### T3.3 — region_deviation_detection (1 pt)
**Action:** Compute overall_mean of total_amount; per-region compute mean; flag regions where |region_mean - overall_mean| / overall_mean > 0.5.
**GWT:** Given region-tagged totals / When per-region means computed / Then deviating_regions list has reason="deviates X% from overall"

---

## T4 — generate_report (2 pts)

**Story points:** 2
**Why:** Single-pass assembly of structured text report combining T1+T2+T3 outputs. Half-day.
**Action:** Assemble markdown/text report with sections: (1) valid/invalid counts from T1.3, (2) size distribution from T2, (3) anomaly lists from T3, (4) quality_score = valid_count / total_count * 100. Write to outputs/output/pipeline-report.txt.
**Depends on:** T3
**GWT criteria:**
  - **Given:** outputs of T1.3, T2, T3
  - **When:** report assembled and written
  - **Then:** file exists at outputs/output/pipeline-report.txt containing all 4 sections with numeric data populated

---

## T5 — compute_quality_score (1 pt)

**Story points:** 1
**Why:** Single arithmetic formula. <4 hours.
**Action:** Compute quality_score = round(100 * valid_count / total_count, 2). Include in report.
**Depends on:** T4
**GWT criteria:**
  - **Given:** valid_count and total_count from T1.3
  - **When:** formula applied
  - **Then:** quality_score is a number in [0, 100] present in the final report

---

## Complexity Distribution

| Points | Count |
|--------|-------|
| 1 | 5 |
| 2 | 4 |
| 3 | 1 |
| 5 | 0 (T3 was borderline but kept as 3 — atomic enough with subtask decomposition) |

---

## Engineering Quality Notes

- All leaf tasks are atomic single-purpose operations
- GWT criteria are observable (file exists, count emitted, list populated)
- Dependencies form a DAG: T1 → T2 → T3 → T4 → T5
- No task exceeds 3 pts (well below the 5-pt expansion threshold)
- Each task has clear input contract (output of predecessor)
- Each task has clear output contract (consumable by successor)
- Anti-redundancy: T1.3 partition is single source of truth for valid/invalid split; downstream consumers reference it
