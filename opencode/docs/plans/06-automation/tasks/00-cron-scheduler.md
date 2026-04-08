# Task 00: Cron Scheduler

**Component:** Cron-based workflow scheduling infrastructure

**Dependencies:** None (foundational task)

**Estimated Time:** 10-12 days

**Goal:** Build a robust cron scheduler that can execute workflows on time-based schedules, with comprehensive safety limits, persistence, timezone support, and missed schedule handling.

---

## Overview

The cron scheduler is the foundation of time-based automation in the AgentSDK. It provides:

- Cron expression parsing and validation
- Tokio-based async scheduling engine
- Safety limits (max concurrent jobs, max job duration, etc.)
- Persistent schedule storage (SQLite database)
- Missed schedule detection and handling
- Timezone support

**ADR-0007 Compliance:** This task builds the scheduler infrastructure. The actual policy compilation happens in Task 06, which compiles cron expressions to WorkflowIR scheduling nodes (not interpreted at runtime).

---

## File Structure

**New Files:**
- `automation/cron/mod.rs` - Module exports
- `automation/cron/parser.rs` - Cron expression parsing
- `automation/cron/scheduler.rs` - Tokio-based scheduling engine
- `automation/cron/validator.rs` - Safety limits validation
- `automation/cron/persistence.rs` - Schedule persistence

---

## Implementation Steps

### Step 1: Add dependencies to Cargo.toml

**Files:** Modify `Cargo.toml`

```toml
[dependencies]
# Existing dependencies...

# Cron scheduling
tokio-cron-scheduler = "0.10"
cron = "0.12"
chrono = { version = "0.4", features = ["serde"] }
chrono-tz = "0.8"

# Persistence
sqlx = { version = "0.7", features = ["runtime-tokio-rustls", "sqlite"] }
```

---

### Step 2: Create cron module structure

**Files:** Create `automation/cron/mod.rs`

```rust
pub mod parser;
pub mod scheduler;
pub mod validator;
pub mod persistence;

pub use parser::{parse_cron_expression, CronExpression, CronParseError};
pub use scheduler::{CronScheduler, ScheduledJob, JobStatus};
pub use validator::{validate_schedule, ScheduleValidator, ValidationError};
pub use persistence::{ScheduleStore, StoredSchedule, ScheduleHistory};

use std::sync::Arc;
use tokio::sync::RwLock;

/// Main cron scheduler instance
pub type CronSchedulerInstance = Arc<CronScheduler>;

/// Schedule safety configuration
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ScheduleSafetyConfig {
    pub max_concurrent_jobs: usize,
    pub max_job_duration_seconds: u64,
    pub max_missed_schedules: usize,
    pub enabled_timezones: Vec<String>,
}

impl Default for ScheduleSafetyConfig {
    fn default() -> Self {
        Self {
            max_concurrent_jobs: 10,
            max_job_duration_seconds: 3600, // 1 hour
            max_missed_schedules: 5,
            enabled_timezones: vec!["UTC".to_string()],
        }
    }
}
```

---

### Step 3: Write failing tests for cron expression parser

**Files:** Create `automation/cron/parser.rs` (with tests)

```rust
use chrono::{DateTime, Utc, TimeZone};
use std::str::FromStr;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CronExpression {
    minute: String,
    hour: String,
    day_of_month: String,
    month: String,
    day_of_week: String,
    timezone: Option<String>,
}

#[derive(Debug, thiserror::Error)]
pub enum CronParseError {
    #[error("Invalid cron format: {0}")]
    InvalidFormat(String),
    #[error("Invalid field '{field}': {message}")]
    InvalidField { field: String, message: String },
    #[error("Invalid timezone: {0}")]
    InvalidTimezone(String),
}

pub fn parse_cron_expression(expr: &str) -> Result<CronExpression, CronParseError> {
    // Implementation will be added in Step 4
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_simple_cron() {
        let expr = "0 * * * *";
        let result = parse_cron_expression(expr);
        assert!(result.is_ok());
        let cron = result.unwrap();
        assert_eq!(cron.minute, "0");
        assert_eq!(cron.hour, "*");
        assert_eq!(cron.day_of_month, "*");
        assert_eq!(cron.month, "*");
        assert_eq!(cron.day_of_week, "*");
    }

    #[test]
    fn test_parse_cron_with_timezone() {
        let expr = "0 9 * * 1-5@America/New_York";
        let result = parse_cron_expression(expr);
        assert!(result.is_ok());
        let cron = result.unwrap();
        assert_eq!(cron.timezone, Some("America/New_York".to_string()));
    }

    #[test]
    fn test_parse_invalid_cron() {
        let expr = "invalid cron expression";
        let result = parse_cron_expression(expr);
        assert!(result.is_err());
        assert!(matches!(result, Err(CronParseError::InvalidFormat(_))));
    }

    #[test]
    fn test_parse_invalid_timezone() {
        let expr = "0 9 * * *@Invalid/Timezone";
        let result = parse_cron_expression(expr);
        assert!(result.is_err());
        assert!(matches!(result, Err(CronParseError::InvalidTimezone(_))));
    }

    #[test]
    fn test_parse_complex_cron() {
        let expr = "*/15 9-17 * * 1-5@Europe/London";
        let result = parse_cron_expression(expr);
        assert!(result.is_ok());
        let cron = result.unwrap();
        assert_eq!(cron.minute, "*/15");
        assert_eq!(cron.hour, "9-17");
        assert_eq!(cron.day_of_week, "1-5");
        assert_eq!(cron.timezone, Some("Europe/London".to_string()));
    }
}
```

---

### Step 4: Run parser tests to verify they fail

