# YAML Substructures

**Source:** # Agentic Categorization - Based on task breakdown with 12 parent tasks and 27 leaf tasks, all categorized as DATA_TRANSFORMER
**Total substructures:** 27
**Verdict:** PASS

---

## Pattern Summary

| Category | Step Count | Hooks Used |
|----------|-----------|------------|
| SEQUENTIAL_PROCESSOR | 0 | N/A |
| PARALLEL_FAN_OUT | 0 | N/A |
| ITERATIVE_REFINER | 0 | N/A |
| CONDITIONAL_ROUTER | 0 | N/A |
| DATA_TRANSFORMER | 27 | save_to, log |
| ACCUMULATOR | 0 | N/A |

---

## T1 - Analyze duplicate README content (DATA_TRANSFORMER, budget=1)

Intent: Read and parse two README files, then produce overlap analysis showing duplicate content ranges between them.

```yaml
step_t1_1_parse_index:
  generative_entity: "${models.qwen35}"
  prompt: |
    Read requirements/index.md and parse its structure. Extract all section headers, summarize content scope in <200 chars, and note if it references "279 requirements" or schema coverage report.

    Output JSON: {sections: [], summary: "", schema_ref: ""}
  model_overrides:
    max_tokens: 1000
    temperature: 0.1
  when:
    after_step_succeeds:
      - save_to:
          - t1_1_parsed
          - "./outputs/t1_1_parsed.json"
      - log:
          to_file_path: "./logs/t1.log"
          event_fields: [step_name, duration_ms]
          level: info

step_t1_2_parse_mvp:
  generative_entity: "${models.qwen35}"
  prompt: |
    Read requirements/benchmark-100-model-userflows/mvp-summary-report.md and parse its structure. Extract all section headers, summarize content scope in <200 chars, and confirm if "Schema Coverage Report" section exists with 279 requirements.

    Output JSON: {sections: [], summary: "", has_schema_coverage: true/false}
  model_overrides:
    max_tokens: 1000
    temperature: 0.1
  when:
    after_step_succeeds:
      - save_to:
          - t1_2_parsed
          - "./outputs/t1_2_parsed.json"
      - log:
          to_file_path: "./logs/t1.log"
          event_fields: [step_name, duration_ms]
          level: info

step_t1_3_document_overlap:
  generative_entity: "${models.qwen35}"
  depends_on: [step_t1_1_parse_index, step_t1_2_parse_mvp]
  prompt: |
    Compare parsed structures from {{bookmarks.t1_1_parsed}} and {{bookmarks.t1_2_parsed}}. Identify overlapping content (duplicate text, shared sections) and unique content per file.

    Create markdown table with columns: Section, File A Line Range, File B Line Range, Overlap Status (identical/similar/unique). Include at least 10 rows and summary paragraph with overlap percentage.
  model_overrides:
    max_tokens: 2000
    temperature: 0.2
  when:
    after_step_succeeds:
      - save_to:
          - t1_overlap
          - "./outputs/overlap-analysis.md"
      - log:
          to_file_path: "./logs/t1.log"
          event_fields: [step_name, duration_ms]
          level: info
```

Fit analysis: Sequential chain with data flowing through bookmarks. Step 3 depends on both parsing steps to generate comparison output.

---

## T1.1 - Read and parse requirements/index.md (DATA_TRANSFORMER, budget=1)

Intent: Read single README file and parse its markdown structure into JSON for later comparison.

```yaml
step_t1_1_parse_index:
  generative_entity: "${models.qwen35}"
  prompt: |
    Read requirements/index.md and parse its structure. Extract all section headers, summarize content scope in <200 chars, and note if it references "279 requirements" or schema coverage report.

    Output JSON: {sections: [], summary: "", schema_ref: ""}
  model_overrides:
    max_tokens: 1000
    temperature: 0.1
  when:
    after_step_succeeds:
      - save_to:
          - t1_1_parsed
          - "./outputs/t1_1_parsed.json"
      - log:
          to_file_path: "./logs/t1.log"
          event_fields: [step_name, duration_ms]
          level: info
```

Fit analysis: Standalone transformation step. Output stored in bookmark for downstream comparison.

---

## T1.2 - Read and parse mvp-summary-report.md (DATA_TRANSFORMER, budget=1)

Intent: Read single mvp summary report file and parse its structure into JSON for later comparison.

```yaml
step_t1_2_parse_mvp:
  generative_entity: "${models.qwen35}"
  prompt: |
    Read requirements/benchmark-100-model-userflows/mvp-summary-report.md and parse its structure. Extract all section headers, summarize content scope in <200 chars, and confirm if "Schema Coverage Report" section exists with 279 requirements.

    Output JSON: {sections: [], summary: "", has_schema_coverage: true/false}
  model_overrides:
    max_tokens: 1000
    temperature: 0.1
  when:
    after_step_succeeds:
      - save_to:
          - t1_2_parsed
          - "./outputs/t1_2_parsed.json"
      - log:
          to_file_path: "./logs/t1.log"
          event_fields: [step_name, duration_ms]
          level: info
```

Fit analysis: Standalone transformation step, parallel-eligible with T1.1 (no dependencies).

---

## T1.3 - Document overlapping sections (DATA_TRANSFORMER, budget=1)

Intent: Compare two parsed structures and create overlap analysis table showing duplicate content.

```yaml
step_t1_3_document_overlap:
  generative_entity: "${models.qwen35}"
  depends_on: [step_t1_1_parse_index, step_t1_2_parse_mvp]
  prompt: |
    Compare parsed structures from {{bookmarks.t1_1_parsed}} and {{bookmarks.t1_2_parsed}}. Identify overlapping content (duplicate text, shared sections) and unique content per file.

    Create markdown table with columns: Section, File A Line Range, File B Line Range, Overlap Status (identical/similar/unique). Include at least 10 rows and summary paragraph with overlap percentage.
  model_overrides:
    max_tokens: 2000
    temperature: 0.2
  when:
    after_step_succeeds:
      - save_to:
          - t1_overlap
          - "./outputs/overlap-analysis.md"
      - log:
          to_file_path: "./logs/t1.log"
          event_fields: [step_name, duration_ms]
          level: info
```

