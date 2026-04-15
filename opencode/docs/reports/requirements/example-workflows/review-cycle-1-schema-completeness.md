# Critical Review Cycle 1: Schema Completeness

**Date**: 2026-03-08
**Scope**: Schema Completeness
**Reviewer**: Sisyphus

---

## Review Process

Reviewed all requirements from workflow-example-requirements.yml against current example workflow schemas (ex01-ex04).

---

## Findings

### 1. ✅ Covered Requirements

**Dual Agent Architectures**
- ✅ Direct LLM pipeline (ex01, ex02)
- ✅ AgentSDK agent with sub-agents (ex03)
- ❌ Hybrid approach (mix of AgentSDK and direct pipelines) - NOT COVERED

**Model Management**
- ✅ Dynamic parallelization with memory constraint (ex01, ex03)
- ✅ Serial execution (ex02)
- ✅ Persistent models (ex01 - model stays loaded)
- ✅ Auto model routing (ex02) <!-- Model routing features moved to ~/code/model-router/ -->
- ✅ Multiple providers (LM Studio, llama.cpp, Ollama) - ex01, ex02
- ❌ Explicit model routing configuration at workflow level - PARTIALLY COVERED (ex02 has priority_adjustment, but not explicit routing rules) <!-- Model routing features moved to ~/code/model-router/ -->

**Loops and Control Flow**
- ✅ Count-based loops - NOT EXPLICITLY SHOWN
- ✅ Time-based loops - NOT EXPLICITLY SHOWN
- ✅ Validation-based loops (abstract) - ex01, ex03
- ✅ Exact validation - ex02
- ✅ Convergence loops - NOT COVERED
- ✅ Infinite loops - NOT COVERED
- ❌ Loop control parameters (max_iterations, stop_condition, break_on, continue_on) - PARTIALLY COVERED

**Retry Logic**
- ✅ Default retry logic (ex01, ex02, ex04)
- ✅ Overridable retry logic (ex02)
- ✅ Failure types (network, model loading, timeout, validation, 500 errors) - SHOWN in requirements but not in examples
- ✅ Escalation to next model (ex02)

**Logging and Output**
- ✅ Logging hierarchy (global, step, tool, agent) - ex03 shows agent-specific logging
- ✅ Output types (chat, log, stateless_direct_io) - ex01 (chat), ex02 (stateless_direct_io)
- ✅ Detail levels (low, medium, high, trace) - SHOWN in examples
- ✅ Logging scopes (execution, memory, model, tool, validation, retry) - ex03 shows multiple scopes
- ❌ Mixed output type - NOT COVERED

**Tools and Permissions**
- ✅ File operations (read, write, delete, move, search) - ex04
- ✅ Directory operations - NOT EXPLICITLY SHOWN
- ✅ Web operations (fetch, scrape, search) - SHOWN in requirements but not in examples
- ✅ Shell operations - ex04
- ✅ RAG operations - NOT COVERED
- ✅ Tool permissions (global, step-specific) - ex04
- ✅ Folder permissions - ex04
- ✅ URL type parameters - SHOWN in requirements but not in examples
- ✅ Script/CLI execution - SHOWN in requirements but not in examples

**Pipeline Step I/O and Variables**
- ✅ Input parameters (static, workflow variables, step references) - ex01, ex02, ex03
- ✅ Output parameters (save_to, format, path, fields) - ex01, ex02, ex03
- ✅ Variable reference patterns (step.step_id.output, workflow.variable) - ex01, ex02
- ✅ Pipeline chaining - ex01, ex02

**RAG Operations**
- ❌ RAG configuration (backend, embedding model, chunking) - NOT COVERED
- ❌ RAG CRUD (add, query, update, delete) - NOT COVERED
- ❌ Web scraping to RAG - NOT COVERED

**Web and Forum Search**
- ❌ Web search - NOT COVERED
- ❌ Forum search - NOT COVERED

**Local File CRUD**
- ✅ Explicit CRUD - SHOWN in ex04 permissions
- ❌ Implicit CRUD - NOT COVERED

**Memory Management**
- ✅ Max allocated memory constraint - ex01, ex02, ex03, ex04
- ✅ Dynamic parallelization - ex01, ex03
- ✅ Serial execution - ex02
- ✅ Hybrid execution - SHOWN in requirements but not in examples
- ❌ GC configuration, swap timeout - SHOWN in examples but not validated

---

## Missing Schema Requirements

### Critical Gaps

1. **Convergence Loops**: No example shows convergence-based loops (threshold, metric, max_iterations)

2. **Infinite Loops**: No example shows infinite loops with manual termination

3. **Time-based Loops**: No example shows time-duration based loops

4. **Count-based Loops**: No example shows fixed iteration count loops

5. **RAG Operations**: Complete gap - no RAG configuration, CRUD, or web scraping to RAG examples

6. **Web/Forum Search**: No examples show web_search or forum_search tool usage

7. **Directory Operations**: No example shows dir_create, dir_delete, dir_list, dir_traverse

8. **Web Operations**: No example shows web_fetch, web_scrape with URL parameters

9. **Script/CLI Execution**: No example shows script_run or cli_run with arguments, env_vars