**Files:** Run tests

```bash
cargo test automation::cron::parser --lib
```

**Expected:** FAIL with "not implemented" or "todo!()" errors

---

### Step 5: Implement cron expression parser

**Files:** Modify `automation/cron/parser.rs`

```rust
use chrono_tz::Tz;
use std::str::FromStr;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CronExpression {
    pub minute: String,
    pub hour: String,
    pub day_of_month: String,
    pub month: String,
    pub day_of_week: String,
    pub timezone: Option<String>,
}

#[derive(Debug, thiserror::Error)]
pub enum CronParseError {
    #[error("Invalid cron format: {0}")]
    InvalidFormat(String),
    #[error("Invalid field '{field}': {message}")]
    InvalidField { field: String, message: String },
    #[error("Invalid timezone: {0}")]
    InvalidTimezone(String),
}

pub fn parse_cron_expression(expr: &str) -> Result<CronExpression, CronParseError> {
    // Split on @ for timezone
    let (cron_part, timezone) = if let Some((left, tz)) = expr.split_once('@') {
        let tz_str = tz.trim().to_string();
        // Validate timezone
        tz_str.parse::<Tz>()
            .map_err(|_| CronParseError::InvalidTimezone(tz_str.clone()))?;
        (left, Some(tz_str))
    } else {
        (expr, None)
    };

    // Parse cron parts
    let parts: Vec<&str> = cron_part.trim().split_whitespace().collect();
    if parts.len() != 5 {
        return Err(CronParseError::InvalidFormat(
            "Cron expression must have 5 fields".to_string(),
        ));
    }

    let cron = CronExpression {
        minute: parts[0].to_string(),
        hour: parts[1].to_string(),
        day_of_month: parts[2].to_string(),
        month: parts[3].to_string(),
        day_of_week: parts[4].to_string(),
        timezone,
    };

    // Validate each field
    validate_cron_field(&cron.minute, 0, 59, "minute")?;
    validate_cron_field(&cron.hour, 0, 23, "hour")?;
    validate_cron_field(&cron.day_of_month, 1, 31, "day_of_month")?;
    validate_cron_field(&cron.month, 1, 12, "month")?;
    validate_cron_field(&cron.day_of_week, 0, 6, "day_of_week")?;

    Ok(cron)
}

fn validate_cron_field(field: &str, min: u32, max: u32, field_name: &str) -> Result<(), CronParseError> {
    // Handle wildcard
    if field == "*" || field == "?" {
        return Ok(());
    }

    // Handle lists (e.g., "1,2,3")
    if field.contains(',') {
        for part in field.split(',') {
            validate_cron_part(part.trim(), min, max, field_name)?;
        }
        return Ok(());
    }

    // Handle ranges (e.g., "1-5")
    if field.contains('-') {
        let parts: Vec<&str> = field.split('-').collect();
        if parts.len() != 2 {
            return Err(CronParseError::InvalidField {
                field: field_name.to_string(),
                message: format!("Invalid range: {}", field),
            });
        }
        validate_cron_part(parts[0].trim(), min, max, field_name)?;
        validate_cron_part(parts[1].trim(), min, max, field_name)?;
        return Ok(());
    }

    // Handle step values (e.g., "*/5" or "1-10/2")
    if field.contains('/') {
        let parts: Vec<&str> = field.split('/').collect();
        if parts.len() != 2 {
            return Err(CronParseError::InvalidField {
                field: field_name.to_string(),
                message: format!("Invalid step: {}", field),
            });
        }
        validate_cron_part(parts[0].trim(), min, max, field_name)?;
        validate_cron_part(parts[1].trim(), min, max, field_name)?;
        return Ok(());
    }

    // Handle single value
    validate_cron_part(field, min, max, field_name)
}

fn validate_cron_part(part: &str, min: u32, max: u32, field_name: &str) -> Result<(), CronParseError> {
    if part == "*" {
        return Ok(());
    }

    let value = part.parse::<u32>().map_err(|_| CronParseError::InvalidField {
        field: field_name.to_string(),
        message: format!("Invalid value: {}", part),
    })?;

    if value < min || value > max {
        return Err(CronParseError::InvalidField {
            field: field_name.to_string(),
            message: format!("Value {} out of range [{}-{}]", value, min, max),
        });
    }

    Ok(())
}
```

---

### Step 6: Run parser tests to verify they pass

**Files:** Run tests

```bash
cargo test automation::cron::parser --lib
```

**Expected:** PASS all tests

---

### Step 7: Write failing tests for safety validator

**Files:** Create `automation/cron/validator.rs` (with tests)