Fit analysis: Aggregation step consuming both parsing outputs via bookmarks.

---

## T2 - Execute README consolidation (DATA_TRANSFORMER, budget=1)

Intent: Consolidate duplicate README content into single file, delete duplicate, update cross-references.

```yaml
step_t2_1_create_merged:
  generative_entity: "${models.qwen35}"
  depends_on: [step_t1_3_document_overlap]
  prompt: |
    Using overlap analysis from {{bookmarks.t1_overlap}}, merge unique content from requirements/index.md into requirements/benchmark-100-model-userflows/mvp-summary-report.md as "## Executive Summary" section.

    Preserve existing content, add summary at top after document title. Ensure no duplicate content.
  model_overrides:
    max_tokens: 2000
    temperature: 0.1
  when:
    after_step_succeeds:
      - save_to:
          - t2_1_merged
          - "./outputs/t2_1_merged.md"
      - log:
          to_file_path: "./logs/t2.log"
          event_fields: [step_name, duration_ms]
          level: info

step_t2_2_remove_or_repurpose:
  depends_on: [step_t2_1_create_merged]
  shell:
    command: "rm requirements/index.md"
    args: []
    fail_on_error: true
  when:
    after_step_succeeds:
      - log:
          to_file_path: "./logs/t2.log"
          event_fields: [step_name, duration_ms]
          level: info

step_t2_3_update_references:
  depends_on: [step_t2_2_remove_or_repurpose]
  shell:
    command: "find docs/ -type f -name '*.md' -exec sed -i 's|requirements/index.md|requirements/benchmark-100-model-userflows/mvp-summary-report.md|g' {} +"
    args: []
    fail_on_error: true
  when:
    after_step_succeeds:
      - log:
          to_file_path: "./logs/t2.log"
          event_fields: [step_name, duration_ms]
          level: info
```

Fit analysis: Sequential chain: LLM merge → shell delete → shell search-replace. Each step's output feeds next.

---

## T2.1 - Create merged executive summary (DATA_TRANSFORMER, budget=1)

Intent: Merge unique content from requirements/index.md into mvp-summary-report.md as executive summary.

```yaml
step_t2_1_create_merged:
  generative_entity: "${models.qwen35}"
  depends_on: [step_t1_3_document_overlap]
  prompt: |
    Using overlap analysis from {{bookmarks.t1_overlap}}, merge unique content from requirements/index.md into requirements/benchmark-100-model-userflows/mvp-summary-report.md as "## Executive Summary" section.

    Preserve existing content, add summary at top after document title. Ensure no duplicate content.
  model_overrides:
    max_tokens: 2000
    temperature: 0.1
  when:
    after_step_succeeds:
      - save_to:
          - t2_1_merged
          - "./outputs/t2_1_merged.md"
      - log:
          to_file_path: "./logs/t2.log"
          event_fields: [step_name, duration_ms]
          level: info
```

Fit analysis: Consumes overlap analysis, produces merged file for next step.

---

## T2.2 - Remove or repurpose requirements/index.md (DATA_TRANSFORMER, budget=1)

Intent: Delete duplicate README file after merge is complete.

```yaml
step_t2_2_remove_or_repurpose:
  depends_on: [step_t2_1_create_merged]
  shell:
    command: "rm requirements/index.md"
    args: []
    fail_on_error: true
  when:
    after_step_succeeds:
      - log:
          to_file_path: "./logs/t2.log"
          event_fields: [step_name, duration_ms]
          level: info
```

Fit analysis: Shell step for file deletion. Depends on merge completion.

---

## T2.3 - Update cross-references (DATA_TRANSFORMER, budget=1)

Intent: Replace all references to deleted requirements/index.md with consolidated location.

```yaml
step_t2_3_update_references:
  depends_on: [step_t2_2_remove_or_repurpose]
  shell:
    command: "find docs/ -type f -name '*.md' -exec sed -i 's|requirements/index.md|requirements/benchmark-100-model-userflows/mvp-summary-report.md|g' {} +"
    args: []
    fail_on_error: true
  when:
    after_step_succeeds:
      - log:
          to_file_path: "./logs/t2.log"
          event_fields: [step_name, duration_ms]
          level: info
```

Fit analysis: Shell search-replace across multiple files. Final step in T2 chain.

---

## T3 - Analyze scattered testing guidance (DATA_TRANSFORMER, budget=1)

Intent: Read three testing documentation files, categorize content by audience/scope, produce matrix.

