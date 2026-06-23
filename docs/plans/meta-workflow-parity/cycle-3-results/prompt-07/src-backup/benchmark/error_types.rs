//! Benchmark error classification taxonomy (UF07: Fault Tolerance)
//!
//! Classifies errors into categories for retry/skip/escalate decisions.
//! Reference: docs/benchmarks/userflows/uf07-fault-tolerance-and-error-recovery.md

use crate::error::Error;
use serde::{Serialize, Deserialize};

/// Error category for retry decisions.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum BenchmarkErrorCategory {
    /// Error may resolve on retry (timeout, OOM, crash)
    Transient,
    /// Error will not resolve on retry (corruption, validation, disk full)
    Permanent,
    /// Error type not yet classified
    Unknown,
}

/// Specific error class for targeted handling.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum BenchmarkErrorClass {
    /// Model OOM during load — try quantization downgrade
    OutOfMemory,
    /// Provider timed out — retry with backoff
    ProviderTimeout,
    /// Model produced garbage output — skip model
    CorruptOutput,
    /// Network error during download — retry with exponential backoff
    NetworkError,
    /// Disk full — pause benchmark, alert operator
    DiskFull,
    /// Provider process crashed — restart provider, retry
    ProviderCrash,
    /// YAML/workflow validation failed — skip workflow
    ValidationError,
    /// Context window exceeded — truncate context
    ContextExceeded,
    /// Step produced no output — record empty, continue
    EmptyOutput,
    /// Unclassified error
    Unknown,
}

impl BenchmarkErrorClass {
    /// Returns the category for this error class.
    pub fn category(&self) -> BenchmarkErrorCategory {
        match self {
            Self::OutOfMemory => BenchmarkErrorCategory::Transient,
            Self::ProviderTimeout => BenchmarkErrorCategory::Transient,
            Self::ProviderCrash => BenchmarkErrorCategory::Transient,
            Self::NetworkError => BenchmarkErrorCategory::Transient,
            Self::CorruptOutput => BenchmarkErrorCategory::Permanent,
            Self::DiskFull => BenchmarkErrorCategory::Permanent,
            Self::ValidationError => BenchmarkErrorCategory::Permanent,
            Self::ContextExceeded => BenchmarkErrorCategory::Permanent,
            Self::EmptyOutput => BenchmarkErrorCategory::Permanent,
            Self::Unknown => BenchmarkErrorCategory::Unknown,
        }
    }

    /// Returns true if this error class is retryable.
    pub fn is_retryable(&self) -> bool {
        matches!(self.category(), BenchmarkErrorCategory::Transient)
    }

    /// Returns recommended max retries for this error class.
    pub fn recommended_retries(&self) -> u32 {
        match self {
            Self::OutOfMemory => 1,
            Self::ProviderTimeout => 3,
            Self::ProviderCrash => 2,
            Self::NetworkError => 5,
            Self::CorruptOutput => 0,
            Self::DiskFull => 0,
            Self::ValidationError => 0,
            Self::ContextExceeded => 0,
            Self::EmptyOutput => 0,
            Self::Unknown => 1,
        }
    }
}

