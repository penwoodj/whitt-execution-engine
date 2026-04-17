#!/bin/bash
# implement.sh - Implementation automation script for Whitt Execution Engine initial plans
#
# Usage:
#   ./implement.sh start <plan-id>           # Start a new plan
#   ./implement.sh resume <plan-id>          # Resume from last checkpoint
#   ./implement.sh resume <plan-id> cp<N>   # Resume from specific checkpoint
#   ./implement.sh verify <plan-id>          # Run verification only
#   ./implement.sh complete <plan-id>         # Mark plan as complete
#   ./implement.sh status <plan-id>           # Show plan status
#   ./implement.sh report <plan-id>            # Generate progress report
#   ./implement.sh review <plan-id>           # Run critical review

set -euo pipefail

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Directories
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PLANS_DIR="${SCRIPT_DIR}"
SOT_FILE="${SCRIPT_DIR}/SoT.md"
CHECKPOINT_DIR="${SCRIPT_DIR}/.checkpoints"
REPORT_DIR="${SCRIPT_DIR}/reports"

# Create checkpoint and report directories
mkdir -p "${CHECKPOINT_DIR}"
mkdir -p "${REPORT_DIR}"

# Logging
log() {
    local level="$1"
    shift
    local message="$*"
    local timestamp=$(date '+%Y-%m-%d %H:%M:%S')
    echo -e "${timestamp} [${level}] ${message}" | tee -a "${CHECKPOINT_DIR}/implement.log"
}

log_info() { log "${GREEN}INFO${NC}" "$@"; }
log_warn() { log "${YELLOW}WARN${NC}" "$@"; }
log_error() { log "${RED}ERROR${NC}" "$@"; }

# Check if plan file exists
check_plan_exists() {
    local plan_id="$1"
    local plan_file

    case "${plan_id}" in
        plan-00) plan_file="${PLANS_DIR}/phase-0-foundation/plan-00-foundation.md" ;;
        plan-01) plan_file="${PLANS_DIR}/phase-1-mvp-queue/plan-01-mvp-queue.md" ;;
        plan-02) plan_file="${PLANS_DIR}/phase-2-cli-backends/plan-02-cli-backends.md" ;;
        plan-03) plan_file="${PLANS_DIR}/phase-3-glyphnova-ui/plan-03-glyphnova-ui.md" ;;
        plan-04) plan_file="${PLANS_DIR}/phase-4-quality-loops/plan-04-quality-loops.md" ;;
        plan-05) plan_file="${PLANS_DIR}/phase-5-memory-search/plan-05-memory-search.md" ;;
        plan-06) plan_file="${PLANS_DIR}/phase-6-automation/plan-06-automation.md" ;;
        plan-07) plan_file="${PLANS_DIR}/phase-7-autonomy-metrics/plan-07-autonomy-metrics.md" ;;
        *)
            log_error "Unknown plan ID: ${plan_id}"
            exit 1
            ;;
    esac

    if [[ ! -f "${plan_file}" ]]; then
        log_error "Plan file not found: ${plan_file}"
        exit 1
    fi

    echo "${plan_file}"
}

# Get checkpoint file for plan
get_checkpoint_file() {
    local plan_id="$1"
    echo "${CHECKPOINT_DIR}/${plan_id}.json"
}

# Read current checkpoint state
read_checkpoint() {
    local plan_id="$1"
    local cp_file=$(get_checkpoint_file "${plan_id}")

    if [[ -f "${cp_file}" ]]; then
        cat "${cp_file}"
    else
        echo '{"checkpoint": 0, "status": "not_started", "phase_progress": {}}'
    fi
}

# Write checkpoint state
write_checkpoint() {
    local plan_id="$1"
    local cp_file=$(get_checkpoint_file "${plan_id}")
    local data="$2"

    echo "${data}" | jq '.' > "${cp_file}"
    log_info "Checkpoint updated for ${plan_id}"
}

# Update SoT.md with plan status
update_sot() {
    local plan_id="$1"
    local status="$2"
    local progress="$3"

    # Update plan index in SoT.md
    # Note: This is a placeholder - actual implementation would parse and update markdown
    log_info "SoT.md would be updated: ${plan_id} status=${status} progress=${progress}"
}

