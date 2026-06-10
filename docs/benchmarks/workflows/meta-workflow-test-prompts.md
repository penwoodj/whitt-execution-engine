# Meta-Workflow Generator — Test Prompts

## SIMPLE (2-3 steps expected)

### Prompt 1: System Info JSON
```
Read /etc/hostname and /etc/os-release, then output the system's hostname and OS name as a single JSON line with keys "hostname" and "os_name"
```

### Prompt 2: File Line Count
```
Count the total number of lines in /etc/passwd and output just the number
```

## MEDIUM (4-6 steps expected)

### Prompt 3: Dependency Analysis
```
Read the Cargo.toml file in the current project, extract all crate dependencies with their versions, then output them as a numbered list sorted alphabetically by crate name
```

### Prompt 4: Model Inventory
```
List all .gguf model files in the models/ directory, extract just the base model name (before the quantization suffix), and output a unique sorted list of model families present
```

## COMPLEX (7+ steps expected)

### Prompt 5: Multi-Source Analysis
```
Read all YAML workflow files in docs/benchmarks/workflows/, for each file extract the workflow_id and step count, then produce a summary table showing workflow ID, number of steps, and which models are referenced
```

### Prompt 6: Config Audit
```
Analyze the unified workflow schema at docs/schema/unified-workflow-schema.yml. Extract all top-level schema sections, identify which sections are currently used by at least one workflow YAML in docs/benchmarks/workflows/, and produce a coverage report showing which schema sections are used vs unused
```
