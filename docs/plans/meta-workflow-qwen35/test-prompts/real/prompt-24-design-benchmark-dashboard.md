# Task: Design Benchmark Dashboard

## Objective
Pipeline runs produce logs and output files but there's no way to visualize results. A dashboard would show step-by-step progress, quality scores, and timing.

## Requirements
1. Read `src/benchmark/runner.rs` to understand what metrics are collected (duration_ms, token_count, quality_score)
2. Read `scripts/meta-v6/analyze-sw-logs.sh` to understand log format
3. Design a dashboard that:
   - Generates HTML report from benchmark.log + workflow YAML
   - Shows step-by-step progress (step name, status, duration, quality score)
   - Shows timeline visualization (Gantt chart of step execution)
   - Shows quality score distribution (bar chart per step)
   - Shows error/skip reasons
4. Implement as a Python script that reads benchmark outputs and generates HTML
5. Write the complete implementation

## Output
A markdown document containing:
- Dashboard architecture design
- Python implementation (standalone script)
- HTML/CSS/JS template for the dashboard
- Example usage: `python3 generate-dashboard.py <benchmark-log> <output-dir>`
