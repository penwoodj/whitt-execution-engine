# Desired Output State

**Source task breakdown:** # 🔍 Documentation Consolidation Analysis - Based on systematic review...
**Total tasks covered:** 12
**Total leaves covered:** 27
**Evaluation verdict:** PASS

---

## T1 - Analyze duplicate README content

**Desired state:** Overlap analysis document showing exact duplicate text ranges between requirements/index.md and mvp-summary-report.md with line ranges and unique content per file
**Artifacts produced:** `docs/plans/meta-v6/baseline/overlap-analysis.md` (markdown with side-by-side comparison)
**Acceptance criteria:**
  - Document lists all section headers from both files
  - Identifies specific text passages that are identical (with line ranges)
  - Marks unique content per file
  - Provides overlap percentage calculation
**Evaluation method:** Manual inspection of overlap-analysis.md content, verify line ranges are accurate
**Downstream dependencies:** T2 (uses overlap analysis for consolidation decision)
**Failure indicators:**
  - Overlap percentages don't sum to 100%
  - Line ranges point to wrong sections
  - No unique content identified

### T1.1 - Read and parse requirements/index.md

**Desired state:** Parsed structure of requirements/index.md with section list, content summary, and reference to schema coverage
**Artifacts produced:** Memory variable `t1_1_parsed` (JSON: {sections: [], summary: "", schema_ref: ""})
**Acceptance criteria:**
  - `sections` array contains all markdown headers from file
  - `summary` string is <200 chars describing file purpose
  - `schema_ref` string contains the exact quote about "279 requirements" if present
**Evaluation method:** Unit test validates JSON structure has required fields
**Downstream dependencies:** T1.3 (uses parsed content for comparison)
**Failure indicators:**
  - `sections` array is empty
  - `summary` exceeds 200 chars
  - `schema_ref` missing when file contains reference

### T1.2 - Read and parse mvp-summary-report.md

**Desired state:** Parsed structure of mvp-summary-report.md with section list, content summary, and schema coverage report confirmation
**Artifacts produced:** Memory variable `t1_2_parsed` (JSON: {sections: [], summary: "", has_schema_coverage: bool})
**Acceptance criteria:**
  - `sections` array contains all markdown headers from file
  - `summary` string is <200 chars describing file purpose
  - `has_schema_coverage` is true if "Schema Coverage Report" section exists
**Evaluation method:** Unit test validates JSON structure and boolean flag
**Downstream dependencies:** T1.3 (uses parsed content for comparison)
**Failure indicators:**
  - `sections` array is empty
  - `has_schema_coverage` is false when section exists

### T1.3 - Document overlapping sections

**Desired state:** Overlap analysis showing exact duplicate passages between the two README files with line ranges
**Artifacts produced:** File `overlap-analysis.md` (markdown with comparison table)
**Acceptance criteria:**
  - Table columns: Section, File A Line Range, File B Line Range, Overlap Status (identical/similar/unique)
  - At least 10 rows of comparison data
  - Summary paragraph with overlap percentage
**Evaluation method:** Manual inspection of table formatting and count
**Downstream dependencies:** T2 (uses overlap analysis for merge planning)
**Failure indicators:**
  - Table has <10 rows
  - Overlap percentages missing
  - No identical sections marked

---

## T2 - Execute README consolidation

**Desired state:** Consolidated README at mvp-summary-report.md with no duplicate content at requirements/index.md, all cross-references updated
**Artifacts produced:** Modified `mvp-summary-report.md`, deleted or repurposed `requirements/index.md`, updated cross-reference files
**Acceptance criteria:**
  - `mvp-summary-report.md` has new "## Executive Summary" section
  - `requirements/index.md` either deleted OR contains only directory listing
  - `grep` returns 0 results for old reference path
**Evaluation method:** File existence check, content inspection, grep command
**Downstream dependencies:** T3 (continues to other consolidation areas)
**Failure indicators:**
  - Duplicate content still exists at both locations
  - Old references still present
  - mvp-summary-report.md lacks executive summary