10. **Hybrid Approach**: No example mixes AgentSDK and direct LLM pipelines

11. **Explicit Model Routing Rules**: No example shows explicit routing configuration (when to use which model)

12. **Implicit File CRUD**: No example shows implicit read/write based on context

13. **Mixed Output Type**: No example shows mixed output (chat + log + stateless_direct_io)

---

## Schema Design Issues

### Identified Inconsistencies

1. **Loop Definition Inconsistency**:
   - ex01 has `validation_loop` at top level
   - ex02 has `validation_loop` at top level
   - ex03 has `improvement_loop` at top level
   - But loop types (count, time, convergence) are not explicitly shown
   - Need consistent loop schema structure

2. **Sub-agent Reference Issue**:
   - ex03 line 98, 157: empty string aliases causing YAML errors
   - Sub-agent spawn syntax needs clarification

3. **Tool Call Definition**:
   - ex03 uses `tool:` inside step
   - ex04 uses `tool:` at same level as `model:`
   - Inconsistent - need unified syntax

4. **Logging Scoping**:
   - ex03 shows logging with `scopes:` key
   - ex01, ex02 show logging without `scopes:`
   - Need consistent approach

---

## Recommended Schema Additions

### 1. Unified Loop Schema

```yaml
loops:
  - id: retry_until_success
    type: validation
    validation_type: exact  # exact or abstract
    criteria:
      - condition: "${result.status} == 'success'"
    max_iterations: 5
    stop_on_success: true

  - id: convergence_optimization
    type: convergence
    threshold: 0.01
    metric: "${current.loss} - ${previous.loss}"
    max_iterations: 20

  - id: timed_scan
    type: time
    duration_secs: 300
    max_iterations: null

  - id: fixed_iterations
    type: count
    iterations: 10

  - id: continuous_monitoring
    type: infinite
    stop_on: manual_termination
```

> **Note (v2.0 update):** The `enabled:` pattern was replaced with presence=enabled convention in the unified schema v2.0. Features are enabled by including their configuration; use `disabled: true` to explicitly disable.

### 2. RAG Schema

```yaml
rag:
  enabled: true
  backend: chroma
  config:
    path: ./workspace/.rag
    embedding_model: lmstudio://all-MiniLM-L6-v2
    chunk_size: 512
    chunk_overlap: 50
    collection_name: codebase_docs

  operations:
    add:
      documents:
        - path: ./workspace/docs/**/*.md
          metadata:
            type: documentation
            project: my_project

    query:
      query: "Find async functions with error handling"
      top_k: 10
      similarity_threshold: 0.7
      include_metadata: true

    update:
      document_id: doc123
      content: "Updated content"
      metadata:
        updated_at: 2026-03-08

    delete:
      document_id: doc123
      filter:
        project: my_project
```

### 3. Web Operations Schema

```yaml
web_operations:
  search:
    query: "Rust async best practices"
    engine: google
    max_results: 10
    safe_search: true
    language: en

  forum_search:
    query: "tokio runtime issues"
    forums:
      - stackoverflow
      - reddit_r_rust
    max_results: 5
    time_range: 1year

  fetch:
    url: "https://docs.rs/tokio"
    method: GET
    headers:
      User-Agent: "My Agent"
    timeout_secs: 30
    follow_redirects: true
    validate_ssl: true

  scrape:
    url: "https://example.com/page"
    extract_text: true
    extract_links: true
    extract_images: false
    output_format: json
```

> **Note (v2.0 update):** The `enabled:` pattern was replaced with presence=enabled convention in the unified schema v2.0. Features are enabled by including their configuration; use `disabled: true` to explicitly disable.

### 4. Explicit Model Routing Schema

```yaml
model_routing:
  enabled: true
  strategy: task_category_based  # task_category_based, cost_based, performance_based

  routing_rules:
    - rule_id: code_analysis_rule
      task_category: code_analysis
      primary_model: llamacpp://code-specialist
      fallback_models:
        - lmstudio://deepseek-coder
        - ollama://llama3.2

    - rule_id: code_generation_rule
      task_category: code_generation
      primary_model: lmstudio://deepseek-coder
      fallback_models:
        - llamacpp://code-specialist
        - ollama://llama3.2

  failure_history:
    enabled: true
    # Note (v2.0): In v2.0, omit enabled: (presence=enabled by default) or use disabled: true
    max_entries: 100
    adjust_on_failure: true
    adjust_on_success: true
```

---

## Action Items

### High Priority
1. Create example ex05-rag-crud-operations.yml
2. Create example ex06-web-scraping-to-rag.yml
3. Create example ex07-web-forum-search.yml
4. Create example ex08-convergence-loops.yml
5. Create example ex09-infinite-loops.yml

### Medium Priority
6. Fix YAML syntax errors in ex03
7. Create example ex10-hybrid-agent-direct.yml
8. Create example ex11-implicit-file-crud.yml
9. Add explicit model routing rules to existing examples

### Low Priority
10. Standardize loop schema across all examples
11. Standardize tool call syntax
12. Standardize logging scoping approach

---

## Next Steps

Proceed to Review Cycle 2: Workflow Coherence
