# Capability Matrix Validation Criteria

## FileCapability Struct Validation

### Required Fields

- [ ] `workflow_id: String` field present
- **Verification:**
  - Workflow identifier
  - Unique per workflow
  - Non-empty string
- **Test Commands:**
  ```bash
  # Verify field presence
  grep "workflow_id:" src/quality/capability.rs
  ```
- **Expected Output:** Field present
- **Evidence:** Type definition output

- [ ] `file_type: ArtifactType` field present
- **Verification:**
  - File type classification
  - One of: YAML, Rust, Python, JavaScript, Markdown, JSON, Text
  - Non-null
- **Test Commands:**
  ```bash
  # Verify field presence
  grep "file_type:" src/quality/capability.rs
  ```
- **Expected Output:** Field present
- **Evidence:** Type definition output

- [ ] `quality_score: f64` field present
- **Verification:**
  - Quality metric (0.0 to 1.0)
  - 1.0 = perfect quality
  - 0.0 = no quality
- **Test Commands:**
  ```bash
  # Verify field presence
  grep "quality_score:" src/quality/capability.rs
  ```
- **Expected Output:** Field present
- **Evidence:** Type definition output

- [ ] `avg_iterations: f64` field present
- **Verification:**
  - Average iterations to converge
  - Based on historical benchmarks
  - >= 1.0
- **Test Commands:**
  ```bash
  # Verify field presence
  grep "avg_iterations:" src/quality/capability.rs
  ```
- **Expected Output:** Field present
- **Evidence:** Type definition output

- [ ] `success_rate: f64` field present
- **Verification:**
  - Success rate (0.0 to 1.0)
  - Based on historical benchmarks
  - Non-null
- **Test Commands:**
  ```bash
  # Verify field presence
  grep "success_rate:" src/quality/capability.rs
  ```
- **Expected Output:** Field present
- **Evidence:** Type definition output

- [ ] `last_updated: DateTime<Utc>` field present
- **Verification:**
  - Timestamp of last update
  - UTC timezone
  - ISO 8601 format
- **Test Commands:**
  ```bash
  # Verify field presence
  grep "last_updated:" src/quality/capability.rs
  ```
- **Expected Output:** Field present
- **Evidence:** Type definition output

- [ ] `supported_operations: Vec<Operation>` field present
- **Verification:**
  - List of operations supported
  - Example: generate, verify, repair
  - At least one operation
- **Test Commands:**
  ```bash
  # Verify field presence
  grep "supported_operations:" src/quality/capability.rs
  ```
- **Expected Output:** Field present
- **Evidence:** Type definition output

- [ ] `max_cost_usd: Option<f64>` field present
- **Verification:**
  - Maximum cost per operation (USD)
  - None = no limit
  - Used for optimization
- **Test Commands:**
  ```bash
  # Verify field presence
  grep "max_cost_usd:" src/quality/capability.rs
  ```
- **Expected Output:** Field present
- **Evidence:** Type definition output

- [ ] `max_duration_ms: Option<u64>` field present
- **Verification:**
  - Maximum duration per operation (ms)
  - None = no limit
  - Used for optimization
- **Test Commands:**
  ```bash
  # Verify field presence
  grep "max_duration_ms:" src/quality/capability.rs
  ```
- **Expected Output:** Field present
- **Evidence:** Type definition output

### Derived Fields

- [ ] `total_benchmarks: u64` field present
- **Verification:**
  - Total benchmarks run
  - Incremented on each benchmark
  - Non-zero after benchmarks
- **Test Commands:**
  ```bash
  # Verify field presence
  grep "total_benchmarks:" src/quality/capability.rs
  ```
- **Expected Output:** Field present
- **Evidence:** Type definition output

- [ ] `total_successes: u64` field present
- **Verification:**
  - Total successful benchmarks
  - Calculated from total_benchmarks
  - Consistent with success_rate
- **Test Commands:**
  ```bash
  # Verify field presence
  grep "total_successes:" src/quality/capability.rs
  ```
- **Expected Output:** Field present
- **Evidence:** Type definition output

- [ ] `confidence_interval: Option<ConfidenceInterval>` field present
- **Verification:**
  - Statistical confidence interval
  - None if insufficient data
  - Used for quality estimates
- **Test Commands:**
  ```bash
  # Verify field presence
  grep "confidence_interval:" src/quality/capability.rs
  ```