### T2.1 - Create merged executive summary

**Desired state:** mvp-summary-report.md has "## Executive Summary" section containing unique content from requirements/index.md
**Artifacts produced:** Modified `mvp-summary-report.md`
**Acceptance criteria:**
  - "## Executive Summary" is first header (after document title)
  - Section contains unique index/navigation content from requirements/index.md
  - No duplicate content with existing mvp-summary-report.md sections
**Evaluation method:** File read, verify section position and content uniqueness
**Downstream dependencies:** T2.2 (deletes original after merge)
**Failure indicators:**
  - Executive summary missing
  - Duplicate content detected
  - Section at wrong position

### T2.2 - Remove or repurpose requirements/index.md

**Desired state:** requirements/index.md either deleted OR contains only simple directory listing (no duplicate content)
**Artifacts produced:** Either (a) deleted file OR (b) modified file with directory listing only
**Acceptance criteria:**
  - File either does NOT exist OR contains only markdown bullet list of subdirectories
  - File does NOT contain any prose content from original
  - File does NOT reference schema coverage report
**Evaluation method:** File existence check or content grep for prose patterns
**Downstream dependencies:** T2.3 (updates references after cleanup)
**Failure indicators:**
  - File exists with duplicate prose content
  - File contains schema coverage reference

### T2.3 - Update cross-references

**Desired state:** All files that referenced requirements/index.md now reference mvp-summary-report.md
**Artifacts produced:** Modified files with updated reference paths
**Acceptance criteria:**
  - `grep -r "requirements/index.md" docs/` returns 0 matches
  - At least one file now references `requirements/benchmark-100-model-userflows/mvp-summary-report.md`
**Evaluation method:** grep command for old and new references
**Downstream dependencies:** None (final step of T2)
**Failure indicators:**
  - Old references still found
  - No new references added

---

## T3 - Analyze scattered testing guidance

**Desired state:** Categorization matrix showing developer-facing, workflow-specific, and strategy-level testing content across 4 locations
**Artifacts produced:** File `testing-categorization.md` (markdown matrix)
**Acceptance criteria:**
  - Matrix rows: guides/testing-guide.md, workflows/workflows-README.md (Testing Guidance section), plans/testing-strategy.md
  - Matrix columns: Audience (developer/workflow/planner), Scope (how-to/patterns/strategy), Unique Content (bullets)
  - All 3 files categorized with non-empty content
**Evaluation method:** Manual inspection of matrix completeness
**Downstream dependencies:** T4 (uses categorization for refinement)
**Failure indicators:**
  - Matrix has missing rows
  - Any row has empty "Unique Content"
  - Audience column has invalid values

### T3.1 - Read testing-guide.md

**Desired state:** Summary of testing-guide.md content with sections and intended audience
**Artifacts produced:** Memory variable `t3_1_guide` (JSON: {sections: [], audience: "", summary: ""})
**Acceptance criteria:**
  - `sections` lists all markdown headers
  - `audience` is "developer" or "workflow-author" or "planner"
  - `summary` is <150 chars
**Evaluation method:** JSON validation for required fields
**Downstream dependencies:** T3.4 (uses for categorization)
**Failure indicators:**
  - Missing fields
  - Invalid audience value
  - Summary too long

### T3.2 - Read workflows-README.md testing section

**Desired state:** Summary of Testing Guidance section (lines 110-166) with testing patterns
**Artifacts produced:** Memory variable `t3_2_workflows` (JSON: {patterns: [], section_summary: ""})
**Acceptance criteria:**
  - `patterns` array lists testing pattern names (validation, functional, integration, feature-specific)
  - `section_summary` is <150 chars
**Evaluation method:** JSON validation
**Downstream dependencies:** T3.4 (uses for categorization)
**Failure indicators:**
  - `patterns` array empty
  - Missing section_summary

### T3.3 - Read testing-strategy.md

