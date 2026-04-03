# Manual Brainstorm Reference

Hand-crafted workflow design reference showing natural agentic patterns before formalization.

## What This Is

`agentic-workflow-manual-brainstorm.yml` is a brainstorm document written by a human thinking through how an agentic workflow should behave. It captures the design intent before being encoded into YAML schema.

```mermaid
graph LR
    A["Human Idea"] --> B["Manual Draft"]
    B --> C["YAML Schema"]
    C --> D["Validated Workflow"]

    style A fill:#FFD54F
    style D fill:#4CAF50,color:#fff
```

## Relationship to Examples

This brainstorm is the conceptual starting point. The categorized examples in `../requirements-oriented-auto/` are the formal, schema-compliant implementations derived from patterns like this.