- **Expected Output:** Field present
- **Evidence:** Type definition output

---

## CapabilityMatrix Implementation Validation

### Data Structure

- [ ] CapabilityMatrix uses (workflow_id, file_type) as key
- **Verification:**
  - Internal map uses tuple key
  - BTreeMap or HashMap for efficient lookup
  - No duplicate keys possible
- **Test Commands:**
  ```bash
  # Verify data structure
  grep -A 5 "struct CapabilityMatrix" src/quality/matrix.rs
  ```
- **Expected Output:** Tuple key structure
- **Evidence:** Struct definition output

- [ ] Each key maps to exactly one FileCapability
- **Verification:**
  - Type: HashMap<(String, ArtifactType), FileCapability>
  - One-to-one mapping
  - No duplicate keys
- **Test Commands:**
  ```bash
  # Verify mapping type
  grep "HashMap<(String, ArtifactType), FileCapability>" src/quality/matrix.rs
  ```
- **Expected Output:** One-to-one mapping type
- **Evidence:** Type definition output

- [ ] No duplicate keys possible
- **Verification:**
  - register() checks for existing key
  - Duplicate returns error
  - Existing key not overwritten
- **Test Commands:**
  ```bash
  # Test duplicate handling
  cargo test --lib quality::tests::duplicate_key_handling
  ```
- **Expected Output:** Duplicates rejected
- **Evidence:** Test output log

- [ ] by_file_type index consistent with main map
- **Verification:**
  - Secondary index: HashMap<ArtifactType, Vec<(String, FileCapability)>>
  - Updated on every register()
  - Consistent with main map
- **Test Commands:**
  ```bash
  # Verify index consistency
  cargo test --lib quality::tests::index_consistency
  ```
- **Expected Output:** Index consistent
- **Evidence:** Test output log

- [ ] All capabilities in by_file_type exist in main map
- **Verification:**
  - Every entry in index exists in main map
  - No orphaned index entries
  - No missing main map entries
- **Test Commands:**
  ```bash
  # Verify index completeness
  cargo test --lib quality::tests::index_completeness
  ```
- **Expected Output:** Index complete
- **Evidence:** Test output log

### Registration API

- [ ] register() adds to main map
- **Verification:**
  - register() inserts FileCapability
  - Key generated from workflow_id and file_type
  - Insert successful
- **Test Commands:**
  ```bash
  # Test registration
  cargo test --lib quality::tests::register_to_main_map
  ```
- **Expected Output:** Registration works
- **Evidence:** Test output log

- [ ] register() adds to by_file_type index
- **Verification:**
  - register() updates secondary index
  - Index entry for file_type created/updated
  - Workflow added to list
- **Test Commands:**
  ```bash
  # Test index update
  cargo test --lib quality::tests::register_to_index
  ```
- **Expected Output:** Index updated
- **Evidence:** Test output log

- [ ] register() returns error on duplicate
- **Verification:**
  - Duplicate registration returns CapabilityError
  - Error type: DuplicateKey
  - Original capability not modified
- **Test Commands:**
  ```bash
  # Test duplicate detection
  cargo test --lib quality::tests::register_duplicate
  ```
- **Expected Output:** Duplicate rejected with error
- **Evidence:** Test output log

- [ ] register() updates existing if allowed
- **Verification:**
  - Optional update flag
  - If true, existing updated
  - If false, error returned
- **Test Commands:**
  ```bash
  # Test update behavior
  cargo test --lib quality::tests::register_update
  ```
- **Expected Output:** Update works when allowed
- **Evidence:** Test output log

- [ ] Thread-safe registration (uses Arc<RwLock>)
- **Verification:**
  - Matrix wrapped in Arc<RwLock>
  - Multiple threads can register
  - No data races
- **Test Commands:**
  ```bash
  # Test thread safety
  cargo test --lib quality::tests::thread_safe_registration
  ```
- **Expected Output:** Thread-safe operations
- **Evidence:** Test output log

### Query Interface

#### Basic Queries

- [ ] get() returns capability for exact (workflow_id, file_type)
- **Verification:**
  - Returns Option<FileCapability>
  - Some() if found
  - None if not found
- **Test Commands:**
  ```bash
  # Test get operation
  cargo test --lib quality::tests::get_exact
  ```
- **Expected Output:** Correct capability returned
- **Evidence:** Test output log