```yaml
step_t3_1_read_guide:
  generative_entity: "${models.qwen35}"
  prompt: |
    Read guides/testing-guide.md and extract: all section headers, summary (<150 chars), intended audience (developer/workflow-author/planner).

    Output JSON: {sections: [], audience: "", summary: ""}
  model_overrides:
    max_tokens: 1000
    temperature: 0.1
  when:
    after_step_succeeds:
      - save_to:
          - t3_1_guide
          - "./outputs/t3_1_guide.json"
      - log:
          to_file_path: "./logs/t3.log"
          event_fields: [step_name, duration_ms]
          level: info

step_t3_2_read_workflows:
  generative_entity: "${models.qwen35}"
  prompt: |
    Read workflows/workflows-README.md lines 110-166 (Testing Guidance section). Extract testing pattern names and section summary (<150 chars).

    Output JSON: {patterns: [], section_summary: ""}
  model_overrides:
    max_tokens: 1000
    temperature: 0.1
  when:
    after_step_succeeds:
      - save_to:
          - t3_2_workflows
          - "./outputs/t3_2_workflows.json"
      - log:
          to_file_path: "./logs/t3.log"
          event_fields: [step_name, duration_ms]
          level: info

step_t3_3_read_strategy:
  generative_entity: "${models.qwen35}"
  prompt: |
    Read plans/testing-strategy.md and extract: 2-5 key strategy bullet points, scope (automation/process/framework).

    Output JSON: {key_points: [], scope: ""}
  model_overrides:
    max_tokens: 1000
    temperature: 0.1
  when:
    after_step_succeeds:
      - save_to:
          - t3_3_strategy
          - "./outputs/t3_3_strategy.json"
      - log:
          to_file_path: "./logs/t3.log"
          event_fields: [step_name, duration_ms]
          level: info

step_t3_4_categorize:
  generative_entity: "${models.qwen35}"
  depends_on: [step_t3_1_read_guide, step_t3_2_read_workflows, step_t3_3_read_strategy]
  prompt: |
    Compare content from {{bookmarks.t3_1_guide}}, {{bookmarks.t3_2_workflows}}, {{bookmarks.t3_3_strategy}}. Categorize by audience (developer/workflow/planner), scope (how-to/patterns/strategy), and list unique content per source.

    Create markdown matrix with 3 rows (one per source) and 3 columns: audience, scope, unique_content.
  model_overrides:
    max_tokens: 2000
    temperature: 0.2
  when:
    after_step_succeeds:
      - save_to:
          - t3_categorization
          - "./outputs/testing-categorization.md"
      - log:
          to_file_path: "./logs/t3.log"
          event_fields: [step_name, duration_ms]
          level: info
```

Fit analysis: Three parallel read steps (no deps) → aggregation step consuming all three via bookmarks.

---

## T3.1 - Read testing-guide.md (DATA_TRANSFORMER, budget=1)

Intent: Read and parse testing-guide.md for categorization.

```yaml
step_t3_1_read_guide:
  generative_entity: "${models.qwen35}"
  prompt: |
    Read guides/testing-guide.md and extract: all section headers, summary (<150 chars), intended audience (developer/workflow-author/planner).

    Output JSON: {sections: [], audience: "", summary: ""}
  model_overrides:
    max_tokens: 1000
    temperature: 0.1
  when:
    after_step_succeeds:
      - save_to:
          - t3_1_guide
          - "./outputs/t3_1_guide.json"
      - log:
          to_file_path: "./logs/t3.log"
          event_fields: [step_name, duration_ms]
          level: info
```

Fit analysis: Standalone read step, parallel-eligible.

---

## T3.2 - Read workflows-README.md testing section (DATA_TRANSFORMER, budget=1)

Intent: Read Testing Guidance section from workflows-README.md for categorization.

```yaml
step_t3_2_read_workflows:
  generative_entity: "${models.qwen35}"
  prompt: |
    Read workflows/workflows-README.md lines 110-166 (Testing Guidance section). Extract testing pattern names and section summary (<150 chars).

    Output JSON: {patterns: [], section_summary: ""}
  model_overrides:
    max_tokens: 1000
    temperature: 0.1
  when:
    after_step_succeeds:
      - save_to:
          - t3_2_workflows
          - "./outputs/t3_2_workflows.json"
      - log:
          to_file_path: "./logs/t3.log"
          event_fields: [step_name, duration_ms]
          level: info
```

Fit analysis: Standalone read step, parallel-eligible.

---

## T3.3 - Read testing-strategy.md (DATA_TRANSFORMER, budget=1)

Intent: Read and parse testing-strategy.md for categorization.

```yaml
step_t3_3_read_strategy:
  generative_entity: "${models.qwen35}"
  prompt: |
    Read plans/testing-strategy.md and extract: 2-5 key strategy bullet points, scope (automation/process/framework).

    Output JSON: {key_points: [], scope: ""}
  model_overrides:
    max_tokens: 1000
    temperature: 0.1
  when:
    after_step_succeeds:
      - save_to:
          - t3_3_strategy
          - "./outputs/t3_3_strategy.json"
      - log:
          to_file_path: "./logs/t3.log"
          event_fields: [step_name, duration_ms]
          level: info
```

Fit analysis: Standalone read step, parallel-eligible.

---

## T3.4 - Categorize testing content overlap (DATA_TRANSFORMER, budget=1)

Intent: Aggregate three parsed files into categorization matrix.

```yaml
step_t3_4_categorize:
  generative_entity: "${models.qwen35}"
  depends_on: [step_t3_1_read_guide, step_t3_2_read_workflows, step_t3_3_read_strategy]
  prompt: |
    Compare content from {{bookmarks.t3_1_guide}}, {{bookmarks.t3_2_workflows}}, {{bookmarks.t3_3_strategy}}. Categorize by audience (developer/workflow/planner), scope (how-to/patterns/strategy), and list unique content per source.

    Create markdown matrix with 3 rows (one per source) and 3 columns: audience, scope, unique_content.
  model_overrides:
    max_tokens: 2000
    temperature: 0.2
  when:
    after_step_succeeds:
      - save_to:
          - t3_categorization
          - "./outputs/testing-categorization.md"
      - log:
          to_file_path: "./logs/t3.log"
          event_fields: [step_name, duration_ms]
          level: info
```

Fit analysis: Aggregation step consuming three parallel reads via bookmarks.

---

## T4 - Refine testing guidance structure (DATA_TRANSFORMER, budget=1)

Intent: Consolidate testing-strategy.md into ARCHITECTURE.md, delete it, add cross-references between all three testing docs.

