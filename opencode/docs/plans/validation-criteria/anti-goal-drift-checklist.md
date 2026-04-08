# Anti-Goal-Drift Checklist

**Purpose:** Systematic detection of scope and architectural drift during AgentSDK development
**Usage:** Run after completing each phase, checkpoint, or significant change
**Version:** 2.0
**Last Updated:** 2026-04-06

---

## Overview

This checklist provides a systematic approach to detecting goal drift during development. Goal drift occurs when implementation deviates from original requirements, architecture, or scope. This checklist helps identify drift early so corrective action can be taken.

**When to Run:**
- After completing each phase
- After completing each checkpoint
- After adding new ADRs
- After modifying schema
- After performance regression detected

---

## 1. Requirements Drift Checklist

**Purpose:** Ensure implementation matches original requirements

### Questions

1. **Feature Completeness:**
   - [ ] All specified features implemented?
   - [ ] No missing features from requirements?
   - [ ] All acceptance criteria met?

2. **Feature Accuracy:**
   - [ ] Features work as specified?
   - [ ] No behavior deviations from spec?
   - [ ] Edge cases handled per requirements?

3. **Feature Prioritization:**
   - [ ] High-priority features implemented?
   - [ ] Low-priority features deferred correctly?
   - [ ] No unauthorized feature additions?

4. **Quality Requirements:**
   - [ ] Performance meets requirements?
   - [ ] Reliability meets requirements?
   - [ ] Security meets requirements?

### Detection Methods

**Manual Review:**
- Compare implementation against requirements document
- Review acceptance criteria checklist
- Check feature priority list

**Automated Checks:**
```bash
# Verify all acceptance criteria met
cargo run --bin acceptance_criteria -- verify

# Check for unauthorized features
cargo run --bin feature_audit -- --requirements requirements.md
```

### Corrective Actions

**Minor Drift (<5% impact):**
- Document drift in drift log
- Get architectural approval
- Update requirements if necessary

**Major Drift (>5% impact):**
- Schedule architecture review
- Create drift remediation plan
- Get stakeholder approval

**Critical Drift (blocks phase exit):**
- Immediate rollback
- Root cause analysis
- Process review to prevent recurrence

---

## 2. Architecture Drift Checklist

**Purpose:** Ensure implementation follows ADRs

### Questions

1. **Module Boundaries:**
   - [ ] Modules follow ADR-defined boundaries?
   - [ ] No cross-module violations?
   - [ ] Dependency rules followed?

2. **Component Interaction:**
   - [ ] Interactions match ADR diagrams?
   - [ ] Communication patterns correct?
   - [ ] No unauthorized direct dependencies?

3. **Data Flow:**
   - [ ] Data flow matches ADR specifications?
   - [ ] No unauthorized data access?
   - [ ] Data structures match schemas?

4. **Deployment Architecture:**
   - [ ] Deployment matches ADR?
   - [ ] Component placement correct?
   - [ ] No unauthorized architectural changes?

### Detection Methods

**Manual Review:**
- Compare implementation against ADRs
- Review architecture diagrams
- Check module boundaries

**Automated Checks:**
```bash
# Verify ADR compliance
cargo run --bin adr_compliance -- --all

# Check for cross-module violations
cargo run --bin dependency_analysis -- --check-boundaries

# Verify deployment architecture
cargo run --bin deployment_audit -- --verify-adr
```

### Corrective Actions

**Minor Drift:**
- Document drift
- Update ADR if necessary
- Get architectural approval

**Major Drift:**
- Architecture review
- Refactor to match ADR
- Update ADR if appropriate

**Critical Drift:**
- Immediate rollback
- Root cause analysis
- Architecture governance review

---

## 3. Scope Creep Checklist

**Purpose:** Prevent adding features not in original scope

### Questions

1. **Feature Addition:**
   - [ ] New features in original scope?
   - [ ] New features necessary?
   - [ ] New features prioritized correctly?

2. **Phase Boundaries:**
   - [ ] Phase 0 features only in Phase 0?
   - [ ] Phase 1 features only in Phase 1?
   - [ ] No features from later phases?

3. **Feature Expansion:**
   - [ ] Features expanded beyond scope?
   - [ ] Scope creep detected early?
   - [ ] Unauthorized additions identified?