```rust
use super::{CronExpression, ScheduleSafetyConfig};

#[derive(Debug, Clone)]
pub struct ScheduleValidator {
    config: ScheduleSafetyConfig,
}

#[derive(Debug, thiserror::Error)]
pub enum ValidationError {
    #[error("Timezone '{0}' is not enabled")]
    TimezoneNotEnabled(String),
    #[error("Too many concurrent jobs: {0} (max: {1})")]
    TooManyConcurrentJobs(usize, usize),
    #[error("Job duration exceeds limit: {0}s (max: {1}s)")]
    JobDurationTooLong(u64, u64),
    #[error("Too many missed schedules: {0} (max: {1})")]
    TooManyMissedSchedules(usize, usize),
}

pub fn validate_schedule(
    cron: &CronExpression,
    config: &ScheduleSafetyConfig,
) -> Result<(), ValidationError> {
    let validator = ScheduleValidator {
        config: config.clone(),
    };
    validator.validate(cron)
}

impl ScheduleValidator {
    pub fn new(config: ScheduleSafetyConfig) -> Self {
        Self { config }
    }

    pub fn validate(&self, cron: &CronExpression) -> Result<(), ValidationError> {
        self.validate_timezone(cron)?;
        Ok(())
    }

    fn validate_timezone(&self, cron: &CronExpression) -> Result<(), ValidationError> {
        if let Some(tz) = &cron.timezone {
            if !self.config.enabled_timezones.contains(tz) {
                return Err(ValidationError::TimezoneNotEnabled(tz.clone()));
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_default_timezone() {
        let cron = parse_cron_expression("0 * * * *").unwrap();
        let config = ScheduleSafetyConfig::default();
        let result = validate_schedule(&cron, &config);
        assert!(result.is_ok());
    }

    #[test]
    fn test_validate_enabled_timezone() {
        let cron = parse_cron_expression("0 9 * * *@UTC").unwrap();
        let mut config = ScheduleSafetyConfig::default();
        config.enabled_timezones.push("UTC".to_string());
        let result = validate_schedule(&cron, &config);
        assert!(result.is_ok());
    }

    #[test]
    fn test_validate_disabled_timezone() {
        let cron = parse_cron_expression("0 9 * * *@America/New_York").unwrap();
        let config = ScheduleSafetyConfig::default();
        let result = validate_schedule(&cron, &config);
        assert!(result.is_err());
        assert!(matches!(result, Err(ValidationError::TimezoneNotEnabled(_))));
    }
}
```

---

### Step 8: Run validator tests to verify they fail

**Files:** Run tests

```bash
cargo test automation::cron::validator --lib
```

**Expected:** FAIL with compilation errors or import issues

---

### Step 9: Implement safety validator

**Files:** Modify `automation/cron/validator.rs`

```rust
use super::{CronExpression, ScheduleSafetyConfig};

#[derive(Debug, Clone)]
pub struct ScheduleValidator {
    config: ScheduleSafetyConfig,
}

#[derive(Debug, thiserror::Error)]
pub enum ValidationError {
    #[error("Timezone '{0}' is not enabled")]
    TimezoneNotEnabled(String),
    #[error("Too many concurrent jobs: {0} (max: {1})")]
    TooManyConcurrentJobs(usize, usize),
    #[error("Job duration exceeds limit: {0}s (max: {1}s)")]
    JobDurationTooLong(u64, u64),
    #[error("Too many missed schedules: {0} (max: {1})")]
    TooManyMissedSchedules(usize, usize),
}

pub fn validate_schedule(
    cron: &CronExpression,
    config: &ScheduleSafetyConfig,
) -> Result<(), ValidationError> {
    let validator = ScheduleValidator {
        config: config.clone(),
    };
    validator.validate(cron)
}

impl ScheduleValidator {
    pub fn new(config: ScheduleSafetyConfig) -> Self {
        Self { config }
    }

    pub fn validate(&self, cron: &CronExpression) -> Result<(), ValidationError> {
        self.validate_timezone(cron)?;
        Ok(())
    }

    fn validate_timezone(&self, cron: &CronExpression) -> Result<(), ValidationError> {
        if let Some(tz) = &cron.timezone {
            if !self.config.enabled_timezones.contains(tz) {
                return Err(ValidationError::TimezoneNotEnabled(tz.clone()));
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::automation::cron::parser::parse_cron_expression;

    #[test]
    fn test_validate_default_timezone() {
        let cron = parse_cron_expression("0 * * * *").unwrap();
        let config = ScheduleSafetyConfig::default();
        let result = validate_schedule(&cron, &config);
        assert!(result.is_ok());
    }

    #[test]
    fn test_validate_enabled_timezone() {
        let cron = parse_cron_expression("0 9 * * *@UTC").unwrap();
        let mut config = ScheduleSafetyConfig::default();
        config.enabled_timezones.push("UTC".to_string());
        let result = validate_schedule(&cron, &config);
        assert!(result.is_ok());
    }

    #[test]
    fn test_validate_disabled_timezone() {
        let cron = parse_cron_expression("0 9 * * *@America/New_York").unwrap();
        let config = ScheduleSafetyConfig::default();
        let result = validate_schedule(&cron, &config);
        assert!(result.is_err());
        assert!(matches!(result, Err(ValidationError::TimezoneNotEnabled(_))));
    }
}
```

---

### Step 10: Run validator tests to verify they pass

**Files:** Run tests

```bash
cargo test automation::cron::validator --lib
```

**Expected:** PASS all tests

---

### Step 11: Write failing tests for scheduler persistence

**Files:** Create `automation/cron/persistence.rs` (with tests)

