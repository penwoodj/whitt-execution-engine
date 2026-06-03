# Aggregate Complex Prompts for Live Testing

These three prompts combine elements from the example prompts into genuinely complex,
multi-phase workflows designed to stress-test the meta-workflow generator.

---

## AGGREGATE PROMPT A: Full-Spectrum Data Pipeline with Quality Gates

Read the file at /tmp/pipeline-input.csv which contains sales transaction data with
columns: transaction_id, customer_email, product_sku, quantity, unit_price, sale_date,
sales_region, payment_method. Execute a complete data pipeline in these phases:

PHASE 1 - INGESTION AND VALIDATION: Load the CSV file. Validate that transaction_id
is unique across all rows. Validate that customer_email matches standard email format
(containing @ and a domain). Validate that quantity is a positive integer and unit_price
is a positive number. Validate that sale_date follows YYYY-MM-DD format. Count and report
how many rows fail each validation check.

PHASE 2 - ENRICHMENT AND COMPUTATION: For each valid transaction, compute the total_amount
as quantity multiplied by unit_price. Classify transactions into size buckets: micro
(under 25), small (25-100), medium (100-500), large (500-1000), enterprise (over 1000).
Determine the most common payment_method per sales_region.

PHASE 3 - ANOMALY DETECTION: Identify transactions where total_amount is more than 3
standard deviations above the mean. Identify customers with more than 10 transactions
in a single day. Flag any sales_region with an average total_amount that differs by more
than 50 percent from the overall average.

PHASE 4 - REPORT GENERATION: Produce a structured report containing: total valid and
invalid row counts from Phase 1, transaction size distribution from Phase 2, complete
list of anomalous transactions with reasons from Phase 3, and a final data quality score
calculated as valid_rows divided by total_rows multiplied by 100. Save the report to
outputs/output/pipeline-report.txt.

---

## AGGREGATE PROMPT B: Configuration Drift Detector with Remediation Plan

Read two YAML configuration files: /tmp/config-baseline.yml and /tmp/config-current.yml.
These represent the expected baseline configuration and the current live configuration
for a web service. Perform a comprehensive drift analysis:

PHASE 1 - STRUCTURAL PARSE: Parse both YAML files into flat key-value maps using
dot-notation for nested keys (e.g., database.connection.pool_size). List all unique
keys across both files.

PHASE 2 - DIFF ANALYSIS: Categorize every key into one of four states: present in both
with identical values (unchanged), present in both with different values (modified),
present only in baseline (removed), present only in current (added). For modified keys,
capture both the old and new values.

PHASE 3 - RISK ASSESSMENT: Classify each difference by severity. Critical: any change
to security-related keys (ssl, auth, token, secret, password, encryption). Warning:
changes to performance-related keys (timeout, pool_size, cache, buffer, workers). Info:
changes to cosmetic or logging keys (log_level, debug, verbose). Compute a drift score
as the weighted sum: critical changes count times 10 plus warning changes count times 3
plus info changes count times 1.

PHASE 4 - REMEDIATION PLAN: For each critical and warning difference, generate a
specific remediation instruction stating which key needs to change, what the current
value is, and what the baseline value should be. Order remediation steps by severity
with critical first. Save the complete drift report including the diff analysis, risk
assessment, and remediation plan to outputs/output/drift-report.txt.

---

## AGGREGATE PROMPT C: Multi-Pass Code Quality Auditor with Trend Analysis

Read the source code file at /tmp/audit-target.py which contains a Python module with
multiple functions and classes. Perform a comprehensive quality audit across multiple
dimensions:

PHASE 1 - STRUCTURAL ANALYSIS: Extract all function and method signatures including
parameter names, default values, and return type annotations. Count total functions,
total classes, total methods per class, and identify the longest function by line count.
Flag any function exceeding 40 lines.

PHASE 2 - COMPLEXITY ANALYSIS: For each function, count decision points: if statements,
elif clauses, for loops, while loops, try-except blocks, and boolean operators (and, or).
Compute a cyclomatic complexity score as decision points plus 1. Classify functions as
simple (1-5), moderate (6-10), complex (11-20), or critical (over 20).

PHASE 3 - PATTERN DETECTION: Scan for these specific anti-patterns: bare except clauses
without specific exception types, mutable default arguments like empty lists or dicts
as parameter defaults, functions with more than 5 parameters, import statements inside
function bodies, and string concatenation in loops instead of join. Report each finding
with the function name and a description of the issue.

PHASE 4 - QUALITY SCORING: Assign an overall quality grade A through F based on:
percentage of functions under 20 lines (weight 25 percent), average cyclomatic complexity
(weight 25 percent), anti-pattern density per 100 lines (weight 25 percent), and type
annotation coverage (weight 25 percent). Produce a final audit report with per-function
complexity table, anti-pattern listing, grade calculation showing each component score,
and top 3 recommended refactorings with the highest impact. Save the complete audit
report to outputs/output/audit-report.txt.
