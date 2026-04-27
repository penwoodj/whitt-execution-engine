#!/bin/bash

# QA Cross-Reference Script for whitt-execution-engine plan tasks
# This script appends QA cross-reference sections to all task files

set -e

# Define QA cross-reference templates
add_qa_cross_ref() {
    local file="$1"
    local qa_criteria="$2"
    local test_case="$3"
    local schema_ref="$4"
    
    # Check if file already has QA Cross-References section
    if grep -q "^## QA Cross-References" "$file"; then
        echo "Skipping $file (already has QA Cross-References)"
        return
    fi
    
    # Append QA cross-reference section
    cat >> "$file" << 'EOF'

---

## QA Cross-References

- **QA Criteria**: ['$qa_criteria']('$file')
- **Test Cases**: ['$test_case']('$file')
- **Schema Ref**: $schema_ref
EOF
    
    echo "Added QA cross-refs to $file"
}

# Phase 00 tasks (12-11)
echo "Processing Phase 00 tasks..."

# Task 02: 01-error-module.md
add_qa_cross_ref "docs/plans/00-foundation/tasks/01-error-module.md" \
    "QA-00-02" \
    "P00-002" \
    "N/A (error handling)"

# Task 03: 02-schema-types.md
add_qa_cross_ref "docs/plans/00-foundation/tasks/02-schema-types.md" \
    "QA-00-03" \
    "P00-003, P00-004" \
    "Lines 14-20 (workflow identification), Lines 27-57 (provider configuration), Lines 64-158 (models section), Lines 196-497 (agentic_workflow section)"

# Task 04: 03-yaml-parser.md
add_qa_cross_ref "docs/plans/00-foundation/tasks/03-yaml-parser.md" \
    "QA-00-04" \
    "P00-005, P00-006, P00-007" \
    "Lines 14-805 (entire unified schema)"

# Task 05: 04-variable-interpolation.md
add_qa_cross_ref "docs/plans/00-foundation/tasks/04-variable-interpolation.md" \
    "QA-00-05" \
    "P00-008, P00-009, P00-010" \
    "Lines 726-740 (variable interpolation syntax section)"

# Task 06: 05-workflow-ir.md
add_qa_cross_ref "docs/plans/00-foundation/tasks/05-workflow-ir.md" \
    "QA-00-06" \
    "P00-011, P00-012" \
    "N/A (internal representation)"

# Task 07: 06-ir-compiler.md
add_qa_cross_ref "docs/plans/00-foundation/tasks/06-ir-compiler.md" \
    "QA-00-07" \
    "P00-013, P00-014" \
    "Full unified schema validation"

# Task 08: 07-dag-validator.md
add_qa_cross_ref "docs/plans/00-foundation/tasks/07-dag-validator.md" \
    "QA-00-08" \
    "P00-015, P00-016" \
    "Lines 293-497 (Pipeline Definition - dependencies, branches)"

# Task 09: 08-policy-compiler.md
add_qa_cross_ref "docs/plans/00-foundation/tasks/08-policy-compiler.md" \
    "QA-00-09" \
    "P00-017, P00-018" \
    "Lines 726-740 (Scope & Inheritance Rules)"

# Task 10: 09-local-storage.md
add_qa_cross_ref "docs/plans/00-foundation/tasks/09-local-storage.md" \
    "QA-00-10" \
    "P00-019, P00-020" \
    "Lines 503-598 (Workspace Configuration)"

# Task 11: 10-workspace-management.md
add_qa_cross_ref "docs/plans/00-foundation/tasks/10-workspace-management.md" \
    "QA-00-11" \
    "P00-021, P00-022" \
    "Lines 503-598 (Workspace Configuration)"

# Task 12: 12-threshold-validation.md
add_qa_cross_ref "docs/plans/00-foundation/tasks/12-threshold-validation.md" \
    "QA-00-12" \
    "P00-023, P00-024, P00-025, P00-026" \
    "Lines 74-88 (Numeric Threshold Guidance)"

# Phase 01 tasks (04-12)
echo "Processing Phase 01 tasks..."

