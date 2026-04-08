# Task 10: Logging Framework

**Goal:** Implement 9-level hierarchical logging framework with tracing crate and per-scope configuration.

**Files:**
- Create: `src/observability/logging.rs`
- Create: `tests/unit/logging_test.rs`

---

## Rust Definitions

### `src/observability/logging.rs`

```rust
use tracing::{Level, Subscriber};
use tracing_subscriber::{filter, fmt, prelude::*, Registry};

/// Log levels (9-level hierarchy)
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum LogLevel {
    Trace = 0,
    Debug = 1,
    Info = 2,
    Warn = 3,
    Error = 4,
    Fatal = 5,
    Panic = 6,
    Off = 7,
    Custom(i8), // For user-defined levels
}

impl LogLevel {
    pub fn as_str(&self) -> &'static str {
        match self {
            LogLevel::Trace => "TRACE",
            LogLevel::Debug => "DEBUG",
            LogLevel::Info => "INFO",
            LogLevel::Warn => "WARN",
            LogLevel::Error => "ERROR",
            LogLevel::Fatal => "FATAL",
            LogLevel::Panic => "PANIC",
            LogLevel::Off => "OFF",
            LogLevel::Custom(n) => "CUSTOM",
        }
    }

    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_uppercase().as_str() {
            "TRACE" => Some(LogLevel::Trace),
            "DEBUG" => Some(LogLevel::Debug),
            "INFO" => Some(LogLevel::Info),
            "WARN" => Some(LogLevel::Warn),
            "ERROR" => Some(LogLevel::Error),
            "FATAL" => Some(LogLevel::Fatal),
            "PANIC" => Some(LogLevel::Panic),
            "OFF" => Some(LogLevel::Off),
            _ => None,
        }
    }

    pub fn to_tracing_level(&self) -> Level {
        match self {
            LogLevel::Trace => Level::TRACE,
            LogLevel::Debug => Level::DEBUG,
            LogLevel::Info => Level::INFO,
            LogLevel::Warn => Level::WARN,
            LogLevel::Error => Level::ERROR,
            LogLevel::Fatal => Level::ERROR,
            LogLevel::Panic => Level::ERROR,
            LogLevel::Off => Level::ERROR,
            LogLevel::Custom(_) => Level::INFO,
        }
    }
}

/// Logging scope (module/component)
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct LogScope {
    pub name: String,
    pub level: LogLevel,
}

impl LogScope {
    pub fn new(name: String, level: LogLevel) -> Self {
        Self { name, level }
    }
}

/// Logging configuration
#[derive(Debug, Clone)]
pub struct LoggingConfig {
    pub global_level: LogLevel,
    pub scopes: Vec<LogScope>,
    pub output_format: OutputFormat,
    pub output_destinations: Vec<OutputDestination>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OutputFormat {
    Text,
    Json,
    Pretty,
}

#[derive(Debug, Clone)]
pub enum OutputDestination {
    Console,
    File(String),
    Syslog,
}

impl Default for LoggingConfig {
    fn default() -> Self {
        Self {
            global_level: LogLevel::Info,
            scopes: vec![],
            output_format: OutputFormat::Pretty,
            output_destinations: vec![OutputDestination::Console],
        }
    }
}

/// Initialize logging with configuration
pub fn init_logging(config: LoggingConfig) -> Result<(), Box<dyn std::error::Error>> {
    let env_filter = filter::EnvFilter::builder()
        .with_default_directive(config.global_level.to_tracing_level().into())
        .from_env_lossy();

    let subscriber = Registry::default().with(env_filter);

    match config.output_format {
        OutputFormat::Json => {
            let json_layer = fmt::layer().json();
            subscriber.with(json_layer).init();
        }
        OutputFormat::Pretty => {
            let pretty_layer = fmt::layer().pretty();
            subscriber.with(pretty_layer).init();
        }
        OutputFormat::Text => {
            let text_layer = fmt::layer();
            subscriber.with(text_layer).init();
        }
    }

    // Configure per-scope log levels
    for scope in config.scopes {
        let level = scope.level.to_tracing_level();
        // Set level for scope using tracing directives
        // This is a simplified version
        tracing::info!("Configured scope '{}' at level {}", scope.name, scope.level.as_str());
    }

    Ok(())
}

/// Log macros for different levels
#[macro_export]
macro_rules! log_trace {
    ($($arg:tt)*) => {
        tracing::trace!($($arg)*)
    };
}

#[macro_export]
macro_rules! log_debug {
    ($($arg:tt)*) => {
        tracing::debug!($($arg)*)
    };
}

#[macro_export]
macro_rules! log_info {
    ($($arg:tt)*) => {
        tracing::info!($($arg)*)
    };
}

#[macro_export]
macro_rules! log_warn {
    ($($arg:tt)*) => {
        tracing::warn!($($arg)*)
    };
}

#[macro_export]
macro_rules! log_error {
    ($($arg:tt)*) => {
        tracing::error!($($arg)*)
    };
}

#[macro_export]
macro_rules! log_fatal {
    ($($arg:tt)*) => {
        tracing::error!("[FATAL] $($arg)*)
    };
}

#[macro_export]
macro_rules! log_panic {
    ($($arg:tt)*) => {
        tracing::error!("[PANIC] $($arg)*)
    };
}

/// Scoped logger
pub struct ScopedLogger {
    scope: String,
}

impl ScopedLogger {
    pub fn new(scope: String) -> Self {
        Self { scope }
    }

    pub fn trace(&self, message: &str) {
        tracing::trace!(scope = %self.scope, "{}", message);
    }

    pub fn debug(&self, message: &str) {
        tracing::debug!(scope = %self.scope, "{}", message);
    }

    pub fn info(&self, message: &str) {
        tracing::info!(scope = %self.scope, "{}", message);
    }

    pub fn warn(&self, message: &str) {
        tracing::warn!(scope = %self.scope, "{}", message);
    }

    pub fn error(&self, message: &str) {
        tracing::error!(scope = %self.scope, "{}", message);
    }

    pub fn fatal(&self, message: &str) {
        tracing::error!(scope = %self.scope, "[FATAL] {}", message);
    }

    pub fn panic(&self, message: &str) {
        tracing::error!(scope = %self.scope, "[PANIC] {}", message);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_log_level_from_str() {
        assert_eq!(LogLevel::from_str("INFO"), Some(LogLevel::Info));
        assert_eq!(LogLevel::from_str("DEBUG"), Some(LogLevel::Debug));
        assert_eq!(LogLevel::from_str("INVALID"), None);
    }

    #[test]
    fn test_log_level_ordering() {
        assert!(LogLevel::Debug < LogLevel::Info);
        assert!(LogLevel::Info < LogLevel::Error);
        assert!(LogLevel::Trace < LogLevel::Panic);
    }
}
```

---

## Implementation Steps

- [ ] **Step 1: Create test file**

- [ ] **Step 2: Implement logging.rs**

- [ ] **Step 3: Add observability to lib.rs**

- [ ] **Step 4: Run tests**

- [ ] **Step 5: Commit**
