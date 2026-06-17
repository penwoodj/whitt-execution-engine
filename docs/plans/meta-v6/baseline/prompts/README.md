# Baseline Prompt Corpus for META-v6 Workflow Generator

## Overview

This directory contains 10 real, complex user prompts extracted from actual opencode sessions. These prompts serve as the baseline dataset for evaluating META-v6 workflow generator quality.

## Corpus Statistics

| Prompt ID | Length (chars) | Topic | Complexity |
|-----------|----------------|-------|------------|
| prompt-01 | 11,395 | Documentation consolidation analysis | High - tables, YAML examples, priority ranking |
| prompt-02 | 11,725 | Reorganize opencode/docs/ structure | High - file system operations, cross-references |
| prompt-03 | 4,534 | Full index plan creation | Medium - planning, indexing strategy |
| prompt-04 | 13,067 | Developer guide creation | High - comprehensive guide, code examples |
| prompt-05 | 13,067 | Phase 06 validation/test filling | High - test suite, validation criteria |
| prompt-06 | 13,067 | Phase 05 test file expansion | High - test patterns, coverage analysis |
| prompt-07 | 13,067 | Phase 05 validation file expansion | High - validation logic, acceptance criteria |
| prompt-08 | 17,075 | Plan phase file v1 reference updates | Very High - multi-file updates, references |
| prompt-09 | 13,067 | Missing acceptance criteria creation | High - requirements analysis, documentation |
| prompt-10 | 13,068 | Inconsistency deduplication | High - data quality, duplicate detection |

**Total Corpus Size**: 121,472 characters  
**Average Prompt Length**: 12,147 characters  
**Complexity Range**: Medium to Very High

## Complexity Characteristics

All prompts demonstrate:
- **Multi-step requirements**: Sequential tasks with dependencies
- **Technical specifications**: YAML schemas, file paths, code examples
- **Context dependencies**: References to existing documentation/architecture
- **Domain knowledge**: Rust, testing strategies, documentation systems, workflow engines

## Usage in META-v6 Testing

These prompts are used as inputs for SW1-SW5 quality benchmarking:

1. **SW1 (Schema Coverage)**: Verify generated YAMLs reference correct schema sections
2. **SW2 (Action Completeness)**: Verify all actions needed to complete prompts are present
3. **SW3 (Complexity Match)**: Verify generated workflows match prompt complexity
4. **SW4 (Hook Appropriateness)**: Verify hooks are used appropriately for prompt intent
5. **SW5 (Interpolation Correctness)**: Verify template interpolation resolves correctly

## Extraction Method

Prompts extracted from opencode session database (`~/.local/share/opencode/opencode.db`):
- Query: `message` table where `role="user"` and `LENGTH(data) > 3000`
- Source: `summary.diffs[].before` field (actual user prompt text)
- Filter: Substantive content (>500 chars), multi-step requirements
- Saved as: Markdown with metadata (session ID, message ID, length)

## File Naming Convention

`prompt-NN.md` where NN = 01-10 (chronological extraction order)

Each file contains:
- Metadata header (session ID, title, message ID, lengths)
- Original user prompt text
- Complexity assessment

## Maintenance

- **Add new prompts**: Extract additional complex prompts from newer sessions
- **Remove outliers**: Delete prompts that don't demonstrate agentic complexity
- **Version control**: Track corpus changes in git for reproducible testing

## Related Documentation

- [META-v6 Quality Benchmarking](../../META-V6-QUALITY-BENCHMARKING.md)
- [Workflow Generator Testing](../../TESTING.md)
- [Unified Workflow Schema](../../../schema/unified-workflow-schema.yml)
