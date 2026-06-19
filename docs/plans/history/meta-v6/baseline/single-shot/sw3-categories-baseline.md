# Agentic Categorization

**Source document:** # Desired Output State - Based on task breakdown with 12 parent tasks and 27 leaf tasks
**Total tasks categorized:** 12 parents, 27 leaves
**Evaluation verdict:** PASS

---

## Category Summary

| Category | Count | Avg Iteration Budget |
|----------|-------|---------------------|
| SEQUENTIAL_PROCESSOR | 0 | N/A |
| PARALLEL_FAN_OUT | 0 | N/A |
| ITERATIVE_REFINER | 0 | N/A |
| CONDITIONAL_ROUTER | 0 | N/A |
| DATA_TRANSFORMER | 27 | 1.0 |
| ACCUMULATOR | 0 | N/A |

---

## T1 - Analyze duplicate README content

**Primary category:** DATA_TRANSFORMER
**Reasoning:** Each subtask performs a deterministic input→output conversion: read file → parsed structure, then parsed structures → overlap analysis. No branching, looping, or conditional routing.
**Substructure hint:** Single LLM step with save_to hook to store parsed output in memory or file
**Inputs needed:** File paths (requirements/index.md, mvp-summary-report.md)
**Outputs produced:** Parsed JSON structures (t1_1_parsed, t1_2_parsed), overlap-analysis.md file
**Evaluation scope:** Single-step check — verify output files exist with correct structure
**Iteration budget:** 1

### T1.1 - Read and parse requirements/index.md

**Primary category:** DATA_TRANSFORMER
**Reasoning:** Pure file read → JSON parse transformation. Single input (file path), single output (parsed structure). No conditional logic.
**Substructure hint:** Single LLM step with prompt to parse markdown, save_to hook storing JSON in memory variable
**Inputs needed:** requirements/index.md file path
**Outputs produced:** Memory variable `t1_1_parsed` (JSON with sections, summary, schema_ref)
**Evaluation scope:** Single-step check — verify JSON has required fields
**Iteration budget:** 1

### T1.2 - Read and parse mvp-summary-report.md

**Primary category:** DATA_TRANSFORMER
**Reasoning:** Pure file read → JSON parse transformation. Same pattern as T1.1, different input file.
**Substructure hint:** Single LLM step with prompt to parse markdown, save_to hook storing JSON in memory variable
**Inputs needed:** requirements/benchmark-100-model-userflows/mvp-summary-report.md file path
**Outputs produced:** Memory variable `t1_2_parsed` (JSON with sections, summary, has_schema_coverage)
**Evaluation scope:** Single-step check — verify JSON has required fields
**Iteration budget:** 1

### T1.3 - Document overlapping sections

**Primary category:** DATA_TRANSFORMER
**Reasoning:** Two JSON inputs → markdown comparison table output. Deterministic comparison logic with well-defined output schema.
**Substructure hint:** Single LLM step with prompt comparing two parsed structures, save_to hook writing overlap-analysis.md
**Inputs needed:** Memory variables t1_1_parsed, t1_2_parsed
**Outputs produced:** overlap-analysis.md file with comparison table
**Evaluation scope:** Single-step check — verify table has 10+ rows
**Iteration budget:** 1

---

## T2 - Execute README consolidation

**Primary category:** DATA_TRANSFORMER
**Reasoning:** Multi-file transformation pipeline: merge → delete → update references. Linear flow where each step's output feeds the next.
**Substructure hint:** Sequential chain: step_2_1 (merge) → step_2_2 (delete) → step_2_3 (update references), each with save_to + log hooks
**Inputs needed:** Overlap analysis from T1, file paths
**Outputs produced:** Modified mvp-summary-report.md, deleted/repurposed requirements/index.md, updated cross-reference files
**Evaluation scope:** Multi-step integration — verify end state (single source of truth)
**Iteration budget:** 1

### T2.1 - Create merged executive summary

**Primary category:** DATA_TRANSFORMER
**Reasoning:** Content synthesis: two inputs (unique content, target file) → single output (merged file). Deterministic markdown transformation.
**Substructure hint:** Single LLM step reading both files, writing merged content with save_to hook
**Inputs needed:** requirements/index.md (unique content), mvp-summary-report.md (target)
**Outputs produced:** Modified mvp-summary-report.md with new Executive Summary section
**Evaluation scope:** Single-step check — verify section exists at correct position
**Iteration budget:** 1