- [ ] get() returns None if not found
- **Verification:**
  - Non-existent key returns None
  - No panic or error
  - Graceful handling
- **Test Commands:**
  ```bash
  # Test not found case
  cargo test --lib quality::tests::get_not_found
  ```
- **Expected Output:** None returned
- **Evidence:** Test output log

- [ ] get_for_file_type() returns all capabilities for file_type
- **Verification:**
  - Returns Vec<FileCapability>
  - All workflows for type included
  - Empty vector if none found
- **Test Commands:**
  ```bash
  # Test file type query
  cargo test --lib quality::tests::get_by_file_type
  ```
- **Expected Output:** All capabilities returned
- **Evidence:** Test output log

- [ ] get_for_file_type() returns empty Vec if none found
- **Verification:**
  - Non-existent file type returns empty vector
  - No panic or error
  - Graceful handling
- **Test Commands:**
  ```bash
  # Test empty result case
  cargo test --lib quality::tests::get_by_file_type_empty
  ```
- **Expected Output:** Empty vector returned
- **Evidence:** Test output log

- [ ] Results sorted consistently
- **Verification:**
  - Results sorted by quality_score (descending)
  - Ties broken by last_updated (descending)
  - Deterministic order
- **Test Commands:**
  ```bash
  # Test sorting
  cargo test --lib quality::tests::result_sorting
  ```
- **Expected Output:** Consistent order
- **Evidence:** Test output log

#### Advanced Queries

- [ ] find_best_workflow() returns highest confidence matching requirement
- **Verification:**
  - Returns Option<FileCapability>
  - Filters by requirements
  - Selects highest quality_score
- **Test Commands:**
  ```bash
  # Test best workflow query
  cargo test --lib quality::tests::find_best_workflow
  ```
- **Expected Output:** Best workflow returned
- **Evidence:** Test output log

- [ ] find_best_workflow() filters by min_quality_score
- **Verification:**
  - Returns None if no workflow >= threshold
  - Threshold parameter: min_quality_score
  - Only workflows above threshold considered
- **Test Commands:**
  ```bash
  # Test quality score filter
  cargo test --lib quality::tests::filter_by_quality_score
  ```
- **Expected Output:** Filter applied correctly
- **Evidence:** Test output log

- [ ] find_best_workflow() filters by required_operations
- **Verification:**
  - Returns None if no workflow supports all operations
  - Required operations parameter: required_ops
  - Only workflows with all operations considered
- **Test Commands:**
  ```bash
  # Test operations filter
  cargo test --lib quality::tests::filter_by_operations
  ```
- **Expected Output:** Filter applied correctly
- **Evidence:** Test output log

- [ ] find_best_workflow() respects max_cost_usd if specified
- **Verification:**
  - Returns None if no workflow <= max_cost
  - Max cost parameter: max_cost_usd
  - Only workflows within cost considered
- **Test Commands:**
  ```bash
  # Test cost filter
  cargo test --lib quality::tests::filter_by_cost
  ```
- **Expected Output:** Filter applied correctly
- **Evidence:** Test output log

- [ ] find_best_workflow() respects max_duration_ms if specified
- **Verification:**
  - Returns None if no workflow <= max_duration
  - Max duration parameter: max_duration_ms
  - Only workflows within duration considered
- **Test Commands:**
  ```bash
  # Test duration filter
  cargo test --lib quality::tests::filter_by_duration
  ```
- **Expected Output:** Filter applied correctly
- **Evidence:** Test output log

### Filtering

- [ ] File type filtering accurate
- **Verification:**
  - Filter parameter: file_type
  - Only matching file types returned
  - Case-sensitive matching
- **Test Commands:**
  ```bash
  # Test file type filtering
  cargo test --lib quality::tests::filter_file_type
  ```
- **Expected Output:** Filtering accurate
- **Evidence:** Test output log

- [ ] Workflow ID filtering accurate
- **Verification:**
  - Filter parameter: workflow_id
  - Only matching workflows returned
  - Exact match required
- **Test Commands:**
  ```bash
  # Test workflow ID filtering
  cargo test --lib quality::tests::filter_workflow_id
  ```
- **Expected Output:** Filtering accurate
- **Evidence:** Test output log

- [ ] Min confidence filtering accurate
- **Verification:**
  - Filter parameter: min_confidence
  - Only workflows >= threshold returned
  - None if no workflows meet threshold
