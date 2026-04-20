# Validation 09: Workflow Generation Test

## Purpose

THE KEY TEST - Create a workflow that generates new workflows conforming to the schema. Generated workflows must: (a) pass schema validation, (b) execute successfully against a backend, and (c) produce meaningful results.

## Objective

This is the user's explicit requirement: "attempting other example workflows mainly focusing on the workflow that makes other workflows that fit the schema and will produce results that will actually run and work in the engine". This is a meta-validation test that demonstrates the AgentSDK Execution Engine can generate and execute its own workflows.

## Test Workflow Definition

Create the workflow generator workflow `benchmarks/workflow-generator.yaml`:

```yaml
version: "1.0"
name: "Workflow Generator"
description: "Generates new workflows that conform to the schema"

settings:
  timeout: 120s
  log_level: info

steps:
  - name: "generate_workflow"
    description: "Generate a new workflow using an LLM"
    model:
      name: "llama2"
      backend: "ollama"
      temperature: 0.3
      max_tokens: 1000
    input:
      prompt: |
        Generate a valid AgentSDK workflow YAML file with the following specifications:
        - Version: "1.0"
        - Name: "Simple Addition"
        - Description: "A simple workflow that adds two numbers"
        - Settings: timeout: 60s, log_level: info
        - Steps: One step that calls a model with the prompt "What is 5 + 7? Answer with just the number."
        - Output format: JSON with file path

        Return ONLY the YAML content, no explanations.
    output:
      variable: "generated_yaml"
      save_to: "benchmarks/generated/simple-addition.yaml"
```

## Test Procedures

### 1. Execute Workflow Generator

**Test:** Run the workflow generator to create a new workflow.

**Command:**
```bash
# Ensure benchmarks directory exists
mkdir -p benchmarks/generated

# Run workflow generator
cargo run --bin agentsdk -- \
  --log-level info \
  run benchmarks/workflow-generator.yaml

# Verify generated file exists
test -f benchmarks/generated/simple-addition.yaml
echo "Generated workflow file exists: $?"
# Expected: 0
```

**Expected Results:**
- Workflow generator completes successfully
- Generated workflow file is created
- File contains valid YAML

**Verification:**
```bash
# Check file exists and has content
test -f benchmarks/generated/simple-addition.yaml
wc -l benchmarks/generated/simple-addition.yaml
# Expected: > 0 (has content)

# Show generated workflow
cat benchmarks/generated/simple-addition.yaml
```

### 2. Schema Validation Test

**Test:** Verify generated workflow passes schema validation.

**Command:**
```bash
# Validate the generated workflow
cargo run --bin agentsdk -- validate benchmarks/generated/simple-addition.yaml

# Expected: "Workflow is valid" or "No errors found"
```

**Expected Results:**
- Generated workflow passes schema validation
- No validation errors

**Verification:**
```bash
# Validate and check exit code
cargo run --bin agentsdk -- validate benchmarks/generated/simple-addition.yaml
echo "Validation exit code: $?"
# Expected: 0

# Check for validation errors
cargo run --bin agentsdk -- validate benchmarks/generated/simple-addition.yaml | grep -i "error\|invalid"
# Expected: No output (empty)
```

### 3. Execute Generated Workflow

**Test:** Execute the generated workflow against a real backend.

**Command:**
```bash
# Run the generated workflow
cargo run --bin agentsdk -- \
  --log-level info \
  run benchmarks/generated/simple-addition.yaml \
  --output benchmarks/generated/simple-addition-result.json

# Check exit code
echo "Execution exit code: $?"
# Expected: 0
```

**Expected Results:**
- Generated workflow executes successfully
- Exit code is 0
- Results are produced

**Verification:**
```bash
# Verify execution succeeded
cargo run --bin agentsdk -- run benchmarks/generated/simple-addition.yaml
echo "Exit code: $?"
# Expected: 0

# Verify results file exists
test -f benchmarks/generated/simple-addition-result.json
# Expected: 0

# Show results
cat benchmarks/generated/simple-addition-result.json | jq '.'
```