### T2.2 - Remove or repurpose requirements/index.md

**Primary category:** DATA_TRANSFORMER
**Reasoning:** Binary operation: either delete OR replace with directory listing. Single file operation with clear pre/post conditions.
**Substructure hint:** Single shell step executing `rm` or overwrite with template, log hook recording action taken
**Inputs needed:** Decision (delete vs repurpose), requirements/index.md path
**Outputs produced:** Either (a) deleted file OR (b) directory listing file
**Evaluation scope:** Single-step check — file existence test
**Iteration budget:** 1

### T2.3 - Update cross-references

**Primary category:** DATA_TRANSFORMER
**Reasoning:** Bulk search-and-replace: old path → new path across multiple files. Deterministic transformation with clear mapping.
**Substructure hint:** Single shell step with `find` + `sed` or `grep` + replace, log hook counting changes
**Inputs needed:** Old reference path, new reference path
**Outputs produced:** Modified files with updated references
**Evaluation scope:** Single-step check — grep returns 0 old references
**Iteration budget:** 1

---

## T3 - Analyze scattered testing guidance

**Primary category:** DATA_TRANSFORMER
**Reasoning:** Multi-file read → categorization matrix. Each subtask is independent read, final task aggregates and categorizes.
**Substructure hint:** Sequential chain: three independent read steps (no deps) → aggregation step (depends on all three), save_to hook for final output
**Inputs needed:** File paths for 3 testing docs
**Outputs produced:** testing-categorization.md matrix
**Evaluation scope:** Multi-step integration — verify matrix completeness
**Iteration budget:** 1

### T3.1 - Read testing-guide.md

**Primary category:** DATA_TRANSFORMER
**Reasoning:** File read → JSON summary transformation. Same pattern as T1.1, different file.
**Substructure hint:** Single LLM step with parse prompt, save_to hook storing t3_1_guide JSON
**Inputs needed:** guides/testing-guide.md path
**Outputs produced:** Memory variable `t3_1_guide` (JSON: sections, audience, summary)
**Evaluation scope:** Single-step check — JSON field validation
**Iteration budget:** 1

### T3.2 - Read workflows-README.md testing section

**Primary category:** DATA_TRANSFORMER
**Reasoning:** Section read → JSON summary transformation. Extracts specific lines 110-166.
**Substructure hint:** Single LLM step with parse prompt + line range, save_to hook storing t3_2_workflows JSON
**Inputs needed:** workflows/workflows-README.md path
**Outputs produced:** Memory variable `t3_2_workflows` (JSON: patterns, section_summary)
**Evaluation scope:** Single-step check — JSON field validation
**Iteration budget:** 1

### T3.3 - Read testing-strategy.md

**Primary category:** DATA_TRANSFORMER
**Reasoning:** File read → JSON summary transformation. Same pattern as T3.1, different file.
**Substructure hint:** Single LLM step with parse prompt, save_to hook storing t3_3_strategy JSON
**Inputs needed:** plans/testing-strategy.md path
**Outputs produced:** Memory variable `t3_3_strategy` (JSON: key_points, scope)
**Evaluation scope:** Single-step check — JSON field validation
**Iteration budget:** 1

### T3.4 - Categorize testing content overlap

**Primary category:** DATA_TRANSFORMER
**Reasoning:** Three JSON inputs → markdown matrix output. Categorization is deterministic based on predefined audience/scope values.
**Substructure hint:** Single LLM step reading three JSONs, writing matrix, save_to hook
**Inputs needed:** Memory variables t3_1_guide, t3_2_workflows, t3_3_strategy
**Outputs produced:** testing-categorization.md file with 3-row matrix
**Evaluation scope:** Single-step check — matrix dimensions (3×3)
**Iteration budget:** 1

---

## T4 - Refine testing guidance structure

**Primary category:** DATA_TRANSFORMER
**Reasoning:** File modifications + deletion with linear dependency flow. Each step produces clear file system changes.
**Substructure hint:** Sequential chain: step_4_1 (merge + delete) → step_4_2 (add cross-refs), each with log hook
**Inputs needed:** Categorization from T3, file paths
**Outputs produced:** Modified ARCHITECTURE.md, deleted testing-strategy.md, cross-referenced files
**Evaluation scope:** Multi-step integration — verify separation of concerns achieved
**Iteration budget:** 1

### T4.1 - Consolidate testing-strategy.md

