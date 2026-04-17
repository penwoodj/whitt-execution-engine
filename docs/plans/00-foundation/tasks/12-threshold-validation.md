# Task 12: Threshold Validation

**Goal:** Implement numeric threshold validation for 11 thresholds with range checks and default enforcement.

**Estimated Time:** 4 hours

**Dependencies:** Task 2

**Files:**
- Modify: `src/validation/thresholds.rs` (implement threshold validation)
- Create: `tests/threshold_test.rs` (threshold tests)

---

## Step 1: Implement threshold validation

Create `src/validation/thresholds.rs`:

```rust
use crate::error::{Error, Result};
use crate::schema::{ResourceLimit, ResourceLimits};

/// Threshold specification
#[derive(Debug, Clone)]
pub struct Threshold {
    pub name: String,
    pub min: f64,
    pub max: f64,
    pub default: f64,
    pub unit: String,
}

/// Get all threshold definitions
pub fn get_all_thresholds() -> Vec<Threshold> {
    vec![
        Threshold {
            name: "max_allowed.ram".to_string(),
            min: 0.0,
            max: 100.0,
            default: 13.0,
            unit: "%".to_string(),
        },
        Threshold {
            name: "max_allowed.vram".to_string(),
            min: 0.0,
            max: 100.0,
            default: 3.7,
            unit: "GB".to_string(),
        },
        Threshold {
            name: "max_allowed.cpu".to_string(),
            min: 0.0,
            max: 100.0,
            default: 49.0,
            unit: "%".to_string(),
        },
        Threshold {
            name: "max_allowed.gpu".to_string(),
            min: 0.0,
            max: 100.0,
            default: 74.0,
            unit: "%".to_string(),
        },
        Threshold {
            name: "max_allowed.attention_tokens".to_string(),
            min: 1.0,
            max: 1_000_000.0,
            default: 150_000.0,
            unit: "tokens".to_string(),
        },
        Threshold {
            name: "min_allowed.ram".to_string(),
            min: 0.0,
            max: 100.0,
            default: 9.0,
            unit: "%".to_string(),
        },
        Threshold {
            name: "min_allowed.vram".to_string(),
            min: 0.0,
            max: 100.0,
            default: 2.4,
            unit: "GB".to_string(),
        },
        Threshold {
            name: "min_allowed.cpu".to_string(),
            min: 0.0,
            max: 100.0,
            default: 49.0,
            unit: "%".to_string(),
        },
        Threshold {
            name: "min_allowed.gpu".to_string(),
            min: 0.0,
            max: 100.0,
            default: 74.0,
            unit: "%".to_string(),
        },
        Threshold {
            name: "min_allowed.attention_tokens".to_string(),
            min: 1.0,
            max: 1_000_000.0,
            default: 73_500.0,
            unit: "tokens".to_string(),
        },
        Threshold {
            name: "max_concurrent_requests".to_string(),
            min: 1.0,
            max: 10.0,
            default: 2.0,
            unit: "requests".to_string(),
        },
    ]
}

/// Validate resource limits against thresholds
pub fn validate_resource_limits(limits: &ResourceLimits, is_max: bool) -> Result<()> {
    validate_limit(&limits.ram, "ram", is_max)?;
    validate_limit(&limits.vram, "vram", is_max)?;
    validate_percentage(&limits.cpu, "cpu")?;
    validate_percentage(&limits.gpu, "gpu")?;
    validate_tokens(&limits.attention_tokens, "attention_tokens")?;

    Ok(())
}

fn validate_limit(limit: &ResourceLimit, name: &str, is_max: bool) -> Result<()> {
    let threshold_name = if is_max {
        format!("max_allowed.{}", name)
    } else {
        format!("min_allowed.{}", name)
    };

    match limit {
        ResourceLimit::Percentage(p) => {
            if *p < 0.0 || *p > 100.0 {
                return Err(Error::threshold(
                    threshold_name,
                    "percentage must be between 0 and 100",
                    Some(*p),
                    Some("0-100".to_string()),
                ));
            }
        }
        ResourceLimit::Absolute { value, unit } => {
            if *value == 0 {
                return Err(Error::threshold(
                    threshold_name,
                    "absolute value must be positive",
                    Some(*value as f64),
                    Some(format!(">0 {}", unit)),
                ));
            }
        }
    }

    Ok(())
}

fn validate_percentage(value: f64, name: &str) -> Result<()> {
    if value < 0.0 || value > 100.0 {
        return Err(Error::threshold(
            name,
            "percentage must be between 0 and 100",
            Some(value),
            Some("0-100".to_string()),
        ));
    }
    Ok(())
}

fn validate_tokens(value: u64, name: &str) -> Result<()> {
    if value < 1 || value > 1_000_000 {
        return Err(Error::threshold(
            name,
            "tokens must be between 1 and 1,000,000",
            Some(value as f64),
            Some("1-1,000,000".to_string()),
        ));
    }
    Ok(())
}

/// Validate concurrent requests
pub fn validate_concurrent_requests(value: u32) -> Result<()> {
    if value < 1 || value > 10 {
        return Err(Error::threshold(
            "max_concurrent_requests",
            "must be between 1 and 10",
            Some(value as f64),
            Some("1-10".to_string()),
        ));
    }
    Ok(())
}
```

