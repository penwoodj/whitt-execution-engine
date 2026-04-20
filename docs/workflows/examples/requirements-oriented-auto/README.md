# Workflow Examples

50 YAML examples demonstrating the unified AgentSDK workflow schema across 18 categories.

## Quick Start

```bash
# Start here - simplest model configuration example
cat 01-model-configuration/01-basic-model-selection-providers.yaml
```

## Navigation Path

```mermaid
graph TD
    A["01 Model Configuration"] --> B["02 Step Types"]
    B --> C["03 Data Flow"]
    C --> D["04 Loops & Convergence"]
    D --> E["05-08 Operations"]
    E --> F["09-11 Workflow Patterns"]
    F --> G["12-15 Infrastructure"]
    G --> H["16-17 User & Lifecycle"]
    H --> I["18 Comprehensive Integration"]

    E --> E1["05 File Operations"]
    E --> E2["06 Web Operations"]
    E --> E3["07 RAG Operations"]
    E --> E4["08 Script & CLI"]

    F --> F1["09 Sub-Workflows"]
    F --> F2["10 Conditional Branching"]
    F --> F3["11 Error Handling"]

    G --> G1["12 Logging & Monitoring"]
    G --> G2["13 Checkpointing"]
    G --> G3["14 Resource Management"]
    G --> G4["15 Tool Permissions"]

    style A fill:#4CAF50,color:#fff
    style I fill:#FF5722,color:#fff
```

## Categories

| # | Category | Files | Focus |
|---|----------|-------|-------|
| 01 | [Model Configuration](01-model-configuration/) | 4 | Providers, parameters, lifecycle, cost tracking |
| 02 | [Step Types](02-step-types/) | 4 | LLM inference, code execution, tool invocation, hybrid |
| 03 | [Data Flow](03-data-flow/) | 3 | Variables, step outputs, context injection |
| 04 | [Loops & Convergence](04-loops-convergence/) | 4 | For/foreach/while, convergence reduction |
| 05 | [File Operations](05-file-operations/) | 2 | Read/write/batch, permissions |
| 06 | [Web Operations](06-web-operations/) | 3 | Fetch/scrape, REST APIs, URL parameters |
| 07 | [RAG Operations](07-rag-operations/) | 2 | Document indexing, context-aware generation |
| 08 | [Script & CLI](08-script-cli/) | 2 | Script execution, environment variables |
| 09 | [Sub-Workflows](09-sub-workflows/) | 2 | Nested references, composition patterns |
| 10 | [Conditional Branching](10-conditional-branching/) | 2 | Event-based branching, decision logic |
| 11 | [Error Handling](11-error-handling-retries/) | 3 | Retry/backoff, error propagation, graceful failure |
| 12 | [Logging & Monitoring](12-logging-monitoring/) | 3 | Hierarchical logging, metrics, structured output |
| 13 | [Checkpointing](13-checkpointing-state/) | 2 | Save/restore, state management |
| 14 | [Resource Management](14-resource-management/) | 3 | Memory allocation, CPU/GPU scheduling, limits |
| 15 | [Tool Permissions](15-tool-permissions/) | 2 | Allow/deny lists, step-level control |
| 16 | [User Inputs & UI](16-user-inputs-ui/) | 2 | Input validation, interactive feedback |
| 17 | [Hooks & Lifecycle](17-hooks-lifecycle/) | 4 | Pre/post hooks, error hooks, lifecycle events |
| 18 | [Comprehensive Integration](18-comprehensive-integration/) | 2 | Orchestration, guardrails, validation aggregation |

## Schema Reference

- **Unified Schema**: `../../schema/unified-workflow-schema.yml`
- **Coverage Analysis**: `../../_metadata/workflows-process/coverage-analysis.md`
- **Manual Brainstorm**: `../manual/agentic-workflow-manual-brainstorm.yml`

## 🔗 Related Documentation

| Document | Description |
|----------|-------------|
| [Schema Reference](../../schema/unified-workflow-schema.yml) | Single source of truth for workflow structure |
| [Requirements Index](../../requirements/index.md) | Complete requirements documentation |
| [Workflows Documentation](../../workflows-README.md) | Comprehensive workflow documentation |