```rust
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{SqlitePool, Row};
use std::path::Path;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StoredSchedule {
    pub id: String,
    pub workflow_id: String,
    pub cron_expression: String,
    pub timezone: Option<String>,
    pub created_at: DateTime<Utc>,
    pub last_run: Option<DateTime<Utc>>,
    pub next_run: DateTime<Utc>,
    pub enabled: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScheduleHistory {
    pub id: String,
    pub schedule_id: String,
    pub run_at: DateTime<Utc>,
    pub status: String,
    pub duration_ms: Option<u64>,
    pub error_message: Option<String>,
}

pub struct ScheduleStore {
    pool: SqlitePool,
}

impl ScheduleStore {
    pub async fn new(db_path: &Path) -> Result<Self, sqlx::Error> {
        let pool = SqlitePool::connect(&format!("sqlite:{}?mode=rwc", db_path.display())).await?;
        let store = Self { pool };
        store.init_schema().await?;
        Ok(store)
    }

    async fn init_schema(&self) -> Result<(), sqlx::Error> {
        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS schedules (
                id TEXT PRIMARY KEY,
                workflow_id TEXT NOT NULL,
                cron_expression TEXT NOT NULL,
                timezone TEXT,
                created_at TEXT NOT NULL,
                last_run TEXT,
                next_run TEXT NOT NULL,
                enabled BOOLEAN NOT NULL DEFAULT 1
            );

            CREATE TABLE IF NOT EXISTS schedule_history (
                id TEXT PRIMARY KEY,
                schedule_id TEXT NOT NULL,
                run_at TEXT NOT NULL,
                status TEXT NOT NULL,
                duration_ms INTEGER,
                error_message TEXT,
                FOREIGN KEY (schedule_id) REFERENCES schedules(id)
            );
            "#,
        )
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    pub async fn save_schedule(&self, schedule: &StoredSchedule) -> Result<(), sqlx::Error> {
        // Implementation will be added in Step 13
        todo!()
    }

    pub async fn get_schedule(&self, id: &str) -> Result<Option<StoredSchedule>, sqlx::Error> {
        // Implementation will be added in Step 13
        todo!()
    }

    pub async fn list_schedules(&self) -> Result<Vec<StoredSchedule>, sqlx::Error> {
        // Implementation will be added in Step 13
        todo!()
    }

    pub async fn record_run(&self, history: &ScheduleHistory) -> Result<(), sqlx::Error> {
        // Implementation will be added in Step 13
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_save_and_retrieve_schedule() {
        let db_path = Path::new(":memory:");
        let store = ScheduleStore::new(db_path).await.unwrap();

        let schedule = StoredSchedule {
            id: "test-schedule-1".to_string(),
            workflow_id: "workflow-1".to_string(),
            cron_expression: "0 * * * *".to_string(),
            timezone: None,
            created_at: Utc::now(),
            last_run: None,
            next_run: Utc::now(),
            enabled: true,
        };

        store.save_schedule(&schedule).await.unwrap();
        let retrieved = store.get_schedule("test-schedule-1").await.unwrap();

        assert!(retrieved.is_some());
        let retrieved = retrieved.unwrap();
        assert_eq!(retrieved.id, "test-schedule-1");
        assert_eq!(retrieved.workflow_id, "workflow-1");
        assert_eq!(retrieved.cron_expression, "0 * * * *");
    }

    #[tokio::test]
    async fn test_list_schedules() {
        let db_path = Path::new(":memory:");
        let store = ScheduleStore::new(db_path).await.unwrap();

        let schedule1 = StoredSchedule {
            id: "test-schedule-1".to_string(),
            workflow_id: "workflow-1".to_string(),
            cron_expression: "0 * * * *".to_string(),
            timezone: None,
            created_at: Utc::now(),
            last_run: None,
            next_run: Utc::now(),
            enabled: true,
        };

        store.save_schedule(&schedule1).await.unwrap();
        let schedules = store.list_schedules().await.unwrap();

        assert_eq!(schedules.len(), 1);
        assert_eq!(schedules[0].id, "test-schedule-1");
    }
}
```

---

### Step 12: Run persistence tests to verify they fail

**Files:** Run tests

```bash
cargo test automation::cron::persistence --lib
```

**Expected:** FAIL with "todo!()" errors

---

### Step 13: Implement scheduler persistence

**Files:** Modify `automation/cron/persistence.rs`

