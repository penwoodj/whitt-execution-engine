# Documentation

Complete documentation for the YAML to Rust AgentSDK project. This directory contains all planning, requirements, research, and architectural decision records.

## Directory Structure

**plans/** - Implementation plans and validation framework
- INDEX.md - Master navigation for all phase plans (00-08), dependency graphs, and execution order
- ARCHITECTURE.md - System architecture and design decisions
- deep-research/** - Research orchestration documents for YAML schema, Rust ecosystem, LLM backends, agent patterns, testing strategy, and performance
- traceability/** - Matrix documents mapping schema domains, ADRs, workflows, and requirements to implementation phases
- validation-criteria/** - Incremental validation framework with anti-goal-drift mechanisms
- unified-schema-simplification/** - Schema simplification plan with 12 execution phases

**reports/** - Requirements, research, and benchmark specifications
- requirements/** - Schema requirements, example workflows, and benchmark specifications
  - unifying-schema/** - Unified workflow schema (801 lines), requirements, consolidation reports, and unification plans
  - benchmark-100-model-userflows/** - 100-model benchmark specifications with 20 userflow specs (uf01-uf20)
- roadmap/** - Architecture Decision Records (ADRs), research plans, and governance documents

**research/** - Future research directions and next steps
- next-steps/** - Research agenda and open questions

## Related

- [Master Plan Index](plans/INDEX.md) - Complete plan suite navigation
- [Schema Source](reports/requirements/unifying-schema/unified-workflow-schema.yml) - Single source of truth for workflow structure
- [Project README](/home/jon/code/yaml-to-rust-agentsdk/README.md) - Project overview and quick start guide