```yaml
step_t4_1_consolidate:
  generative_entity: "${models.qwen35}"
  depends_on: [step_t3_4_categorize]
  prompt: |
    Using categorization from {{bookmarks.t3_categorization}}, merge unique strategy content from plans/testing-strategy.md into plans/ARCHITECTURE.md as new "## Testing Strategy" section. Add cross-references to guides/testing-guide.md and workflows/workflows-README.md.
  model_overrides:
    max_tokens: 2000
    temperature: 0.1
  when:
    after_step_succeeds:
      - save_to:
          - t4_1_architecture
          - "./outputs/t4_1_architecture.md"
      - shell:
          command: "rm plans/testing-strategy.md"
          fail_on_error: true
      - log:
          to_file_path: "./logs/t4.log"
          event_fields: [step_name, duration_ms]
          level: info

step_t4_2_add_crossrefs:
  generative_entity: "${models.qwen35}"
  depends_on: [step_t4_1_consolidate]
  prompt: |
    Add "## Related Documents" sections to guides/testing-guide.md, workflows/workflows-README.md, and plans/ARCHITECTURE.md. Each section should link to the other two files using proper markdown syntax.
  model_overrides:
    max_tokens: 1500
    temperature: 0.1
  when:
    after_step_succeeds:
      - log:
          to_file_path: "./logs/t4.log"
          event_fields: [step_name, duration_ms]
          level: info
```

Fit analysis: Sequential: LLM merge + shell delete → LLM cross-reference addition.

---

## T4.1 - Consolidate testing-strategy.md (DATA_TRANSFORMER, budget=1)

Intent: Merge strategy content into ARCHITECTURE.md and delete strategy file.

```yaml
step_t4_1_consolidate:
  generative_entity: "${models.qwen35}"
  depends_on: [step_t3_4_categorize]
  prompt: |
    Using categorization from {{bookmarks.t3_categorization}}, merge unique strategy content from plans/testing-strategy.md into plans/ARCHITECTURE.md as new "## Testing Strategy" section. Add cross-references to guides/testing-guide.md and workflows/workflows-README.md.
  model_overrides:
    max_tokens: 2000
    temperature: 0.1
  when:
    after_step_succeeds:
      - save_to:
          - t4_1_architecture
          - "./outputs/t4_1_architecture.md"
      - shell:
          command: "rm plans/testing-strategy.md"
          fail_on_error: true
      - log:
          to_file_path: "./logs/t4.log"
          event_fields: [step_name, duration_ms]
          level: info
```

Fit analysis: LLM step with post-success shell hook for file deletion.

---

## T4.2 - Add cross-references between testing docs (DATA_TRANSFORMER, budget=1)

Intent: Add bidirectional cross-reference sections to all three testing docs.

```yaml
step_t4_2_add_crossrefs:
  generative_entity: "${models.qwen35}"
  depends_on: [step_t4_1_consolidate]
  prompt: |
    Add "## Related Documents" sections to guides/testing-guide.md, workflows/workflows-README.md, and plans/ARCHITECTURE.md. Each section should link to the other two files using proper markdown syntax.
  model_overrides:
    max_tokens: 1500
    temperature: 0.1
  when:
    after_step_succeeds:
      - log:
          to_file_path: "./logs/t4.log"
          event_fields: [step_name, duration_ms]
          level: info
```

Fit analysis: LLM step modifying three files. No output save needed (modifications in-place).

---

## T5 - Analyze schema reference gaps (DATA_TRANSFORMER, budget=1)

Intent: Catalog all schema references, document missing navigation resources.

```yaml
step_t5_1_catalog_refs:
  shell:
    command: "grep -r 'unified-workflow-schema.yml' docs/ --include='*.md' -H | sed 's/:/ | /' | awk '{print $1 \" | \" $0}'"
    fail_on_error: true
  when:
    after_step_succeeds:
      - save_to:
          - t5_1_catalog
          - "./outputs/schema-ref-catalog.md"
      - log:
          to_file_path: "./logs/t5.log"
          event_fields: [step_name, duration_ms]
          level: info

step_t5_2_document_gaps:
  generative_entity: "${models.qwen35}"
  depends_on: [step_t5_1_catalog_refs]
  prompt: |
    Using reference catalog from {{bookmarks.t5_1_catalog}}, analyze reference patterns and document missing navigation resources.

    List 4 missing resources: quick start guide, element reference (step types, loops, validations), cross-reference matrix (element → schema section → example workflow), navigation shortcuts. Provide 2-sentence description for each and conclude that current reference pattern is ad-hoc.
  model_overrides:
    max_tokens: 1500
    temperature: 0.2
  when:
    after_step_succeeds:
      - save_to:
          - t5_2_gaps
          - "./outputs/schema-nav-gaps.md"
      - log:
          to_file_path: "./logs/t5.log"
          event_fields: [step_name, duration_ms]
          level: info
```

Fit analysis: Shell grep → LLM analysis. Sequential chain.

---

## T5.1 - Catalog schema references (DATA_TRANSFORMER, budget=1)

Intent: Use grep to find all files referencing unified-workflow-schema.yml.

```yaml
step_t5_1_catalog_refs:
  shell:
    command: "grep -r 'unified-workflow-schema.yml' docs/ --include='*.md' -H | sed 's/:/ | /' | awk '{print $1 \" | \" $0}'"
    fail_on_error: true
  when:
    after_step_succeeds:
      - save_to:
          - t5_1_catalog
          - "./outputs/schema-ref-catalog.md"
      - log:
          to_file_path: "./logs/t5.log"
          event_fields: [step_name, duration_ms]
          level: info
```

Fit analysis: Shell step with complex pipeline. Output saved as catalog.

---

## T5.2 - Document missing navigation (DATA_TRANSFORMER, budget=1)

Intent: Analyze reference catalog and document missing navigation resources.

