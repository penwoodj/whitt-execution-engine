# Agent Patterns Research Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Research and document agent orchestration patterns including ReAct loops, tool calling chains, multi-agent coordination (merge/vote/collect/average), sub-agent orchestration, event-driven execution, validation loops, and convergence detection with Rust implementation approaches.

**Architecture**: Agent pattern catalog documenting orchestration patterns, Rust implementation strategies, test strategies, and reference implementations for Phase 1 (Agent Runtime) and Phase 2 (Multi-Agent Coordination).

**Tech Stack**: Rust async/await, tokio runtime, futures, state machines, event-driven architecture, message passing, trait-based design.

---

## Research Questions Being Answered

### Question 1: What are the Essential Agent Orchestration Patterns?
**Why it matters**: Understanding orchestration patterns enables implementing complex multi-agent workflows with proven approaches.

**Success criteria**: Documented agent orchestration patterns (ReAct loops, tool calling chains, multi-agent coordination) with Rust implementation strategies.

**Integration point**: Phase 1 (Agent Runtime), Phase 2 (Multi-Agent Coordination)

---

### Question 2: How to Implement Multi-Agent Coordination in Rust?
**Why it matters**: Multi-agent coordination enables complex workflows with multiple specialized agents working together.

**Success criteria**: Documented multi-agent coordination patterns (merge, vote, collect, average) with Rust implementation approaches.

**Integration point**: Phase 2 (Multi-Agent Coordination)

---

### Question 3: How to Detect Convergence and Handle Validation Loops?
**Why it matters**: Convergence detection and validation loops enable autonomous agents that can self-correct and improve outputs.

**Success criteria**: Documented convergence detection strategies and validation loop implementations with Rust examples.

**Integration point**: Phase 1 (Agent Runtime), Phase 2 (Enhanced Features)

---

## Findings with Evidence

### Finding 1: ReAct Loops are the Foundation of Single-Agent Reasoning

**Evidence sources**:
- ReAct paper: https://arxiv.org/abs/2210.03629
- ReAct implementations: https://github.com/ysymyth/ReAct
- LangChain ReAct: https://python.langchain.com/docs/modules/agents/agents/react

**Summary**:
ReAct (Reasoning + Acting) loops combine reasoning and acting in an iterative cycle. The agent observes the environment, reasons about the next action, acts, and repeats until a goal is reached.

**Key pattern**:
```
1. Thought: Reason about current state
2. Action: Choose and execute an action (tool call)
3. Observation: Observe the result
4. Repeat: Go back to step 1 until done
```

**Rust implementation approach**:
```rust
use async_trait::async_trait;

/// ReAct agent trait
#[async_trait]
pub trait ReactAgent: Send + Sync {
    async fn think(&self, context: &AgentContext) -> Result<Thought>;
    async fn act(&self, action: &Action) -> Result<Observation>;
    async fn is_done(&self, context: &AgentContext) -> bool;
}

/// Run ReAct loop
pub async fn run_react_loop<A: ReactAgent>(
    agent: &A,
    initial_context: AgentContext,
    max_iterations: usize,
) -> Result<Vec<Action>> {
    let mut context = initial_context;
    let mut actions = Vec::new();

    for _ in 0..max_iterations {
        let thought = agent.think(&context).await?;
        tracing::info!("Thought: {}", thought.content);

        let action = thought.suggested_action;
        let observation = agent.act(&action).await?;
        tracing::info!("Observation: {}", observation.result);

        actions.push(action.clone());
        context = context.apply_observation(observation);

        if agent.is_done(&context).await {
            break;
        }
    }

    Ok(actions)
}
```

---

### Finding 2: Tool Calling Chains Enable Multi-Step Workflows

**Evidence sources**:
- OpenAI function calling: https://platform.openai.com/docs/guides/function-calling
- LangChain tool calling: https://python.langchain.com/docs/modules/agents/tools/toolkits
- Tool calling research: https://arxiv.org/abs/2305.14314