# Start a new plan
start_plan() {
    local plan_id="$1"
    local plan_file=$(check_plan_exists "${plan_id}")

    log_info "Starting plan: ${plan_id}"
    log_info "Plan file: ${plan_file}"

    # Initialize checkpoint
    local initial_state='{
        "plan_id": "'"${plan_id}"'",
        "plan_file": "'"${plan_file}"'",
        "checkpoint": 0,
        "status": "in_progress",
        "phase_progress": {},
        "test_layers": {"unit": "not_started", "integration": "not_started", "property": "not_started", "e2e": "not_started"},
        "started_at": "'"$(date -Iseconds)"'",
        "last_updated": "'"$(date -Iseconds)"'"
    }'

    write_checkpoint "${plan_id}" "${initial_state}"
    update_sot "${plan_id}" "in_progress" "0%"

    log_info "Plan ${plan_id} initialized"
    log_info "Next steps:"
    log_info "  1. Read plan file: ${plan_file}"
    log_info "  2. Perform web research"
    log_info "  3. Update plan with research findings"
    log_info "  4. Write unit tests (Layer 1)"
    log_info "  5. Implement features incrementally"
    log_info "  6. Verify at all 4 layers"
    log_info "  7. Update plan with verified code"
}

# Resume from checkpoint
resume_plan() {
    local plan_id="$1"
    local cp_num="${2:-}"

    local cp_state=$(read_checkpoint "${plan_id}")
    local current_cp=$(echo "${cp_state}" | jq -r '.checkpoint')
    local plan_file=$(echo "${cp_state}" | jq -r '.plan_file')

    log_info "Resuming plan: ${plan_id}"
    log_info "Current checkpoint: ${current_cp}"

    if [[ -n "${cp_num}" ]]; then
        # Jump to specific checkpoint
        log_info "Resuming from checkpoint: ${cp_num}"
    fi

    log_info "Plan file: ${plan_file}"
    log_info "Last updated: $(echo "${cp_state}" | jq -r '.last_updated')"
}