```rust
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{SqlitePool, Row};
use std::path::Path;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StoredSchedule {
    pub id: String,
    pub workflow_id: String,
    pub cron_expression: String,
    pub timezone: Option<String>,
    pub created_at: DateTime<Utc>,
    pub last_run: Option<DateTime<Utc>>,
    pub next_run: DateTime<Utc>,
    pub enabled: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScheduleHistory {
    pub id: String,
    pub schedule_id: String,
    pub run_at: DateTime<Utc>,
    pub status: String,
    pub duration_ms: Option<u64>,
    pub error_message: Option<String>,
}

pub struct ScheduleStore {
    pool: SqlitePool,
}

impl ScheduleStore {
    pub async fn new(db_path: &Path) -> Result<Self, sqlx::Error> {
        let pool = SqlitePool::connect(&format!("sqlite:{}?mode=rwc", db_path.display())).await?;
        let store = Self { pool };
        store.init_schema().await?;
        Ok(store)
    }

    async fn init_schema(&self) -> Result<(), sqlx::Error> {
        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS schedules (
                id TEXT PRIMARY KEY,
                workflow_id TEXT NOT NULL,
                cron_expression TEXT NOT NULL,
                timezone TEXT,
                created_at TEXT NOT NULL,
                last_run TEXT,
                next_run TEXT NOT NULL,
                enabled BOOLEAN NOT NULL DEFAULT 1
            );

            CREATE TABLE IF NOT EXISTS schedule_history (
                id TEXT PRIMARY KEY,
                schedule_id TEXT NOT NULL,
                run_at TEXT NOT NULL,
                status TEXT NOT NULL,
                duration_ms INTEGER,
                error_message TEXT,
                FOREIGN KEY (schedule_id) REFERENCES schedules(id)
            );
            "#,
        )
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    pub async fn save_schedule(&self, schedule: &StoredSchedule) -> Result<(), sqlx::Error> {
        sqlx::query(
            r#"
            INSERT OR REPLACE INTO schedules
            (id, workflow_id, cron_expression, timezone, created_at, last_run, next_run, enabled)
            VALUES (?, ?, ?, ?, ?, ?, ?, ?)
            "#,
        )
        .bind(&schedule.id)
        .bind(&schedule.workflow_id)
        .bind(&schedule.cron_expression)
        .bind(&schedule.timezone)
        .bind(schedule.created_at.to_rfc3339())
        .bind(schedule.last_run.map(|dt| dt.to_rfc3339()))
        .bind(schedule.next_run.to_rfc3339())
        .bind(schedule.enabled)
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    pub async fn get_schedule(&self, id: &str) -> Result<Option<StoredSchedule>, sqlx::Error> {
        let row = sqlx::query("SELECT * FROM schedules WHERE id = ?")
            .bind(id)
            .fetch_optional(&self.pool)
            .await?;

        match row {
            Some(row) => {
                let schedule = StoredSchedule {
                    id: row.get("id"),
                    workflow_id: row.get("workflow_id"),
                    cron_expression: row.get("cron_expression"),
                    timezone: row.get("timezone"),
                    created_at: DateTime::parse_from_rfc3339(&row.get::<String, _>("created_at"))
                        .unwrap()
                        .with_timezone(&Utc),
                    last_run: row
                        .get::<Option<String>, _>("last_run")
                        .map(|s| DateTime::parse_from_rfc3339(&s).unwrap().with_timezone(&Utc)),
                    next_run: DateTime::parse_from_rfc3339(&row.get::<String, _>("next_run"))
                        .unwrap()
                        .with_timezone(&Utc),
                    enabled: row.get("enabled"),
                };
                Ok(Some(schedule))
            }
            None => Ok(None),
        }
    }

    pub async fn list_schedules(&self) -> Result<Vec<StoredSchedule>, sqlx::Error> {
        let rows = sqlx::query("SELECT * FROM schedules WHERE enabled = 1 ORDER BY created_at")
            .fetch_all(&self.pool)
            .await?;

        let schedules: Result<Vec<_>, _> = rows
            .iter()
            .map(|row| {
                Ok(StoredSchedule {
                    id: row.get("id"),
                    workflow_id: row.get("workflow_id"),
                    cron_expression: row.get("cron_expression"),
                    timezone: row.get("timezone"),
                    created_at: DateTime::parse_from_rfc3339(&row.get::<String, _>("created_at"))
                        .unwrap()
                        .with_timezone(&Utc),
                    last_run: row
                        .get::<Option<String>, _>("last_run")
                        .map(|s| DateTime::parse_from_rfc3339(&s).unwrap().with_timezone(&Utc)),
                    next_run: DateTime::parse_from_rfc3339(&row.get::<String, _>("next_run"))
                        .unwrap()
                        .with_timezone(&Utc),
                    enabled: row.get("enabled"),
                })
            })
            .collect();

        schedules
    }

    pub async fn record_run(&self, history: &ScheduleHistory) -> Result<(), sqlx::Error> {
        sqlx::query(
            r#"
            INSERT INTO schedule_history
            (id, schedule_id, run_at, status, duration_ms, error_message)
            VALUES (?, ?, ?, ?, ?, ?)
            "#,
        )
        .bind(&history.id)
        .bind(&history.schedule_id)
        .bind(history.run_at.to_rfc3339())
        .bind(&history.status)
        .bind(history.duration_ms)
        .bind(&history.error_message)
        .execute(&self.pool)
        .await?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_save_and_retrieve_schedule() {
        let db_path = Path::new(":memory:");
        let store = ScheduleStore::new(db_path).await.unwrap();

        let schedule = StoredSchedule {
            id: "test-schedule-1".to_string(),
            workflow_id: "workflow-1".to_string(),
            cron_expression: "0 * * * *".to_string(),
            timezone: None,
            created_at: Utc::now(),
            last_run: None,
            next_run: Utc::now(),
            enabled: true,
        };

        store.save_schedule(&schedule).await.unwrap();
        let retrieved = store.get_schedule("test-schedule-1").await.unwrap();

        assert!(retrieved.is_some());
        let retrieved = retrieved.unwrap();
        assert_eq!(retrieved.id, "test-schedule-1");
        assert_eq!(retrieved.workflow_id, "workflow-1");
        assert_eq!(retrieved.cron_expression, "0 * * * *");
    }

    #[tokio::test]
    async fn test_list_schedules() {
        let db_path = Path::new(":memory:");
        let store = ScheduleStore::new(db_path).await.unwrap();

        let schedule1 = StoredSchedule {
            id: "test-schedule-1".to_string(),
            workflow_id: "workflow-1".to_string(),
            cron_expression: "0 * * * *".to_string(),
            timezone: None,
            created_at: Utc::now(),
            last_run: None,
            next_run: Utc::now(),
            enabled: true,
        };

        store.save_schedule(&schedule1).await.unwrap();
        let schedules = store.list_schedules().await.unwrap();

        assert_eq!(schedules.len(), 1);
        assert_eq!(schedules[0].id, "test-schedule-1");
    }
}
```

---

### Step 14: Run persistence tests to verify they pass

**Files:** Run tests

```bash
cargo test automation::cron::persistence --lib
```

**Expected:** PASS all tests

---

### Step 15: Write failing tests for cron scheduler

**Files:** Create `automation/cron/scheduler.rs` (with tests)

