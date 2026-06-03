# Example Complex Prompts for Meta-Workflow Generation

These prompts are designed to test the meta-workflow generator's ability to produce
valid, multi-step YAML agentic workflows from natural language descriptions.

## Prompt 1: Data Quality Pipeline

Create a multi-step data quality assessment pipeline that reads a CSV file containing
customer transaction records with columns for transaction_id, customer_id, amount,
date, merchant_category, and fraud_flag. The pipeline should: (1) Load the CSV and
validate that all required columns exist and contain no null values in transaction_id
or amount fields. (2) Compute summary statistics including total transaction count,
average amount, median amount, and count of transactions above $500. (3) Identify
duplicate transaction_ids and flag them in a report. (4) Detect anomalous amounts
that are more than 3 standard deviations from the mean. (5) Generate a final quality
report with pass/fail status for each check and save it to a file. Each step should
validate its input from the previous step before proceeding.

## Prompt 2: API Contract Testing Suite

Build an automated API contract testing workflow that takes a base URL and a list of
endpoint definitions, then for each endpoint: (1) Sends a properly formatted request
with required headers and validates the response status code matches the contract.
(2) Checks that response body structure matches the expected schema including nested
objects and arrays. (3) Tests edge cases including missing required fields, invalid
data types, and boundary values. (4) Measures response latency and flags any endpoint
responding slower than 2 seconds. (5) Compiles results into a structured test report
with pass/fail per endpoint, lists all schema mismatches with expected vs actual values,
and includes a summary of performance characteristics. The workflow should continue
testing remaining endpoints even if one fails.

## Prompt 3: Code Review Automation

Design a code review automation workflow that analyzes a Python source file and produces
a comprehensive review report. Step 1: Parse the file and extract all function signatures,
class definitions, and import statements. Step 2: For each function, evaluate code
complexity by counting branching statements, nesting depth, and function length. Flag
any function exceeding 50 lines or nesting depth of 4. Step 3: Check for common
anti-patterns including bare except clauses, mutable default arguments, unused imports,
and functions with more than 5 parameters. Step 4: Generate a summary showing total
functions analyzed, complexity distribution (simple/moderate/complex), list of flagged
issues with line references, and specific refactoring suggestions. Step 5: Output the
final review as a structured report file.

## Prompt 4: Multi-Source Data Integration

Create a data integration workflow that merges customer data from three different
sources: (1) A CSV file with basic customer info (id, name, email, signup_date).
(2) A JSON file with purchase history (customer_id, purchases array with product_id,
amount, date). (3) A second CSV with support tickets (ticket_id, customer_id,
category, resolution_status, satisfaction_score). The workflow should load each source,
validate data integrity (no duplicate IDs, valid email formats, amounts are positive),
join all three on customer_id, compute derived metrics (total_spend, purchase_frequency,
avg_satisfaction, ticket_count), flag customers with low satisfaction AND high ticket
count as "at_risk", and produce a final integrated dataset sorted by total_spend
descending with an at_risk column.

## Prompt 5: Infrastructure Compliance Audit

Build an infrastructure compliance audit workflow that examines system configuration
files and validates them against security best practices. Step 1: Read a Docker
Compose file and check that no containers run as root, all images use specific tags
(not latest), and no privileged mode is enabled. Step 2: Read an Nginx config file
and verify SSL is enabled, HTTP redirects to HTTPS, server_tokens are off, and
security headers (X-Frame-Options, X-Content-Type-Options, Content-Security-Policy)
are present. Step 3: Read environment variable files and flag any that contain
patterns matching passwords, API keys, or secrets (look for common key names like
PASSWORD, SECRET, TOKEN, KEY combined with non-placeholder values). Step 4: Produce
a compliance report with pass/fail per check, severity levels (critical/warning/info),
and specific remediation steps for each failure.

## Prompt 6: Documentation Generation Pipeline

Create a documentation generation pipeline that takes a Rust source file and produces
comprehensive documentation. Step 1: Parse the file extracting all pub functions,
struct definitions, trait implementations, and module declarations. Step 2: For each
public function, analyze the signature to determine parameter types, return type,
and any generic constraints. Step 3: Generate structured documentation including
a module overview, function reference with parameter descriptions and return value
explanations, type hierarchy showing struct relationships, and usage examples derived
from the function signatures. Step 4: Cross-reference types mentioned in function
signatures with their definitions elsewhere in the file. Step 5: Output a markdown
documentation file with proper headers, code blocks, and a table of contents.

## Prompt 7: Log Analysis and Anomaly Detection

Design a log analysis workflow that processes application log files to identify
patterns and anomalies. Step 1: Read a log file and parse each line extracting
timestamp, log level, service name, and message. Step 2: Compute frequency
distributions for log levels (ERROR, WARN, INFO, DEBUG) per hour. Step 3: Identify
error spikes where the ERROR count in any 5-minute window exceeds 3x the average
ERROR rate. Step 4: Extract and group error messages by similarity, identifying
the top 5 most frequent error patterns. Step 5: Check for specific concerning
patterns including repeated connection failures, authentication errors from the
same IP, and out-of-memory events. Step 6: Generate a summary report with
anomaly timeline, error pattern breakdown, and recommended investigation priorities.

## Prompt 8: Configuration Migration Validator

Build a configuration migration validator that compares old and new versions of a
YAML configuration file to ensure no critical settings are lost during migration.
Step 1: Parse both YAML files into structured key-value maps, flattening nested
structures into dot-notation paths. Step 2: Identify keys present in the old config
but missing from the new config. Step 3: For keys present in both, compare values
and flag any that changed type (string to number, etc.) or changed significantly
(port numbers, hostnames, boolean toggles). Step 4: Validate that all new keys
have corresponding documentation or are recognized as valid schema fields. Step 5:
Produce a migration report listing removed keys, changed values with old vs new,
new undocumented keys, and an overall migration safety score from 0-100 based on
the number and severity of differences.

## Prompt 9: Test Coverage Analyzer

Create a test coverage analysis workflow that examines a test results file and
source code to provide actionable coverage insights. Step 1: Parse a test results
XML file (JUnit format) extracting test names, statuses (pass/fail/skip), durations,
and failure messages. Step 2: Parse the corresponding source code file listing all
public functions and methods. Step 3: Match test names to source functions using
naming conventions (test_ prefix, _test suffix, or CamelCase test method names).
Step 4: Identify uncovered functions that have no corresponding test. Step 5:
Analyze test quality by checking if failing tests have meaningful assertion messages.
Step 6: Produce a coverage matrix showing function-to-test mapping, uncovered
functions list, test quality scores, and recommendations for which functions need
tests most urgently based on complexity and usage patterns.

## Prompt 10: Release Readiness Checklist

Build a release readiness assessment workflow that checks multiple criteria before
allowing a software release. Step 1: Verify all unit tests pass by reading test
output. Step 2: Check that the changelog file has been updated with entries for
the new version. Step 3: Validate that version numbers are consistent across
Cargo.toml, package.json, and any config files. Step 4: Scan for TODO or FIXME
comments in source files and report them. Step 5: Verify no debug-only code
remains (console.log, println!, dbg! macros). Step 6: Check that no files with
common secret patterns are staged for commit. Step 7: Generate a release readiness
report with go/no-go decision, listing all passing and failing checks with specific
details on what needs to be resolved before release can proceed.