- **Test Commands:**
  ```bash
  # Test confidence filtering
  cargo test --lib quality::tests::filter_min_confidence
  ```
- **Expected Output:** Filtering accurate
- **Evidence:** Test output log

- [ ] Operations support filtering accurate
- **Verification:**
  - Filter parameter: operations
  - Only workflows supporting all operations returned
  - Empty vector if none found
- **Test Commands:**
  ```bash
  # Test operations filtering
  cargo test --lib quality::tests::filter_operations
  ```
- **Expected Output:** Filtering accurate
- **Evidence:** Test output log

- [ ] Multiple filters combined with AND
- **Verification:**
  - Multiple filters applied
  - All filters must pass
  - Result is intersection
- **Test Commands:**
  ```bash
  # Test combined filters
  cargo test --lib quality::tests::combined_filters
  ```
- **Expected Output:** Filters combined with AND
- **Evidence:** Test output log

---

## Gap Analysis Validation

### Missing Capabilities

- [ ] identify_missing_capabilities() finds workflows with no capability for file_type
- **Verification:**
  - Returns Vec<(workflow_id, file_type)>
  - All gaps identified
  - No false positives
- **Test Commands:**
  ```bash
  # Test gap identification
  cargo test --lib quality::tests::identify_missing_capabilities
  ```
- **Expected Output:** All gaps found
- **Evidence:** Test output log

- [ ] Returns list of (workflow_id, file_type) pairs
- **Verification:**
  - Each missing capability is a tuple
  - Workflow ID present
  - File type present
  - List is complete
- **Test Commands:**
  ```bash
  # Verify return type
  grep -A 2 "fn identify_missing_capabilities" src/quality/matrix.rs
  ```
- **Expected Output:** Correct return type
- **Evidence:** Function signature output

- [ ] Doesn't report false positives
- **Verification:**
  - Existing capabilities not reported
  - Only true gaps reported
  - Zero false positives
- **Test Commands:**
  ```bash
  # Test false positives
  cargo test --lib quality::tests::gap_analysis_false_positives
  ```
- **Expected Output:** No false positives
- **Evidence:** Test output log

- [ ] Doesn't miss actual gaps
- **Verification:**
  - All missing capabilities reported
  - No gaps missed
  - Zero false negatives
- **Test Commands:**
  ```bash
  # Test false negatives
  cargo test --lib quality::tests::gap_analysis_false_negatives
  ```
- **Expected Output:** No false negatives
- **Evidence:** Test output log

### Quality Gaps

- [ ] identify_quality_gaps() finds capabilities below threshold
- **Verification:**
  - Returns Vec<(workflow_id, file_type, gap_size)>
  - Gap size = required - actual
  - Only gaps > 0 included
- **Test Commands:**
  ```bash
  # Test quality gap identification
  cargo test --lib quality::tests::identify_quality_gaps
  ```
- **Expected Output:** All quality gaps found
- **Evidence:** Test output log

- [ ] Compares actual quality to required quality
- **Verification:**
  - Threshold parameter: min_quality_score
  - Gap calculation: threshold - quality_score
  - Negative gaps (exceeds threshold) excluded
- **Test Commands:**
  ```bash
  # Verify gap calculation
  cargo test --lib quality::tests::gap_calculation
  ```
- **Expected Output:** Gaps calculated correctly
- **Evidence:** Test output log

- [ ] Returns gap size (required - actual)
- **Verification:**
  - Gap size in result tuple
  - Range: 0.0 to 1.0
  - Positive values only
- **Test Commands:**
  ```bash
  # Verify gap size field
  grep -A 3 "fn identify_quality_gaps" src/quality/matrix.rs
  ```
- **Expected Output:** Gap size present
- **Evidence:** Function signature output

- [ ] Prioritizes by gap size
- **Verification:**
  - Results sorted by gap_size (descending)
  - Largest gaps first
  - Useful for prioritization
- **Test Commands:**
  ```bash
  # Test gap prioritization
  cargo test --lib quality::tests::gap_prioritization
  ```
- **Expected Output:** Gaps prioritized
- **Evidence:** Test output log

### Opportunities

- [ ] identify_improvement_opportunities() suggests high-impact improvements
- **Verification:**
  - Returns Vec<ImprovementOpportunity>
  - Opportunities prioritized by impact
  - Recommendations actionable
