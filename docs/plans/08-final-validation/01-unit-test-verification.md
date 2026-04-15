# Validation 01: Unit Test Verification

## Purpose
Run all unit tests across all modules, verify 90%+ code coverage, verify no compiler warnings, and verify proptest passes with 1000 iterations.

## Test Procedures

### 1. Run All Unit Tests

**Test:** Execute all unit tests in the workspace.

**Command:**
```bash
cargo test --lib --bins --all-features
```

**Expected Results:**
- All unit tests pass
- Test output shows `test result: ok. XX passed; 0 failed; 0 ignored; 0 measured; 0 filtered out`
- No tests are marked as `ignored`

**Verification:**
```bash
# Run tests with detailed output
cargo test --lib --bins --all-features -- --nocapture

# Verify exit code is 0
cargo test --lib --bins --all-features
echo "Exit code: $?"
# Expected: Exit code: 0

# Count passed tests
cargo test --lib --bins --all-features 2>&1 | grep "test result: ok"
# Expected: test result: ok. XX passed; 0 failed; 0 ignored; 0 measured; 0 filtered out

# Verify no tests failed
cargo test --lib --bins --all-features 2>&1 | grep "test result: FAILED"
# Expected: No output (empty)
```

### 2. Run Unit Tests by Module

**Test:** Run tests for each module individually to isolate failures.

**Command:**
```bash
# Run tests for each workspace member
for crate in $(cargo metadata --format-version 1 | jq -r '.workspace_members[]'); do
  echo "Testing crate: $crate"
  cargo test -p "$crate" --lib --all-features
done
```

**Expected Results:**
- All crates pass their unit tests
- Each crate shows `test result: ok`

**Verification:**
```bash
# Test individual modules
cargo test -p agentsdk-core --lib --all-features
cargo test -p agentsdk-cli --lib --all-features
cargo test -p agentsdk-backend --lib --all-features
cargo test -p agentsdk-queue --lib --all-features
cargo test -p agentsdk-scheduler --lib --all-features
cargo test -p agentsdk-metrics --lib --all-features

# Verify each test suite passed
cargo test -p agentsdk-core --lib --all-features 2>&1 | grep "test result: ok"
cargo test -p agentsdk-cli --lib --all-features 2>&1 | grep "test result: ok"
cargo test -p agentsdk-backend --lib --all-features 2>&1 | grep "test result: ok"
cargo test -p agentsdk-queue --lib --all-features 2>&1 | grep "test result: ok"
cargo test -p agentsdk-scheduler --lib --all-features 2>&1 | grep "test result: ok"
cargo test -p agentsdk-metrics --lib --all-features 2>&1 | grep "test result: ok"
# Expected: All show test result: ok
```

### 3. Verify Code Coverage

**Test:** Measure code coverage using `cargo-tarpaulin` or `cargo-llvm-cov`.

**Using cargo-tarpaulin:**
```bash
# Install tarpaulin if not already installed
cargo install cargo-tarpaulin

# Run coverage tests
cargo tarpaulin --out Html --output-dir ./coverage --all-features --workspace --timeout 300
```

**Expected Results:**
- Overall code coverage >= 90%
- Coverage report generated in `./coverage/index.html`
- Coverage report shows breakdown by module

**Verification:**
```bash
# Run tarpaulin with text output for quick verification
cargo tarpaulin --out Stdout --all-features --workspace --timeout 300

# Expected output includes:
# || Tested/Total Lines:
# || src/: 90% (900/1000)
# || ...

# Verify coverage threshold (grep for overall percentage)
cargo tarpaulin --out Stdout --all-features --workspace --timeout 300 2>&1 | grep "Overall"
# Expected: Overall coverage >= 90%

# Open HTML report (manual verification)
firefox ./coverage/index.html
# Expected: Coverage report shows >= 90% for most modules

# Check coverage for each module
cargo tarpaulin --out Stdout -p agentsdk-core --all-features --timeout 300
cargo tarpaulin --out Stdout -p agentsdk-cli --all-features --timeout 300
cargo tarpaulin --out Stdout -p agentsdk-backend --all-features --timeout 300
cargo tarpaulin --out Stdout -p agentsdk-queue --all-features --timeout 300
cargo tarpaulin --out Stdout -p agentsdk-scheduler --all-features --timeout 300
cargo tarpaulin --out Stdout -p agentsdk-metrics --all-features --timeout 300
# Expected: Each module shows >= 80% coverage
```

**Using cargo-llvm-cov (alternative):**
```bash
# Install cargo-llvm-cov
cargo install cargo-llvm-cov

# Setup and run coverage
cargo llvm-cov --all-features --workspace --html --output-dir ./coverage

# Check coverage summary
cargo llvm-cov report --all-features --workspace
```