**Desired state:** Summary of testing-strategy.md high-level strategy content
**Artifacts produced:** Memory variable `t3_3_strategy` (JSON: {key_points: [], scope: ""})
**Acceptance criteria:**
  - `key_points` array lists 2-5 strategy bullet points
  - `scope` is "automation" or "process" or "framework"
**Evaluation method:** JSON validation
**Downstream dependencies:** T3.4 (uses for categorization)
**Failure indicators:**
  - `key_points` array has <2 or >5 items
  - Invalid scope value

### T3.4 - Categorize testing content overlap

**Desired state:** Categorization matrix showing unique vs shared content per testing documentation source
**Artifacts produced:** File `testing-categorization.md` (markdown table)
**Acceptance criteria:**
  - 3 rows (one per source), 3 columns (audience, scope, unique_content)
  - `unique_content` lists specific topics only in that source
  - Shared topics noted in overlap section
**Evaluation method:** Table inspection, row/column count verification
**Downstream dependencies:** T4 (uses matrix for separation decisions)
**Failure indicators:**
  - Matrix missing rows or columns
  - All unique_content lists empty

---

## T4 - Refine testing guidance structure

**Desired state:** Clear separation of concerns with guides/testing-guide.md (developer), workflows/workflows-README.md (patterns), plans/ARCHITECTURE.md (strategy), all cross-referenced
**Artifacts produced:** Modified guides/testing-guide.md, modified workflows/workflows-README.md, modified plans/ARCHITECTURE.md, deleted plans/testing-strategy.md
**Acceptance criteria:**
  - Each of 3 files has "## Related Documents" section
  - testing-strategy.md no longer exists
  - Each "Related Documents" section links to the other 2 files
**Evaluation method:** File existence checks, grep for "Related Documents" sections
**Downstream dependencies:** T5 (moves to schema analysis)
**Failure indicators:**
  - testing-strategy.md still exists
  - Missing "Related Documents" sections
  - Broken links in cross-references

### T4.1 - Consolidate testing-strategy.md

**Desired state:** Strategy content moved to plans/ARCHITECTURE.md, testing-strategy.md deleted with cross-reference added
**Artifacts produced:** Modified plans/ARCHITECTURE.md, deleted plans/testing-strategy.md
**Acceptance criteria:**
  - ARCHITECTURE.md has new section "## Testing Strategy" with unique content from testing-strategy.md
  - testing-strategy.md does not exist
  - ARCHITECTURE.md cross-references testing-guide.md and workflows-README.md
**Evaluation method:** File existence check, grep for strategy content
**Downstream dependencies:** T4.2 (adds remaining cross-references)
**Failure indicators:**
  - testing-strategy.md still exists
  - ARCHITECTURE.md missing testing strategy section
  - Missing cross-references

### T4.2 - Add cross-references between testing docs

**Desired state:** All three testing docs have bidirectional cross-references in "## Related Documents" sections
**Artifacts produced:** Modified guides/testing-guide.md, workflows/workflows-README.md, plans/ARCHITECTURE.md
**Acceptance criteria:**
  - Each file has "## Related Documents" section with links to the other 2
  - All links use proper markdown syntax
  - No broken links
**Evaluation method:** Grep for "Related Documents", count links per file
**Downstream dependencies:** None (final step of T4)
**Failure indicators:**
  - Any file missing "Related Documents"
  - Any file has <2 links in that section

---

## T5 - Analyze schema reference gaps

