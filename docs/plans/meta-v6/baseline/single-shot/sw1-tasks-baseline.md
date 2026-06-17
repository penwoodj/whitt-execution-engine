# Task Breakdown

**Source prompt:** # 🔍 Documentation Consolidation Analysis - Based on systematic review of directory structure and file content, the following consolidation opportunities have been identified...

**Total tasks:** 12
**Total story points:** 27
**Maximum leaf complexity:** 3 pts

---

## T1 - Analyze duplicate README content (3pts)

**Story points:** 3
**Why:** Multi-file analysis with content comparison — medium complexity due to reading two files and identifying overlap
**Action:** Read requirements/index.md and benchmark-100-model-userflows/mvp-summary-report.md, compare content sections, document specific overlap areas (quotes, line ranges)
**Depends on:** T0
**GWT criteria:**
  - **Given:** Documentation repo with 313 markdown files
  - **When:** Analyze both README files and extract overlapping content sections
  - **Then:** Produce overlap analysis showing exact duplicate text ranges and unique content per file

### T1.1 - Read and parse requirements/index.md (2pts)

**Story points:** 2
**Why:** Single file read with structured extraction — small task with clear parsing requirements
**Action:** Read requirements/index.md, extract all sections, document section headers, summarize content scope, identify references to schema coverage report
**GWT:**
  - **Given:** File exists at requirements/index.md
  - **When:** Read file and parse markdown structure
  - **Then:** Output section list, content summary, and any reference to "279 requirements" or schema coverage

### T1.2 - Read and parse mvp-summary-report.md (2pts)

**Story points:** 2
**Why:** Single file read with structured extraction — small task with clear parsing requirements
**Action:** Read requirements/benchmark-100-model-userflows/mvp-summary-report.md, extract all sections, document section headers, summarize content scope, identify "Schema Coverage Report" section
**GWT:**
  - **Given:** File exists at requirements/benchmark-100-model-userflows/mvp-summary-report.md
  - **When:** Read file and parse markdown structure
  - **Then:** Output section list, content summary, and confirm "Schema Coverage Report" exists with 279 requirements

### T1.3 - Document overlapping sections (2pts)

**Story points:** 2
**Why:** Content comparison and documentation — small task requiring comparison logic
**Action:** Compare parsed outputs from T1.1 and T1.2, identify overlapping content (duplicate text, shared sections), create overlap report with line ranges
**GWT:**
  - **Given:** Parsed content from both files
  - **When:** Compare content sections and identify text matches
  - **Then:** Produce overlap analysis showing exact duplicate passages and unique content per file

---

## T2 - Execute README consolidation (3pts)

**Story points:** 3
**Why:** Multi-step file operation (merge, delete, update references) — medium complexity due to dependencies
**Action:** Merge requirements/index.md content into mvp-summary-report.md as executive summary, delete requirements/index.md or repurpose, update cross-references to point to consolidated location
**Depends on:** T1
**GWT criteria:**
  - **Given:** Overlap analysis from T1 showing duplicate content
  - **When:** Consolidate into single file and clean up duplicates
  - **Then:** Single source of truth at mvp-summary-report.md, no duplicate README at requirements/index.md, all references updated

### T2.1 - Create merged executive summary (2pts)

**Story points:** 2
**Why:** Content synthesis and markdown writing — small task requiring clear structuring
**Action:** Take unique content from requirements/index.md (index/navigation content), format as executive summary section in mvp-summary-report.md, preserve existing schema coverage report
**GWT:**
  - **Given:** Both files' parsed content, overlap analysis
  - **When:** Synthesize index.md unique content as executive summary
  - **Then:** mvp-summary-report.md has new "## Executive Summary" section at top, no duplicate content

### T2.2 - Remove or repurpose requirements/index.md (1pts)

**Story points:** 1
**Why:** Single file operation — trivial action
**Action:** Delete requirements/index.md OR repurpose as simple directory index (just file listing, no content)
**GWT:**
  - **Given:** Merge complete, content preserved in mvp-summary-report.md
  - **When:** Execute file deletion or repurpose
  - **Then:** requirements/index.md either deleted OR contains only directory listing (no duplicate content)

### T2.3 - Update cross-references (2pts)

**Story points:** 2
**Why:** Multi-file search-and-replace — small task with file I/O
**Action:** Search all docs/ for references to requirements/index.md, update to point to requirements/benchmark-100-model-userflows/mvp-summary-report.md
**GWT:**
  - **Given:** grep results showing all files referencing requirements/index.md
  - **When:** Replace references with consolidated location
  - **Then:** Zero files contain old reference path, all reference consolidated mvp-summary-report.md