**Summary**:
Tool calling chains enable agents to execute multiple tools in sequence, with each tool's output becoming input for the next. This enables complex multi-step workflows.

**Key pattern**:
```
1. Agent receives task
2. Agent decides which tool to call
3. Agent executes tool call
4. Agent receives tool output
5. Agent uses tool output as context
6. Repeat: Go back to step 2 until done
```

**Rust implementation approach**:
```rust
/// Tool calling chain
pub async fn run_tool_chain(
    agent: &dyn ReactAgent,
    initial_task: String,
    available_tools: Vec<Box<dyn Tool>>,
) -> Result<Vec<ToolResult>> {
    let mut context = AgentContext::new(initial_task);
    let mut results = Vec::new();

    loop {
        let thought = agent.think(&context).await?;
        let action = thought.suggested_action;

        if action.is_tool_call() {
            let tool = available_tools.iter()
                .find(|t| t.name() == action.tool_name())
                .ok_or_else(|| anyhow!("Tool not found"))?;

            let result = tool.execute(&action.args()).await?;
            results.push(result.clone());
            context = context.apply_tool_result(result);
        } else {
            break;
        }

        if agent.is_done(&context).await {
            break;
        }
    }

    Ok(results)
}
```

---

### Finding 3: Multi-Agent Coordination Patterns

**Evidence sources**:
- Multi-agent systems research: https://arxiv.org/abs/2307.11346
- LangChain multi-agent: https://python.langchain.com/docs/modules/agents/multi_agent
- AutoAgents framework: https://github.com/liquidos-ai/AutoAgents

**Summary**:
Multi-agent coordination patterns enable multiple specialized agents to work together on complex tasks.

**Key patterns**:

**Merge**: Combine outputs from multiple agents into a single output.
```rust
pub async fn merge_agent_outputs(outputs: Vec<AgentOutput>) -> Result<AgentOutput> {
    let merged_content = outputs.iter()
        .map(|o| o.content.as_str())
        .collect::<Vec<_>>()
        .join("\n\n");

    Ok(AgentOutput { content: merged_content })
}
```

**Vote**: Agents vote on the best output.
```rust
pub async fn vote_agent_outputs(outputs: Vec<AgentOutput>) -> Result<AgentOutput> {
    // Count votes for each output
    let mut votes: HashMap<&str, usize> = HashMap::new();
    for output in &outputs {
        *votes.entry(output.content.as_str()).or_insert(0) += 1;
    }

    // Find most voted output
    let best_output = votes.iter()
        .max_by_key(|&(_, count)| count)
        .map(|(content, _)| *content)
        .ok_or_else(|| anyhow!("No outputs"))?;

    Ok(AgentOutput { content: best_output.to_string() })
}
```

**Collect**: Collect all outputs without merging.
```rust
pub async fn collect_agent_outputs(outputs: Vec<AgentOutput>) -> Result<Vec<AgentOutput>> {
    Ok(outputs)
}
```

**Average**: Average numerical outputs from multiple agents.
```rust
pub async fn average_agent_outputs(outputs: Vec<AgentOutput>) -> Result<AgentOutput> {
    let numeric_values: Result<Vec<f64>> = outputs.iter()
        .map(|o| o.content.parse::<f64>().map_err(Into::into))
        .collect();

    let values = numeric_values?;
    let average = values.iter().sum::<f64>() / values.len() as f64;

    Ok(AgentOutput { content: average.to_string() })
}
```

---

### Finding 4: Sub-Agent Orchestration Enables Hierarchical Workflows

**Evidence sources**:
- Hierarchical agents research: https://arxiv.org/abs/2304.04091
- LangChain hierarchical: https://python.langchain.com/docs/modules/agents/multi_agent/hierarchical
- BabyAGI: https://github.com/yoheinakajima/babyagi