/// Classify a crate::Error into a benchmark error class.
pub fn classify_error(error: &Error) -> BenchmarkErrorClass {
    let msg = error.to_string().to_lowercase();

    // Check specific error variants first
    match error {
        Error::YamlParse(_) => BenchmarkErrorClass::ValidationError,
        Error::Validation { .. } => BenchmarkErrorClass::ValidationError,
        Error::ModelLoadFailed { reason, .. } => {
            let reason_lower = reason.to_lowercase();
            if reason_lower.contains("oom") || reason_lower.contains("out of memory")
                || reason_lower.contains("cuda") || reason_lower.contains("vram") {
                BenchmarkErrorClass::OutOfMemory
            } else if reason_lower.contains("timeout") || reason_lower.contains("timed out") {
                BenchmarkErrorClass::ProviderTimeout
            } else if reason_lower.contains("crash") || reason_lower.contains("killed")
                || reason_lower.contains("signal") {
                BenchmarkErrorClass::ProviderCrash
            } else {
                BenchmarkErrorClass::Unknown
            }
        }
        Error::Io(ref io_err) => {
            match io_err.kind() {
                std::io::ErrorKind::TimedOut => BenchmarkErrorClass::ProviderTimeout,
                std::io::ErrorKind::ConnectionRefused => BenchmarkErrorClass::ProviderCrash,
                std::io::ErrorKind::ConnectionReset => BenchmarkErrorClass::ProviderCrash,
                std::io::ErrorKind::ConnectionAborted => BenchmarkErrorClass::ProviderCrash,
                _ => {
                    if msg.contains("no space") || msg.contains("disk full") {
                        BenchmarkErrorClass::DiskFull
                    } else {
                        BenchmarkErrorClass::Unknown
                    }
                }
            }
        }
        _ => {
            // Fall back to message-based classification
            if msg.contains("timeout") || msg.contains("timed out") {
                BenchmarkErrorClass::ProviderTimeout
            } else if msg.contains("oom") || msg.contains("out of memory")
                || msg.contains("cuda error") || msg.contains("vram") {
                BenchmarkErrorClass::OutOfMemory
            } else if msg.contains("crash") || msg.contains("killed")
                || msg.contains("signal 9") || msg.contains("segfault") {
                BenchmarkErrorClass::ProviderCrash
            } else if msg.contains("network") || msg.contains("connection")
                || msg.contains("refused") || msg.contains("reset") {
                BenchmarkErrorClass::NetworkError
            } else if msg.contains("context") || msg.contains("token limit")
                || msg.contains("too many tokens") {
                BenchmarkErrorClass::ContextExceeded
            } else if msg.contains("validation") || msg.contains("invalid")
                || msg.contains("parse error") {
                BenchmarkErrorClass::ValidationError
            } else if msg.contains("empty") || msg.contains("no output") {
                BenchmarkErrorClass::EmptyOutput
            } else if msg.contains("disk") || msg.contains("space")
                || msg.contains("no space") {
                BenchmarkErrorClass::DiskFull
            } else if msg.contains("corrupt") || msg.contains("garbage")
                || msg.contains("gibberish") {
                BenchmarkErrorClass::CorruptOutput
            } else {
                BenchmarkErrorClass::Unknown
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn given_oom_error_when_classified_then_out_of_memory() {
        let error = Error::ModelLoadFailed {
            model_name: "test".into(),
            reason: "CUDA out of memory".into(),
        };
        let class = classify_error(&error);
        assert_eq!(class, BenchmarkErrorClass::OutOfMemory);
        assert!(class.is_retryable());
        assert_eq!(class.category(), BenchmarkErrorCategory::Transient);
    }

    #[test]
    fn given_timeout_error_when_classified_then_provider_timeout() {
        let error = Error::Execution {
            message: "Request timed out after 30s".into(),
        };
        let class = classify_error(&error);
        assert_eq!(class, BenchmarkErrorClass::ProviderTimeout);
        assert!(class.is_retryable());
        assert_eq!(class.recommended_retries(), 3);
    }

    #[test]
    fn given_validation_error_when_classified_then_permanent() {
        let error = Error::Validation {
            message: "Invalid YAML structure".into(),
        };
        let class = classify_error(&error);
        assert_eq!(class, BenchmarkErrorClass::ValidationError);
        assert!(!class.is_retryable());
        assert_eq!(class.category(), BenchmarkErrorCategory::Permanent);
        assert_eq!(class.recommended_retries(), 0);
    }

    #[test]
    fn given_io_no_space_when_classified_then_disk_full() {
        let io_error = std::io::Error::new(
            std::io::ErrorKind::Other,
            "No space left on device"
        );
        let error = Error::Io(io_error);
        let class = classify_error(&error);
        assert_eq!(class, BenchmarkErrorClass::DiskFull);
        assert!(!class.is_retryable());
    }

    #[test]
    fn given_io_connection_refused_when_classified_then_provider_crash() {
        let io_error = std::io::Error::new(
            std::io::ErrorKind::ConnectionRefused,
            "Connection refused"
        );
        let error = Error::Io(io_error);
        let class = classify_error(&error);
        assert_eq!(class, BenchmarkErrorClass::ProviderCrash);
        assert!(class.is_retryable());
    }

    #[test]
    fn given_unknown_error_when_classified_then_unknown() {
        let error = Error::Benchmark {
            message: "Something unexpected happened".into(),
        };
        let class = classify_error(&error);
        assert_eq!(class, BenchmarkErrorClass::Unknown);
        assert_eq!(class.category(), BenchmarkErrorCategory::Unknown);
        assert_eq!(class.recommended_retries(), 1); // Conservative
    }

    #[test]
    fn given_all_transient_classes_when_checked_then_retryable() {
        assert!(BenchmarkErrorClass::OutOfMemory.is_retryable());
        assert!(BenchmarkErrorClass::ProviderTimeout.is_retryable());
        assert!(BenchmarkErrorClass::ProviderCrash.is_retryable());
        assert!(BenchmarkErrorClass::NetworkError.is_retryable());
    }

    #[test]
    fn given_all_permanent_classes_when_checked_then_not_retryable() {
        assert!(!BenchmarkErrorClass::CorruptOutput.is_retryable());
        assert!(!BenchmarkErrorClass::DiskFull.is_retryable());
        assert!(!BenchmarkErrorClass::ValidationError.is_retryable());
        assert!(!BenchmarkErrorClass::ContextExceeded.is_retryable());
        assert!(!BenchmarkErrorClass::EmptyOutput.is_retryable());
    }

    #[test]
    fn given_context_exceeded_message_when_classified_then_correct() {
        let error = Error::Execution {
            message: "Token limit exceeded: too many tokens in context".into(),
        };
        let class = classify_error(&error);
        assert_eq!(class, BenchmarkErrorClass::ContextExceeded);
    }
}