```yaml
step_t5_2_document_gaps:
  generative_entity: "${models.qwen35}"
  depends_on: [step_t5_1_catalog_refs]
  prompt: |
    Using reference catalog from {{bookmarks.t5_1_catalog}}, analyze reference patterns and document missing navigation resources.

    List 4 missing resources: quick start guide, element reference (step types, loops, validations), cross-reference matrix (element → schema section → example workflow), navigation shortcuts. Provide 2-sentence description for each and conclude that current reference pattern is ad-hoc.
  model_overrides:
    max_tokens: 1500
    temperature: 0.2
  when:
    after_step_succeeds:
      - save_to:
          - t5_2_gaps
          - "./outputs/schema-nav-gaps.md"
      - log:
          to_file_path: "./logs/t5.log"
          event_fields: [step_name, duration_ms]
          level: info
```

Fit analysis: LLM analysis step consuming catalog via bookmark.

---

## T6 - Create schema navigation hub (DATA_TRANSFORMER, budget=1)

Intent: Create NAVIGATION.md with 4 sections, update READMEs to link to it.

```yaml
step_t6_1_write_nav:
  generative_entity: "${models.qwen35}"
  depends_on: [step_t5_2_document_gaps]
  prompt: |
    Using gap analysis from {{bookmarks.t5_2_gaps}}, create schema/NAVIGATION.md with 4 comprehensive sections:

    1. "## Quick Start" - 3-5 bullet points for schema newcomers
    2. "## Element Reference" - list step types, loop types, validation strategies
    3. "## Cross-Reference Matrix" - table: Element → Schema Section → Example Workflow
    4. "## Navigation Shortcuts" - 3-5 links to related docs

    Each section must be non-empty.
  model_overrides:
    max_tokens: 2000
    temperature: 0.1
  when:
    after_step_succeeds:
      - save_to:
          - t6_1_nav
          - "schema/NAVIGATION.md"
      - log:
          to_file_path: "./logs/t6.log"
          event_fields: [step_name, duration_ms]
          level: info

step_t6_2_update_readmes:
  depends_on: [step_t6_1_write_nav]
  shell:
    command: |
      for file in schema/README.md roadmap/README.md workflows/workflows-README.md; do
        echo -e "\n\nFor comprehensive schema navigation, see: [[../schema/NAVIGATION.md]]" >> "$file"
      done
    fail_on_error: true
  when:
    after_step_succeeds:
      - log:
          to_file_path: "./logs/t6.log"
          event_fields: [step_name, duration_ms]
          level: info
```

Fit analysis: LLM file creation → shell bulk update. Sequential chain.

---

## T6.1 - Write schema/NAVIGATION.md content (DATA_TRANSFORMER, budget=1)

Intent: Create new NAVIGATION.md file with 4 navigation sections.

```yaml
step_t6_1_write_nav:
  generative_entity: "${models.qwen35}"
  depends_on: [step_t5_2_document_gaps]
  prompt: |
    Using gap analysis from {{bookmarks.t5_2_gaps}}, create schema/NAVIGATION.md with 4 comprehensive sections:

    1. "## Quick Start" - 3-5 bullet points for schema newcomers
    2. "## Element Reference" - list step types, loop types, validation strategies
    3. "## Cross-Reference Matrix" - table: Element → Schema Section → Example Workflow
    4. "## Navigation Shortcuts" - 3-5 links to related docs

    Each section must be non-empty.
  model_overrides:
    max_tokens: 2000
    temperature: 0.1
  when:
    after_step_succeeds:
      - save_to:
          - t6_1_nav
          - "schema/NAVIGATION.md"
      - log:
          to_file_path: "./logs/t6.log"
          event_fields: [step_name, duration_ms]
          level: info
```

Fit analysis: LLM step writing new file. Save_to hook creates file directly.

---

## T6.2 - Update READMEs to reference NAVIGATION.md (DATA_TRANSFORMER, budget=1)

Intent: Add NAVIGATION.md link to three README files.

```yaml
step_t6_2_update_readmes:
  depends_on: [step_t6_1_write_nav]
  shell:
    command: |
      for file in schema/README.md roadmap/README.md workflows/workflows-README.md; do
        echo -e "\n\nFor comprehensive schema navigation, see: [[../schema/NAVIGATION.md]]" >> "$file"
      done
    fail_on_error: true
  when:
    after_step_succeeds:
      - log:
          to_file_path: "./logs/t6.log"
          event_fields: [step_name, duration_ms]
          level: info
```

Fit analysis: Shell loop appending to three files. Final step in T6.

---

## T7 - Analyze ADR metadata structure (DATA_TRANSFORMER, budget=1)

Intent: Document ADR metadata organization showing it's appropriately distributed.

```yaml
step_t7_1_document_adr:
  generative_entity: "${models.qwen35}"
  prompt: |
    Analyze ADR metadata organization across: individual ADR files (adr-000*.yml), master index (adr-0000-roadmap-index.yml), and roadmap/metadata/ directory.

    List metadata fields per location, describe structure, and conclude that distribution is intentional and appropriate.
  model_overrides:
    max_tokens: 1500
    temperature: 0.2
  when:
    after_step_succeeds:
      - save_to:
          - t7_1_adr
          - "./outputs/adr-metadata-distribution.md"
      - log:
          to_file_path: "./logs/t7.log"
          event_fields: [step_name, duration_ms]
          level: info
```

Fit analysis: Single LLM analysis step. No dependencies.

---

## T7.1 - Document ADR metadata distribution (DATA_TRANSFORMER, budget=1)

Intent: Create complete documentation of ADR metadata organization.

```yaml
step_t7_1_document_adr:
  generative_entity: "${models.qwen35}"
  prompt: |
    Analyze ADR metadata organization across: individual ADR files (adr-000*.yml), master index (adr-0000-roadmap-index.yml), and roadmap/metadata/ directory.

    List metadata fields per location, describe structure, and conclude that distribution is intentional and appropriate.
  model_overrides:
    max_tokens: 1500
    temperature: 0.2
  when:
    after_step_succeeds:
      - save_to:
          - t7_1_adr
          - "./outputs/adr-metadata-distribution.md"
      - log:
          to_file_path: "./logs/t7.log"
          event_fields: [step_name, duration_ms]
          level: info
```