**Summary**:
Sub-agent orchestration enables hierarchical workflows where a higher-level agent delegates tasks to specialized sub-agents.

**Key pattern**:
```
1. Manager agent receives task
2. Manager decomposes task into subtasks
3. Manager delegates subtasks to specialist agents
4. Specialist agents execute subtasks
5. Manager collects results
6. Manager synthesizes final output
```

**Rust implementation approach**:
```rust
/// Manager agent for sub-agent orchestration
pub struct ManagerAgent {
    specialist_agents: Vec<Box<dyn ReactAgent>>,
}

impl ManagerAgent {
    pub fn new(specialist_agents: Vec<Box<dyn ReactAgent>>) -> Self {
        Self { specialist_agents }
    }

    pub async fn orchestrate(&self, task: String) -> Result<AgentOutput> {
        // Decompose task
        let subtasks = self.decompose_task(&task).await?;

        // Execute subtasks in parallel
        let results = futures::future::join_all(
            subtasks.iter().map(|subtask| {
                let agent = self.select_specialist(subtask);
                async move {
                    agent.execute_subtask(subtask.clone()).await
                }
            })
        ).await;

        // Synthesize results
        self.synthesize_results(results).await
    }

    async fn decompose_task(&self, task: &str) -> Result<Vec<SubTask>> {
        // Use LLM to decompose task
        todo!()
    }

    fn select_specialist(&self, subtask: &SubTask) -> &dyn ReactAgent {
        // Select specialist based on subtask type
        todo!()
    }

    async fn synthesize_results(&self, results: Vec<Result<AgentOutput>>) -> Result<AgentOutput> {
        // Use LLM to synthesize results
        todo!()
    }
}
```

---

### Finding 5: Event-Driven Execution Enables Real-Time Agent Coordination

**Evidence sources**:
- Event-driven architecture: https://martinfowler.com/articles/20170114-event-driven.html
- Tokio channels: https://tokio.rs/tokio/tutorial/channels
- Rust async patterns: https://rust-lang.github.io/async-book/

**Summary**:
Event-driven execution enables real-time agent coordination through message passing and event handling.

**Rust implementation approach**:
```rust
use tokio::sync::mpsc;

/// Event type
#[derive(Debug, Clone)]
pub enum AgentEvent {
    TaskReceived { id: String, task: String },
    TaskCompleted { id: String, result: AgentOutput },
    TaskFailed { id: String, error: String },
    SubTaskAssigned { id: String, subtask: SubTask },
    SubTaskCompleted { id: String, result: AgentOutput },
}

/// Event-driven agent
pub struct EventDrivenAgent {
    event_tx: mpsc::UnboundedSender<AgentEvent>,
    event_rx: mpsc::UnboundedReceiver<AgentEvent>,
}

impl EventDrivenAgent {
    pub fn new() -> Self {
        let (event_tx, event_rx) = mpsc::unbounded_channel();
        Self { event_tx, event_rx }
    }

    pub async fn run(&mut self) -> Result<()> {
        while let Some(event) = self.event_rx.recv().await {
            self.handle_event(event).await?;
        }
        Ok(())
    }

    async fn handle_event(&self, event: AgentEvent) -> Result<()> {
        match event {
            AgentEvent::TaskReceived { id, task } => {
                tracing::info!("Task received: {} - {}", id, task);
                // Process task
            }
            AgentEvent::TaskCompleted { id, result } => {
                tracing::info!("Task completed: {} - {}", id, result.content);
                // Notify subscribers
            }
            AgentEvent::TaskFailed { id, error } => {
                tracing::error!("Task failed: {} - {}", id, error);
                // Handle error
            }
            _ => {}
        }
        Ok(())
    }
}
```

---

### Finding 6: Validation Loops Enable Autonomous Convergence

**Evidence sources**:
- Validation loop research: https://arxiv.org/abs/2310.08441
- AutoGPT validation: https://github.com/Significant-Gravitas/Auto-GPT
- AgentGPT validation: https://github.com/reworkd/AgentGPT