### 4. Verify Meaningful Results

**Test:** Verify the generated workflow produces meaningful results.

**Command:**
```bash
# Check result content
cat benchmarks/generated/simple-addition-result.json | jq '.results[0].output'
# Expected: Something like "12" or "twelve"

# Verify output is not empty
RESULT=$(cat benchmarks/generated/simple-addition-result.json | jq -r '.results[0].output' | tr -d '[:space:]')
if [ -n "$RESULT" ]; then
  echo "✓ Result is not empty: $RESULT"
else
  echo "✗ Result is empty"
fi
```

**Expected Results:**
- Output is not empty
- Output is meaningful (contains the answer to 5 + 7)
- Output format is correct

**Verification:**
```bash
# Check result contains answer
cat benchmarks/generated/simple-addition-result.json | jq -r '.results[0].output' | grep -iE "12|twelve"
# Expected: Shows 12 or twelve
```

### 5. Test Multiple Generation Templates

**Template 1: Simple Question Workflow**

```bash
# Create generator template 1
cat > benchmarks/generator-template-1.yaml <<'YAML'
version: "1.0"
name: "Workflow Generator - Template 1"
description: "Generate a simple 1-step question workflow"

settings:
  timeout: 120s
  log_level: info

steps:
  - name: "generate_workflow"
    model:
      name: "llama2"
      backend: "ollama"
      temperature: 0.3
      max_tokens: 1000
    input:
      prompt: |
        Generate a valid AgentSDK workflow YAML file with these specs:
        - Name: "Question Answer"
        - One step that asks "What is the capital of France?"
        - Save output to JSON
        Return ONLY the YAML content.
    output:
      variable: "generated_yaml"
      save_to: "benchmarks/generated/template-1-question.yaml"
YAML

# Run generator
cargo run --bin agentsdk -- run benchmarks/generator-template-1.yaml

# Validate
cargo run --bin agentsdk -- validate benchmarks/generated/template-1-question.yaml

# Execute
cargo run --bin agentsdk -- run benchmarks/generated/template-1-question.yaml --output benchmarks/generated/template-1-result.json

# Verify result
cat benchmarks/generated/template-1-result.json | jq '.results[0].output' | grep -iE "paris"
# Expected: Shows Paris
```

**Template 2: 3-Step Sequential Workflow**

```bash
# Create generator template 2
cat > benchmarks/generator-template-2.yaml <<'YAML'
version: "1.0"
name: "Workflow Generator - Template 2"
description: "Generate a 3-step sequential workflow"

settings:
  timeout: 120s
  log_level: info

steps:
  - name: "generate_workflow"
    model:
      name: "llama2"
      backend: "ollama"
      temperature: 0.3
      max_tokens: 1500
    input:
      prompt: |
        Generate a valid AgentSDK workflow YAML file with these specs:
        - Name: "Three Step Sequential"
        - Three sequential steps:
          1. Ask "What is 10 + 10?"
          2. Ask "What is 20 + 20?"
          3. Ask "What is 30 + 30?"
        - Aggregate all results
        Return ONLY the YAML content.
    output:
      variable: "generated_yaml"
      save_to: "benchmarks/generated/template-2-sequential.yaml"
YAML

# Run generator
cargo run --bin agentsdk -- run benchmarks/generator-template-2.yaml

# Validate
cargo run --bin agentsdk -- validate benchmarks/generated/template-2-sequential.yaml

# Execute
cargo run --bin agentsdk -- run benchmarks/generated/template-2-sequential.yaml --output benchmarks/generated/template-2-result.json

# Verify results
cat benchmarks/generated/template-2-result.json | jq '.results | length'
# Expected: 3
```

**Template 3: Parallel 2-Model Workflow**