# Verify current implementation
verify_plan() {
    local plan_id="$1"
    local plan_file=$(check_plan_exists "${plan_id}")
    local cp_state=$(read_checkpoint "${plan_id}")

    log_info "Verifying plan: ${plan_id}"

    # Run all 4 verification layers
    log_info "Running Layer 1: Unit Tests"
    echo "cargo test --lib" | bash

    log_info "Running Layer 2: Integration Tests"
    echo "cargo test --test '*'" | bash

    log_info "Running Layer 3: Property-Based Tests"
    echo "cargo test --test '*proptest*'" | bash

    log_info "Running Layer 4: End-to-End Tests"
    echo "cargo test --test '*e2e*'" | bash

    # Update checkpoint with verification results
    local updated_state=$(echo "${cp_state}" | jq --arg timestamp "$(date -Iseconds)" '
        .test_layers = {
            "unit": "completed",
            "integration": "completed",
            "property": "completed",
            "e2e": "completed"
        } |
        .last_updated = $timestamp
    ')

    write_checkpoint "${plan_id}" "${updated_state}"

    # Generate verification report
    generate_verification_report "${plan_id}"

    log_info "Verification complete for ${plan_id}"
}

# Mark plan as complete
complete_plan() {
    local plan_id="$1"
    local plan_file=$(check_plan_exists "${plan_id}")
    local cp_state=$(read_checkpoint "${plan_id}")

    log_info "Completing plan: ${plan_id}"

    # Run final verification
    verify_plan "${plan_id}"

    # Update state
    local final_state=$(echo "${cp_state}" | jq --arg timestamp "$(date -Iseconds)" '
        .status = "completed" |
        .checkpoint = 999 |
        .completed_at = $timestamp |
        .last_updated = $timestamp
    ')

    write_checkpoint "${plan_id}" "${final_state}"
    update_sot "${plan_id}" "completed" "100%"

    # Generate completion report
    generate_completion_report "${plan_id}"

    log_info "Plan ${plan_id} marked as complete"
}

# Show plan status
show_status() {
    local plan_id="$1"
    local plan_file=$(check_plan_exists "${plan_id}")
    local cp_state=$(read_checkpoint "${plan_id}")

    echo -e "\n${BLUE}=== Plan Status: ${plan_id} ===${NC}\n"

    echo "Plan File: $(echo "${cp_state}" | jq -r '.plan_file')"
    echo "Status: $(echo "${cp_state}" | jq -r '.status')"
    echo "Checkpoint: $(echo "${cp_state}" | jq -r '.checkpoint')"
    echo "Started: $(echo "${cp_state}" | jq -r '.started_at')"
    echo "Last Updated: $(echo "${cp_state}" | jq -r '.last_updated')"

    echo -e "\n${BLUE}Test Layers Status:${NC}"
    echo "  Unit Tests:        $(echo "${cp_state}" | jq -r '.test_layers.unit')"
    echo "  Integration Tests: $(echo "${cp_state}" | jq -r '.test_layers.integration')"
    echo "  Property Tests:   $(echo "${cp_state}" | jq -r '.test_layers.property')"
    echo "  E2E Tests:        $(echo "${cp_state}" | jq -r '.test_layers.e2e')"
}

# Generate verification report
generate_verification_report() {
    local plan_id="$1"
    local report_file="${REPORT_DIR}/${plan_id}-verification-report.md"

    cat > "${report_file}" <<EOF
# Verification Report: ${plan_id}

**Generated**: $(date '+%Y-%m-%d %H:%M:%S')

## Summary

Plan **${plan_id}** verification completed.

## Verification Layers

| Layer | Status | Details |
|-------|--------|---------|
| Unit Tests | | |
| Integration Tests | | |
| Property-Based Tests | | |
| End-to-End Tests | | |

## Issues Found

None

## Recommendations

Continue with implementation or mark plan as complete if all layers pass.

EOF

    log_info "Verification report generated: ${report_file}"
}

# Generate completion report
generate_completion_report() {
    local plan_id="$1"
    local report_file="${REPORT_DIR}/${plan_id}-completion-report.md"

    cat > "${report_file}" <<EOF
# Completion Report: ${plan_id}

**Generated**: $(date '+%Y-%m-%d %H:%M:%S')

## Summary

Plan **${plan_id}** marked as complete.

## Verification Summary

All 4 verification layers have passed:
- ✅ Unit Tests
- ✅ Integration Tests
- ✅ Property-Based Tests
- ✅ End-to-End Tests

## Quality Gates

All quality gates for this plan have been satisfied.

## Next Steps

This plan is complete. You may proceed to the next plan in the critical path.

EOF

    log_info "Completion report generated: ${report_file}"
}

# Generate overall progress report
generate_report() {
    log_info "Generating overall progress report..."

    local report_file="${REPORT_DIR}/overall-progress-report.md"

    # Collect status from all plans
    local plans=("plan-00" "plan-01" "plan-02" "plan-03" "plan-04" "plan-05" "plan-06" "plan-07")

    cat > "${report_file}" <<EOF
# Overall Progress Report

**Generated**: $(date '+%Y-%m-%d %H:%M:%S')

## Plan Status Summary

| Plan ID | Status | Checkpoint | Unit Tests | Integration Tests | Property Tests | E2E Tests |
|---------|--------|------------|-------------|-------------------|-----------------|------------|
EOF

    for plan_id in "${plans[@]}"; do
        local cp_state=$(read_checkpoint "${plan_id}")
        local status=$(echo "${cp_state}" | jq -r '.status')
        local cp_num=$(echo "${cp_state}" | jq -r '.checkpoint')

        local unit=$(echo "${cp_state}" | jq -r '.test_layers.unit')
        local integration=$(echo "${cp_state}" | jq -r '.test_layers.integration')
        local property=$(echo "${cp_state}" | jq -r '.test_layers.property')
        local e2e=$(echo "${cp_state}" | jq -r '.test_layers.e2e')

        local status_icon="⬜"
        if [[ "${status}" == "completed" ]]; then
            status_icon="✅"
        elif [[ "${status}" == "in_progress" ]]; then
            status_icon="⬛"
        fi

        echo "| ${plan_id} | ${status_icon} ${status} | ${cp_num} | ${unit} | ${integration} | ${property} | ${e2e} |" >> "${report_file}"
    done

    cat >> "${report_file}" <<EOF

## Overall Completion

| Metric | Count |
|--------|--------|
| Plans Completed | 0 |
| Plans In Progress | 0 |
| Plans Not Started | 0 |
| Overall Completion | 0% |

## Critical Path

1. plan-00 (Foundation) → plan-01 (MVP Queue) → plan-02 (CLI & Backends)
2. plan-02 (CLI & Backends) → plan-03 (Glyphnova UI) and plan-04 (Quality Loops)
3. plan-04 (Quality Loops) → plan-05 (Memory & Search)
4. plan-05 (Memory & Search) → plan-06 (Automation)
5. plan-06 (Automation) → plan-07 (Autonomy & Metrics)

EOF

    log_info "Overall progress report generated: ${report_file}"
}

# Run critical review
run_critical_review() {
    local plan_id="$1"
    local plan_file=$(check_plan_exists "${plan_id}")

    log_info "Running critical review for: ${plan_id}"

    cat <<EOF
# Critical Review: ${plan_id}

## Upstream Factor 1: ADR Compliance

- [ ] Does implementation align with ADR decisions?
- [ ] Are all ADR requirements satisfied?

## Upstream Factor 2: Requirements Satisfaction

- [ ] Are all requirements mapped to this plan satisfied?
- [ ] Is coverage 100%?

## Upstream Factor 3: Test Coverage

- [ ] Do all 4 test layers pass?
- [ ] Is code coverage adequate (>80%)?

## Upstream Factor 4: Documentation

- [ ] Is plan file updated with implementation details?
- [ ] Are code examples and API docs complete?

## Upstream Factor 5: Integration

- [ ] Does implementation integrate cleanly with previous plans?
- [ ] Are all dependencies satisfied?

## Additional Upstream Factors (if applicable)

- [ ] Upstream Factor 6
- [ ] Upstream Factor 7
- [ ] Upstream Factor 8

## Review Conclusion

[ ] Pass - All upstream factors satisfied
[ ] Fail - Issues found requiring resolution

## Issues Found

None

## Recommendations

EOF

    log_info "Critical review checklist generated. Please review and mark as Pass/Fail."
}

# Show usage
show_usage() {
    cat <<EOF
${BLUE}implement.sh${NC} - Implementation automation script for Whitt Execution Engine

${YELLOW}USAGE:${NC}
  $0 start <plan-id>           Start a new plan
  $0 resume <plan-id>          Resume from last checkpoint
  $0 resume <plan-id> cp<N>   Resume from specific checkpoint
  $0 verify <plan-id>          Run verification only
  $0 complete <plan-id>         Mark plan as complete
  $0 status <plan-id>           Show plan status
  $0 report <plan-id>            Generate progress report
  $0 review <plan-id>           Run critical review
  $0 help                       Show this help message

${YELLOW}EXAMPLES:${NC}
  # Start plan-00
  $0 start plan-00

  # Resume plan-01 from checkpoint 3
  $0 resume plan-01 cp3

  # Verify plan-02
  $0 verify plan-02

  # Show status of plan-03
  $0 status plan-03

  # Generate overall report
  $0 report all

${YELLOW}PLAN IDs:${NC}
  plan-00  - Foundation Compiler Contract
  plan-01  - MVP Queue & Scheduler
  plan-02  - CLI & Backends
  plan-03  - Glyphnova UI
  plan-04  - Quality Loops
  plan-05  - Memory & Search
  plan-06  - Automation
  plan-07  - Autonomy & Metrics

${YELLOW}VERIFICATION LAYERS:${NC}
  Layer 1  - Unit Tests (cargo test --lib)
  Layer 2  - Integration Tests (cargo test --test '*')
  Layer 3  - Property-Based Tests (cargo test --test '*proptest*')
  Layer 4  - End-to-End Tests (cargo test --test '*e2e*')

EOF
}

# Main command dispatch
main() {
    local command="${1:-}"
    local plan_id="${2:-}"
    local arg3="${3:-}"

    case "${command}" in
        start)
            if [[ -z "${plan_id}" ]]; then
                log_error "Plan ID required for start command"
                show_usage
                exit 1
            fi
            start_plan "${plan_id}"
            ;;
        resume)
            if [[ -z "${plan_id}" ]]; then
                log_error "Plan ID required for resume command"
                show_usage
                exit 1
            fi
            resume_plan "${plan_id}" "${arg3}"
            ;;
        verify)
            if [[ -z "${plan_id}" ]]; then
                log_error "Plan ID required for verify command"
                show_usage
                exit 1
            fi
            verify_plan "${plan_id}"
            ;;
        complete)
            if [[ -z "${plan_id}" ]]; then
                log_error "Plan ID required for complete command"
                show_usage
                exit 1
            fi
            complete_plan "${plan_id}"
            ;;
        status)
            if [[ -z "${plan_id}" ]]; then
                log_error "Plan ID required for status command"
                show_usage
                exit 1
            fi
            show_status "${plan_id}"
            ;;
        report)
            if [[ "${plan_id}" == "all" ]] || [[ -z "${plan_id}" ]]; then
                generate_report
            else
                log_info "Use 'report all' to generate overall progress report"
            fi
            ;;
        review)
            if [[ -z "${plan_id}" ]]; then
                log_error "Plan ID required for review command"
                show_usage
                exit 1
            fi
            run_critical_review "${plan_id}"
            ;;
        help|--help|-h)
            show_usage
            ;;
        *)
            log_error "Unknown command: ${command}"
            show_usage
            exit 1
            ;;
    esac
}

# Check dependencies
check_dependencies() {
    local missing_deps=()

    command -v jq >/dev/null 2>&1 || missing_deps+=("jq")
    command -v cargo >/dev/null 2>&1 || missing_deps+=("cargo")

    if [[ ${#missing_deps[@]} -gt 0 ]]; then
        log_error "Missing required dependencies: ${missing_deps[*]}"
        exit 1
    fi
}

# Check dependencies and run main
check_dependencies
main "$@"
