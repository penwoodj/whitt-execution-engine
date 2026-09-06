# Reflexion: Language Agents with Verbal Reinforcement Learning

## URL
https://arxiv.org/abs/2303.11366

## Method Mechanics

**Core Components:**
- **Actor Model (Ma):** Generates text/actions conditioned on state observations, uses Chain of Thought or ReAct as base
- **Evaluator Model (Me):** Scores outputs produced by actor, provides scalar reward (0-1 or task-specific)
- **Self-Reflection Model (Msr):** Generates verbal self-reflections as "semantic gradient" feedback
- **Memory Buffer (mem):** Stores episodic reflections for future trials

**Loop Structure (Algorithm 1):**
```
Initialize Actor, Evaluator, Self-Reflection models
Initialize policy πθ = {Ma, mem}
Generate initial trajectory τ0 using πθ
Evaluate τ0 using Me → reward r0
Generate initial self-reflection sr0 using Msr
Set mem ← [sr0]
Set t ← 0

while Me not pass OR t < max_trials do:
    Generate τt = [a0, o0, ... ai, oi] using πθ
    Evaluate τt using Me → reward rt
    Generate self-reflection srt using Msr
    Append srt to mem
    Increment t
end while
```

**Memory Management:**
- Short-term memory: trajectory history (current trial's action-observation pairs)
- Long-term memory: episodic reflections (bound to max experiences Ω = 1-3 for context limits)
- FAISS vector search for retrieval (task-specific embeddings)

**Replay Mechanism:**
- Agent conditions decisions on both short-term and long-term memory
- Recall searches memory for similar task contexts via embedding similarity

## Healing Trigger

**Primary Triggers:**
- **Evaluator failure:** Binary success/fail signal indicates task incomplete
- **Heuristic triggers (AlfWorld-specific):**
  - Same action repeated >3 cycles with same response
  - Actions taken >30 (inefficient planning)
- **Explicit self-reflection:** Agent can trigger reflection proactively

**Feedback Signal Types:**
- Scalar reward from environment (task-specific success metric)
- Binary success/fail from Evaluator
- Free-form language feedback from Self-Reflection model

## Stop Condition

**Hard Limits:**
- `max_trials`: Maximum number of trials (task-specific, often 3-5)
- `max_episodes`: Total number of environment episodes allowed

**Soft Termination:**
- Evaluator deems trajectory τt correct (pass)
- Environment signals task completion
- Reflections indicate task mastery (rare in practice)

**Memory Eviction:**
- FIFO or LRU eviction when memory exceeds size Ω
- Typically keeps last 1-3 most relevant reflections

## Evaluation Metrics

**Task-Specific:**
- **Sequential decision-making (AlfWorld):** Task completion rate, action efficiency
- **Coding (HumanEval):** Pass@1 accuracy (91% vs GPT-4's 80%)
- **Reasoning (HotPotQA):** Multi-hop QA accuracy
- **Navigation (WebShop):** Product finding success rate

**Generic Agent Metrics:**
- **Trial reduction:** How many trials needed per task (memory transfer effect)
- **Reflection quality:** Whether reflections contain actionable corrections
- **Sample efficiency:** Cost per success vs baseline

**Ablation Metrics:**
- With vs without memory
- With vs without self-reflection
- Different memory sizes (Ω)
- Different feedback signal types (scalar vs language)

## What to Borrow

### For Meta-Workflow Generator (Experiment Design)

**1. Failure Detection Signal → Classify Step:**
- **Pattern:** Evaluator provides scalar success/fail + detailed reasoning about failure
- **Implementation:** Task-specific success criteria (e.g., code passes unit tests, answer matches ground truth)
- **Borrow:** Use structured Evaluator output as feedback signal for meta-workflow generator's classification step

**2. Reflective Feedback Generation → Heal Step:**
- **Pattern:** Self-Reflection model converts {trajectory, reward, memory} → actionable verbal lesson
- **Implementation:** LLM prompt: "Given this trajectory that failed, what went wrong and how should it be fixed?"
- **Key insight:** Reflections are task-AGNOSTIC (transferable), not task-specific
- **Borrow:** Use this pattern for meta-workflow to generate corrective prompts that apply to future similar tasks

**3. Episodic Memory → History/State:**
- **Pattern:** FAISS-indexed vector store of reflections with retrieval
- **Implementation:** Store {task_embedding, reflection_text, metadata} and query by similarity
- **Key insight:** Enables cross-task learning - agent benefits from past failures on related tasks
- **Borrow:** For meta-workflow: store healing episodes (failure→diagnosis→fix→outcome) and retrieve by task similarity

**4. Bounded Iteration Strategy:**
- **Pattern:** `while (not passed && t < max_trials)` with memory-driven acceleration
- **Implementation:** Set max iteration budget per task, but allow early termination if evaluator passes
- **Borrow:** Define iteration budgets in meta-workflow YAML (e.g., `max_iterations: 5`)

### For Meta-Workflow Engine (Runtime Support)

**1. Three-Model Architecture:**
```
Actor → generates candidate solution
Evaluator → validates against objective (can be deterministic checker or LLM-as-judge)
Self-Reflection → generates correction guidance
Memory → stores/retrieves past lessons
```

**2. Feedback Signal Integration:**
- Combine scalar reward (task-specific) with verbal reflection (actionable advice)
- Example: "FAILED (0.3) - The code has syntax error on line 5. Missing closing brace causes compilation failure."

**3. Memory Retrieval Mechanism:**
- Embedding-based similarity search: `cosine_similarity(task_embedding, stored_reflections)` 
- Return top-k reflections as context for next iteration

**4. Episode Management:**
- Track trials per task across workflow executions
- Enable cross-session learning (persistent memory across runs)

## Concrete Implementation Details

**Reflexion Reflection Schema:**
```
{
  "root_cause": "Action a_i led to incorrect subsequent actions a_i+1 and a_i+2",
  "correction": "Should have taken alternative action a_i' instead",
  "reflection": "..."
}
```

**Evaluation Prompt Pattern (Self-Refine influence):**
```
Given the trajectory τt and reward rt:
1. Identify what went wrong
2. Provide actionable correction
3. Store as structured reflection
```

**Memory Update Rule:**
```python
mem.append(new_reflection)
if len(mem) > Ω:
    mem.pop(0)  # FIFO or LRU
```

**Key Constraints:**
- Requires evaluator that can produce success/fail signal (task-specific)
- Memory bounded by context window (typically 1-3 reflections)
- Self-reflection quality depends on base model capability
- No parameter updates - all in-context learning

## Limitations

1. **Local Optima:** Can get stuck when reflections don't identify root cause
2. **Evaluator Dependency:** Requires reliable success/fail signal (hard for some tasks)
3. **Memory Confusion:** Reflections from unrelated tasks can pollute context if retrieval is imprecise
4. **No Model Training:** Limited by base model's reasoning capability; cannot improve beyond prompt engineering

## References

- Paper: Shinn et al. (2023). "Reflexion: Language Agents with Verbal Reinforcement Learning." NeurIPS 2023.
- Algorithm: Algorithm 1 in paper (Reinforcement via self-reflection)
- Code: https://github.com/noahshinn024/reflexion
- Related: Tree of Thoughts (Yao et al., 2023), ReAct (Yao et al., 2023b), In-context Policy Iteration (Brooks et al., 2022)