**Primary category:** DATA_TRANSFORMER
**Reasoning:** Content merge + file deletion. Two inputs (strategy file, target file) → two outputs (merged target, deleted source).
**Substructure hint:** Single LLM step reading strategy content, appending to ARCHITECTURE.md, then shell step deleting strategy file, save_to hook on LLM step
**Inputs needed:** plans/testing-strategy.md, plans/ARCHITECTURE.md
**Outputs produced:** Modified plans/ARCHITECTURE.md, deleted plans/testing-strategy.md
**Evaluation scope:** Single-step check — file existence tests
**Iteration budget:** 1

### T4.2 - Add cross-references between testing docs

**Primary category:** DATA_TRANSFORMER
**Reasoning:** Bulk update: append "Related Documents" sections to three files. Deterministic append operation.
**Substructure hint:** Single LLM step reading three files, appending sections, save_to + log hooks
**Inputs needed:** guides/testing-guide.md, workflows/workflows-README.md, plans/ARCHITECTURE.md paths
**Outputs produced:** Three modified files with cross-reference sections
**Evaluation scope:** Single-step check — grep for "Related Documents" in all three
**Iteration budget:** 1

---

## T5 - Analyze schema reference gaps

**Primary category:** DATA_TRANSFORMER
**Reasoning:** Search → catalog → gap analysis. Linear transformation from grep results to gap documentation.
**Substructure hint:** Sequential chain: step_5_1 (grep catalog) → step_5_2 (gap analysis), save_to hooks
**Inputs needed:** docs/ directory path
**Outputs produced:** schema-reference-gaps.md, schema-nav-gaps.md
**Evaluation scope:** Multi-step integration — verify gaps documented
**Iteration budget:** 1

### T5.1 - Catalog schema references

**Primary category:** DATA_TRANSFORMER
**Reasoning:** Grep results → markdown table transformation. Deterministic extraction and formatting.
**Substructure hint:** Single shell step running grep, output piped to file; or LLM step parsing grep output into table, save_to hook
**Inputs needed:** docs/ directory
**Outputs produced:** schema-ref-catalog.md file with reference table
**Evaluation scope:** Single-step check — table row count (≥10)
**Iteration budget:** 1

### T5.2 - Document missing navigation

**Primary category:** DATA_TRANSFORMER
**Reasoning:** Reference catalog → gap analysis output. Deterministic identification of missing resources based on catalog patterns.
**Substructure hint:** Single LLM step reading catalog, identifying missing items, writing gap analysis, save_to hook
**Inputs needed:** schema-ref-catalog.md
**Outputs produced:** schema-nav-gaps.md file listing 4 missing resources
**Evaluation scope:** Single-step check — verify 4 gaps listed
**Iteration budget:** 1

---

## T6 - Create schema navigation hub

**Primary category:** DATA_TRANSFORMER
**Reasoning:** File creation + reference updates. Linear flow: create NAVIGATION.md → update READMEs to link to it.
**Substructure hint:** Sequential chain: step_6_1 (write content) → step_6_2 (update READMEs), save_to + log hooks
**Inputs needed:** Gap analysis from T5, schema file, three README paths
**Outputs produced:** schema/NAVIGATION.md, updated README files
**Evaluation scope:** Multi-step integration — verify hub exists and all READMEs link to it
**Iteration budget:** 1

### T6.1 - Write schema/NAVIGATION.md content

**Primary category:** DATA_TRANSFORMER
**Reasoning:** Gap analysis → navigation hub file creation. Deterministic 4-section markdown generation.
**Substructure hint:** Single LLM step reading schema file and gap analysis, writing 4-section NAVIGATION.md, save_to hook
**Inputs needed:** schema/unified-workflow-schema.yml, schema-nav-gaps.md
**Outputs produced:** New schema/NAVIGATION.md file
**Evaluation scope:** Single-step check — verify 4 sections exist
**Iteration budget:** 1

### T6.2 - Update READMEs to reference NAVIGATION.md

**Primary category:** DATA_TRANSFORMER
**Reasoning:** Bulk append: add link line to three README files. Deterministic single-line append per file.
**Substructure hint:** Single shell step with `for file in ...; do echo '...' >> $file; done`, log hook counting updates
**Inputs needed:** schema/README.md, roadmap/README.md, workflows/workflows-README.md paths
**Outputs produced:** Three modified README files with NAVIGATION.md links
**Evaluation scope:** Single-step check — grep for NAVIGATION.md in all three
**Iteration budget:** 1