- **Test Commands:**
  ```bash
  # Test opportunity identification
  cargo test --lib quality::tests::identify_improvement_opportunities
  ```
- **Expected Output:** High-impact opportunities identified
- **Evidence:** Test output log

- [ ] Considers frequency of use
- **Verification:**
  - Frequently used workflows prioritized
  - Frequency data from benchmarks
  - High impact = high frequency + large gap
- **Test Commands:**
  ```bash
  # Test frequency consideration
  cargo test --lib quality::tests::opportunity_frequency
  ```
- **Expected Output:** Frequency considered
- **Evidence:** Test output log

- [ ] Considers gap size
- **Verification:**
  - Large gaps prioritized
  - Gap size from identify_quality_gaps()
  - High impact = large gap + high frequency
- **Test Commands:**
  ```bash
  # Test gap size consideration
  cargo test --lib quality::tests::opportunity_gap_size
  ```
- **Expected Output:** Gap size considered
- **Evidence:** Test output log

- [ ] Considers ease of improvement
- **Verification:**
  - Easy improvements prioritized
  - Ease estimated from benchmark data
  - High impact = easy + large gap + high frequency
- **Test Commands:**
  ```bash
  # Test ease consideration
  cargo test --lib quality::tests::opportunity_ease
  ```
- **Expected Output:** Ease considered
- **Evidence:** Test output log

- [ ] Returns prioritized list
- **Verification:**
  - Results sorted by impact score
  - Impact score = f(gap_size, frequency, ease)
  - Highest impact first
- **Test Commands:**
  ```bash
  # test prioritization
  cargo test --lib quality::tests::opportunity_prioritization
  ```
- **Expected Output:** Opportunities prioritized
- **Evidence:** Test output log

---

## Recommendations Engine Validation

### Recommendation Generation

- [ ] generate_recommendations() creates specific recommendations
- **Verification:**
  - Returns Vec<Recommendation>
  - Each recommendation is actionable
  - Recommendations are specific
- **Test Commands:**
  ```bash
  # Test recommendation generation
  cargo test --lib quality::tests::generate_recommendations
  ```
- **Expected Output:** Specific recommendations generated
- **Evidence:** Test output log

- [ ] Recommendations include problem description
- **Verification:**
  - Each recommendation has problem field
  - Problem is clear and specific
  - Problem relates to gap or opportunity
- **Test Commands:**
  ```bash
  # Test problem descriptions
  cargo test --lib quality::tests::recommendation_problems
  ```
- **Expected Output:** Problems clear and specific
- **Evidence:** Test output log

- [ ] Recommendations include solution
- **Verification:**
  - Each recommendation has solution field
  - Solution is specific and implementable
  - Solution addresses problem
- **Test Commands:**
  ```bash
  # Test solutions
  cargo test --lib quality::tests::recommendation_solutions
  ```
- **Expected Output:** Solutions specific and implementable
- **Evidence:** Test output log

- [ ] Recommendations include priority
- **Verification:**
  - Each recommendation has priority field
  - Priority levels: high, medium, low
  - Prioritization based on impact
- **Test Commands:**
  ```bash
  # Test priorities
  cargo test --lib quality::tests::recommendation_priorities
  ```
- **Expected Output:** Priorities assigned correctly
- **Evidence:** Test output log

- [ ] Recommendations include effort estimate
- **Verification:**
  - Each recommendation has effort field
  - Effort levels: easy, medium, hard
  - Effort based on implementation complexity
- **Test Commands:**
  ```bash
  # Test effort estimates
  cargo test --lib quality::tests::recommendation_effort
  ```
- **Expected Output:** Efforts estimated accurately
- **Evidence:** Test output log

- [ ] Recommendations include impact estimate
- **Verification:**
  - Each recommendation has impact field
  - Impact levels: high, medium, low
  - Impact based on quality improvement
- **Test Commands:**
  ```bash
  # Test impact estimates
  cargo test --lib quality::tests::recommendation_impact
  ```
- **Expected Output:** Impacts estimated accurately
- **Evidence:** Test output log

---

## Visualization Format Validation

### Matrix Display

- [ ] Display as table with workflow_id, file_type, quality_score
- **Verification:**
  - CLI command: `quality matrix`
  - Table format with headers
  - All capabilities shown
- **Test Commands:**
  ```bash
  # Test display
  cargo run --bin agentsdk -- quality matrix
  ```
