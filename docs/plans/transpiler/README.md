# Transpiler Plans

Documentation for transpiler architecture, research, and implementation plans.

## Overview

The transpiler converts YAML workflow definitions into executable Rust code using the AgentSDK runtime. It follows a multi-stage pipeline from YAML input through parsing, validation, intermediate representation, and code generation to Rust output.

## Documentation

- [transpiler_architecture.md](./transpiler_architecture.md) - Complete transpiler architecture including 7 layers (input & parsing, intermediate representation, code generation, quality & safety, build & testing, LLM backend integration, CLI & documentation), data flow, module structure, design decisions, invariants, performance targets, testing strategy, and future extensions

- [transpiler_implementation_plan.yml](./transpiler_implementation_plan.yml) - Detailed implementation plan with 5 phases and 34 tasks covering foundation (schema, parser, AST), code generation (templates, bindings, layout), LLM integration (llama.cpp, Vulkan, GPU offload, model lifecycle), validation & safety (validation, linting, sandbox), testing & quality (golden tests, property tests, caching), and CLI integration

- [transpiler_research_plan.yml](./transpiler_research_plan.yml) - Research plan with 8 areas: YAML schema design, parser & AST design, code generation patterns, project layout & packaging, validation & linting, testing strategies, caching & performance, sandboxing & security, and ModelProvider & LLM backend integration