---

## T7 - Analyze ADR metadata structure

**Primary category:** DATA_TRANSFORMER
**Reasoning:** Directory inspection → structure documentation. Single step analysis producing markdown summary.
**Substructure hint:** Single LLM step reading ADR files and metadata directory, writing structure analysis, save_to hook
**Inputs needed:** roadmap/ directory path
**Outputs produced:** adr-metadata-structure.md
**Evaluation scope:** Single-step check — verify document covers 3 metadata locations
**Iteration budget:** 1

### T7.1 - Document ADR metadata distribution

**Desired state:** Complete documentation of ADR metadata organization showing it's appropriately distributed
**Primary category:** DATA_TRANSFORMER
**Reasoning:** File reading → structure documentation. Deterministic extraction and summarization of metadata locations.
**Substructure hint:** Single LLM step reading ADR files and index, writing distribution document, save_to hook
**Inputs needed:** roadmap/adr-000*.yml, roadmap/adr-0000-roadmap-index.yml
**Outputs produced:** adr-metadata-distribution.md file
**Evaluation scope:** Single-step check — verify field counts match spec
**Iteration budget:** 1

---

## T8 - Create metadata README

**Primary category:** DATA_TRANSFORMER
**Reasoning:** Single file creation with structured content about ADR metadata organization.
**Substructure hint:** Single LLM step reading T7 analysis, writing README with 3 sections (rationale, directory contents, future enhancement), save_to hook
**Inputs needed:** adr-metadata-distribution.md from T7
**Outputs produced:** New roadmap/metadata/README.md file
**Evaluation scope:** Single-step check — verify file exists with 3 sections
**Iteration budget:** 1

---

## T9 - Analyze formatting status

**Primary category:** DATA_TRANSFORMER
**Reasoning:** Pattern extraction → file count calculation. Linear two-step flow.
**Substructure hint:** Sequential chain: step_9_1 (pattern extraction) → step_9_2 (file counting), save_to hooks
**Inputs needed:** docs/ directory, formatting status from prompt
**Outputs produced:** formatting-status.md, unformatted-count.md
**Evaluation scope:** Multi-step integration — verify patterns listed and count correct
**Iteration budget:** 1

### T9.1 - Document completed formatting patterns

**Primary category:** DATA_TRANSFORMER
**Reasoning:** Prompt extraction → pattern list with examples. Deterministic text extraction and formatting.
**Substructure hint:** Single LLM step reading prompt formatting section, extracting 5 patterns with examples, save_to hook
**Inputs needed:** Prompt content (formatting status section)
**Outputs produced:** applied-formats.md file
**Evaluation scope:** Single-step check — verify 5 patterns listed
**Iteration budget:** 1

### T9.2 - Count unformatted files

**Primary category:** DATA_TRANSFORMER
**Reasoning:** Directory scan → count calculation. Deterministic file count operation.
**Substructure hint:** Single shell step with `find docs/ -name '*.md' | wc -l` minus formatted count, log hook
**Inputs needed:** docs/ directory
**Outputs produced:** unformatted-count.md file with count (~260)
**Evaluation scope:** Single-step check — verify count matches 313 - formatted
**Iteration budget:** 1

---

## T10 - Prioritize consolidation roadmap

**Primary category:** DATA_TRANSFORMER
**Reasoning:** Priority calculation → phased schedule. Linear two-step flow: score tasks → assign to phases.
**Substructure hint:** Sequential chain: step_10_1 (prioritization) → step_10_2 (scheduling), save_to hooks
**Inputs needed:** Tasks T1-T8 with impact/effort data
**Outputs produced:** priority-sorted.md, phased-roadmap.md
**Evaluation scope:** Multi-step integration — verify phases have correct tasks
**Iteration budget:** 1

### T10.1 - Prioritize by impact/effort ratio

**Primary category:** DATA_TRANSFORMER
**Reasoning:** Task list → prioritized table with scores. Deterministic calculation (impact ÷ effort).
**Substructure hint:** Single LLM step reading tasks with impact/effort, calculating scores, writing sorted table, save_to hook
**Inputs needed:** Task data (T1-T8)
**Outputs produced:** priority-sorted.md file with table sorted by score descending
**Evaluation scope:** Single-step check — verify high-priority tasks at top
**Iteration budget:** 1

### T10.2 - Create phased schedule

