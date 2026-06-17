# Extracted Prompt Dataset

Extracted from OpenCode session database on 2026-06-17
Total prompts: 15
Curated active: 11 (prompts 05-15)
Excluded (context dumps): 4 (prompts 01-04, oversized)

## Curation Status

| # | File | Chars | Status | Notes |
|---|------|-------|--------|-------|
| 1 | prompt-01-*.md | 120300 | ❌ EXCLUDED | Context dump (120K chars), not a task prompt |
| 2 | prompt-02-*.md | 102828 | ❌ EXCLUDED | Context dump (102K chars), not a task prompt |
| 3 | prompt-03-*.md | 53544 | ❌ EXCLUDED | Error log paste, not an agentic prompt |
| 4 | prompt-04-kb.md | 38967 | ❌ EXCLUDED | KB article paste, not an agentic prompt |
| 5 | prompt-05-task.md | 16684 | ✅ ACTIVE | Engineering task with full context |
| 6 | prompt-06-*.md | 16183 | ✅ ACTIVE | PDF generation task (WeasyPrint) |
| 7 | prompt-07-task.md | 15561 | ✅ ACTIVE | Engineering task |
| 8 | prompt-08-*.md | 15505 | ✅ ACTIVE | Debugging task with error context |
| 9 | prompt-09-*.md | 13245 | ✅ ACTIVE | UI/CSS update task |
| 10 | prompt-10-*.md | 12137 | ✅ ACTIVE | Analysis task |
| 11 | prompt-11-*.md | 12137 | ✅ ACTIVE | Analysis task (duplicate of 10) |
| 12 | prompt-12-*.md | 11944 | ✅ ACTIVE | Skill instruction task |
| 13 | prompt-13-task.md | 11728 | ✅ ACTIVE | Engineering task |
| 14 | prompt-14-*.md | 11288 | ✅ **BASELINE** | Parallel inference task — clean, well-scoped |
| 15 | prompt-15-*.md | 11189 | ✅ **BASELINE** | ReAct layer implementation — clean, well-scoped |

## Baseline Task Breakdowns

Manual task breakdowns created by opencode (Sisyphus) for quality comparison:
- `baselines/baseline-14-parallel-inference.md` — for prompt-14
- `baselines/baseline-15-react-layer.md` — for prompt-15

These baselines serve as the quality bar that SW1 must surpass via iterative improvement.

## Session Sources

| # | Session ID | Description |
|---|-----------|-------------|
| 14 | ses_17b1ba6e5ffeVeItkFuCd221Lg | Parallel inference implementation |
| 15 | ses_237b25c23ffenPrlbYu6fi64en | ReAct layer implementation |
| 5-13 | various | Mixed engineering tasks from project sessions |