**Verification:**
```bash
# Check coverage output
cargo llvm-cov report --all-features --workspace

# Expected output:
# Filename                           Regions    Missed Regions     Cover   Functions  Missed Functions     Executed       Lines      Missed Lines     Cover    Branches   Missed Branches     Cover
# ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------
# agentsdk-core/src/lib.rs           100          10               90.0%           50                5          45          1000              100   90.0%          500                50    90.0%
# ...

# Verify overall coverage >= 90%
```

### 4. Verify No Compiler Warnings

**Test:** Build the project and verify no warnings are emitted.

**Command:**
```bash
# Clean build to ensure all code is compiled
cargo clean

# Build with warnings as errors
cargo build --release --all-features --all-targets 2>&1 | tee build.log

# Check for warnings
grep "warning:" build.log
# Expected: No output (empty)
```

**Expected Results:**
- Build completes successfully
- No warnings in build output
- Exit code is 0

**Verification:**
```bash
# Build with deny-warnings
cargo clippy --all-targets --all-features -- -D warnings

# Expected: No clippy warnings, exit code 0

# Check for specific warning types
cargo build --release --all-features 2>&1 | grep "warning: unused"
# Expected: No output (empty)

cargo build --release --all-features 2>&1 | grep "warning: dead_code"
# Expected: No output (empty)

cargo build --release --all-features 2>&1 | grep "warning: unused_variables"
# Expected: No output (empty)

# Verify no clippy warnings
cargo clippy --all-targets --all-features 2>&1 | grep "warning:"
# Expected: No output (empty)
```

### 5. Run Proptest with 1000 Iterations

**Test:** Run property-based tests with 1000 iterations per test.

**Command:**
```bash
# Run proptest tests with 1000 iterations
PROPTEST_CASES=1000 cargo test --lib --all-features -- --test-threads=1 proptest
```

**Expected Results:**
- All property-based tests pass
- Each test runs 1000 iterations
- No shrinking failures
- No counterexamples found

**Verification:**
```bash
# Run proptest with verbose output
PROPTEST_CASES=1000 cargo test --lib --all-features -- --test-threads=1 --nocapture proptest

# Expected output includes:
# test proptest::test_properties_name ... ok in 2.34s
#   passed 1000 tests in 2.34s

# Verify all proptest tests passed
PROPTEST_CASES=1000 cargo test --lib --all-features -- proptest 2>&1 | grep "test result: ok"
# Expected: test result: ok

# Verify 1000 iterations ran
PROPTEST_CASES=1000 cargo test --lib --all-features -- proptest 2>&1 | grep "passed 1000 tests"
# Expected: Multiple lines showing "passed 1000 tests"

# Check for any failures
PROPTEST_CASES=1000 cargo test --lib --all-features -- proptest 2>&1 | grep "FAILED"
# Expected: No output (empty)
```

### 6. Verify Test Documentation

**Test:** Ensure all public functions have documented tests.

**Command:**
```bash
# Run doctests
cargo test --doc --all-features

# Check for undocumented public functions
cargo doc --no-deps --all-features 2>&1 | grep "missing documentation for"
# Expected: No output (empty)
```

**Expected Results:**
- All doctests pass
- No missing documentation warnings for public functions

**Verification:**
```bash
# Run doctests
cargo test --doc --all-features

# Expected output:
# Documenting agentsdk-core v0.1.0
#    Compiling agentsdk-core v0.1.0
#     Finished dev [unoptimized + debuginfo] target(s) in X.XXs
#    Checking agentsdk-core v0.1.0's documentation
#     Finished dev [unoptimized + debuginfo] target(s) in X.XXs
#    Running unittests (lib.rs)
# running X tests
# test src/lib.rs - ... (line X) ... ok
# test src/lib.rs - ... (line X) ... ok
# ...
# test result: ok. X passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```

### 7. Run Tests in Release Mode

**Test:** Run tests in release mode to catch issues with optimization.

**Command:**
```bash
# Run tests in release mode
cargo test --release --lib --all-features
```

**Expected Results:**
- All tests pass in release mode
- No optimization-related failures

**Verification:**
```bash
# Verify release mode tests pass
cargo test --release --lib --all-features 2>&1 | grep "test result: ok"
# Expected: test result: ok

# Check for any release-specific failures
cargo test --release --lib --all-features 2>&1 | grep "FAILED"
# Expected: No output (empty)
```

### 8. Verify Test Coverage for Critical Paths

**Test:** Ensure critical code paths are covered by tests.

**Command:**
```bash
# List critical files to check coverage
CRITICAL_FILES=(
  "agentsdk-core/src/schema/mod.rs"
  "agentsdk-core/src/execution/mod.rs"
  "agentsdk-backend/src/ollama.rs"
  "agentsdk-backend/src/lm_studio.rs"
  "agentsdk-backend/src/llama_cpp.rs"
  "agentsdk-queue/src/queue.rs"
  "agentsdk-scheduler/src/scheduler.rs"
)

# Check coverage for each critical file
for file in "${CRITICAL_FILES[@]}"; do
  echo "Checking coverage for: $file"
  if [ -f "$file" ]; then
    cargo tarpaulin --out Stdout --all-features --timeout 300 --files "$file" 2>&1 | grep "Cover"
  fi
done
```