- **Expected Output:** Table with correct columns
- **Evidence:** Command output screenshot

- [ ] Color coding by quality_score ranges
- **Verification:**
  - High quality (>= 0.8): green
  - Medium quality (0.5 - 0.8): yellow
  - Low quality (< 0.5): red
  - Colors applied consistently
- **Test Commands:**
  ```bash
  # Test color coding
  cargo run --bin agentsdk -- quality matrix
  ```
- **Expected Output:** Correct colors displayed
- **Evidence:** Command output screenshot

- [ ] Sorting by column supported
- **Verification:**
  - CLI option: `--sort-by <column>`
  - Supports: workflow_id, file_type, quality_score
  - Ascending and descending
- **Test Commands:**
  ```bash
  # Test sorting
  cargo run --bin agentsdk -- quality matrix --sort-by quality_score
  ```
- **Expected Output:** Results sorted correctly
- **Evidence:** Command output screenshot

- [ ] Filtering by file_type supported
- **Verification:**
  - CLI option: `--filter-type <type>`
  - Only matching file types shown
  - Default: all types
- **Test Commands:**
  ```bash
  # Test filtering
  cargo run --bin agentsdk -- quality matrix --filter-type rust
  ```
- **Expected Output:** Results filtered correctly
- **Evidence:** Command output screenshot

---

## Testing Criteria Validation

### Unit Tests

- [ ] Matrix creation and registration
- **Verification:**
  - New matrix created successfully
  - Register adds capabilities
  - Index updated correctly
- **Test Commands:**
  ```bash
  # Run tests
  cargo test --lib quality::tests::matrix_creation
  ```
- **Expected Output:** All tests pass
- **Evidence:** Test output log

- [ ] Duplicate registration handling
- **Verification:**
  - Duplicate registration rejected
  - Error returned correctly
  - Original capability unchanged
- **Test Commands:**
  ```bash
  # Run tests
  cargo test --lib quality::tests::duplicate_handling
  ```
- **Expected Output:** All tests pass
- **Evidence:** Test output log

- [ ] get() with existing capability
- **Verification:**
  - Existing capability returned
  - Correct capability returned
  - None returned for non-existent
- **Test Commands:**
  ```bash
  # Run tests
  cargo test --lib quality::tests::get_operations
  ```
- **Expected Output:** All tests pass
- **Evidence:** Test output log

- [ ] get() with missing capability
- **Verification:**
  - None returned for missing
  - No panic or error
  - Graceful handling
- **Test Commands:**
  ```bash
  # Run tests
  cargo test --lib quality::tests::get_missing
  ```
- **Expected Output:** All tests pass
- **Evidence:** Test output log

- [ ] get_for_file_type() with multiple capabilities
- **Verification:**
  - All matching capabilities returned
  - Results sorted correctly
  - Count accurate
- **Test Commands:**
  ```bash
  # Run tests
  cargo test --lib quality::tests::get_multiple
  ```
- **Expected Output:** All tests pass
- **Evidence:** Test output log

- [ ] get_for_file_type() with no capabilities
- **Verification:**
  - Empty vector returned
  - No panic or error
  - Graceful handling
- **Test Commands:**
  ```bash
  # Run tests
  cargo test --lib quality::tests::get_empty
  ```
- **Expected Output:** All tests pass
- **Evidence:** Test output log

- [ ] find_best_workflow() with single match
- **Verification:**
  - Best workflow returned
  - Correct workflow returned
  - None for no matches
- **Test Commands:**
  ```bash
  # Run tests
  cargo test --lib quality::tests::find_best_single
  ```
- **Expected Output:** All tests pass
- **Evidence:** Test output log

- [ ] find_best_workflow() with multiple matches
- **Verification:**
  - Best workflow returned
  - Highest quality_score selected
  - Ties broken deterministically
- **Test Commands:**
  ```bash
  # Run tests
  cargo test --lib quality::tests::find_best_multiple
  ```
- **Expected Output:** All tests pass
- **Evidence:** Test output log

- [ ] find_best_workflow() with no matches
- **Verification:**
  - None returned
  - No panic or error
  - Graceful handling
- **Test Commands:**
  ```bash
  # Run tests
  cargo test --lib quality::tests::find_best_none
  ```
- **Expected Output:** All tests pass
- **Evidence:** Test output log