---

## T3 - Analyze scattered testing guidance (3pts)

**Story points:** 3
**Why:** Multi-file content analysis with categorization — medium complexity due to 4 locations
**Action:** Read guides/testing-guide.md, workflows/workflows-README.md (lines 110-166), plans/testing-strategy.md, plans/*/tests/ directories, categorize content by audience and scope
**Depends on:** T0
**GWT criteria:**
  - **Given:** Multiple testing documentation locations identified
  - **When:** Read and categorize each file's testing content
  - **Then:** Produce categorization showing developer-facing vs workflow-specific vs strategy-level content

### T3.1 - Read testing-guide.md (1pts)

**Story points:** 1
**Why:** Single file read — trivial
**Action:** Read guides/testing-guide.md, extract testing strategies and procedures sections, note intended audience
**GWT:**
  - **Given:** File exists
  - **When:** Read and extract
  - **Then:** Summary of content scope and audience

### T3.2 - Read workflows-README.md testing section (1pts)

**Story points:** 1
**Why:** Specific section read — trivial
**Action:** Read workflows/workflows-README.md lines 110-166 (Testing Guidance section), extract testing patterns
**GWT:**
  - **Given:** File exists with Testing Guidance section
  - **When:** Extract lines 110-166
  - **Then:** Summary of workflow-specific testing patterns

### T3.3 - Read testing-strategy.md (1pts)

**Story points:** 1
**Why:** Single file read — trivial
**Action:** Read plans/testing-strategy.md, extract high-level strategy content
**GWT:**
  - **Given:** File exists
  - **When:** Read and extract
  - **Then:** Summary of strategy-level content

### T3.4 - Categorize testing content overlap (2pts)

**Story points:** 2
**Why:** Content categorization and overlap analysis — small task requiring synthesis
**Action:** Compare content from T3.1-T3.3, categorize by audience (developer, workflow author, planner), identify unique vs overlapping content, recommend separation
**GWT:**
  - **Given:** Parsed content from 3 sources
  - **When:** Categorize and identify overlap
  - **Then:** Categorization matrix showing unique vs shared content per source

---

## T4 - Refine testing guidance structure (3pts)

**Story points:** 3
**Why:** Multi-file refinement with reference updates — medium complexity
**Action:** Keep guides/testing-guide.md as developer-facing, keep workflows/workflows-README.md as workflow patterns, consolidate plans/testing-strategy.md into plans/ARCHITECTURE.md, create cross-references
**Depends on:** T3
**GWT criteria:**
  - **Given:** Categorization matrix from T3
  - **When:** Refine files and add cross-references
  - **Then:** Clear separation of concerns, bidirectional cross-references between all three

### T4.1 - Consolidate testing-strategy.md (2pts)

**Story points:** 2
**Why:** Content merge and file cleanup — small task
**Action:** Merge unique strategy content from plans/testing-strategy.md into plans/ARCHITECTURE.md, delete testing-strategy.md, add cross-reference link
**GWT:**
  - **Given:** Categorization showing strategy-level content is unique
  - **When:** Merge and delete
  - **Then:** Strategy content in ARCHITECTURE.md, testing-strategy.md deleted, cross-reference added

### T4.2 - Add cross-references between testing docs (1pts)

**Story points:** 1
**Why:** Link addition — trivial
**Action:** Add "## Related Documents" sections to guides/testing-guide.md, workflows/workflows-README.md, plans/ARCHITECTURE.md linking to each other
**GWT:**
  - **Given:** Three files with testing-related content
  - **When:** Add bidirectional links
  - **Then:** Each file links to the other two

---

## T5 - Analyze schema reference gaps (2pts)

**Story points:** 2
**Why:** Multi-file reference analysis — small task
**Action:** Check schema/README.md, roadmap/README.md, requirements/ files, plans/ files, workflows/ files for schema references, identify lack of centralized navigation
**Depends on:** T0
**GWT criteria:**
  - **Given:** Files reference unified-workflow-schema.yml independently
  - **When:** Analyze reference patterns
  - **Then:** Document that no central schema navigation hub exists

### T5.1 - Catalog schema references (1pts)

**Story points:** 1
**Why:** Search and catalog — trivial
**Action:** grep docs/ for "unified-workflow-schema.yml", list all files referencing schema, extract reference context
**GWT:**
  - **Given:** docs/ directory
  - **When:** Search for schema references
  - **Then:** List of files with schema references and reference context

### T5.2 - Document missing navigation (1pts)

**Story points:** 1
**Why:** Analysis and documentation — trivial
**Action:** Analyze reference catalog, note each file independently references schema with no context on how to read it or navigate schema sections
**GWT:**
  - **Given:** Reference catalog from T5.1
  - **When:** Analyze patterns
  - **Then:** Document missing: schema quick start, element reference, navigation shortcuts

---

## T6 - Create schema navigation hub (3pts)

**Story points:** 3
**Why:** Multi-section document creation with cross-references — medium complexity
**Action:** Create schema/NAVIGATION.md with quick start, element reference, cross-reference matrix, update all READMEs to point to it
**Depends on:** T5
**GWT criteria:**
  - **Given:** Missing navigation documented in T5
  - **When:** Create NAVIGATION.md and update references
  - **Then:** New NAVIGATION.md exists, all READMEs link to it as schema entry point

### T6.1 - Write schema/NAVIGATION.md content (2pts)

**Story points:** 2
**Why:** Multi-section technical document — small task
**Action:** Create schema/NAVIGATION.md with sections: Quick Start, Element Reference (step types, loops, validations), Cross-Reference Matrix, Navigation Shortcuts
**GWT:**
  - **Given:** Schema file exists at schema/unified-workflow-schema.yml
  - **When:** Create NAVIGATION.md
  - **Then:** New file with 4 sections, each non-empty

### T6.2 - Update READMEs to reference NAVIGATION.md (1pts)

**Story points:** 1
**Why:** Multi-file update — trivial with search-replace
**Action:** Add "See: schema/NAVIGATION.md for comprehensive schema navigation" to schema/README.md, roadmap/README.md, workflows/workflows-README.md
**GWT:**
  - **Given:** Three README files
  - **When:** Add NAVIGATION.md reference
  - **Then:** All three READMEs link to schema/NAVIGATION.md

---

## T7 - Analyze ADR metadata structure (1pts)

**Story points:** 1
**Why:** Single directory analysis — trivial
**Action:** Review roadmap/adr-0000-roadmap-index.yml, roadmap/adr-000*.yml files, roadmap/metadata/ directory, document current structure
**Depends on:** T0
**GWT criteria:**
  - **Given:** ADR files and metadata directory exist
  - **When:** Analyze structure
  - **Then:** Document current ADR metadata distribution

### T7.1 - Document ADR metadata distribution (1pts)

**Story points:** 1
**Why:** Documentation — trivial
**Action:** List ADR files with embedded metadata, note master index structure, describe metadata/ directory contents
**GWT:**
  - **Given:** ADR directory structure
  - **When:** Catalog and document
  - **Then:** Document showing metadata is appropriately distributed

---

## T8 - Create metadata README (1pts)

**Story points:** 1
**Why:** Single document creation — trivial
**Action:** Create roadmap/metadata/README.md explaining metadata organization, individual ADR metadata, master index role, future enhancement considerations
**Depends on:** T7
**GWT criteria:**
  - **Given:** ADR metadata structure documented in T7
  - **When:** Create README
  - **Then:** roadmap/metadata/README.md exists explaining structure

---

## T9 - Analyze formatting status (2pts)

**Story points:** 2
**Why:** Multi-directory link density analysis — small task
**Action:** Review link density table in prompt, note formatting progress (50+ files done, 260 remaining), identify patterns applied (emoji headings, Foam links, bidirectional nav)
**Depends on:** T0
**GWT criteria:**
  - **Given:** Formatting status from prompt
  - **When:** Analyze formatting patterns and remaining work
  - **Then:** Summary of completed patterns and unformatted file count

### T9.1 - Document completed formatting patterns (1pts)

**Story points:** 1
**Why:** Documentation — trivial
**Action:** Extract patterns from prompt (emoji headings, Foam syntax, bidirectional navigation, cross-directory links), list as "Applied Patterns"
**GWT:**
  - **Given:** Prompt contains formatting status
  - **When:** Extract pattern list
  - **Then:** List of 5 applied formatting patterns

### T9.2 - Count unformatted files (1pts)

**Story points:** 1
**Why:** File counting — trivial
**Action:** Use prompt data to calculate unformatted count (313 total - 50+ formatted = ~260 remaining), verify with find command
**GWT:**
  - **Given:** docs/ directory
  - **When:** Count markdown files lacking Foam links
  - **Then:** Unformatted file count documented

---

## T10 - Prioritize consolidation roadmap (2pts)

**Story points:** 2
**Why:** Priority assignment and roadmap sequencing — small task
**Action:** Assign priorities to consolidation tasks based on effort/impact from prompt, create phased roadmap (Week 1: high-impact, Week 2-4: continue formatting, Week 5: low-impact)
**Depends on:** T9
**GWT criteria:**
  - **Given:** Effort/impact analysis from prompt
  - **When:** Sequence into phases
  - **Then:** Phased roadmap with tasks per week

### T10.1 - Prioritize by impact/effort ratio (1pts)

**Story points:** 1
**Why:** Priority sorting — trivial
**Action:** Order consolidation tasks T1-T8 by (impact ÷ effort), categorize as High/Medium/Low priority
**GWT:**
  - **Given:** Effort/impact data from prompt
  - **When:** Calculate priority scores
  - **Then:** Tasks categorized and ordered by priority

### T10.2 - Create phased schedule (1pts)

**Story points:** 1
**Why:** Scheduling — trivial
**Action:** Create 3-phase roadmap: Phase 1 (Week 1) high-impact consolidations, Phase 2 (Week 2-4) continue formatting 260 files, Phase 3 (Week 5) low-impact improvements
**GWT:**
  - **Given:** Prioritized task list
  - **When:** Assign to phases
  - **Then:** 3-phase roadmap with tasks per week

---

## T11 - Create cross-directory overlap matrix (2pts)

**Story points:** 2
**Why:** Matrix creation with topic mapping — small task
**Action:** Create markdown table showing topic overlap across directories (Testing, Schema, Validation, CLI, Models, Error Handling, Hooks, Memory/Storage, Autonomy, Automation), populate from prompt data
**Depends on:** T0
**GWT criteria:**
  - **Given:** Topic list from prompt
  - **When:** Create matrix table
  - **Then:** Markdown table with 8 directories × 10 topics, cells populated with file names or "N/A"

### T11.1 - Extract topic-directory mapping (1pts)

**Story points:** 1
**Why:** Data extraction — trivial
**Action:** Extract topic-directory mappings from prompt "## 📋 Cross-Directory Overlap Matrix" section
**GWT:**
  - **Given:** Prompt contains matrix
  - **When:** Extract mapping data
  - **Then:** List of (topic, directory, file) tuples

### T11.2 - Format as markdown table (1pts)

**Story points:** 1
**Why:** Markdown formatting — trivial
**Action:** Create markdown table with topics as rows, directories as columns, populate cells
**GWT:**
  - **Given:** Mapping data from T11.1
  - **When:** Create table
  - **Then:** Formatted markdown matrix

---

## T12 - Document automation recommendations (3pts)

**Story points:** 3
**Why:** Multi-part recommendation document — medium complexity
**Action:** Document automated validation recommendations (link validation, duplicate detection, metrics tracking, doc CI checks), include implementation hints
**Depends on:** T10
**GWT criteria:**
  - **Given:** Long-term considerations from prompt
  - **When:** Write recommendations document
  - **Then:** Recommendations markdown with 4 automation areas

### T12.1 - Write link validation recommendation (1pts)

**Story points:** 1
**Why:** Single recommendation — trivial
**Action:** Write "Automated link validation" recommendation: use Foam or script to detect broken links, suggest implementation approach
**GWT:**
  - **Given:** Long-term section mentions link validation
  - **When:** Write recommendation
  - **Then:** Recommendation paragraph with implementation hint

### T12.2 - Write duplicate detection recommendation (1pts)

**Story points:** 1
**Why:** Single recommendation — trivial
**Action:** Write "Duplicate content detection" recommendation: periodic scan for overlapping content, suggest approach
**GWT:**
  - **Given:** Long-term section mentions duplicate detection
  - **When:** Write recommendation
  - **Then:** Recommendation paragraph with approach

### T12.3 - Write metrics and CI recommendations (1pts)

**Story points:** 1
**Why:** Two recommendations — trivial
**Action:** Write "Metrics tracking" and "Doc CI checks" recommendations: track link density, orphaned files, doc coverage; prevent broken links via CI
**GWT:**
  - **Given:** Long-term section mentions metrics and CI
  - **When:** Write recommendations
  - **Then:** Two recommendation paragraphs

---

## Complexity Distribution

| Points | Count |
|--------|-------|
| 1 | 7 |
| 2 | 14 |
| 3 | 6 |
| 5 | 0 |
| 8 | 0 |
| 13 | 0 |