**Expected Results:**
- All critical files have >= 85% coverage
- No critical code paths are uncovered

**Verification:**
```bash
# Check specific critical paths have tests
cargo test --lib --all-features schema 2>&1 | grep "test result: ok"
cargo test --lib --all-features execution 2>&1 | grep "test result: ok"
cargo test --lib --all-features backend 2>&1 | grep "test result: ok"
cargo test --lib --all-features queue 2>&1 | grep "test result: ok"
cargo test --lib --all-features scheduler 2>&1 | grep "test result: ok"
# Expected: All show test result: ok
```

### 9. Verify Test Performance

**Test:** Ensure tests run within acceptable time limits.

**Command:**
```bash
# Run tests with timing
cargo test --lib --all-features -- --nocapture 2>&1 | tee test_timing.log

# Check test execution time
grep "test result:" test_timing.log
```

**Expected Results:**
- All tests complete in < 5 minutes
- No individual test takes > 30 seconds
- No tests timeout

**Verification:**
```bash
# Measure test execution time
time cargo test --lib --all-features

# Expected:
# real    2mXX.XXXs
# user    XmXX.XXXs
# sys     XmXX.XXXs
# Total time should be < 5 minutes

# Check for slow tests (> 30s)
cargo test --lib --all-features -- --test-threads=1 --show-output 2>&1 | grep -E "test .*ok in [3-9][0-9]\.[0-9]+s"
# Expected: No output (no tests take 30+ seconds)
```

### 10. Verify Test Isolation

**Test:** Ensure tests don't interfere with each other.

**Command:**
```bash
# Run tests in random order multiple times
for i in {1..5}; do
  echo "Run $i:"
  cargo test --lib --all-features -- --test-threads=1 --shuffle
done
```

**Expected Results:**
- All tests pass in random order
- No test failures due to ordering

**Verification:**
```bash
# Run tests with shuffling
cargo test --lib --all-features -- --test-threads=1 --shuffle 2>&1 | grep "test result: ok"
# Expected: test result: ok

# Run multiple times to ensure consistency
for i in {1..3}; do
  cargo test --lib --all-features -- --test-threads=1 --shuffle 2>&1 | grep "test result: FAILED" && echo "FAILED on run $i"
done
# Expected: No output (all runs pass)
```

## Success Criteria

- ✅ All unit tests pass (0 failures)
- ✅ Overall code coverage >= 90%
- ✅ No compiler warnings
- ✅ Proptest passes with 1000 iterations per test
- ✅ All doctests pass
- ✅ All tests pass in release mode
- ✅ Critical code paths have >= 85% coverage
- ✅ All tests complete in < 5 minutes
- ✅ No individual test takes > 30 seconds
- ✅ Tests pass in random order (good isolation)

## Performance Targets

- Total test execution time: < 5 minutes
- Individual test time: < 30 seconds
- Proptest iteration time: < 5ms per iteration
- Coverage report generation: < 2 minutes

## Failure Procedures

**If tests fail:**
1. Identify which test(s) failed
2. Run the failing test with verbose output: `cargo test <test_name> -- --nocapture`
3. Review test output for error messages
4. Check if the test is flaky (run multiple times)
5. Review code changes for regressions
6. Fix the issue and re-run the test
7. Only proceed when all tests pass

**If coverage is below 90%:**
1. Identify low-coverage modules
2. Add unit tests for uncovered code
3. Ensure tests exercise all branches and edge cases
4. Re-run coverage measurement
5. Document why code cannot be tested (if applicable)
6. Only proceed when coverage >= 90%

**If compiler warnings exist:**
1. Review each warning in build output
2. Fix the issue (e.g., remove unused code, add #[allow] attribute if necessary)
3. Re-run build
4. Only proceed when no warnings exist

**If proptest fails:**
1. Identify the failing test and the counterexample
2. Review the proptest strategy for edge cases
3. Fix the implementation to handle the counterexample
4. Re-run proptest with 1000 iterations
5. Only proceed when proptest passes

**If tests are slow:**
1. Identify slow tests (> 30 seconds)
2. Optimize test implementation (reduce sleep times, use mocks, etc.)
3. Consider splitting slow tests into integration tests
4. Re-run tests
5. Only proceed when all tests complete in < 5 minutes

## Sign-Off Checklist

- [ ] All unit tests pass (0 failures)
- [ ] Code coverage >= 90% overall
- [ ] Code coverage >= 85% for critical paths
- [ ] No compiler warnings
- [ ] Proptest passes with 1000 iterations
- [ ] All doctests pass
- [ ] All tests pass in release mode
- [ ] All tests complete in < 5 minutes
- [ ] No individual test takes > 30 seconds
- [ ] Tests pass in random order
- [ ] Coverage report generated and reviewed