- [ ] Gap analysis with missing capabilities
- **Verification:**
  - All gaps identified
  - No false positives
  - No false negatives
- **Test Commands:**
  ```bash
  # Run tests
  cargo test --lib quality::tests::gap_analysis_missing
  ```
- **Expected Output:** All tests pass
- **Evidence:** Test output log

- [ ] Gap analysis with quality gaps
- **Verification:**
  - All quality gaps identified
  - Gap sizes calculated correctly
  - Results prioritized
- **Test Commands:**
  ```bash
  # Run tests
  cargo test --lib quality::tests::gap_analysis_quality
  ```
- **Expected Output:** All tests pass
- **Evidence:** Test output log

### Integration Tests

- [ ] Register many capabilities efficiently
- **Verification:**
  - 1000+ capabilities registered
  - Registration time acceptable (< 1s)
  - Index updated correctly
- **Test Commands:**
  ```bash
  # Run tests
  cargo test --test quality_integration::tests::bulk_registration
  ```
- **Expected Output:** All tests pass
- **Evidence:** Test output log

- [ ] Query across all file types
- **Verification:**
  - Query returns correct results
  - Performance acceptable
  - No missing results
- **Test Commands:**
  ```bash
  # Run tests
  cargo test --test quality_integration::tests::query_all_types
  ```
- **Expected Output:** All tests pass
- **Evidence:** Test output log

- [ ] Concurrent registration and querying
- **Verification:**
  - Multiple threads register
  - Multiple threads query
  - No data races
  - Results correct
- **Test Commands:**
  ```bash
  # Run tests
  cargo test --test quality_integration::tests::concurrent_ops
  ```
- **Expected Output:** All tests pass
- **Evidence:** Test output log

- [ ] Full gap analysis workflow
- **Verification:**
  - Gaps identified correctly
  - Opportunities identified correctly
  - Recommendations generated correctly
- **Test Commands:**
  ```bash
  # Run tests
  cargo test --test quality_integration::tests::full_gap_analysis
  ```
- **Expected Output:** All tests pass
- **Evidence:** Test output log

- [ ] Opportunity ranking
- **Verification:**
  - Opportunities prioritized correctly
  - Impact scores accurate
  - Ranking stable
- **Test Commands:**
  ```bash
  # Run tests
  cargo test --test quality_integration::tests::opportunity_ranking
  ```
- **Expected Output:** All tests pass
- **Evidence:** Test output log

### Edge Cases

- [ ] Empty matrix
- **Verification:**
  - Empty matrix handled correctly
  - No panics or errors
  - Queries return empty results
- **Test Commands:**
  ```bash
  # Run tests
  cargo test --lib quality::tests::empty_matrix
  ```
- **Expected Output:** All tests pass
- **Evidence:** Test output log

- [ ] Single capability
- **Verification:**
  - Single capability handled correctly
  - All operations work
  - Queries return correct results
- **Test Commands:**
  ```bash
  # Run tests
  cargo test --lib quality::tests::single_capability
  ```
- **Expected Output:** All tests pass
- **Evidence:** Test output log

- [ ] Very large matrix (1000+ capabilities)
- **Verification:**
  - Large matrix handled correctly
  - Performance acceptable
  - No memory issues
- **Test Commands:**
  ```bash
  # Run tests
  cargo test --lib quality::tests::large_matrix
  ```
- **Expected Output:** All tests pass
- **Evidence:** Test output log

- [ ] All workflows support same file type
- **Verification:**
  - Single file type handled correctly
  - All capabilities listed
  - Queries work correctly
- **Test Commands:**
  ```bash
  # Run tests
  cargo test --lib quality::tests::single_file_type
  ```
- **Expected Output:** All tests pass
- **Evidence:** Test output log

- [ ] Each workflow supports unique file type
- **Verification:**
  - Many file types handled correctly
  - All capabilities listed
  - Queries work correctly
- **Test Commands:**
  ```bash
  # Run tests
  cargo test --lib quality::tests::many_file_types
  ```
- **Expected Output:** All tests pass
- **Evidence:** Test output log

- [ ] Confidence ties (deterministic selection)
- **Verification:**
  - Ties broken deterministically
  - Selection stable across runs
  - No randomness
- **Test Commands:**
  ```bash
  # Run tests
  cargo test --lib quality::tests::confidence_ties
  ```
- **Expected Output:** All tests pass
- **Evidence:** Test output log