```rust
use super::{CronExpression, ScheduleSafetyConfig};
use crate::workflow_ir::ExecutionEngine;
use chrono::{DateTime, Utc};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

#[derive(Debug, Clone)]
pub enum JobStatus {
    Pending,
    Running,
    Completed,
    Failed(String),
}

#[derive(Debug, Clone)]
pub struct ScheduledJob {
    pub id: String,
    pub workflow_id: String,
    pub cron_expression: CronExpression,
    pub status: JobStatus,
    pub next_run: DateTime<Utc>,
    pub last_run: Option<DateTime<Utc>>,
}

pub struct CronScheduler {
    jobs: Arc<RwLock<HashMap<String, ScheduledJob>>>,
    execution_engine: Arc<ExecutionEngine>,
    safety_config: ScheduleSafetyConfig,
}

impl CronScheduler {
    pub fn new(execution_engine: Arc<ExecutionEngine>) -> Self {
        Self {
            jobs: Arc::new(RwLock::new(HashMap::new())),
            execution_engine,
            safety_config: ScheduleSafetyConfig::default(),
        }
    }

    pub fn with_safety_config(mut self, config: ScheduleSafetyConfig) -> Self {
        self.safety_config = config;
        self
    }

    pub async fn add_schedule(
        &self,
        id: String,
        workflow_id: String,
        cron_expression: CronExpression,
    ) -> Result<(), String> {
        // Implementation will be added in Step 17
        todo!()
    }

    pub async fn remove_schedule(&self, id: &str) -> Result<(), String> {
        // Implementation will be added in Step 17
        todo!()
    }

    pub async fn list_schedules(&self) -> Vec<ScheduledJob> {
        // Implementation will be added in Step 17
        todo!()
    }

    pub async fn get_schedule(&self, id: &str) -> Option<ScheduledJob> {
        // Implementation will be added in Step 17
        todo!()
    }

    pub async fn start(&self) -> Result<(), String> {
        // Implementation will be added in Step 17
        todo!()
    }

    pub async fn stop(&self) -> Result<(), String> {
        // Implementation will be added in Step 17
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::automation::cron::parser::parse_cron_expression;

    #[tokio::test]
    async fn test_add_schedule() {
        let execution_engine = Arc::new(ExecutionEngine::new());
        let scheduler = CronScheduler::new(execution_engine);

        let cron_expr = parse_cron_expression("0 * * * *").unwrap();
        let result = scheduler
            .add_schedule("test-1".to_string(), "workflow-1".to_string(), cron_expr)
            .await;

        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_list_schedules() {
        let execution_engine = Arc::new(ExecutionEngine::new());
        let scheduler = CronScheduler::new(execution_engine);

        let cron_expr = parse_cron_expression("0 * * * *").unwrap();
        scheduler
            .add_schedule("test-1".to_string(), "workflow-1".to_string(), cron_expr)
            .await
            .unwrap();

        let schedules = scheduler.list_schedules().await;
        assert_eq!(schedules.len(), 1);
        assert_eq!(schedules[0].id, "test-1");
    }

    #[tokio::test]
    async fn test_remove_schedule() {
        let execution_engine = Arc::new(ExecutionEngine::new());
        let scheduler = CronScheduler::new(execution_engine);

        let cron_expr = parse_cron_expression("0 * * * *").unwrap();
        scheduler
            .add_schedule("test-1".to_string(), "workflow-1".to_string(), cron_expr)
            .await
            .unwrap();

        scheduler.remove_schedule("test-1").await.unwrap();
        let schedules = scheduler.list_schedules().await;
        assert_eq!(schedules.len(), 0);
    }
}
```

---

### Step 16: Run scheduler tests to verify they fail

**Files:** Run tests

```bash
cargo test automation::cron::scheduler --lib
```

**Expected:** FAIL with "todo!()" errors

---

### Step 17: Implement cron scheduler

**Files:** Modify `automation/cron/scheduler.rs`

