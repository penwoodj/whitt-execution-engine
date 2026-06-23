use std::time::{Duration, Instant};
use crate::benchmark::error_types::BenchmarkErrorCategory;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CircuitState {
    Closed,
    Open,
    HalfOpen,
}

#[derive(Debug, Clone)]
pub struct CircuitBreakerConfig {
    pub failure_threshold: u32,
    pub recovery_timeout: Duration,
    pub half_open_max_calls: u32,
    pub success_threshold: u32,
}

impl Default for CircuitBreakerConfig {
    fn default() -> Self {
        Self {
            failure_threshold: 5,
            recovery_timeout: Duration::from_secs(30),
            half_open_max_calls: 3,
            success_threshold: 2,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct CircuitBreakerStats {
    pub state: CircuitState,
    pub consecutive_failures: u32,
    pub consecutive_successes: u32,
    pub half_open_calls: u32,
    pub total_failures: u32,
    pub total_successes: u32,
    pub total_rejected: u32,
    pub last_failure_elapsed: Option<Duration>,
}

pub struct CircuitBreaker {
    config: CircuitBreakerConfig,
    state: CircuitState,
    consecutive_failures: u32,
    consecutive_successes: u32,
    half_open_calls: u32,
    last_failure_time: Option<Instant>,
    opened_at: Option<Instant>,
    total_failures: u32,
    total_successes: u32,
    total_rejected: u32,
}

impl CircuitBreaker {
    pub fn new() -> Self {
        Self::with_config(CircuitBreakerConfig::default())
    }

    pub fn with_config(config: CircuitBreakerConfig) -> Self {
        Self {
            config,
            state: CircuitState::Closed,
            consecutive_failures: 0,
            consecutive_successes: 0,
            half_open_calls: 0,
            last_failure_time: None,
            opened_at: None,
            total_failures: 0,
            total_successes: 0,
            total_rejected: 0,
        }
    }

    pub fn record_success(&mut self) {
        self.total_successes += 1;

        match self.state {
            CircuitState::Closed => {
                self.consecutive_failures = 0;
                self.consecutive_successes += 1;
            }
            CircuitState::HalfOpen => {
                self.consecutive_failures = 0;
                self.consecutive_successes += 1;
                self.half_open_calls += 1;

                if self.consecutive_successes >= self.config.success_threshold {
                    self.state = CircuitState::Closed;
                    self.consecutive_successes = 0;
                    self.half_open_calls = 0;
                }
            }
            CircuitState::Open => {
                self.state = CircuitState::Closed;
                self.consecutive_failures = 0;
                self.consecutive_successes = 0;
            }
        }
    }

    pub fn record_failure(&mut self, category: BenchmarkErrorCategory) {
        self.total_failures += 1;
        self.last_failure_time = Some(Instant::now());

        if matches!(category, BenchmarkErrorCategory::Permanent) {
            self.state = CircuitState::Open;
            self.consecutive_failures = self.config.failure_threshold;
            self.consecutive_successes = 0;
            self.half_open_calls = 0;
            self.opened_at = Some(Instant::now());
            return;
        }

        match self.state {
            CircuitState::Closed => {
                self.consecutive_failures += 1;
                self.consecutive_successes = 0;

                if self.consecutive_failures >= self.config.failure_threshold {
                    self.state = CircuitState::Open;
                    self.opened_at = Some(Instant::now());
                }
            }
            CircuitState::HalfOpen => {
                self.state = CircuitState::Open;
                self.consecutive_failures = self.config.failure_threshold;
                self.consecutive_successes = 0;
                self.half_open_calls = 0;
                self.opened_at = Some(Instant::now());
            }
            CircuitState::Open => {}
        }
    }

    pub fn is_call_permitted(&mut self) -> bool {
        match self.state {
            CircuitState::Closed => true,
            CircuitState::Open => {
                if let Some(opened_at) = self.opened_at {
                    if opened_at.elapsed() >= self.config.recovery_timeout {
                        self.state = CircuitState::HalfOpen;
                        self.consecutive_failures = 0;
                        self.consecutive_successes = 0;
                        self.half_open_calls = 0;
                        self.opened_at = None;
                        true
                    } else {
                        self.total_rejected += 1;
                        false
                    }
                } else {
                    self.total_rejected += 1;
                    false
                }
            }
            CircuitState::HalfOpen => {
                if self.half_open_calls < self.config.half_open_max_calls {
                    true
                } else {
                    self.state = CircuitState::Open;
                    self.consecutive_failures = self.config.failure_threshold;
                    self.opened_at = Some(Instant::now());
                    self.total_rejected += 1;
                    false
                }
            }
        }
    }

    pub fn state(&self) -> CircuitState {
        self.state
    }

    pub fn stats(&self) -> CircuitBreakerStats {
        let last_failure_elapsed = self.last_failure_time
            .map(|t| t.elapsed());

        CircuitBreakerStats {
            state: self.state,
            consecutive_failures: self.consecutive_failures,
            consecutive_successes: self.consecutive_successes,
            half_open_calls: self.half_open_calls,
            total_failures: self.total_failures,
            total_successes: self.total_successes,
            total_rejected: self.total_rejected,
            last_failure_elapsed,
        }
    }

    pub fn reset(&mut self) {
        self.state = CircuitState::Closed;
        self.consecutive_failures = 0;
        self.consecutive_successes = 0;
        self.half_open_calls = 0;
        self.last_failure_time = None;
        self.opened_at = None;
    }
}

impl Default for CircuitBreaker {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn given_new_breaker_when_checked_then_permitted() {
        let mut breaker = CircuitBreaker::new();
        assert!(breaker.is_call_permitted());
        assert_eq!(breaker.state(), CircuitState::Closed);
    }

    #[test]
    fn given_threshold_failures_when_checked_then_open() {
        let mut breaker = CircuitBreaker::new();
        let config = breaker.config.clone();

        for _ in 0..config.failure_threshold {
            breaker.record_failure(BenchmarkErrorCategory::Transient);
        }

        assert_eq!(breaker.state(), CircuitState::Open);
        assert!(!breaker.is_call_permitted());
        assert_eq!(breaker.stats().total_rejected, 1);
    }

    #[test]
    fn given_open_after_timeout_when_checked_then_half_open() {
        let mut breaker = CircuitBreaker::new();

        for _ in 0..5 {
            breaker.record_failure(BenchmarkErrorCategory::Transient);
        }

        assert_eq!(breaker.state(), CircuitState::Open);
        assert!(!breaker.is_call_permitted());

        let config = CircuitBreakerConfig {
            recovery_timeout: Duration::from_millis(0),
            ..Default::default()
        };
        let mut breaker = CircuitBreaker::with_config(config);
        for _ in 0..5 {
            breaker.record_failure(BenchmarkErrorCategory::Transient);
        }

        assert!(breaker.is_call_permitted());
        assert_eq!(breaker.state(), CircuitState::HalfOpen);
    }

    #[test]
    fn given_half_open_successes_when_checked_then_closed() {
        let config = CircuitBreakerConfig {
            recovery_timeout: Duration::from_millis(0),
            success_threshold: 2,
            ..Default::default()
        };
        let mut breaker = CircuitBreaker::with_config(config);

        for _ in 0..5 {
            breaker.record_failure(BenchmarkErrorCategory::Transient);
        }

        assert!(breaker.is_call_permitted());
        assert_eq!(breaker.state(), CircuitState::HalfOpen);

        breaker.record_success();
        assert_eq!(breaker.state(), CircuitState::HalfOpen);

        breaker.record_success();
        assert_eq!(breaker.state(), CircuitState::Closed);
    }

    #[test]
    fn given_half_open_failure_when_checked_then_open() {
        let mut breaker = CircuitBreaker::new();

        for _ in 0..5 {
            breaker.record_failure(BenchmarkErrorCategory::Transient);
        }

        let config = breaker.config.clone();
        std::thread::sleep(config.recovery_timeout);
        assert!(breaker.is_call_permitted());
        assert_eq!(breaker.state(), CircuitState::HalfOpen);

        breaker.record_failure(BenchmarkErrorCategory::Transient);
        assert_eq!(breaker.state(), CircuitState::Open);
        assert!(!breaker.is_call_permitted());
    }

    #[test]
    fn given_permanent_error_when_recorded_then_immediately_open() {
        let mut breaker = CircuitBreaker::new();

        breaker.record_failure(BenchmarkErrorCategory::Permanent);

        assert_eq!(breaker.state(), CircuitState::Open);
        assert!(!breaker.is_call_permitted());
        assert_eq!(breaker.stats().consecutive_failures, 5);
    }

    #[test]
    fn given_reset_when_called_then_state_is_closed() {
        let mut breaker = CircuitBreaker::new();

        for _ in 0..5 {
            breaker.record_failure(BenchmarkErrorCategory::Transient);
        }

        assert_eq!(breaker.state(), CircuitState::Open);

        breaker.reset();

        assert_eq!(breaker.state(), CircuitState::Closed);
        assert!(breaker.is_call_permitted());
        assert_eq!(breaker.stats().consecutive_failures, 0);
    }

    #[test]
    fn given_operations_when_stats_checked_then_accurate() {
        let mut breaker = CircuitBreaker::new();

        breaker.record_success();
        breaker.record_success();
        breaker.record_success();

        breaker.record_failure(BenchmarkErrorCategory::Transient);
        breaker.record_failure(BenchmarkErrorCategory::Transient);

        for _ in 0..5 {
            breaker.record_failure(BenchmarkErrorCategory::Transient);
        }

        assert!(!breaker.is_call_permitted());
        assert!(!breaker.is_call_permitted());
        assert!(!breaker.is_call_permitted());

        let stats = breaker.stats();
        assert_eq!(stats.total_successes, 3);
        assert_eq!(stats.total_failures, 7);
        assert_eq!(stats.total_rejected, 3);
        assert!(stats.last_failure_elapsed.is_some());
    }

    #[test]
    fn given_success_in_closed_when_recorded_then_resets_failure_count() {
        let mut breaker = CircuitBreaker::new();

        breaker.record_failure(BenchmarkErrorCategory::Transient);
        breaker.record_failure(BenchmarkErrorCategory::Transient);
        assert_eq!(breaker.stats().consecutive_failures, 2);

        breaker.record_success();
        assert_eq!(breaker.stats().consecutive_failures, 0);
        assert_eq!(breaker.stats().consecutive_successes, 1);
    }

    #[test]
    fn given_half_open_quota_exceeded_when_checked_then_open() {
        let config = CircuitBreakerConfig {
            recovery_timeout: Duration::from_millis(0),
            half_open_max_calls: 2,
            success_threshold: 10,
            ..Default::default()
        };
        let mut breaker = CircuitBreaker::with_config(config);

        for _ in 0..5 {
            breaker.record_failure(BenchmarkErrorCategory::Transient);
        }

        assert!(breaker.is_call_permitted());
        assert_eq!(breaker.state(), CircuitState::HalfOpen);

        breaker.record_success();
        assert!(breaker.is_call_permitted());

        breaker.record_success();
        assert_eq!(breaker.stats().half_open_calls, 2);

        assert!(!breaker.is_call_permitted());
        assert_eq!(breaker.state(), CircuitState::Open);
    }

    #[test]
    fn given_unknown_category_when_recorded_then_counts_toward_threshold() {
        let mut breaker = CircuitBreaker::new();

        breaker.record_failure(BenchmarkErrorCategory::Unknown);
        breaker.record_failure(BenchmarkErrorCategory::Unknown);
        breaker.record_failure(BenchmarkErrorCategory::Unknown);
        breaker.record_failure(BenchmarkErrorCategory::Unknown);
        breaker.record_failure(BenchmarkErrorCategory::Unknown);

        assert_eq!(breaker.state(), CircuitState::Open);
        assert!(!breaker.is_call_permitted());
    }
}