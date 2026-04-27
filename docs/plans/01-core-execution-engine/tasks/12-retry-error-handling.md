# Task 12: Retry & Error Handling

**Goal:** Implement retry strategies (exponential/linear/fixed backoff), error escalation, and graceful recovery mechanisms.

**Files:**
- Create: `src/observability/retry.rs`
- Create: `tests/unit/retry_test.rs`

---

## Rust Definitions

### `src/observability/retry.rs`

```rust
use async_trait::async_trait;
use std::time::Duration;
use tokio::time::sleep;

/// Retry strategy
#[derive(Debug, Clone, Copy)]
pub enum RetryStrategy {
    /// No retry
    None,
    /// Fixed delay between retries
    Fixed { delay_ms: u64 },
    /// Linear backoff (delay = base * attempt)
    Linear { base_ms: u64 },
    /// Exponential backoff (delay = base * 2^attempt)
    Exponential { base_ms: u64, max_ms: u64 },
}

impl RetryStrategy {
    /// Calculate delay for given attempt
    pub fn delay(&self, attempt: u32) -> Duration {
        match self {
            RetryStrategy::None => Duration::ZERO,
            RetryStrategy::Fixed { delay_ms } => Duration::from_millis(*delay_ms as u64),
            RetryStrategy::Linear { base_ms } => {
                let delay = *base_ms as u64 * attempt as u64;
                Duration::from_millis(delay)
            }
            RetryStrategy::Exponential { base_ms, max_ms } => {
                let delay = (*base_ms as u64) * 2u64.pow(attempt.min(10));
                let delay = delay.min(*max_ms as u64);
                Duration::from_millis(delay)
            }
        }
    }
}

/// Error classification
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ErrorClass {
    /// Transient error - can be retried
    Transient,
    /// Permanent error - no point retrying
    Permanent,
    /// User error - should not retry
    User,
}

/// Error with classification
#[derive(Debug, Clone)]
pub struct ClassifiedError {
    pub error: String,
    pub class: ErrorClass,
    pub retryable: bool,
}

impl ClassifiedError {
    pub fn new(error: String, class: ErrorClass) -> Self {
        let retryable = matches!(class, ErrorClass::Transient);
        Self {
            error,
            class,
            retryable,
        }
    }

    pub fn transient(error: String) -> Self {
        Self::new(error, ErrorClass::Transient)
    }

    pub fn permanent(error: String) -> Self {
        Self::new(error, ErrorClass::Permanent)
    }

    pub fn user(error: String) -> Self {
        Self::new(error, ErrorClass::User)
    }

    /// Classify error based on error message
    pub fn classify(error: &str) -> Self {
        let error_lower = error.to_lowercase();

        if error_lower.contains("timeout")
            || error_lower.contains("connection reset")
            || error_lower.contains("temporarily unavailable")
            || error_lower.contains("rate limit")
            || error_lower.contains("503")
            || error_lower.contains("502")
        {
            Self::transient(error.to_string())
        } else if error_lower.contains("unauthorized")
            || error_lower.contains("forbidden")
            || error_lower.contains("invalid input")
            || error_lower.contains("validation error")
        {
            Self::user(error.to_string())
        } else {
            Self::permanent(error.to_string())
        }
    }
}

/// Retry configuration
#[derive(Debug, Clone)]
pub struct RetryConfig {
    pub max_attempts: u32,
    pub strategy: RetryStrategy,
    pub on_error_class: Vec<ErrorClass>,
}

impl RetryConfig {
    pub fn new(max_attempts: u32, strategy: RetryStrategy) -> Self {
        Self {
            max_attempts,
            strategy,
            on_error_class: vec![ErrorClass::Transient],
        }
    }

    pub fn should_retry(&self, error: &ClassifiedError, attempt: u32) -> bool {
        if attempt >= self.max_attempts {
            return false;
        }

        if !error.retryable {
            return false;
        }

        self.on_error_class.contains(&error.class)
    }
}

impl Default for RetryConfig {
    fn default() -> Self {
        Self::new(
            3,
            RetryStrategy::Exponential {
                base_ms: 1000,
                max_ms: 60000,
            },
        )
    }
}

/// Retry result
#[derive(Debug, Clone)]
pub enum RetryResult<T> {
    Success { value: T, attempts: u32 },
    Failure { error: ClassifiedError, attempts: u32 },
}

/// Retry policy
#[async_trait]
pub trait RetryPolicy: Send + Sync {
    async fn execute<F, Fut, T, E>(&self, operation: F) -> RetryResult<T>
    where
        F: Fn() -> Fut + Send + Sync,
        Fut: std::future::Future<Output = Result<T, E>> + Send,
        E: std::error::Error + Send + 'static;
}

/// Default retry policy implementation
pub struct DefaultRetryPolicy {
    config: RetryConfig,
}

impl DefaultRetryPolicy {
    pub fn new(config: RetryConfig) -> Self {
        Self { config }
    }

    pub fn exponential(max_attempts: u32, base_ms: u64, max_ms: u64) -> Self {
        Self::new(RetryConfig {
            max_attempts,
            strategy: RetryStrategy::Exponential { base_ms, max_ms },
            on_error_class: vec![ErrorClass::Transient],
        })
    }

    pub fn linear(max_attempts: u32, base_ms: u64) -> Self {
        Self::new(RetryConfig {
            max_attempts,
            strategy: RetryStrategy::Linear { base_ms },
            on_error_class: vec![ErrorClass::Transient],
        })
    }

    pub fn fixed(max_attempts: u32, delay_ms: u64) -> Self {
        Self::new(RetryConfig {
            max_attempts,
            strategy: RetryStrategy::Fixed { delay_ms },
            on_error_class: vec![ErrorClass::Transient],
        })
    }
}

#[async_trait]
impl RetryPolicy for DefaultRetryPolicy {
    async fn execute<F, Fut, T, E>(&self, operation: F) -> RetryResult<T>
    where
        F: Fn() -> Fut + Send + Sync,
        Fut: std::future::Future<Output = Result<T, E>> + Send,
        E: std::error::Error + Send + 'static,
    {
        let mut attempt = 0;

        loop {
            attempt += 1;

            match operation().await {
                Ok(value) => {
                    return RetryResult::Success { value, attempts: attempt };
                }
                Err(e) => {
                    let error_msg = e.to_string();
                    let classified_error = ClassifiedError::classify(&error_msg);

                    if !self.config.should_retry(&classified_error, attempt) {
                        return RetryResult::Failure {
                            error: classified_error,
                            attempts: attempt,
                        };
                    }

                    // Calculate delay and sleep
                    let delay = self.config.strategy.delay(attempt);
                    sleep(delay).await;
                }
            }
        }
    }
}

/// Error escalation handler
pub struct ErrorEscalation {
    escalation_chain: Vec<(ErrorClass, Box<dyn ErrorHandler>)>,
}

impl ErrorEscalation {
    pub fn new() -> Self {
        Self {
            escalation_chain: Vec::new(),
        }
    }

    pub fn add_handler(&mut self, class: ErrorClass, handler: Box<dyn ErrorHandler>) {
        self.escalation_chain.push((class, handler));
    }

    pub async fn handle(&self, error: &ClassifiedError) -> Result<(), String> {
        for (class, handler) in &self.escalation_chain {
            if *class == error.class {
                return handler.handle(error).await;
            }
        }

        // Default: log error
        tracing::error!("Error: {} (class: {:?})", error.error, error.class);
        Ok(())
    }
}

impl Default for ErrorEscalation {
    fn default() -> Self {
        Self::new()
    }
}

/// Error handler trait
#[async_trait]
pub trait ErrorHandler: Send + Sync {
    async fn handle(&self, error: &ClassifiedError) -> Result<(), String>;
}

/// Default error handlers
pub struct LogErrorHandler;

#[async_trait]
impl ErrorHandler for LogErrorHandler {
    async fn handle(&self, error: &ClassifiedError) -> Result<(), String> {
        match error.class {
            ErrorClass::Transient => {
                tracing::warn!("Transient error (will retry): {}", error.error);
            }
            ErrorClass::Permanent => {
                tracing::error!("Permanent error: {}", error.error);
            }
            ErrorClass::User => {
                tracing::error!("User error: {}", error.error);
            }
        }

        Ok(())
    }
}

pub struct PanicErrorHandler;

#[async_trait]
impl ErrorHandler for PanicErrorHandler {
    async fn handle(&self, error: &ClassifiedError) -> Result<(), String> {
        if error.class == ErrorClass::Permanent || error.class == ErrorClass::User {
            panic!("Fatal error: {}", error.error);
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_retry_policy_success() {
        let policy = DefaultRetryPolicy::fixed(3, 100);
        let mut attempt = 0;

        let result = policy
            .execute(|| {
                attempt += 1;
                if attempt < 2 {
                    Err(std::io::Error::new(
                        std::io::ErrorKind::ConnectionRefused,
                        "timeout",
                    ))
                } else {
                    Ok::<_, std::io::Error>("success")
                }
            })
            .await;

        match result {
            RetryResult::Success { value, attempts } => {
                assert_eq!(value, "success");
                assert_eq!(attempts, 2);
            }
            _ => panic!("Expected success"),
        }
    }

    #[tokio::test]
    async fn test_retry_policy_failure() {
        let policy = DefaultRetryPolicy::fixed(2, 100);
        let attempt = std::sync::Arc::new(std::sync::atomic::AtomicU32::new(0));

        let result = policy
            .execute(|| {
                attempt.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
                Err::<(), std::io::Error>(std::io::Error::new(
                    std::io::ErrorKind::ConnectionRefused,
                    "timeout",
                ))
            })
            .await;

        match result {
            RetryResult::Failure { attempts, .. } => {
                assert_eq!(attempts, 2);
            }
            _ => panic!("Expected failure"),
        }
    }

    #[test]
    fn test_error_classification() {
        let transient = ClassifiedError::classify("Connection timeout");
        assert_eq!(transient.class, ErrorClass::Transient);

        let permanent = ClassifiedError::classify("Not found");
        assert_eq!(permanent.class, ErrorClass::Permanent);

        let user = ClassifiedError::classify("Unauthorized");
        assert_eq!(user.class, ErrorClass::User);
    }

    #[test]
    fn test_retry_strategy_exponential() {
        let strategy = RetryStrategy::Exponential {
            base_ms: 1000,
            max_ms: 10000,
        };

        assert_eq!(strategy.delay(0).as_millis(), 1000);
        assert_eq!(strategy.delay(1).as_millis(), 2000);
        assert_eq!(strategy.delay(2).as_millis(), 4000);
        assert_eq!(strategy.delay(3).as_millis(), 8000);
        assert_eq!(strategy.delay(4).as_millis(), 10000); // Maxed out
    }
}
```

---

## Implementation Steps

- [ ] **Step 1: Create test file**

- [ ] **Step 2: Implement retry.rs**

- [ ] **Step 3: Add to observability/mod.rs**

- [ ] **Step 4: Run tests**

- [ ] **Step 5: Commit**

---

## QA Cross-References

- **QA Criteria**: ['$qa_criteria']('$file')
- **Test Cases**: ['$test_case']('$file')
- **Schema Ref**: $schema_ref