Fit analysis: Standalone LLM step. Output saved as markdown doc.

---

## T8 - Create metadata README (DATA_TRANSFORMER, budget=1)

Intent: Create roadmap/metadata/README.md explaining ADR metadata organization.

```yaml
step_t8_1_create_readme:
  generative_entity: "${models.qwen35}"
  depends_on: [step_t7_1_document_adr]
  prompt: |
    Using ADR metadata analysis from {{bookmarks.t7_1_adr}}, create roadmap/metadata/README.md explaining:

    1. Why metadata is distributed (self-documenting ADRs, coordination via index, version history)
    2. Files in metadata/ directory and their purpose
    3. Future enhancement: single metadata YAML if directory grows
  model_overrides:
    max_tokens: 1500
    temperature: 0.1
  when:
    after_step_succeeds:
      - save_to:
          - t8_1_readme
          - "roadmap/metadata/README.md"
      - log:
          to_file_path: "./logs/t8.log"
          event_fields: [step_name, duration_ms]
          level: info
```

Fit analysis: LLM step consuming T7 output via bookmark.

---

## T9 - Analyze formatting status (DATA_TRANSFORMER, budget=1)

Intent: Document completed formatting patterns and count unformatted files.

```yaml
step_t9_1_document_patterns:
  generative_entity: "${models.qwen35}"
  prompt: |
    Extract and document 5 formatting patterns that have been applied to 50+ files:

    1. Emoji headings (🔗📋🎯🛠️⚙️🧪)
    2. Foam wiki-link syntax
    3. Bidirectional navigation ("See Also"/"Related")
    4. Cross-directory links
    5. Table of contents (noted as inconsistent)

    Provide 1-sentence example for each pattern.
  model_overrides:
    max_tokens: 1000
    temperature: 0.1
  when:
    after_step_succeeds:
      - save_to:
          - t9_1_patterns
          - "./outputs/applied-formats.md"
      - log:
          to_file_path: "./logs/t9.log"
          event_fields: [step_name, duration_ms]
          level: info

step_t9_2_count_unformatted:
  depends_on: [step_t9_1_document_patterns]
  shell:
    command: |
      formatted=$(find docs/ -name '*.md' -exec grep -l '🔗\|📋\|🎯' {} + | wc -l)
      total=$(find docs/ -name '*.md' | wc -l)
      unformatted=$((total - formatted))
      echo "Total: $total, Formatted: $formatted, Unformatted: $unformatted" > "./outputs/unformatted-count.md"
    fail_on_error: true
  when:
    after_step_succeeds:
      - log:
          to_file_path: "./logs/t9.log"
          event_fields: [step_name, duration_ms]
          level: info
```

Fit analysis: LLM extraction → shell calculation. Sequential chain.

---

## T9.1 - Document completed formatting patterns (DATA_TRANSFORMER, budget=1)

Intent: Extract and list 5 formatting patterns applied to formatted files.

```yaml
step_t9_1_document_patterns:
  generative_entity: "${models.qwen35}"
  prompt: |
    Extract and document 5 formatting patterns that have been applied to 50+ files:

    1. Emoji headings (🔗📋🎯🛠️⚙️🧪)
    2. Foam wiki-link syntax
    3. Bidirectional navigation ("See Also"/"Related")
    4. Cross-directory links
    5. Table of contents (noted as inconsistent)

    Provide 1-sentence example for each pattern.
  model_overrides:
    max_tokens: 1000
    temperature: 0.1
  when:
    after_step_succeeds:
      - save_to:
          - t9_1_patterns
          - "./outputs/applied-formats.md"
      - log:
          to_file_path: "./logs/t9.log"
          event_fields: [step_name, duration_ms]
          level: info
```

Fit analysis: Standalone LLM extraction from context.

---

## T9.2 - Count unformatted files (DATA_TRANSFORMER, budget=1)

Intent: Calculate number of unformatted markdown files.

```yaml
step_t9_2_count_unformatted:
  depends_on: [step_t9_1_document_patterns]
  shell:
    command: |
      formatted=$(find docs/ -name '*.md' -exec grep -l '🔗\|📋\|🎯' {} + | wc -l)
      total=$(find docs/ -name '*.md' | wc -l)
      unformatted=$((total - formatted))
      echo "Total: $total, Formatted: $formatted, Unformatted: $unformatted" > "./outputs/unformatted-count.md"
    fail_on_error: true
  when:
    after_step_succeeds:
      - log:
          to_file_path: "./logs/t9.log"
          event_fields: [step_name, duration_ms]
          level: info
```

Fit analysis: Shell step with inline calculation. Writes count to file.

---

## T10 - Prioritize consolidation roadmap (DATA_TRANSFORMER, budget=1)

Intent: Calculate priority scores for tasks, create 3-phase roadmap.

```yaml
step_t10_1_prioritize:
  generative_entity: "${models.qwen35}"
  depends_on: [step_t9_2_count_unformatted]
  prompt: |
    Given tasks T1-T8 with impact/effort data, calculate priority scores (impact ÷ effort) and categorize as High/Medium/Low.

    Impact: High (T1, T2, T6), Medium (T3, T5, T10, T11), Low (T4, T7, T8, T9, T12)
    Effort: Low (T5, T7, T8, T9.1, T9.2, T10.1, T10.2, T11.1, T11.2, T12.1, T12.2, T12.3), Medium (T1, T3, T4, T6), High (T2)

    Create markdown table with columns: Task ID, Impact, Effort, Priority Score, Priority Category. Sort by Priority Score descending.
  model_overrides:
    max_tokens: 1500
    temperature: 0.1
  when:
    after_step_succeeds:
      - save_to:
          - t10_1_prioritized
          - "./outputs/priority-sorted.md"
      - log:
          to_file_path: "./logs/t10.log"
          event_fields: [step_name, duration_ms]
          level: info

step_t10_2_create_schedule:
  generative_entity: "${models.qwen35}"
  depends_on: [step_t10_1_prioritize]
  prompt: |
    Using priority-sorted tasks from {{bookmarks.t10_1_prioritized}}, create 3-phase roadmap:

    Phase 1 (Week 1): High-Impact Consolidations - T1, T2, T6
    Phase 2 (Weeks 2-4): Continue Formatting - describe 260-file formatting work
    Phase 3 (Week 5): Low-Impact Improvements - T4, T8
  model_overrides:
    max_tokens: 1500
    temperature: 0.1
  when:
    after_step_succeeds:
      - save_to:
          - t10_2_schedule
          - "./outputs/phased-roadmap.md"
      - log:
          to_file_path: "./logs/t10.log"
          event_fields: [step_name, duration_ms]
          level: info
```