**Commit:** `feat: implement numeric threshold validation`

---

## Step 2: Write threshold tests

Create `tests/threshold_test.rs`:

```rust
use whitt_execution_engine::validation::thresholds::*;
use whitt_execution_engine::schema::{ResourceLimit, ResourceLimits};

#[test]
fn test_get_all_thresholds() {
    let thresholds = get_all_thresholds();
    assert_eq!(thresholds.len(), 11);
}

#[test]
fn test_validate_valid_percentage() {
    let limits = ResourceLimits {
        ram: ResourceLimit::Percentage(50.0),
        vram: ResourceLimit::Percentage(30.0),
        cpu: 50.0,
        gpu: 80.0,
        attention_tokens: 100_000,
        concurrent_requests: 2,
    };

    let result = validate_resource_limits(&limits, true);
    assert!(result.is_ok());
}

#[test]
fn test_validate_invalid_percentage() {
    let limits = ResourceLimits {
        ram: ResourceLimit::Percentage(150.0),
        vram: ResourceLimit::Percentage(30.0),
        cpu: 50.0,
        gpu: 80.0,
        attention_tokens: 100_000,
        concurrent_requests: 2,
    };

    let result = validate_resource_limits(&limits, true);
    assert!(result.is_err());
}

#[test]
fn test_validate_tokens() {
    let limits = ResourceLimits {
        ram: ResourceLimit::Percentage(50.0),
        vram: ResourceLimit::Percentage(30.0),
        cpu: 50.0,
        gpu: 80.0,
        attention_tokens: 50_000,
        concurrent_requests: 2,
    };

    let result = validate_resource_limits(&limits, true);
    assert!(result.is_ok());

    let limits = ResourceLimits {
        ram: ResourceLimit::Percentage(50.0),
        vram: ResourceLimit::Percentage(30.0),
        cpu: 50.0,
        gpu: 80.0,
        attention_tokens: 2_000_000,
        concurrent_requests: 2,
    };

    let result = validate_resource_limits(&limits, true);
    assert!(result.is_err());
}

#[test]
fn test_validate_concurrent_requests() {
    assert!(validate_concurrent_requests(2).is_ok());
    assert!(validate_concurrent_requests(10).is_ok());
    assert!(validate_concurrent_requests(0).is_err());
    assert!(validate_concurrent_requests(11).is_err());
}
```

**Commit:** `test: add threshold validation tests`

---

## Step 3: Run tests

Verify all tests pass:

```bash
cargo test threshold_test

# Expected output:
# test result: ok. X passed in Y.ZZs
```

**Commit:** `fix: resolve any test failures`

---

## Verification

After completing all steps, verify:

```bash
# 1. Build passes
cargo build
# Expected: Finished dev [unoptimized + debuginfo] target(s)

# 2. All threshold tests pass
cargo test threshold_test
# Expected: test result: ok. X passed

# 3. All 11 thresholds validated
```

**Checkpoint Criteria:**
- ✅ All 11 numeric thresholds defined
- ✅ Range checks enforce min/max values
- ✅ Default values applied when not specified
- ✅ Error messages include actual vs expected values
- ✅ Threshold tests created and passing
- ✅ Validation happens at parse time

**Final Checkpoint:** All Phase 0 tasks complete!

**Anti-Drift Check:** Verify task 12 implements ONLY threshold validation. All other validation (DAG, policy) already tested.

**Phase 0 Complete!** Proceed to validation and acceptance criteria.