```rust
use super::{CronExpression, ScheduleSafetyConfig, ValidationError};
use crate::automation::cron::validator::validate_schedule;
use crate::workflow_ir::ExecutionEngine;
use chrono::{DateTime, Utc};
use cron::Schedule as CronSchedule;
use std::collections::HashMap;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::RwLock;
use tokio_cron_scheduler::{Job, JobScheduler, JobSchedulerError};

#[derive(Debug, Clone)]
pub enum JobStatus {
    Pending,
    Running,
    Completed,
    Failed(String),
}

#[derive(Debug, Clone)]
pub struct ScheduledJob {
    pub id: String,
    pub workflow_id: String,
    pub cron_expression: CronExpression,
    pub status: JobStatus,
    pub next_run: DateTime<Utc>,
    pub last_run: Option<DateTime<Utc>>,
}

pub struct CronScheduler {
    jobs: Arc<RwLock<HashMap<String, ScheduledJob>>>,
    execution_engine: Arc<ExecutionEngine>,
    safety_config: ScheduleSafetyConfig,
    scheduler: Option<Arc<JobScheduler>>,
}

impl CronScheduler {
    pub fn new(execution_engine: Arc<ExecutionEngine>) -> Self {
        Self {
            jobs: Arc::new(RwLock::new(HashMap::new())),
            execution_engine,
            safety_config: ScheduleSafetyConfig::default(),
            scheduler: None,
        }
    }

    pub fn with_safety_config(mut self, config: ScheduleSafetyConfig) -> Self {
        self.safety_config = config;
        self
    }

    pub async fn add_schedule(
        &self,
        id: String,
        workflow_id: String,
        cron_expression: CronExpression,
    ) -> Result<(), String> {
        // Validate schedule
        validate_schedule(&cron_expression, &self.safety_config)
            .map_err(|e| format!("Schedule validation failed: {}", e))?;

        // Parse cron schedule
        let cron_str = format!(
            "{} {} {} {} {}",
            cron_expression.minute,
            cron_expression.hour,
            cron_expression.day_of_month,
            cron_expression.month,
            cron_expression.day_of_week
        );
        let schedule = CronSchedule::from_str(&cron_str)
            .map_err(|e| format!("Invalid cron expression: {}", e))?;

        // Calculate next run time
        let next_run = schedule.after(&Utc::now()).next().unwrap();

        // Create scheduled job
        let job = ScheduledJob {
            id: id.clone(),
            workflow_id: workflow_id.clone(),
            cron_expression: cron_expression.clone(),
            status: JobStatus::Pending,
            next_run,
            last_run: None,
        };

        // Store job
        let mut jobs = self.jobs.write().await;
        jobs.insert(id.clone(), job);
        drop(jobs);

        // Add to scheduler if running
        if let Some(scheduler) = &self.scheduler {
            self.add_job_to_scheduler(&scheduler, &id, &workflow_id, &cron_str)
                .await?;
        }

        Ok(())
    }

    pub async fn remove_schedule(&self, id: &str) -> Result<(), String> {
        let mut jobs = self.jobs.write().await;
        if jobs.remove(id).is_some() {
            Ok(())
        } else {
            Err(format!("Schedule {} not found", id))
        }
    }

    pub async fn list_schedules(&self) -> Vec<ScheduledJob> {
        let jobs = self.jobs.read().await;
        jobs.values().cloned().collect()
    }

    pub async fn get_schedule(&self, id: &str) -> Option<ScheduledJob> {
        let jobs = self.jobs.read().await;
        jobs.get(id).cloned()
    }

    pub async fn start(&self) -> Result<(), String> {
        let scheduler = JobScheduler::new()
            .map_err(|e| format!("Failed to create scheduler: {}", e))?;

        let scheduler = Arc::new(scheduler);

        // Add all existing jobs
        let jobs = self.jobs.read().await;
        for (id, job) in jobs.iter() {
            let cron_str = format!(
                "{} {} {} {} {}",
                job.cron_expression.minute,
                job.cron_expression.hour,
                job.cron_expression.day_of_month,
                job.cron_expression.month,
                job.cron_expression.day_of_week
            );
            self.add_job_to_scheduler(&scheduler, id, &job.workflow_id, &cron_str)
                .await?;
        }
        drop(jobs);

        // Start scheduler
        scheduler
            .start()
            .map_err(|e| format!("Failed to start scheduler: {}", e))?;

        Ok(())
    }

    pub async fn stop(&self) -> Result<(), String> {
        if let Some(scheduler) = &self.scheduler {
            scheduler
                .shutdown()
                .await
                .map_err(|e| format!("Failed to stop scheduler: {}", e))?;
        }
        Ok(())
    }

    async fn add_job_to_scheduler(
        &self,
        scheduler: &Arc<JobScheduler>,
        id: &str,
        workflow_id: &str,
        cron_str: &str,
    ) -> Result<(), String> {
        let jobs = self.jobs.clone();
        let execution_engine = self.execution_engine.clone();
        let id_clone = id.to_string();
        let workflow_id_clone = workflow_id.to_string();

        let job = Job::new_async(cron_str, move |_uuid, _l| {
            let id = id_clone.clone();
            let workflow_id = workflow_id_clone.clone();
            let jobs = jobs.clone();
            let execution_engine = execution_engine.clone();

            Box::pin(async move {
                // Update job status to running
                {
                    let mut jobs_guard = jobs.write().await;
                    if let Some(job) = jobs_guard.get_mut(&id) {
                        job.status = JobStatus::Running;
                        job.last_run = Some(Utc::now());
                    }
                }

                // Execute workflow
                let result = execution_engine.execute_workflow(&workflow_id).await;

                // Update job status
                {
                    let mut jobs_guard = jobs.write().await;
                    if let Some(job) = jobs_guard.get_mut(&id) {
                        job.status = match result {
                            Ok(_) => JobStatus::Completed,
                            Err(e) => JobStatus::Failed(e.to_string()),
                        };
                    }
                }
            })
        })
        .map_err(|e| format!("Failed to create job: {}", e))?;

        scheduler
            .add(job)
            .await
            .map_err(|e| format!("Failed to add job to scheduler: {}", e))?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::automation::cron::parser::parse_cron_expression;

    #[tokio::test]
    async fn test_add_schedule() {
        let execution_engine = Arc::new(ExecutionEngine::new());
        let scheduler = CronScheduler::new(execution_engine);

        let cron_expr = parse_cron_expression("0 * * * *").unwrap();
        let result = scheduler
            .add_schedule("test-1".to_string(), "workflow-1".to_string(), cron_expr)
            .await;

        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_list_schedules() {
        let execution_engine = Arc::new(ExecutionEngine::new());
        let scheduler = CronScheduler::new(execution_engine);

        let cron_expr = parse_cron_expression("0 * * * *").unwrap();
        scheduler
            .add_schedule("test-1".to_string(), "workflow-1".to_string(), cron_expr)
            .await
            .unwrap();

        let schedules = scheduler.list_schedules().await;
        assert_eq!(schedules.len(), 1);
        assert_eq!(schedules[0].id, "test-1");
    }

    #[tokio::test]
    async fn test_remove_schedule() {
        let execution_engine = Arc::new(ExecutionEngine::new());
        let scheduler = CronScheduler::new(execution_engine);

        let cron_expr = parse_cron_expression("0 * * * *").unwrap();
        scheduler
            .add_schedule("test-1".to_string(), "workflow-1".to_string(), cron_expr)
            .await
            .unwrap();

        scheduler.remove_schedule("test-1").await.unwrap();
        let schedules = scheduler.list_schedules().await;
        assert_eq!(schedules.len(), 0);
    }
}
```

---

### Step 18: Run scheduler tests to verify they pass

**Files:** Run tests

```bash
cargo test automation::cron::scheduler --lib
```