# Task 04: 04-step-executor.md
add_qa_cross_ref "docs/plans/01-core-execution-engine/tasks/04-step-executor.md" \
    "QA-01-05, QA-01-06, QA-01-07, QA-01-08, QA-01-09" \
    "P01-010, P01-011, P01-012, P01-013, P01-014" \
    "Section 3 (Agentic Workflow - step_types), Section 4 (Pipeline Definition - branches, loops)"

# Task 05: 05-loop-runner.md
add_qa_cross_ref "docs/plans/01-core-execution-engine/tasks/05-loop-runner.md" \
    "QA-01-10" \
    "P01-015, P01-016, P01-017, P01-018" \
    "Section 4 (Pipeline Definition - loops)"

# Task 06: 06-branch-evaluator.md
add_qa_cross_ref "docs/plans/01-core-execution-engine/tasks/06-branch-evaluator.md" \
    "QA-01-11" \
    "P01-019" \
    "Section 4 (Pipeline Definition - branches)"

# Task 09: 09-execution-modes.md
add_qa_cross_ref "docs/plans/01-core-execution-engine/tasks/09-execution-modes.md" \
    "QA-01-12" \
    "P01-020, P01-021" \
    "Section 5 (Workflow Execution Strategy - execution_modes)"

# Task 10: 10-logging-framework.md
add_qa_cross_ref "docs/plans/01-core-execution-engine/tasks/10-logging-framework.md" \
    "QA-01-14, QA-01-15" \
    "P01-025, P01-026" \
    "Section 7 (Logging Configuration - 9-level hierarchy), Section 8 (Metrics Configuration)"

# Task 11: 11-metrics-collection.md
add_qa_cross_ref "docs/plans/01-core-execution-engine/tasks/11-metrics-collection.md" \
    "QA-01-15" \
    "P01-027, P01-028, P01-029" \
    "Section 8 (Metrics Configuration - pipeline_metrics, step_metrics, model_metrics, tool_metrics)"

# Task 12: 12-retry-error-handling.md
add_qa_cross_ref "docs/plans/01-core-execution-engine/tasks/12-retry-error-handling.md" \
    "QA-01-16" \
    "P01-030, P01-031, P01-032" \
    "Section 5 (Workflow Execution Strategy - retry_policy, error_handling)"

# Phase 02 tasks (00-12)
echo "Processing Phase 02 tasks..."

# Task 00: 00-cli-foundation.md
add_qa_cross_ref "docs/plans/02-cli-and-llm-backend-integration/tasks/00-cli-foundation.md" \
    "QA-02-01" \
    "P02-001, P02-002, P02-003" \
    "N/A (CLI interface)"

# Task 01: 01-llm-backend-trait.md
add_qa_cross_ref "docs/plans/02-cli-and-llm-backend-integration/tasks/01-llm-backend-trait.md" \
    "QA-02-02" \
    "P02-004, P02-005" \
    "Lines 27-57 (providers section - backend interface)"

# Task 02: 02-lmstudio-backend.md
add_qa_cross_ref "docs/plans/02-cli-and-llm-backend-integration/tasks/02-lmstudio-backend.md" \
    "QA-02-03" \
    "P02-006, P02-007" \
    "Lines 27-57 (lmstudio provider)"

# Task 03: 03-ollama-backend.md
add_qa_cross_ref "docs/plans/02-cli-and-llm-backend-integration/tasks/03-ollama-backend.md" \
    "QA-02-04" \
    "P02-008" \
    "Lines 53-57 (ollama provider)"

# Task 04: 04-llamacpp-backend.md
add_qa_cross_ref "docs/plans/02-cli-and-llm-backend-integration/tasks/04-llamacpp-backend.md" \
    "QA-02-05" \
    "P02-009" \
    "Lines 56-57 (llama_cpp_with_vulkan provider)"

# Task 05: 05-openai-backend.md
add_qa_cross_ref "docs/plans/02-cli-and-llm-backend-integration/tasks/05-openai-backend.md" \
    "QA-02-06" \
    "P02-010" \
    "N/A (OpenAI external provider)"