**Summary**:
Validation loops enable agents to iteratively improve their outputs by validating against criteria and refining until convergence.

**Key pattern**:
```
1. Agent generates output
2. Validator checks output against criteria
3. If valid: Done
4. If invalid: Agent refines output based on feedback
5. Repeat: Go back to step 2 until done or max iterations
```

**Rust implementation approach**:
```rust
/// Validation loop
pub async fn run_validation_loop<A: ReactAgent>(
    agent: &A,
    validator: &dyn Validator,
    initial_task: String,
    max_iterations: usize,
) -> Result<AgentOutput> {
    let mut task = initial_task;
    let mut output = AgentOutput::default();

    for iteration in 0..max_iterations {
        // Agent generates output
        output = agent.execute_task(task.clone()).await?;

        // Validator checks output
        let validation_result = validator.validate(&output).await?;

        if validation_result.is_valid {
            tracing::info!("Validation passed on iteration {}", iteration);
            break;
        }

        tracing::info!("Validation failed on iteration {}: {}",
            iteration, validation_result.reason);

        // Refine task based on feedback
        task = format!("{}\n\nFeedback: {}\n\nRefine the output.",
            task, validation_result.feedback);
    }

    Ok(output)
}

/// Validator trait
#[async_trait]
pub trait Validator: Send + Sync {
    async fn validate(&self, output: &AgentOutput) -> Result<ValidationResult>;
}

#[derive(Debug)]
pub struct ValidationResult {
    pub is_valid: bool,
    pub reason: String,
    pub feedback: String,
}
```

---

### Finding 7: Convergence Detection Strategies

**Evidence sources**:
- Convergence detection research: https://arxiv.org/abs/2306.02241
- LangChain convergence: https://python.langchain.com/docs/modules/agents/agent_executors/async
- Rust convergence patterns: https://rust-lang.github.io/async-book/

**Summary**:
Convergence detection strategies enable agents to detect when outputs are stable or criteria are met, preventing infinite loops.

**Key strategies**:

**Output stability**: Detect when outputs stop changing significantly.
```rust
pub fn detect_output_stability(outputs: &[AgentOutput], threshold: f64) -> bool {
    if outputs.len() < 2 {
        return false;
    }

    let last = &outputs[outputs.len() - 1];
    let second_last = &outputs[outputs.len() - 2];

    let similarity = compute_similarity(&last.content, &second_last.content);
    similarity >= threshold
}

fn compute_similarity(a: &str, b: &str) -> f64 {
    // Implement similarity metric (e.g., Jaccard, cosine)
    todo!()
}
```

**Score threshold**: Detect when validation scores meet threshold.
```rust
pub fn detect_score_threshold(scores: &[f64], threshold: f64) -> bool {
    scores.iter().all(|&s| s >= threshold)
}
```

**Iteration limit**: Stop after max iterations.
```rust
pub fn detect_iteration_limit(iteration: usize, max_iterations: usize) -> bool {
    iteration >= max_iterations
}
```

---

## Recommendations with Rationale

### Recommendation 1: Implement ReAct Loops as Foundation Pattern

**Why**: ReAct loops are the foundation of single-agent reasoning, proven by research and widely adopted.

**Trade-offs**:
- Pros: Simple, proven, widely used
- Cons: May not be optimal for all tasks

**Alternatives considered**:
- Plan-and-solve: More complex, but better for complex tasks
- Chain-of-thought: Simpler, but less effective

**Adoption priority**: **P0 (Critical for Phase 1)**

---

### Recommendation 2: Implement Multi-Agent Coordination Patterns

**Why**: Multi-agent coordination enables complex workflows with specialized agents, improving quality and efficiency.

**Trade-offs**:
- Pros: Specialized expertise, parallel execution, better quality
- Cons: More complex, requires coordination logic