4. **Phase Dependencies:**
   - [ ] Phase dependencies respected?
   - [ ] No phase skipping?
   - [ ] No phase overlap?

### Detection Methods

**Manual Review:**
- Compare features against phase requirements
- Review feature scope documents
- Check phase boundaries

**Automated Checks:**
```bash
# Verify phase boundaries
cargo run --bin phase_audit -- --verify-boundaries

# Check for scope creep
cargo run --bin scope_audit -- --compare requirements.md

# Verify phase dependencies
cargo run --bin dependency_audit -- --check-phase-deps
```

### Corrective Actions

**Minor Scope Creep:**
- Document scope creep
- Get approval for new scope
- Update phase requirements if necessary

**Major Scope Creep:**
- Remove unauthorized features
- Defer to correct phase
- Get stakeholder approval

**Critical Scope Creep:**
- Immediate rollback
- Root cause analysis
- Scope governance review

---

## 4. Performance Drift Checklist

**Purpose:** Ensure performance meets targets

### Questions

1. **Performance Targets:**
   - [ ] Latency meets targets?
   - [ ] Throughput meets targets?
   - [ ] Resource usage within limits?

2. **Performance Regression:**
   - [ ] No performance regression > 10%?
   - [ ] Regressions investigated?
   - [ ] Regressions fixed?

3. **Benchmarks:**
   - [ ] All benchmarks run?
   - [ ] Benchmarks pass?
   - [ ] Benchmark results documented?

4. **SLA Compliance:**
   - [ ] SLAs met?
   - [ ] SLA violations documented?
   - [ ] SLA violations remediated?

### Detection Methods

**Manual Review:**
- Review benchmark results
- Check performance metrics
- Verify SLA compliance

**Automated Checks:**
```bash
# Run all benchmarks
cargo bench --bench all

# Check for performance regression
cargo run --bin performance_audit -- --check-regression

# Verify SLA compliance
cargo run --bin sla_audit -- --verify-compliance
```

### Corrective Actions

**Minor Regression (<5%):**
- Document regression
- Monitor for improvement
- Optimize if persistent

**Major Regression (5-10%):**
- Investigate root cause
- Optimize performance
- Update performance ADR if necessary

**Critical Regression (>10%):**
- Immediate rollback
- Root cause analysis
- Performance optimization

---

## 5. Testing Drift Checklist

**Purpose:** Ensure test coverage maintained

### Questions

1. **Test Coverage:**
   - [ ] Coverage >= 90%?
   - [ ] No uncovered critical paths?
   - [ ] Coverage trends monitored?

2. **Test Quality:**
   - [ ] All tests passing?
   - [ ] Tests maintainable?
   - [ ] Tests meaningful?

3. **Test Types:**
   - [ ] Unit tests present?
   - [ ] Integration tests present?
   - [ ] E2E tests present?

4. **Property Tests:**
   - [ ] Property tests invariants hold?
   - [ ] Property tests run regularly?
   - [ ] Property tests comprehensive?

### Detection Methods

**Manual Review:**
- Review test coverage reports
- Check test quality
- Verify test types present

**Automated Checks:**
```bash
# Run all tests
cargo test --all

# Check test coverage
cargo tarpaulin --out Html --workspace

# Run property tests
PROPTEST_NUMBER_OF_TESTS=1000 cargo test --lib property_based
```

### Corrective Actions

**Coverage < 90%:**
- Add missing tests
- Document uncovered code
- Prioritize critical paths

**Test Quality Issues:**
- Refactor tests
- Improve test maintainability
- Update test documentation

**Missing Test Types:**
- Add missing test types
- Ensure comprehensive coverage
- Document test strategy

---

## 6. Documentation Drift Checklist

**Purpose:** Ensure documentation up to date

### Questions

1. **Code Documentation:**
   - [ ] All public APIs documented?
   - [ ] Documentation accurate?
   - [ ] Documentation current?

2. **User Documentation:**
   - [ ] User guides current?
   - [ ] Examples working?
   - [ ] Screenshots current?

3. **Architecture Documentation:**
   - [ ] ADRs current?
   - [ ] Architecture diagrams current?
   - [ ] Design docs current?