Fit analysis: LLM prioritization → LLM scheduling. Sequential chain.

---

## T10.1 - Prioritize by impact/effort ratio (DATA_TRANSFORMER, budget=1)

Intent: Calculate priority scores and sort tasks.

```yaml
step_t10_1_prioritize:
  generative_entity: "${models.qwen35}"
  depends_on: [step_t9_2_count_unformatted]
  prompt: |
    Given tasks T1-T8 with impact/effort data, calculate priority scores (impact ÷ effort) and categorize as High/Medium/Low.

    Impact: High (T1, T2, T6), Medium (T3, T5, T10, T11), Low (T4, T7, T8, T9, T12)
    Effort: Low (T5, T7, T8, T9.1, T9.2, T10.1, T10.2, T11.1, T11.2, T12.1, T12.2, T12.3), Medium (T1, T3, T4, T6), High (T2)

    Create markdown table with columns: Task ID, Impact, Effort, Priority Score, Priority Category. Sort by Priority Score descending.
  model_overrides:
    max_tokens: 1500
    temperature: 0.1
  when:
    after_step_succeeds:
      - save_to:
          - t10_1_prioritized
          - "./outputs/priority-sorted.md"
      - log:
          to_file_path: "./logs/t10.log"
          event_fields: [step_name, duration_ms]
          level: info
```

Fit analysis: LLM calculation step consuming T9 output.

---

## T10.2 - Create phased schedule (DATA_TRANSFORMER, budget=1)

Intent: Assign prioritized tasks to 3 phases.

```yaml
step_t10_2_create_schedule:
  generative_entity: "${models.qwen35}"
  depends_on: [step_t10_1_prioritize]
  prompt: |
    Using priority-sorted tasks from {{bookmarks.t10_1_prioritized}}, create 3-phase roadmap:

    Phase 1 (Week 1): High-Impact Consolidations - T1, T2, T6
    Phase 2 (Weeks 2-4): Continue Formatting - describe 260-file formatting work
    Phase 3 (Week 5): Low-Impact Improvements - T4, T8
  model_overrides:
    max_tokens: 1500
    temperature: 0.1
  when:
    after_step_succeeds:
      - save_to:
          - t10_2_schedule
          - "./outputs/phased-roadmap.md"
      - log:
          to_file_path: "./logs/t10.log"
          event_fields: [step_name, duration_ms]
          level: info
```

Fit analysis: LLM scheduling step consuming T10.1 output.

---

## T11 - Create cross-directory overlap matrix (DATA_TRANSFORMER, budget=1)

Intent: Extract topic-directory mappings from prompt, format as 10×8 markdown table.

```yaml
step_t11_1_extract_mapping:
  generative_entity: "${models.qwen35}"
  prompt: |
    Extract all non-empty cells from the "## 📋 Cross-Directory Overlap Matrix" section of the input prompt. Format as CSV with columns: topic, directory, file.

    Include at least 20 rows covering all 10 topics.
  model_overrides:
    max_tokens: 2000
    temperature: 0.1
  when:
    after_step_succeeds:
      - save_to:
          - t11_1_mapping
          - "./outputs/topic-mapping.csv"
      - log:
          to_file_path: "./logs/t11.log"
          event_fields: [step_name, duration_ms]
          level: info

step_t11_2_format_table:
  generative_entity: "${models.qwen35}"
  depends_on: [step_t11_1_extract_mapping]
  prompt: |
    Using topic-mapping.csv from {{bookmarks.t11_1_mapping}}, create markdown overlap matrix with:

    - Header row: | Topic | guides/ | plans/ | requirements/ | roadmap/ | schema/ | workflows/ | research/ | benchmark dirs |
    - 10 data rows (one per topic)
    - Cells contain file name or "N/A"

    Total dimensions: 10 topics × 8 directories = 80 cells.
  model_overrides:
    max_tokens: 2000
    temperature: 0.1
  when:
    after_step_succeeds:
      - save_to:
          - t11_2_matrix
          - "./outputs/overlap-matrix.md"
      - log:
          to_file_path: "./logs/t11.log"
          event_fields: [step_name, duration_ms]
          level: info
```

Fit analysis: LLM extraction → LLM table formatting. Sequential chain.

---

## T11.1 - Extract topic-directory mapping (DATA_TRANSFORMER, budget=1)

Intent: Extract overlap matrix data as CSV.

```yaml
step_t11_1_extract_mapping:
  generative_entity: "${models.qwen35}"
  prompt: |
    Extract all non-empty cells from the "## 📋 Cross-Directory Overlap Matrix" section of the input prompt. Format as CSV with columns: topic, directory, file.

    Include at least 20 rows covering all 10 topics.
  model_overrides:
    max_tokens: 2000
    temperature: 0.1
  when:
    after_step_succeeds:
      - save_to:
          - t11_1_mapping
          - "./outputs/topic-mapping.csv"
      - log:
          to_file_path: "./logs/t11.log"
          event_fields: [step_name, duration_ms]
          level: info
```

Fit analysis: LLM extraction from input prompt context.

---