**Alternatives considered**:
- Single agent: Simpler, but less effective
- No coordination: No multi-agent workflows

**Adoption priority**: **P1 (Important for Phase 2)**

---

### Recommendation 3: Implement Validation Loops with Convergence Detection

**Why**: Validation loops enable autonomous convergence, improving output quality without human intervention.

**Trade-offs**:
- Pros: Autonomous improvement, better quality
- Cons: More iterations, higher cost

**Alternatives considered**:
- No validation: Lower quality, faster
- Human validation: Higher quality, but slower

**Adoption priority**: **P1 (Important for Phase 1)**

---

## Integration Instructions

### Integration Point 1: Phase 1 (Agent Runtime)

**What to implement**:
Implement ReAct loop foundation. Implement tool calling chains. Implement validation loops with convergence detection.

**File locations**:
- `src/agent/react.rs`: ReAct loop implementation
- `src/agent/tools.rs`: Tool calling chain implementation
- `src/agent/validation.rs`: Validation loop implementation

**Code patterns**:

```rust
// src/agent/react.rs
use async_trait::async_trait;

/// ReAct agent
#[async_trait]
pub trait ReactAgent: Send + Sync {
    async fn think(&self, context: &AgentContext) -> Result<Thought>;
    async fn act(&self, action: &Action) -> Result<Observation>;
    async fn is_done(&self, context: &AgentContext) -> bool;
}

pub async fn run_react_loop<A: ReactAgent>(
    agent: &A,
    initial_context: AgentContext,
    max_iterations: usize,
) -> Result<Vec<Action>> {
    let mut context = initial_context;
    let mut actions = Vec::new();

    for i in 0..max_iterations {
        tracing::info!("ReAct iteration {}", i);

        let thought = agent.think(&context).await?;
        tracing::info!("Thought: {}", thought.content);

        let action = thought.suggested_action.clone();
        let observation = agent.act(&action).await?;
        tracing::info!("Observation: {}", observation.result);

        actions.push(action);
        context = context.apply_observation(observation);

        if agent.is_done(&context).await {
            tracing::info!("ReAct loop complete");
            break;
        }
    }

    Ok(actions)
}
```

**Testing requirements**:
- Unit tests for ReAct loop
- Integration tests with mock tools
- Validation loop tests with mock validators

---

### Integration Point 2: Phase 2 (Multi-Agent Coordination)

**What to implement**:
Implement multi-agent coordination patterns (merge, vote, collect, average). Implement sub-agent orchestration. Implement event-driven execution.

**File locations**:
- `src/agent/multi_agent.rs`: Multi-agent coordination
- `src/agent/sub_agent.rs`: Sub-agent orchestration
- `src/agent/events.rs`: Event-driven execution

**Code patterns**:

```rust
// src/agent/multi_agent.rs
/// Multi-agent coordinator
pub struct MultiAgentCoordinator {
    agents: Vec<Box<dyn ReactAgent>>,
    coordination_strategy: CoordinationStrategy,
}

#[derive(Debug, Clone)]
pub enum CoordinationStrategy {
    Merge,
    Vote,
    Collect,
    Average,
}

impl MultiAgentCoordinator {
    pub async fn coordinate(&self, task: String) -> Result<AgentOutput> {
        // Execute all agents in parallel
        let outputs = futures::future::join_all(
            self.agents.iter().map(|agent| {
                let task = task.clone();
                async move {
                    agent.execute_task(task).await
                }
            })
        ).await;

        let outputs: Result<Vec<AgentOutput>> = outputs.into_iter().collect();
        let outputs = outputs?;

        // Apply coordination strategy
        match self.coordination_strategy {
            CoordinationStrategy::Merge => self.merge_outputs(outputs),
            CoordinationStrategy::Vote => self.vote_outputs(outputs),
            CoordinationStrategy::Collect => self.collect_outputs(outputs),
            CoordinationStrategy::Average => self.average_outputs(outputs),
        }
    }

    fn merge_outputs(&self, outputs: Vec<AgentOutput>) -> Result<AgentOutput> {
        let merged_content = outputs.iter()
            .map(|o| o.content.as_str())
            .collect::<Vec<_>>()
            .join("\n\n");

        Ok(AgentOutput { content: merged_content })
    }

    fn vote_outputs(&self, outputs: Vec<AgentOutput>) -> Result<AgentOutput> {
        // Count votes for each output
        let mut votes: HashMap<&str, usize> = HashMap::new();
        for output in &outputs {
            *votes.entry(output.content.as_str()).or_insert(0) += 1;
        }

        // Find most voted output
        let best_output = votes.iter()
            .max_by_key(|&(_, count)| count)
            .map(|(content, _)| *content)
            .ok_or_else(|| anyhow!("No outputs"))?;

        Ok(AgentOutput { content: best_output.to_string() })
    }

    fn collect_outputs(&self, outputs: Vec<AgentOutput>) -> Result<AgentOutput> {
        // Return all outputs
        let collected = serde_json::to_string(&outputs)?;
        Ok(AgentOutput { content: collected })
    }

    fn average_outputs(&self, outputs: Vec<AgentOutput>) -> Result<AgentOutput> {
        // Average numerical outputs
        let numeric_values: Result<Vec<f64>> = outputs.iter()
            .map(|o| o.content.parse::<f64>().map_err(Into::into))
            .collect();

        let values = numeric_values?;
        let average = values.iter().sum::<f64>() / values.len() as f64;

        Ok(AgentOutput { content: average.to_string() })
    }
}
```

**Testing requirements**:
- Unit tests for all coordination strategies
- Integration tests with multiple agents
- Event-driven tests with mock events

---

## Validation Criteria

### Criteria 1: ReAct Loop Executes Correctly

**How to verify**:
1. Run ReAct loop tests with mock agents
2. Verify thoughts, actions, observations are logged
3. Verify loop terminates when done or max iterations

**Step-by-step verification process**:
```bash
# Run ReAct loop tests
cargo test agent::react

# Verify logs contain thoughts, actions, observations
cargo test agent::react -- --nocapture

# Verify loop termination
cargo test agent::react::termination
```

**Expected outcome**:
- ReAct loop tests pass
- Logs contain thoughts, actions, observations
- Loop terminates when done or max iterations

**Integration point**: Phase 1 (Agent Runtime)

---

### Criteria 2: Multi-Agent Coordination Works Correctly

**How to verify**:
1. Run multi-agent coordination tests
2. Verify all coordination strategies work (merge, vote, collect, average)
3. Verify parallel execution works

**Step-by-step verification process**:
```bash
# Run multi-agent tests
cargo test agent::multi_agent

# Verify each coordination strategy
cargo test agent::multi_agent::merge
cargo test agent::multi_agent::vote
cargo test agent::multi_agent::collect
cargo test agent::multi_agent::average

# Verify parallel execution
cargo test agent::multi_agent::parallel
```

**Expected outcome**:
- All coordination strategies work correctly
- Parallel execution works
- Results are correct for each strategy

**Integration point**: Phase 2 (Multi-Agent Coordination)

---

## Anti-Goal-Drift Checkpoints

### Checkpoint 1: Prevent Drift into Over-Complex Patterns

**Drift risk**: Research could recommend overly complex agent patterns that are not proven or practical.

**Detection method**: Verify all patterns are backed by research papers and real-world implementations. Reject experimental or unproven patterns.

**Validation**:
```bash
# Check for research citations
grep -r "arxiv.org" opencode/docs/plans/deep-research/04-agent-patterns-research.md

# Check for real-world implementations
grep -r "github.com" opencode/docs/plans/deep-research/04-agent-patterns-research.md
```

**Correction action**: If over-complex patterns are recommended, replace with proven, simple patterns.

---