**Desired state:** Document showing all files reference schema independently with no centralized navigation hub
**Artifacts produced:** File `schema-reference-gaps.md` (markdown)
**Acceptance criteria:**
  - Lists all files with schema references (grep results)
  - Documents missing: quick start guide, element reference, navigation shortcuts
  - Each reference shows context (how it's referenced)
**Evaluation method:** File content inspection, grep verification
**Downstream dependencies:** T6 (creates navigation hub)
**Failure indicators:**
  - Missing reference catalog
  - No documentation of gaps
  - Reference count doesn't match grep

### T5.1 - Catalog schema references

**Desired state:** Complete list of all files referencing unified-workflow-schema.yml with context
**Artifacts produced:** File `schema-ref-catalog.md` (markdown table)
**Acceptance criteria:**
  - Table columns: File, Reference Pattern, Context (line snippet)
  - At least 10 rows (prompt shows multiple locations reference schema)
  - Reference patterns grouped by type (bare path, descriptive text, link)
**Evaluation method:** Table row count, grep verification
**Downstream dependencies:** T5.2 (analyzes patterns for gaps)
**Failure indicators:**
  - Table has <10 rows
  - Missing context column
  - No pattern grouping

### T5.2 - Document missing navigation

**Desired state:** Analysis documenting absence of centralized schema navigation resources
**Artifacts produced:** File `schema-nav-gaps.md` (markdown)
**Acceptance criteria:**
  - Lists 4 missing resources: quick start, element reference, cross-reference matrix, navigation shortcuts
  - Each resource has 2-sentence description of what it should provide
  - Concludes that current reference pattern is ad-hoc
**Evaluation method:** Count missing resources (must be 4)
**Downstream dependencies:** T6 (creates navigation hub)
**Failure indicators:**
  - <4 missing resources listed
  - No descriptions
  - No conclusion about ad-hoc pattern

---

## T6 - Create schema navigation hub

**Desired state:** New schema/NAVIGATION.md with 4 sections, all READMEs link to it as schema entry point
**Artifacts produced:** New file schema/NAVIGATION.md, modified schema/README.md, modified roadmap/README.md, modified workflows/workflows-README.md
**Acceptance criteria:**
  - NAVIGATION.md has sections: Quick Start, Element Reference, Cross-Reference Matrix, Navigation Shortcuts
  - Each section is non-empty
  - All 3 READMEs link to NAVIGATION.md
**Evaluation method:** File existence check, section count verification, grep for links
**Downstream dependencies:** T7 (moves to ADR metadata)
**Failure indicators:**
  - NAVIGATION.md missing or has <4 sections
  - Any README missing link to NAVIGATION.md

### T6.1 - Write schema/NAVIGATION.md content

**Desired state:** New schema/NAVIGATION.md file with 4 comprehensive navigation sections
**Artifacts produced:** New file schema/NAVIGATION.md
**Acceptance criteria:**
  - "## Quick Start" section has 3-5 bullet points for schema newcomers
  - "## Element Reference" section lists step types, loop types, validation strategies
  - "## Cross-Reference Matrix" has table: Element → Schema Section → Example Workflow
  - "## Navigation Shortcuts" has 3-5 links to related docs
**Evaluation method:** Section count (4), bullet count per section
**Downstream dependencies:** T6.2 (adds links from READMEs)
**Failure indicators:**
  - Missing any of 4 sections
  - Any section empty
  - Element Reference lacks specific types

### T6.2 - Update READMEs to reference NAVIGATION.md

**Desired state:** Three README files link to schema/NAVIGATION.md as primary schema navigation entry point
**Artifacts produced:** Modified schema/README.md, roadmap/README.md, workflows/workflows-README.md
**Acceptance criteria:**
  - Each README has line: "For comprehensive schema navigation, see: [[../schema/NAVIGATION.md]]"
  - All 3 files contain the link
  - Link uses correct relative path
**Evaluation method:** Grep for NAVIGATION.md in all 3 files
**Downstream dependencies:** None (final step of T6)
**Failure indicators:**
  - Any README missing the link
  - Incorrect relative path

---

## T7 - Analyze ADR metadata structure

**Desired state:** Document showing ADR metadata is appropriately distributed across individual files, master index, and metadata directory
**Artifacts produced:** File `adr-metadata-structure.md` (markdown)
**Acceptance criteria:**
  - Describes 3 metadata locations: individual ADR files, adr-0000-roadmap-index.yml, roadmap/metadata/
  - Lists metadata fields per location
  - Concludes structure is appropriate
**Evaluation method:** Content inspection
**Downstream dependencies:** T8 (creates metadata README)
**Failure indicators:**
  - <3 locations documented
  - Missing field lists
  - No conclusion

### T7.1 - Document ADR metadata distribution

**Desired state:** Complete documentation of ADR metadata organization showing it's appropriately distributed
**Artifacts produced:** File `adr-metadata-distribution.md` (markdown)
**Acceptance criteria:**
  - Lists individual ADR metadata fields (adr_id, title, status, date, context, decision, consequences)
  - Lists master index fields (links, execution order, dependency mapping, quality gates)
  - Describes roadmap/metadata/ directory purpose
  - Concludes distribution is intentional and appropriate
**Evaluation method:** Field count verification (must match spec)
**Downstream dependencies:** None (parent T7 complete)
**Failure indicators:**
  - Missing any metadata location
  - Field counts don't match spec
  - No appropriateness conclusion

---

## T8 - Create metadata README

**Desired state:** New roadmap/metadata/README.md explaining ADR metadata organization
**Artifacts produced:** New file roadmap/metadata/README.md
**Acceptance criteria:**
  - Explains why metadata is distributed (self-documenting ADRs, coordination via index, version history)
  - Lists files in metadata/ directory and their purpose
  - Notes future enhancement possibility (single metadata YAML if directory grows)
**Evaluation method:** File existence check, content inspection
**Downstream dependencies:** T9 (moves to formatting analysis)
**Failure indicators:**
  - File missing
  - No explanation of distribution rationale
  - No future enhancement note

---

## T9 - Analyze formatting status

**Desired state:** Summary of completed formatting patterns and count of unformatted files
**Artifacts produced:** File `formatting-status.md` (markdown)
**Acceptance criteria:**
  - Lists 5 applied patterns: emoji headings, Foam syntax, bidirectional navigation, cross-directory links, TOC (noted as inconsistent)
  - Documents 50+ files formatted, ~260 remaining
  - Shows link density table from prompt
**Evaluation method:** Pattern count (must be 5), file count verification
**Downstream dependencies:** T10 (uses for roadmap)
**Failure indicators:**
  - <5 patterns listed
  - Missing unformatted count

### T9.1 - Document completed formatting patterns

**Desired state:** List of 5 formatting patterns that have been applied to 50+ files
**Artifacts produced:** File `applied-formats.md` (markdown)
**Acceptance criteria:**
  - Lists: emoji headings (🔗📋🎯🛠️⚙️🧪), Foam wiki-link syntax, bidirectional navigation ("See Also"/"Related"), cross-directory links, table of contents (noted inconsistent)
  - Each pattern has 1-sentence example
**Evaluation method:** Count patterns (must be 5)
**Downstream dependencies:** T9.2 (counts remaining files)
**Failure indicators:**
  - <5 patterns
  - Missing examples

### T9.2 - Count unformatted files

**Desired state:** Accurate count of markdown files in docs/ that lack Foam links
**Artifacts produced:** File `unformatted-count.md` (markdown)
**Acceptance criteria:**
  - Documents count: 260 (313 total - 53 formatted)
  - Shows verification method (find command or similar)
  - Lists directories with most unformatted files
**Evaluation method:** Count verification
**Downstream dependencies:** None (parent T9 complete)
**Failure indicators:**
  - Count doesn't match 313 - formatted
  - No verification method shown

---

## T10 - Prioritize consolidation roadmap

**Desired state:** Phased roadmap (Week 1: high-impact, Week 2-4: formatting, Week 5: low-impact) with tasks assigned
**Artifacts produced:** File `consolidation-roadmap.md` (markdown)
**Acceptance criteria:**
  - Week 1 lists T1, T2, T6 (high-impact consolidations)
  - Week 2-4 lists remaining formatting work (260 files)
  - Week 5 lists T4, T8 (low-impact improvements)
**Evaluation method:** Phase count (3), task assignment verification
**Downstream dependencies:** T11 (creates overlap matrix)
**Failure indicators:**
  - Missing phases
  - Tasks assigned to wrong phases

### T10.1 - Prioritize by impact/effort ratio

**Desired state:** Tasks ordered by (impact ÷ effort) with High/Medium/Low priority labels
**Artifacts produced:** File `priority-sorted.md` (markdown table)
**Acceptance criteria:**
  - Table columns: Task ID, Impact (High/Medium/Low), Effort (Low/Medium/High), Priority Score, Priority Category
  - Tasks T1, T2, T6 labeled High priority
  - Tasks sorted by Priority Score descending
**Evaluation method:** Table inspection, sort verification
**Downstream dependencies:** T10.2 (creates phased schedule)
**Failure indicators:**
  - Missing columns
  - High-priority tasks not at top

### T10.2 - Create phased schedule

**Desired state:** 3-phase roadmap with weeks and task assignments
**Artifacts produced:** File `phased-roadmap.md` (markdown)
**Acceptance criteria:**
  - Phase 1: "Week 1 - High-Impact Consolidations" lists T1, T2, T6
  - Phase 2: "Weeks 2-4 - Continue Formatting" describes 260-file formatting
  - Phase 3: "Week 5 - Low-Impact Improvements" lists T4, T8
**Evaluation method:** Phase count (3), task listing verification
**Downstream dependencies:** None (parent T10 complete)
**Failure indicators:**
  - Missing phases
  - Wrong tasks in phases

---

## T11 - Create cross-directory overlap matrix

**Desired state:** Markdown matrix showing topic overlap across 8 directories (guides, plans, requirements, roadmap, schema, workflows, research, benchmark dirs)
**Artifacts produced:** File `overlap-matrix.md` (markdown table)
**Acceptance criteria:**
  - Table dimensions: 10 topics × 8 directories
  - Topics: Testing, Schema, Validation, CLI, Models, Error Handling, Hooks, Memory/Storage, Autonomy, Automation
  - Cells contain file name or "N/A"
**Evaluation method:** Table dimensions (10×8), cell count (80)
**Downstream dependencies:** T12 (writes recommendations)
**Failure indicators:**
  - Wrong dimensions
  - Missing topics or directories

### T11.1 - Extract topic-directory mapping

**Desired state:** Complete list of (topic, directory, file) tuples from prompt overlap matrix
**Artifacts produced:** File `topic-mapping.csv` (CSV)
**Acceptance criteria:**
  - CSV has 3 columns: topic, directory, file
  - At least 20 rows (prompt shows matrix has many entries)
  - All 10 topics represented
**Evaluation method:** CSV parsing, row count verification
**Downstream dependencies:** T11.2 (formats as table)
**Failure indicators:**
  - Missing columns
  - <20 rows
  - Missing topics

### T11.2 - Format as markdown table

**Desired state:** Markdown table with topics as rows, directories as columns, cells populated with file names
**Artifacts produced:** File `overlap-matrix.md` (markdown)
**Acceptance criteria:**
  - Table has header row with 8 directory names
  - 10 data rows (one per topic)
  - 80 cells total, each contains file name or "N/A"
**Evaluation method:** Row count (11 including header), column count (8)
**Downstream dependencies:** None (parent T11 complete)
**Failure indicators:**
  - Wrong dimensions
  - Empty cells without "N/A"

---

## T12 - Document automation recommendations

**Desired state:** Recommendations document with 4 automation areas: link validation, duplicate detection, metrics tracking, doc CI checks
**Artifacts produced:** File `automation-recommendations.md` (markdown)
**Acceptance criteria:**
  - Section per automation area (4 total)
  - Each section has recommendation + implementation hint
  - Links to related tools (Foam, grep, GitHub Actions)
**Evaluation method:** Section count (4), content verification
**Downstream dependencies:** None (final task)
**Failure indicators:**
  - <4 sections
  - Missing implementation hints

### T12.1 - Write link validation recommendation

**Desired state:** Recommendation for automated link validation using Foam or custom script
**Artifacts produced:** Section in automation-recommendations.md
**Acceptance criteria:**
  - Title: "## Automated Link Validation"
  - 2-3 sentence recommendation
  - Implementation hint: "Use Foam's link check command or bash script with grep"
**Evaluation method:** Section existence, sentence count
**Downstream dependencies:** T12.2 (writes next recommendation)
**Failure indicators:**
  - Missing section
  - No implementation hint

### T12.2 - Write duplicate detection recommendation

**Desired state:** Recommendation for periodic duplicate content detection scan
**Artifacts produced:** Section in automation-recommendations.md
**Acceptance criteria:**
  - Title: "## Duplicate Content Detection"
  - 2-3 sentence recommendation
  - Implementation hint: "Use text similarity algorithm (e.g., TF-IDF) or manual review on schedule"
**Evaluation method:** Section existence, sentence count
**Downstream dependencies:** T12.3 (writes remaining recommendations)
**Failure indicators:**
  - Missing section
  - No implementation hint

### T12.3 - Write metrics and CI recommendations

**Desired state:** Recommendations for metrics tracking and doc CI checks to prevent link drift
**Artifacts produced:** Two sections in automation-recommendations.md
**Acceptance criteria:**
  - Section "## Metrics Tracking": recommend tracking link density, orphaned files, doc coverage
  - Section "## Doc CI Checks": recommend GitHub Actions workflow to prevent broken links
  - Each has 2-3 sentences + implementation hint
**Evaluation method:** Two sections exist, each with hint
**Downstream dependencies:** None (final task)
**Failure indicators:**
  - Missing either section
  - Missing implementation hints

---

## Coverage Summary

| Task | Has Output State | Acceptance Criteria Count | Verifiable |
|------|------------------|---------------------------|------------|
| T1 | ✅ | 3 | Yes |
| T1.1 | ✅ | 2 | Yes |
| T1.2 | ✅ | 2 | Yes |
| T1.3 | ✅ | 2 | Yes |
| T2 | ✅ | 2 | Yes |
| T2.1 | ✅ | 2 | Yes |
| T2.2 | ✅ | 2 | Yes |
| T2.3 | ✅ | 2 | Yes |
| T3 | ✅ | 2 | Yes |
| T3.1 | ✅ | 2 | Yes |
| T3.2 | ✅ | 2 | Yes |
| T3.3 | ✅ | 2 | Yes |
| T3.4 | ✅ | 2 | Yes |
| T4 | ✅ | 2 | Yes |
| T4.1 | ✅ | 2 | Yes |
| T4.2 | ✅ | 2 | Yes |
| T5 | ✅ | 2 | Yes |
| T5.1 | ✅ | 2 | Yes |
| T5.2 | ✅ | 2 | Yes |
| T6 | ✅ | 2 | Yes |
| T6.1 | ✅ | 2 | Yes |
| T6.2 | ✅ | 2 | Yes |
| T7 | ✅ | 2 | Yes |
| T7.1 | ✅ | 2 | Yes |
| T8 | ✅ | 2 | Yes |
| T9 | ✅ | 2 | Yes |
| T9.1 | ✅ | 2 | Yes |
| T9.2 | ✅ | 2 | Yes |
| T10 | ✅ | 2 | Yes |
| T10.1 | ✅ | 2 | Yes |
| T10.2 | ✅ | 2 | Yes |
| T11 | ✅ | 2 | Yes |
| T11.1 | ✅ | 2 | Yes |
| T11.2 | ✅ | 2 | Yes |
| T12 | ✅ | 2 | Yes |
| T12.1 | ✅ | 2 | Yes |
| T12.2 | ✅ | 2 | Yes |
| T12.3 | ✅ | 2 | Yes |