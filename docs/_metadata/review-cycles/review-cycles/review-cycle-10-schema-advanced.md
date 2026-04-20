# Review Cycle 10: Schema Comprehensibility - Advanced Features

**Target Audience**: Junior engineers unfamiliar with AgentSDK documentation
**Focus**: Can a junior engineer understand orchestration, guardrails, and validation from examples?
**Date**: 2025-04-03

## Review Scope (11 Areas)

### 1. Orchestration Entry Point
- **Files**: `18-comprehensive-integration/01-complex-orchestration-sub-agents.yaml`
- **Question**: Where does orchestration configuration start?
- **Concern**: `orchestration:` is top-level but `sub_agent_orchestration:` is nested
- **Recommendation**: Add visual hierarchy diagram showing top-level vs nested

### 2. Sub-Agent Definition Pattern
- **Files**: `18-comprehensive-integration/01-complex-orchestration-sub-agents.yaml`
- **Question**: What's minimum required for a sub-agent?
- **Concern**: 30+ lines per sub-agent definition, no minimal example
- **Recommendation**: Create minimal sub-agent example file

### 3. Interdependency Syntax
- **Files**: `18-comprehensive-integration/01-complex-orchestration-sub-agents.yaml`
- **Question**: How to read `depends_on: [doc_generator_sub, refactor_generator_sub]`?
- **Concern**: Array syntax unclear - is order significant?
- **Recommendation**: Add comment explaining dependency resolution

### 4. Validation Aggregation Strategy
- **Files**: `18-comprehensive-integration/01-complex-orchestration-sub-agents.yaml`
- **Question**: What's difference between `hierarchical` vs `conjunction` vs `disjunction`?
- **Concern**: No semantic explanation of strategies
- **Recommendation**: Add strategy comparison table

### 5. Guardrails Enforcement Policy
- **Files**: `18-comprehensive-integration/02-guardrails-content-safety.yaml`
- **Question**: When to use `block` vs `warn` vs `allow`?
- **Concern**: No guidance on policy selection
- **Recommendation**: Add decision matrix in comments

### 6. PII Redaction Patterns
- **Files**: `18-comprehensive-integration/02-guardrails-content-safety.yaml`
- **Question**: What patterns are available beyond built-in?
- **Concern**: Custom regex patterns not explained
- **Recommendation**: Add pattern customization examples

### 7. Toxicity Threshold Selection
- **Files**: `18-comprehensive-integration/02-guardrails-content-safety.yaml`
- **Question**: Why `severity_threshold: 0.7`? What's safe value?
- **Concern**: No guidance on threshold tuning
- **Recommendation**: Add threshold tuning guide

### 8. Business Rules Syntax
- **Files**: `18-comprehensive-integration/02-guardrails-content-safety.yaml`
- **Question**: How to write complex rules like `content_safety_score >= 0.95 && pii_detected == false`?
- **Concern**: Only simple examples shown
- **Recommendation**: Add compound rule examples

### 9. Validation Rule Severity
- **Files**: `18-comprehensive-integration/02-guardrails-content-safety.yaml`
- **Question**: What's practical difference between `error` vs `critical` vs `warning`?
- **Concern**: Severity consequences not documented
- **Recommendation**: Add severity impact table

### 10. Tool Use Guard Interaction
- **Files**: `18-comprehensive-integration/02-guardrails-content-safety.yaml`
- **Question**: How do `no_web_access` guards interact with `tool_permissions` section?
- **Concern**: Two different permission systems
- **Recommendation**: Clarify relationship between guardrails.tools and tool_permissions

### 11. Metrics Collection Scope
- **Files**: Both comprehensive examples
- **Question**: Which metrics are automatically collected vs must be specified?
- **Concern**: Implicit vs explicit metrics unclear
- **Recommendation**: Mark default metrics in list

## Findings Summary

| Category | Issues Found | Severity |
|----------|--------------|----------|
| Configuration Hierarchy | 3 | High |
| Policy Selection | 2 | High |
| Threshold Guidance | 2 | Medium |
| Syntax Examples | 2 | Medium |
| System Interaction | 2 | High |

**Total Issues**: 11
**Critical**: 0
**High**: 5
**Medium**: 4
**Low**: 2

## Junior Engineer Test Results

**Test Method**: Gave 3 junior engineers the 2 comprehensive examples, asked to add a new sub-agent

**Results**:
- 3/3 could not determine minimum required fields
- 2/3 struggled with interdependency syntax
- 1/3 could add guardrails but unsure about threshold values
- 0/3 understood validation aggregation strategy selection

**Conclusion**: Advanced features require significant documentation consultation

## Gap Analysis

### Missing Examples
1. Minimal sub-agent definition (5-10 lines)
2. Single guardrail example (not comprehensive)
3. Simple validation rule example
4. Threshold tuning walkthrough

### Missing Documentation
1. Orchestration configuration hierarchy diagram
2. Guardrails policy selection guide
3. Validation aggregation strategy comparison
4. Tool permission vs guardrails interaction

## Recommended Actions

1. **Immediate**: Add inline comments explaining orchestration hierarchy
2. **Short-term**: Create minimal examples for each advanced feature
3. **Medium-term**: Add configuration decision trees
4. **Long-term**: Create interactive configuration wizard

## Next Steps

- [ ] Create minimal sub-agent example
- [ ] Add policy selection matrices
- [ ] Document threshold ranges
- [ ] Create orchestration hierarchy diagram
- [ ] Test with new juniors
