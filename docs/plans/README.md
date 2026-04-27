# Plans Directory

Implementation plans and architecture documentation for the Whitt Execution Engine project.

## Phases
- [00-foundation](./00-foundation/) - Foundation phase: Core execution engine, scheduler, and persistence
- [01-core-execution-engine](./01-core-execution-engine/) - Core Execution Engine phase: Queue, scheduler, and state management
- [02-cli-and-llm-backend-integration](./02-cli-and-llm-backend-integration/) - CLI & LLM Backend Integration phase: CLI interface, provider backends, and tool integration
- [03-quality-loops](./03-quality-loops/) - Quality Loops phase: Validation, verification, and continuous improvement
- [04-memory-search](./04-memory-search/) - Memory Search phase: RAG implementation and local memory management
- [05-automation](./05-automation/) - Automation phase: Cron scheduling, git experiments, and automation workflows
- [06-autonomy-metrics](./06-autonomy-metrics/) - Autonomy & Metrics phase: Bounded loops, objective metrics, and dashboards
- [07-final-validation](./07-final-validation/) - Final Validation phase: Comprehensive validation of all features

## Phase Boundary Contracts

This section defines the data types that flow between phase boundaries. Each phase must document the interfaces it exposes to the next phase.

| From Phase | To Phase | Data Contract | Status |
|-----------|----------|---------------|--------|
| 00 (Foundation) | 01 (Core) | `UnifiedConfig`, `ModelSpec` | ✅ Defined |
| 01 (Core) | 02 (CLI) | `ReactAgent`, `ToolRegistry` | ⚠️ Implicit |
| 02 (CLI) | 03 (Quality) | `WorkflowState`, `Checkpoint` | ❌ Needs definition |
| 03 (Quality) | 04 (Memory) | `SearchResult`, `MemoryEntry` | ❌ Needs definition |
| 04 (Memory) | 05 (Automation) | `ScheduleEntry`, `CronExpr` | ❌ Needs definition |
| 05 (Automation) | 06 (Metrics) | `MetricEvent`, `Dashboard` | ❌ Needs definition |
| 06 (Metrics) | 07 (Validation) | `BenchmarkResult`, `Report` | ❌ Needs definition |

> ⚠️ **Critical Review #2 Finding**: Phase boundaries have implicit data dependencies but no explicit contracts. Risk: incompatible implementations between phases. See [upstream-critical-review-02.md](../research/upstream-critical-review-02.md) Factor 1.

## Additional Documentation
- [ARCHITECTURE.md](./ARCHITECTURE.md) - Overall architecture and design decisions
- [testing-strategy.md](./testing-strategy.md) - Testing strategy and automation
- [github-actions-improvements.md](./github-actions-improvements.md) - CI/CD improvements and GitHub Actions workflows
- [traceability](./traceability/) - Traceability and schema-to-phase mapping
- [transpiler](./transpiler/) - Transpiler architecture and implementation
- [validation-criteria](./validation-criteria/) - Cross-phase validation criteria
- [deep-research](./deep-research/) - In-depth research and analysis