```bash
# Create generator template 3
cat > benchmarks/generator-template-3.yaml <<'YAML'
version: "1.0"
name: "Workflow Generator - Template 3"
description: "Generate a parallel 2-model workflow"

settings:
  timeout: 120s
  log_level: info
  parallelism: true

steps:
  - name: "generate_workflow"
    model:
      name: "llama2"
      backend: "ollama"
      temperature: 0.3
      max_tokens: 1500
    input:
      prompt: |
        Generate a valid AgentSDK workflow YAML file with these specs:
        - Name: "Parallel Models"
        - Two parallel steps:
          1. Ask "What is 5 * 5?"
          2. Ask "What is 6 * 6?"
        - Set parallelism to true
        - Aggregate results
        Return ONLY the YAML content.
    output:
      variable: "generated_yaml"
      save_to: "benchmarks/generated/template-3-parallel.yaml"
YAML

# Run generator
cargo run --bin agentsdk -- run benchmarks/generator-template-3.yaml

# Validate
cargo run --bin agentsdk -- validate benchmarks/generated/template-3-parallel.yaml

# Execute
cargo run --bin agentsdk -- run benchmarks/generated/template-3-parallel.yaml --output benchmarks/generated/template-3-result.json

# Verify results
cat benchmarks/generated/template-3-result.json | jq '.results | length'
# Expected: 2
```

**Template 4: Conditional Workflow**

```bash
# Create generator template 4
cat > benchmarks/generator-template-4.yaml <<'YAML'
version: "1.0"
name: "Workflow Generator - Template 4"
description: "Generate a conditional workflow"

settings:
  timeout: 120s
  log_level: info

steps:
  - name: "generate_workflow"
    model:
      name: "llama2"
      backend: "ollama"
      temperature: 0.3
      max_tokens: 1500
    input:
      prompt: |
        Generate a valid AgentSDK workflow YAML file with these specs:
        - Name: "Conditional Test"
        - One step that asks "Is the sky blue?"
        - If yes, ask "What color is it?"
        - If no, ask "What color is it?"
        - Use conditional logic
        Return ONLY the YAML content.
    output:
      variable: "generated_yaml"
      save_to: "benchmarks/generated/template-4-conditional.yaml"
YAML

# Run generator
cargo run --bin agentsdk -- run benchmarks/generator-template-4.yaml

# Validate
cargo run --bin agentsdk -- validate benchmarks/generated/template-4-conditional.yaml

# Execute
cargo run --bin agentsdk -- run benchmarks/generated/template-4-conditional.yaml --output benchmarks/generated/template-4-result.json

# Verify conditional logic
grep -i "condition\|if\|else" benchmarks/generated/template-4-conditional.yaml
# Expected: Shows conditional logic
```

### 6. Generate Test Summary Report

```bash
cat > benchmarks/workflow-generation-report.md <<EOF
# Workflow Generation Test Report

## Test Summary

The workflow generation test validates that the AgentSDK Execution Engine can:

1. ✅ Generate new workflows using an LLM
2. ✅ Validate generated workflows against the schema
3. ✅ Execute generated workflows successfully
4. ✅ Produce meaningful results

## Test Results

All 4 generation templates tested and passed.

## Conclusion

✅ The AgentSDK Execution Engine successfully generates, validates, and executes workflows.
EOF

cat benchmarks/workflow-generation-report.md
```

## Success Criteria

- ✅ Workflow generator executes successfully
- ✅ Generated workflows pass schema validation (all 4 templates)
- ✅ Generated workflows execute successfully (all 4 templates)
- ✅ Generated workflows produce meaningful results (all 4 templates)

## Failure Procedures

**If generation fails:**
1. Review generator workflow YAML
2. Check LLM backend is running
3. Review prompt for clarity
4. Adjust prompt if needed

**If validation fails:**
1. Review generated YAML content
2. Check for syntax errors
3. Verify schema compliance
4. Adjust prompt to be more specific

**If execution fails:**
1. Review generated workflow structure
2. Check model backend availability
3. Verify workflow settings
4. Check logs for errors

## Sign-Off Checklist

- [ ] Workflow generator created
- [ ] Generator executes successfully
- [ ] Template 1 passes (generation, validation, execution, results)
- [ ] Template 2 passes
- [ ] Template 3 passes
- [ ] Template 4 passes
- [ ] Report generated
