# Review Cycle 11: Schema Comprehensibility - Integration & Edge Cases

**Target Audience**: Junior engineers unfamiliar with AgentSDK documentation
**Focus**: Can juniors handle multi-provider, local/remote, and="real" workflows
**Date**: 2025-04-03

## Review Scope (11 Areas)

- **Files**: All workflow files using multiple providers or data flows
- **Question**: Is the: `type: lmstudio`, vs `type: ollama` vs `type: llama_cpp` vs `type: custom`
- **Concern**: No clear type comparison
- **Recommendation**: Add provider selection decision tree with clear examples

- **Files**: `01-model-configuration/04-cost-tracking-budget.yaml`
- **Question**: How does cost tracking actually work?
- **Concern**: No explanation of what triggers cost calculations
- **Recommendation**: Add cost calculation trigger comments

- **Files**: `04-parallel-execution/*`
- **Question**: What's difference between `workflow_execution_strategy.parallel.load_balancing.strategy` vs `parallel.algorithm: round_robin`?
- **Concern**: Two separate parallelism configurations
- **Recommendation**: Consolidate into single configuration section
- **Files**: `01-model-configuration/03-model-lifecycle-management.yaml`
- **Question**: When does model warmup vs cooldown happen?
- **Concern**: No clear state transition description
- **Recommendation**: Add lifecycle state diagram

- **Files**: `07-web-operations/01-web-fetch-scrape.yaml`
- **Question**: Is this example still relevant with modern web APIs?
- **Concern**: Could mislead juni about web scraping
- **Recommendation**: Add modern web guidance section or file header
- **Files**: `08-rag-operations/01-document-indexing-retrie.yaml`
- **Question**: Is this RAG example showing data persistence?
- **Concern**: No explicit persistence layer explanation
- **Recommendation**: Add database schema comments
- **Files**: `09-script-cli/02-cli-commands-environment.yaml`
- **Question**: Are environment variables inherited or or isolated?
- **Concern**: No inheritance documentation
- **Recommendation**: Add environment isolation section
- **Files**: `10-sub-workflows/02-workflow-composition-pattern.yaml`
- **Question**: How do parameter passing between parent and child workflows work?
- **Concern**: No explicit parameter mapping syntax
- **Recommendation**: Add parameter passing examples with actual values
- **Files**: `13-logging-monitoring/01-hierarchical-logging-system.yaml`
- **Question**: What's actual difference between log levels?
- **Concern**: Global vs model vs step logging separation unclear
- **Recommendation**: Unify logging configuration under single `logging:` key
- **Files**: `16-tool-permissions/01-allow-deny-lists-scopes.yaml`
- **Question**: Can I restrict tool access for by step?
- **Concern**: Global vs step-level permissions unclear
- **Recommendation**: Add scope precedence explanation
- **Files**: `17-user-inputs-ui/01-user-input-prompts-validation.yaml`
- **Question**: Does validation run before or after user input?
- **Concern**: No visible validation trigger examples
- **Recommendation**: Add validation timing comments
- **Files**: `19-comprehensive-integration/02-guardrails-content-safety.yaml`
- **Question**: How to test guardrails locally without running workflow?
- **Concern**: No guidance on guardrails testing
- **Recommendation**: Add guardrails testing section
- **Files**: `19-comprehensive-integration/01-complex-orchestration-sub-agents.yaml`
- **Question**: Can I test sub-agent orchestration manually?
- **Concern**: Complex file hard to test in isolation
- **Recommendation**: Create minimal test workflow file
- **Files**: `19-comprehensive-integration/02-guardrails-content-safety.yaml`
- **Question**: How to test if guardrails are blocking legitimate content?
- **Concern**: No `on_match` test cases
- **Recommendation**: Add guardrails test matrix in comments