---

## Performance Criteria Validation

### Performance Requirements

- [ ] register() < 10ms
- **Verification:**
  - Single registration completes < 10ms
  - Includes index update
  - Measured on typical workload
- **Test Commands:**
  ```bash
  # Run benchmarks
  cargo bench --bench matrix_register
  ```
- **Expected Output:** Registration < 10ms
- **Evidence:** Benchmark output

- [ ] get() < 1ms
- **Verification:**
  - Single get completes < 1ms
  - Includes lock acquisition
  - Measured on typical workload
- **Test Commands:**
  ```bash
  # Run benchmarks
  cargo bench --bench matrix_get
  ```
- **Expected Output:** Get < 1ms
- **Evidence:** Benchmark output

- [ ] get_for_file_type() < 5ms
- **Verification:**
  - Query by file type completes < 5ms
  - Includes lock acquisition
  - Includes sorting
- **Test Commands:**
  ```bash
  # Run benchmarks
  cargo bench --bench matrix_get_type
  ```
- **Expected Output:** Query < 5ms
- **Evidence:** Benchmark output

- [ ] find_best_workflow() < 10ms
- **Verification:**
  - Best workflow query completes < 10ms
  - Includes filtering
  - Includes sorting
- **Test Commands:**
  ```bash
  # Run benchmarks
  cargo bench --bench matrix_find_best
  ```
- **Expected Output:** Query < 10ms
- **Evidence:** Benchmark output

- [ ] Full gap analysis < 100ms
- **Verification:**
  - Complete gap analysis completes < 100ms
  - Includes all gap types
  - Includes opportunity identification
- **Test Commands:**
  ```bash
  # Run benchmarks
  cargo bench --bench gap_analysis
  ```
- **Expected Output:** Analysis < 100ms
- **Evidence:** Benchmark output

- [ ] Memory usage < 10MB per 1000 capabilities
- **Verification:**
  - Memory overhead acceptable
  - Measured with memory profiling
  - Includes index overhead
- **Test Commands:**
  ```bash
  # Run memory tests
  cargo test --lib quality::tests::memory_usage
  ```
- **Expected Output:** Memory < 10MB per 1000
- **Evidence:** Memory profile output

---

## Error Handling Validation

### Error Variants

- [ ] CapabilityError covers all failure modes
- **Verification:**
  - DuplicateKey variant present
  - NotFound variant present
  - InvalidParameter variant present
  - LockError variant present
- **Test Commands:**
  ```bash
  # Verify error variants
  grep -A 10 "enum CapabilityError" src/quality/error.rs
  ```
- **Expected Output:** All variants present
- **Evidence:** Error definition output

- [ ] Duplicate registration returns specific error
- **Verification:**
  - DuplicateKey error returned
  - Includes workflow_id and file_type
  - Message is clear
- **Test Commands:**
  ```bash
  # Test duplicate error
  cargo test --lib quality::tests::duplicate_error
  ```
- **Expected Output:** Specific error returned
- **Evidence:** Test output log

- [ ] Missing capability returns specific error
- **Verification:**
  - NotFound error returned
  - Includes searched key
  - Message is clear
- **Test Commands:**
  ```bash
  # Test not found error
  cargo test --lib quality::tests::not_found_error
  ```
- **Expected Output:** Specific error returned
- **Evidence:** Test output log

- [ ] Invalid query parameters handled
- **Verification:**
  - InvalidParameter error returned
  - Invalid values rejected
  - Message explains valid values
- **Test Commands:**
  ```bash
  # Test invalid parameter handling
  cargo test --lib quality::tests::invalid_parameter_error
  ```
- **Expected Output:** Invalid parameters rejected
- **Evidence:** Test output log

- [ ] Error messages actionable
- **Verification:**
  - Each error explains the problem
  - Each error suggests a fix
  - Messages are user-friendly
- **Test Commands:**
  ```bash
  # Verify error messages
  grep -A 2 "DuplicateKey" src/quality/error.rs
  grep -A 2 "NotFound" src/quality/error.rs
  ```
- **Expected Output:** Actionable messages
- **Evidence:** Error definition output

---

**Document Version:** 1.0
**Last Updated:** 2026-04-07
**Plan Location:** `/home/jon/code/whitt-execution-engine/opencode/docs/plans/04-quality-loops/validation/capability-matrix.md`