# Task 06: 06-backend-registry.md
add_qa_cross_ref "docs/plans/02-cli-and-llm-backend-integration/tasks/06-backend-registry.md" \
    "QA-02-07" \
    "P02-011, P02-022" \
    "Lines 64-91 (model routing + backend selection)"

# Task 07: 07-tool-permissions.md
add_qa_cross_ref "docs/plans/02-cli-and-llm-backend-integration/tasks/07-tool-permissions.md" \
    "QA-02-08" \
    "P02-012, P02-013" \
    "Lines 606-677 (tool_permissions)"

# Task 08: 08-tool-execution-framework.md
add_qa_cross_ref "docs/plans/02-cli-and-llm-backend-integration/tasks/08-tool-execution-framework.md" \
    "QA-02-09" \
    "P02-014" \
    "Lines 606-677 (tool permissions)"

# Task 09: 09-sub-workflow-execution.md
add_qa_cross_ref "docs/plans/02-cli-and-llm-backend-integration/tasks/09-sub-workflow-execution.md" \
    "QA-02-10" \
    "P02-015" \
    "Lines 165-191 (sub_workflows)"

# Task 10: 10-code-generation.md
add_qa_cross_ref "docs/plans/02-cli-and-llm-backend-integration/tasks/10-code-generation.md" \
    "QA-02-11" \
    "P02-016, P02-017" \
    "Lines 1-805 (full schema → Rust code)"

# Task 11: 11-rag-integration.md
add_qa_cross_ref "docs/plans/02-cli-and-llm-backend-integration/tasks/11-rag-integration.md" \
    "QA-02-12" \
    "P02-018, P02-019" \
    "Lines 682-697 (memory/rag section)"

# Task 12: 12-self-improvement-loop.md
add_qa_cross_ref "docs/plans/02-cli-and-llm-backend-integration/tasks/12-self-improvement-loop.md" \
    "QA-02-13" \
    "P02-020, P02-021" \
    "N/A (meta-optimization)"

# Phase 03 tasks (00-06)
echo "Processing Phase 03 tasks..."

# Task 00: 00-verifier-interface.md
add_qa_cross_ref "docs/plans/03-quality-loops/tasks/00-verifier-interface.md" \
    "QA-03-01" \
    "P03-001, P03-002, P03-003, P03-004, P03-005" \
    "N/A (verification infrastructure)"

# Task 01: 01-generate-verify-repair-runtime.md
add_qa_cross_ref "docs/plans/03-quality-loops/tasks/01-generate-verify-repair-runtime.md" \
    "QA-03-02" \
    "P03-006, P03-007, P03-008" \
    "Lines 503-536 (advanced execution strategy - quality loops)"

# Task 02: 02-benchmark-harness.md
add_qa_cross_ref "docs/plans/03-quality-loops/tasks/02-benchmark-harness.md" \
    "QA-03-03" \
    "P03-009, P03-010, P03-011" \
    "Lines 688-703 (performance optimization)"

# Task 03: 03-filetype-capability-matrix.md
add_qa_cross_ref "docs/plans/03-quality-loops/tasks/03-filetype-capability-matrix.md" \
    "QA-03-04" \
    "P03-012, P03-013, P03-014" \
    "N/A (knowledge graph of capabilities)"

# Task 04: 04-artifact-workflow-library.md
add_qa_cross_ref "docs/plans/03-quality-loops/tasks/04-artifact-workflow-library.md" \
    "QA-03-05" \
    "P03-015, P03-016" \
    "N/A (workflow library)"

# Task 05: 05-report-generation.md
add_qa_cross_ref "docs/plans/03-quality-loops/tasks/05-report-generation.md" \
    "QA-03-06" \
    "P03-017, P03-018, P03-019, P03-020" \
    "N/A (reporting infrastructure)"

# Task 06: 06-quality-dashboard-integration.md
add_qa_cross_ref "docs/plans/03-quality-loops/tasks/06-quality-dashboard-integration.md" \
    "QA-03-07" \
    "P03-021, P03-022, P03-023" \
    "N/A (dashboard API)"

echo "Done adding QA cross-refs to task files!"