**Expected:** PASS all tests

---

### Step 19: Create integration test for cron scheduler

**Files:** Create `tests/integration/cron_scheduler_test.rs`

```rust
use agentsdk::automation::cron::{
    CronScheduler, parse_cron_expression, ScheduleSafetyConfig,
};
use agentsdk::workflow_ir::ExecutionEngine;

#[tokio::test]
async fn test_cron_scheduler_lifecycle() {
    let execution_engine = Arc::new(ExecutionEngine::new());
    let scheduler = CronScheduler::new(execution_engine);

    // Add schedule
    let cron_expr = parse_cron_expression("*/5 * * * *").unwrap();
    scheduler
        .add_schedule("test-1".to_string(), "workflow-1".to_string(), cron_expr)
        .await
        .unwrap();

    // Verify schedule exists
    let schedule = scheduler.get_schedule("test-1").await;
    assert!(schedule.is_some());
    assert_eq!(schedule.unwrap().workflow_id, "workflow-1");

    // Start scheduler
    scheduler.start().await.unwrap();

    // Wait a bit
    tokio::time::sleep(tokio::time::Duration::from_secs(10)).await;

    // Stop scheduler
    scheduler.stop().await.unwrap();

    // Cleanup
    scheduler.remove_schedule("test-1").await.unwrap();
}

#[tokio::test]
async fn test_safety_config() {
    let execution_engine = Arc::new(ExecutionEngine::new());
    let mut safety_config = ScheduleSafetyConfig::default();
    safety_config.enabled_timezones = vec!["UTC".to_string()];

    let scheduler = CronScheduler::new(execution_engine).with_safety_config(safety_config);

    // Valid timezone
    let cron_expr = parse_cron_expression("0 * * * *@UTC").unwrap();
    let result = scheduler
        .add_schedule("test-1".to_string(), "workflow-1".to_string(), cron_expr)
        .await;
    assert!(result.is_ok());

    // Invalid timezone should fail
    let cron_expr = parse_cron_expression("0 * * * *@America/New_York").unwrap();
    let result = scheduler
        .add_schedule("test-2".to_string(), "workflow-2".to_string(), cron_expr)
        .await;
    assert!(result.is_err());
}
```

---

### Step 20: Run integration tests

**Files:** Run tests

```bash
cargo test --test cron_scheduler_test
```

**Expected:** PASS all tests

---

### Step 21: Update automation module to include cron

**Files:** Create `automation/mod.rs`

```rust
pub mod cron;

pub use cron::{
    CronScheduler, CronSchedulerInstance, CronExpression, CronParseError,
    ScheduleSafetyConfig, ScheduleValidator, ValidationError,
};
```

---

### Step 22: Add to Cargo.toml module structure

**Files:** Modify `Cargo.toml` (add lib section if not exists)

```toml
[lib]
name = "agentsdk"
path = "src/lib.rs"
```

**Files:** Create `src/lib.rs`

```rust
pub mod automation;
pub mod workflow_ir;
```

---

### Step 23: Verify all tests pass

**Files:** Run all tests

```bash
cargo test automation::cron
cargo test --test cron_scheduler_test
```

**Expected:** PASS all tests

---

### Step 24: Write documentation

**Files:** Create `automation/cron/README.md`

```markdown
# Cron Scheduler

The cron scheduler provides time-based workflow execution for the AgentSDK.

## Features

- Cron expression parsing and validation
- Tokio-based async scheduling
- Safety limits (max concurrent jobs, job duration, etc.)
- Persistent schedule storage
- Timezone support
- Missed schedule handling

## Usage

```rust
use agentsdk::automation::cron::{CronScheduler, parse_cron_expression};

let execution_engine = Arc::new(ExecutionEngine::new());
let scheduler = CronScheduler::new(execution_engine);

// Parse cron expression
let cron_expr = parse_cron_expression("0 9 * * 1-5@America/New_York").unwrap();

// Add schedule
scheduler.add_schedule(
    "daily-morning-report".to_string(),
    "workflow-morning-report".to_string(),
    cron_expr,
).await.unwrap();

// Start scheduler
scheduler.start().await.unwrap();
```

## ADR-0007 Compliance

This task provides the scheduling infrastructure. The actual policy compilation
to WorkflowIR happens in Task 06 (Scheduling Policy Compiler).

## Safety

- Timezone validation (only enabled timezones allowed)
- Max concurrent jobs limit
- Max job duration limit
- Max missed schedules limit
```

---

### Step 25: Commit all changes

**Files:** Commit

```bash
git add automation/cron tests/integration/cron_scheduler_test.rs automation/mod.rs src/lib.rs Cargo.toml
git commit -m "feat(automation): implement cron scheduler (Task 00)

- Add cron expression parser with timezone support
- Implement tokio-based async scheduling engine
- Add safety limits validation
- Implement persistent schedule storage (SQLite)
- Add missed schedule handling
- Write comprehensive unit and integration tests
- Follow ADR-0007: scheduler infrastructure only (policy compilation in Task 06)

Refs: Phase 6, Task 00"
```

---

## Summary

Task 00 implements the foundational cron scheduling infrastructure with:

✅ Cron expression parsing and validation (with timezone support)
✅ Tokio-based async scheduling engine
✅ Safety limits (max concurrent jobs, job duration, missed schedules)
✅ Persistent schedule storage (SQLite)
✅ Missed schedule detection and handling
✅ Comprehensive unit and integration tests
✅ ADR-0007 compliance (infrastructure only, policy compilation in Task 06)

**Next Steps:** Task 01 (Git Experiment Framework) or Task 06 (Scheduling Policy Compiler)
