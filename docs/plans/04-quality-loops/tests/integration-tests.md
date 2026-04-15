# Integration Test Specifications

## Test Suite 1: Full Quality Loop Integration

### Test 1.1: Quality Loop Converges Successfully

**Setup:**
- Mock LLM: AlwaysConvergeLLM
- Verifiers: CodeVerifier (Rust)
- Max iterations: 10
- Quality thresholds: min_confidence=0.95, min_pass_rate=1.0

**Steps:**
1. Register verifier with registry
2. Create RepairLoopConfig with thresholds
3. Call run_quality_loop() with prompt
4. Wait for completion

**Expected Results:**
- Loop completes successfully
- Iteration count = 1 (first generation passes)
- State = Completed
- Artifact quality score >= 0.95
- No errors in result

### Test 1.2: Quality Loop Requires Repairs

**Setup:**
- Mock LLM: FixedIterationsLLM (converge_after=2)
- Verifiers: CodeVerifier (Rust)
- Max iterations: 10

**Steps:**
1. Register verifier with registry
2. Create RepairLoopConfig
3. Call run_quality_loop() with prompt
4. Wait for completion

**Expected Results:**
- Loop completes successfully
- Iteration count = 2
- State = Completed
- First iteration fails verification
- Repair step fixes issue
- Second iteration passes verification

### Test 1.3: Quality Loop Times Out

**Setup:**
- Mock LLM: NeverConvergeLLM
- Verifiers: CodeVerifier (Rust)
- Max iterations: 5
- Iteration timeout: 1s

**Steps:**
1. Register verifier with registry
2. Create RepairLoopConfig with timeout
3. Call run_quality_loop() with prompt
4. Wait for timeout

**Expected Results:**
- Loop times out
- State = TimedOut
- Returns Timeout error
- Generation count >= 1

### Test 1.4: Quality Loop Reaches Max Iterations

**Setup:**
- Mock LLM: NeverConvergeLLM
- Verifiers: CodeVerifier (Rust)
- Max iterations: 3

**Steps:**
1. Register verifier with registry
2. Create RepairLoopConfig with max_iterations=3
3. Call run_quality_loop() with prompt
4. Wait for max iterations

**Expected Results:**
- Loop reaches max iterations
- State = Failed
- Returns MaxIterationsExceeded error
- Iteration count = 3

## Test Suite 2: Benchmark Execution Integration

### Test 2.1: Execute Single Benchmark

**Setup:**
- Mock LLM: AlwaysConvergeLLM
- Verifiers: CodeVerifier, DocsVerifier
- Storage: Temporary SQLite

**Steps:**
1. Create ExecutionContext
2. Create BenchmarkSpec with valid prompt
3. Call execute_benchmark()
4. Store result
5. Retrieve and verify

**Expected Results:**
- Benchmark executes successfully
- Quality score >= threshold
- Duration captured
- Cost calculated
- Stored correctly
- Retrieved unchanged

### Test 2.2: Execute Benchmark Suite

**Setup:**
- Mock LLM: AlwaysConvergeLLM
- Verifiers: Multiple verifiers
- Storage: Temporary SQLite

**Steps:**
1. Create ExecutionContext
2. Create 5 BenchmarkSpecs with different file types
3. Call execute_suite()
4. Store all results
5. Query and verify

**Expected Results:**
- All 5 benchmarks execute
- All benchmarks stored
- Query returns all 5
- Results accurate

### Test 2.3: Execute Parallel Benchmarks

**Setup:**
- Mock LLM: AlwaysConvergeLLM
- Verifiers: Multiple verifiers
- Storage: Temporary SQLite
- Max concurrency: 3

**Steps:**
1. Create ExecutionContext
2. Create 10 BenchmarkSpecs
3. Call execute_parallel() with max_concurrency=3
4. Wait for completion
5. Verify order and count

**Expected Results:**
- All 10 benchmarks execute
- No more than 3 concurrent
- All results stored
- Total time < sequential time

## Test Suite 3: Full System Integration

### Test 3.1: Generate → Verify → Repair → Store

**Setup:**
- Mock LLM: FixedIterationsLLM (converge_after=2)
- Verifiers: CodeVerifier
- Storage: Temporary SQLite

**Steps:**
1. Create and run quality loop
2. Get final artifact
3. Run verifiers on artifact
4. Create benchmark from result
5. Store benchmark
6. Generate report

**Expected Results:**
- Loop converges in 2 iterations
- Artifact passes all verifiers
- Benchmark stored with correct provenance
- Report generated with correct metrics

### Test 3.2: Multi-File-Type Benchmarking

**Setup:**
- Mock LLM: AlwaysConvergeLLM
- Verifiers: CodeVerifier, DocsVerifier, ConfigVerifier
- Storage: Temporary SQLite

**Steps:**
1. Create benchmarks for code, docs, config
2. Execute all benchmarks
3. Store results
4. Calculate statistics by file type
5. Generate comparison report

**Expected Results:**
- 3 benchmarks execute successfully
- Statistics calculated per file type
- Comparison report generated
- Differences highlighted

### Test 3.3: Trend Analysis Over Time

**Setup:**
- Trend generator (30 days, improving)
- Storage: Temporary SQLite

**Steps:**
1. Generate 30 benchmarks with improving quality
2. Store all benchmarks
3. Calculate trends
4. Generate trend report
5. Verify insights

**Expected Results:**
- All 30 benchmarks stored
- Trend shows improvement
- Report highlights upward trend
- Insight suggests continuing strategy

## Test Suite 4: Error Recovery

### Test 4.1: LLM Error Handling

**Setup:**
- Mock LLM: Returns error on first call
- Verifiers: CodeVerifier
- Max iterations: 5

**Steps:**
1. Create quality loop with failing LLM
2. Attempt to run loop
3. Verify error handling

**Expected Results:**
- Error returned immediately
- No infinite loops
- Clear error message
- State clean

### Test 4.2: Verifier Error Handling

**Setup:**
- Mock LLM: AlwaysConvergeLLM
- Verifiers: One verifier throws error
- Max iterations: 5

**Steps:**
1. Create quality loop with failing verifier
2. Attempt to run loop
3. Verify error handling

**Expected Results:**
- Error returned
- Partial results captured
- Clear error message
- State consistent

### Test 4.3: Storage Error Handling

**Setup:**
- Mock LLM: AlwaysConvergeLLM
- Verifiers: CodeVerifier
- Storage: Invalid path (read-only)

**Steps:**
1. Attempt to store benchmark
2. Verify error handling

**Expected Results:**
- Storage error returned
- Benchmark data not lost
- Can retry with valid path

## Test Suite 5: Performance Integration

### Test 5.1: Large Benchmark Set

**Setup:**
- Mock LLM: AlwaysConvergeLLM
- Verifiers: CodeVerifier
- Generator: 1000 random benchmarks

**Steps:**
1. Generate 1000 benchmarks
2. Store all benchmarks
3. Query with various filters
4. Generate report

**Expected Results:**
- All 1000 stored
- Query performance acceptable (< 200ms)
- Report generation acceptable (< 1s)
- Memory usage bounded

### Test 5.2: Concurrent Quality Loops

**Setup:**
- Mock LLM: AlwaysConvergeLLM
- Verifiers: CodeVerifier
- Parallel loops: 10

**Steps:**
1. Launch 10 quality loops concurrently
2. Wait for all to complete
3. Verify no conflicts

**Expected Results:**
- All 10 loops complete
- No data corruption
- No deadlocks
- Performance scales