## Research Tasks

### Task 1: Document Agent Orchestration Patterns

**Files:**
- Create: `./workspace/plans/research/evidence/react-loop-pattern.md`
- Create: `./workspace/plans/research/evidence/tool-calling-chain-pattern.md`
- Create: `./workspace/plans/research/evidence/multi-agent-coordination-patterns.md`

- [ ] **Step 1: Document ReAct loop pattern**

Document ReAct loop pattern with Rust implementation

Expected output: `react-loop-pattern.md`

- [ ] **Step 2: Document tool calling chain pattern**

Document tool calling chain pattern with Rust implementation

Expected output: `tool-calling-chain-pattern.md`

- [ ] **Step 3: Document multi-agent coordination patterns**

Document merge, vote, collect, average patterns with Rust implementations

Expected output: `multi-agent-coordination-patterns.md`

- [ ] **Step 4: Commit evidence artifacts**

Run: `git add ./workspace/plans/research/evidence/ && git commit -m "feat: add agent orchestration patterns evidence"`
Expected: Git commit successful

---

### Task 2: Document Convergence Detection Strategies

**Files:**
- Create: `./workspace/plans/research/evidence/convergence-detection-strategies.md`

- [ ] **Step 1: Research convergence detection strategies**

Research output stability, score threshold, iteration limit strategies

Expected output: Convergence detection strategies

- [ ] **Step 2: Document convergence detection with Rust implementations**

Document all convergence detection strategies with Rust code

Expected output: `convergence-detection-strategies.md`

- [ ] **Step 3: Commit evidence artifacts**

Run: `git add ./workspace/plans/research/evidence/ && git commit -m "feat: add convergence detection strategies evidence"`
Expected: Git commit successful

---

### Task 3: Complete All Research Validation and Integration

**Files:**
- Modify: `opencode/docs/plans/deep-research/04-agent-patterns-research.md`
- Create: `./workspace/plans/research/validation/agent-patterns-validation-report.md`

- [ ] **Step 1: Run all validation scripts**

Run: `bash scripts/validate-research-completeness.sh opencode/docs/plans/deep-research/04-agent-patterns-research.md`
Expected: All validation checks pass

- [ ] **Step 2: Run evidence quality validation**

Run: `bash scripts/validate-evidence-quality.sh opencode/docs/plans/deep-research/04-agent-patterns-research.md`
Expected: 100% evidence quality

- [ ] **Step 3: Run integration completeness validation**

Run: `bash scripts/validate-integration-completeness.sh opencode/docs/plans/deep-research/04-agent-patterns-research.md`
Expected: 100% integration completeness

- [ ] **Step 4: Run traceability validation**

Run: `bash scripts/validate-traceability.sh opencode/docs/plans/deep-research/04-agent-patterns-research.md`
Expected: 100% traceability

- [ ] **Step 5: Create validation report**

Write validation report summarizing all validation results and confirming research completion

Expected output: `agent-patterns-validation-report.md`

- [ ] **Step 6: Commit validation report**

Run: `git add ./workspace/plans/research/validation/ && git commit -m "feat: add agent patterns research validation report"`
Expected: Git commit successful

---

## References

1. **ReAct Paper**: https://arxiv.org/abs/2210.03629
2. **OpenAI Function Calling**: https://platform.openai.com/docs/guides/function-calling
3. **Multi-Agent Systems**: https://arxiv.org/abs/2307.11346
4. **LangChain**: https://python.langchain.com/
5. **AutoAgents**: https://github.com/liquidos-ai/AutoAgents
6. **Tokio Channels**: https://tokio.rs/tokio/tutorial/channels
7. **Event-Driven Architecture**: https://martinfowler.com/articles/20170114-event-driven.html
8. **ADR-0001**: Foundation Phase Architecture Decision
9. **ADR-0002**: MVP Queue & Scheduler Architecture Decision

---

**End of Agent Patterns Research Plan**