## T11.2 - Format as markdown table (DATA_TRANSFORMER, budget=1)

Intent: Convert CSV to 10×8 markdown matrix.

```yaml
step_t11_2_format_table:
  generative_entity: "${models.qwen35}"
  depends_on: [step_t11_1_extract_mapping]
  prompt: |
    Using topic-mapping.csv from {{bookmarks.t11_1_mapping}}, create markdown overlap matrix with:

    - Header row: | Topic | guides/ | plans/ | requirements/ | roadmap/ | schema/ | workflows/ | research/ | benchmark dirs |
    - 10 data rows (one per topic)
    - Cells contain file name or "N/A"

    Total dimensions: 10 topics × 8 directories = 80 cells.
  model_overrides:
    max_tokens: 2000
    temperature: 0.1
  when:
    after_step_succeeds:
      - save_to:
          - t11_2_matrix
          - "./outputs/overlap-matrix.md"
      - log:
          to_file_path: "./logs/t11.log"
          event_fields: [step_name, duration_ms]
          level: info
```

Fit analysis: LLM formatting step consuming CSV via bookmark.

---

## T12 - Document automation recommendations (DATA_TRANSFORMER, budget=1)

Intent: Write 4 recommendation sections for automation improvements.

```yaml
step_t12_1_link_validation:
  generative_entity: "${models.qwen35}"
  prompt: |
    Write "## Automated Link Validation" recommendation section (2-3 sentences) with implementation hint: use Foam's link check command or bash script with grep.
  model_overrides:
    max_tokens: 500
    temperature: 0.1
  when:
    after_step_succeeds:
      - append_to:
          - t12_recommendations
          - "./outputs/automation-recommendations.md"
      - log:
          to_file_path: "./logs/t12.log"
          event_fields: [step_name, duration_ms]
          level: info

step_t12_2_duplicate_detection:
  generative_entity: "${models.qwen35}"
  depends_on: [step_t12_1_link_validation]
  prompt: |
    Write "## Duplicate Content Detection" recommendation section (2-3 sentences) with implementation hint: use text similarity algorithm (TF-IDF) or manual review on schedule.
  model_overrides:
    max_tokens: 500
    temperature: 0.1
  when:
    after_step_succeeds:
      - append_to:
          - t12_recommendations
          - "./outputs/automation-recommendations.md"
      - log:
          to_file_path: "./logs/t12.log"
          event_fields: [step_name, duration_ms]
          level: info

step_t12_3_metrics_and_ci:
  generative_entity: "${models.qwen35}"
  depends_on: [step_t12_2_duplicate_detection]
  prompt: |
    Write two recommendation sections:

    1. "## Metrics Tracking" (2-3 sentences): recommend tracking link density, orphaned files, doc coverage
    2. "## Doc CI Checks" (2-3 sentences): recommend GitHub Actions workflow to prevent broken links

    Each with implementation hint.
  model_overrides:
    max_tokens: 800
    temperature: 0.1
  when:
    after_step_succeeds:
      - append_to:
          - t12_recommendations
          - "./outputs/automation-recommendations.md"
      - log:
          to_file_path: "./logs/t12.log"
          event_fields: [step_name, duration_ms]
          level: info
```

Fit analysis: Sequential append chain: recommendation 1 → recommendation 2 → recommendations 3&4. All append to same file.

---

## T12.1 - Write link validation recommendation (DATA_TRANSFORMER, budget=1)

Intent: Write first recommendation section.

```yaml
step_t12_1_link_validation:
  generative_entity: "${models.qwen35}"
  prompt: |
    Write "## Automated Link Validation" recommendation section (2-3 sentences) with implementation hint: use Foam's link check command or bash script with grep.
  model_overrides:
    max_tokens: 500
    temperature: 0.1
  when:
    after_step_succeeds:
      - append_to:
          - t12_recommendations
          - "./outputs/automation-recommendations.md"
      - log:
          to_file_path: "./logs/t12.log"
          event_fields: [step_name, duration_ms]
          level: info
```

Fit analysis: LLM step with append_to hook. First of three appenders.

---

## T12.2 - Write duplicate detection recommendation (DATA_TRANSFORMER, budget=1)

Intent: Write second recommendation section.

```yaml
step_t12_2_duplicate_detection:
  generative_entity: "${models.qwen35}"
  depends_on: [step_t12_1_link_validation]
  prompt: |
    Write "## Duplicate Content Detection" recommendation section (2-3 sentences) with implementation hint: use text similarity algorithm (TF-IDF) or manual review on schedule.
  model_overrides:
    max_tokens: 500
    temperature: 0.1
  when:
    after_step_succeeds:
      - append_to:
          - t12_recommendations
          - "./outputs/automation-recommendations.md"
      - log:
          to_file_path: "./logs/t12.log"
          event_fields: [step_name, duration_ms]
          level: info
```

Fit analysis: LLM step with append_to hook. Depends on prior step.

---

## T12.3 - Write metrics and CI recommendations (DATA_TRANSFORMER, budget=1)

Intent: Write final two recommendation sections.

```yaml
step_t12_3_metrics_and_ci:
  generative_entity: "${models.qwen35}"
  depends_on: [step_t12_2_duplicate_detection]
  prompt: |
    Write two recommendation sections:

    1. "## Metrics Tracking" (2-3 sentences): recommend tracking link density, orphaned files, doc coverage
    2. "## Doc CI Checks" (2-3 sentences): recommend GitHub Actions workflow to prevent broken links

    Each with implementation hint.
  model_overrides:
    max_tokens: 800
    temperature: 0.1
  when:
    after_step_succeeds:
      - append_to:
          - t12_recommendations
          - "./outputs/automation-recommendations.md"
      - log:
          to_file_path: "./logs/t12.log"
          event_fields: [step_name, duration_ms]
          level: info
```

Fit analysis: LLM step with append_to hook. Final step in T12.