**Primary category:** DATA_TRANSFORMER
**Reasoning:** Prioritized tasks → phased roadmap. Deterministic assignment based on priority tiers.
**Substructure hint:** Single LLM step reading priority-sorted.md, assigning tasks to 3 phases, writing roadmap, save_to hook
**Inputs needed:** priority-sorted.md
**Outputs produced:** phased-roadmap.md file with 3 phases
**Evaluation scope:** Single-step check — verify phases have correct tasks
**Iteration budget:** 1

---

## T11 - Create cross-directory overlap matrix

**Primary category:** DATA_TRANSFORMER
**Reasoning:** Prompt matrix → formatted table. Two-step linear flow: extract mapping → format table.
**Substructure hint:** Sequential chain: step_11_1 (CSV extraction) → step_11_2 (markdown table), save_to hooks
**Inputs needed:** Prompt overlap matrix section
**Outputs produced:** topic-mapping.csv, overlap-matrix.md
**Evaluation scope:** Multi-step integration — verify table dimensions (10×8)
**Iteration budget:** 1

### T11.1 - Extract topic-directory mapping

**Primary category:** DATA_TRANSFORMER
**Reasoning:** Prompt table → CSV transformation. Deterministic extraction of cell contents.
**Substructure hint:** Single LLM step reading prompt matrix, extracting all non-empty cells as (topic, directory, file) tuples, writing CSV, save_to hook
**Inputs needed:** Prompt "## 📋 Cross-Directory Overlap Matrix" section
**Outputs produced:** topic-mapping.csv file
**Evaluation scope:** Single-step check — verify 20+ rows
**Iteration budget:** 1

### T11.2 - Format as markdown table

**Primary category:** DATA_TRANSFORMER
**Reasoning:** CSV → markdown table transformation. Deterministic formatting with proper headers.
**Substructure hint:** Single LLM step reading CSV, formatting as 10×8 markdown table with headers, save_to hook
**Inputs needed:** topic-mapping.csv
**Outputs produced:** overlap-matrix.md file
**Evaluation scope:** Single-step check — verify table dimensions (10 topics × 8 directories = 80 cells)
**Iteration budget:** 1

---

## T12 - Document automation recommendations

**Primary category:** DATA_TRANSFORMER
**Reasoning:** Long-term considerations → recommendations document. Linear flow: write 4 recommendation sections.
**Substructure hint:** Sequential chain of 4 steps (one per recommendation), each writing to same recommendations.md with append_to hook
**Inputs needed:** Prompt "## 🚀 Recommendations for Next Steps" section
**Outputs produced:** automation-recommendations.md with 4 sections
**Evaluation scope:** Multi-step integration — verify all 4 sections exist
**Iteration budget:** 1

### T12.1 - Write link validation recommendation

**Primary category:** DATA_TRANSFORMER
**Reasoning:** Prompt consideration → single recommendation section. Deterministic text generation.
**Substructure hint:** Single LLM step reading prompt link validation section, writing 2-3 sentence recommendation + hint, save_to hook
**Inputs needed:** Prompt "Automated link validation" consideration
**Outputs produced:** "## Automated Link Validation" section in recommendations.md
**Evaluation scope:** Single-step check — verify section exists with hint
**Iteration budget:** 1

### T12.2 - Write duplicate detection recommendation

**Primary category:** DATA_TRANSFORMER
**Reasoning:** Prompt consideration → single recommendation section. Deterministic text generation.
**Substructure hint:** Single LLM step reading prompt duplicate detection section, writing 2-3 sentence recommendation + hint, save_to hook
**Inputs needed:** Prompt "Duplicate content detection" consideration
**Outputs produced:** "## Duplicate Content Detection" section in recommendations.md
**Evaluation scope:** Single-step check — verify section exists with hint
**Iteration budget:** 1

### T12.3 - Write metrics and CI recommendations

**Primary category:** DATA_TRANSFORMER
**Reasoning:** Prompt considerations → two recommendation sections. Deterministic text generation.
**Substructure hint:** Single LLM step reading prompt metrics and CI sections, writing two 2-3 sentence recommendations + hints, save_to hook
**Inputs needed:** Prompt "Metrics tracking" and "Doc CI checks" considerations
**Outputs produced:** "## Metrics Tracking" and "## Doc CI Checks" sections in recommendations.md
**Evaluation scope:** Single-step check — verify both sections exist with hints
**Iteration budget:** 1