4. **API Documentation:**
   - [ ] API documentation current?
   - [ ] API examples working?
   - [ ] API deprecations documented?

### Detection Methods

**Manual Review:**
- Review documentation
- Check examples
- Verify accuracy

**Automated Checks:**
```bash
# Check for missing documentation
cargo run --bin doc_audit -- --check-missing

# Verify API documentation
cargo run --bin api_doc_audit -- --verify

# Check example code
cargo run --bin example_audit -- --verify-all
```

### Corrective Actions

**Missing Documentation:**
- Add missing documentation
- Prioritize public APIs
- Document critical paths

**Outdated Documentation:**
- Update documentation
- Verify examples
- Update screenshots

---

## 7. Dependency Drift Checklist

**Purpose:** Ensure dependencies appropriate

### Questions

1. **Dependency Necessity:**
   - [ ] All dependencies necessary?
   - [ ] No unused dependencies?
   - [ ] No redundant dependencies?

2. **Dependency Quality:**
   - [ ] Dependencies well-maintained?
   - [ ] Dependencies secure?
   - [ ] Dependencies compatible?

3. **Dependency Versions:**
   - [ ] Dependency versions appropriate?
   - [ ] No breaking changes?
   - [ ] Security vulnerabilities fixed?

4. **Dependency Licensing:**
   - [ ] Licenses compatible?
   - [ ] License compliance verified?
   - [ ] License documentation present?

### Detection Commands

```bash
# Check for unused dependencies
cargo machete

# Check for security vulnerabilities
cargo audit

# Check for outdated dependencies
cargo outdated

# Verify license compliance
cargo run --bin license_audit -- --verify-compliance
```

### Corrective Actions

**Unused Dependencies:**
- Remove unused dependencies
- Document dependency rationale

**Security Vulnerabilities:**
- Update vulnerable dependencies
- Document security fixes
- Review security practices

**Breaking Changes:**
- Pin dependency versions
- Update code for breaking changes
- Document migration guide

**License Issues:**
- Replace incompatible dependencies
- Document license compliance
- Review license strategy

---

## Drift Response Protocol

### Drift Severity Levels

**Level 1: Informational (Green)**
- Impact: < 5%
- Action: Document, monitor
- Timeline: Next checkpoint

**Level 2: Warning (Yellow)**
- Impact: 5-10%
- Action: Document, plan fix
- Timeline: Next phase

**Level 3: Critical (Red)**
- Impact: > 10%
- Action: Immediate action required
- Timeline: Immediate

### Response Process

**1. Detect Drift:**
- Run appropriate checklist
- Identify drift category
- Assess drift severity

**2. Document Drift:**
- Log drift in drift log
- Capture evidence
- Note impact assessment

**3. Assess Impact:**
- Determine severity level
- Identify affected components
- Estimate remediation effort

**4. Plan Response:**
- Choose corrective action
- Create remediation plan
- Get approvals if needed

**5. Execute Response:**
- Implement corrective action
- Verify fix effectiveness
- Document lessons learned

---

## Drift Log Template

```markdown
## Drift Log Entry

**Date:** YYYY-MM-DD
**Phase:** N
**Checkpoint:** K
**Drift Category:** Requirements/Architecture/Scope/Performance/Testing/Documentation/Dependency

**Severity:** Informational/Warning/Critical

**Description:**
[Describe drift detected]

**Evidence:**
[Attach evidence logs, screenshots, etc.]

**Impact Assessment:**
- Percentage Impact: X%
- Affected Components: [list]
- Estimated Remediation Effort: X hours

**Corrective Action:**
[Chosen corrective action]

**Status:** Open/In Progress/Resolved
**Resolved Date:** YYYY-MM-DD (if resolved)

**Lessons Learned:**
[What can be improved to prevent recurrence?]
```

---

## Anti-Goal-Drift Summary

**Key Principles:**
1. Early detection prevents major issues
2. Evidence-based assessment required
3. Corrective action proportional to impact
4. Documentation of drift essential
5. Continuous improvement through lessons learned

**Success Metrics:**
- Drift detected before phase exit
- Corrective actions completed
- No critical drift at phase exit
- Drift trends decreasing over time

---

**End of Anti-Goal-Drift Checklist**
