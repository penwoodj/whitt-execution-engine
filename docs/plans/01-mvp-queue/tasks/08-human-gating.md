# Task 08: Human Gating

**Goal:** Implement human gating system with operation classification (safe/risky/dangerous), confirmation prompts, diff preview, and staged execution (ADR-0002 requirement).

**Files:**
- Create: `src/safety/mod.rs`
- Create: `src/safety/classify.rs`
- Create: `src/safety/prompt.rs`
- Create: `src/safety/diff.rs`
- Create: `tests/unit/human_gating_test.rs`

---

## Rust Definitions

### `src/safety/classify.rs`

```rust
use serde::{Deserialize, Serialize};

/// Risk level for operations
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum RiskLevel {
    Safe,
    Risky,
    Dangerous,
}

/// Operation classification
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OperationClassification {
    pub risk_level: RiskLevel,
    pub reason: String,
    pub requires_approval: bool,
}

/// Operation classifier
pub struct OperationClassifier;

impl OperationClassifier {
    /// Classify an operation based on type and inputs
    pub fn classify(operation_type: &str, inputs: &serde_json::Value) -> OperationClassification {
        match operation_type {
            // Safe operations: read-only, computation
            "read_file" | "read_directory" | "echo" | "compute" => OperationClassification {
                risk_level: RiskLevel::Safe,
                reason: "Read-only operation".to_string(),
                requires_approval: false,
            },

            // Risky operations: write operations, API calls
            "write_file" | "create_directory" | "delete_file" | "api_call" => OperationClassification {
                risk_level: RiskLevel::Risky,
                reason: "Modifies external state or makes network requests".to_string(),
                requires_approval: true,
            },

            // Dangerous operations: destructive system changes
            "delete_directory" | "rm" | "system_command" | "execute_code" => OperationClassification {
                risk_level: RiskLevel::Dangerous,
                reason: "Destructive operation with potential system impact".to_string(),
                requires_approval: true,
            },

            // Default to risky for unknown operations
            _ => OperationClassification {
                risk_level: RiskLevel::Risky,
                reason: "Unknown operation type".to_string(),
                requires_approval: true,
            },
        }
    }

    /// Check if path is dangerous (e.g., system directories)
    pub fn is_dangerous_path(path: &str) -> bool {
        let dangerous_patterns = [
            "/bin/",
            "/sbin/",
            "/usr/bin/",
            "/usr/sbin/",
            "/etc/",
            "/sys/",
            "/proc/",
            "/dev/",
        ];

        dangerous_patterns.iter().any(|pattern| path.starts_with(pattern))
    }
}
```

### `src/safety/prompt.rs`

```rust
use crate::safety::classify::{OperationClassification, RiskLevel};
use crate::safety::diff::DiffPreview;
use serde::{Deserialize, Serialize};

/// Confirmation prompt
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConfirmationPrompt {
    pub operation_id: String,
    pub operation_type: String,
    pub risk_level: RiskLevel,
    pub classification: OperationClassification,
    pub diff_preview: Option<DiffPreview>,
    pub message: String,
}

impl ConfirmationPrompt {
    pub fn new(
        operation_id: String,
        operation_type: String,
        classification: OperationClassification,
        diff_preview: Option<DiffPreview>,
    ) -> Self {
        let message = match classification.risk_level {
            RiskLevel::Safe => format!("Safe operation: {} - {}", operation_type, classification.reason),
            RiskLevel::Risky => format!(
                "⚠️  RISKY operation: {}\n\nReason: {}\n\nPlease review carefully before proceeding.",
                operation_type, classification.reason
            ),
            RiskLevel::Dangerous => format!(
                "🚨 DANGEROUS operation: {}\n\nReason: {}\n\n⚠️  This operation could have destructive impact. Proceed with extreme caution.",
                operation_type, classification.reason
            ),
        };

        Self {
            operation_id,
            operation_type,
            risk_level: classification.risk_level,
            classification,
            diff_preview,
            message,
        }
    }

    /// Display prompt to user (CLI format)
    pub fn display(&self) -> String {
        let mut display = String::new();

        display.push_str(&self.message);
        display.push_str("\n\n");

        if let Some(diff) = &self.diff_preview {
            display.push_str("=== Diff Preview ===\n");
            display.push_str(&diff.display());
            display.push_str("\n\n");
        }

        display.push_str("Proceed? [yes/no]: ");

        display
    }
}
```

### `src/safety/diff.rs`

```rust
use serde::{Deserialize, Serialize};

/// Diff change type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DiffChangeType {
    Added,
    Removed,
    Modified,
}

/// Diff entry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiffEntry {
    pub change_type: DiffChangeType,
    pub path: String,
    pub old_value: Option<String>,
    pub new_value: Option<String>,
}

impl DiffEntry {
    pub fn display(&self) -> String {
        match self.change_type {
            DiffChangeType::Added => {
                format!("+ {} -> {}", self.path, self.new_value.as_deref().unwrap_or(""))
            }
            DiffChangeType::Removed => {
                format!("- {} <- {}", self.path, self.old_value.as_deref().unwrap_or(""))
            }
            DiffChangeType::Modified => {
                format!(
                    "~ {}: {} -> {}",
                    self.path,
                    self.old_value.as_deref().unwrap_or(""),
                    self.new_value.as_deref().unwrap_or("")
                )
            }
        }
    }
}

/// Diff preview for staged changes
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiffPreview {
    pub entries: Vec<DiffEntry>,
    pub summary: String,
}

impl DiffPreview {
    pub fn new(entries: Vec<DiffEntry>) -> Self {
        let summary = format!(
            "{} changes ({} additions, {} removals, {} modifications)",
            entries.len(),
            entries.iter().filter(|e| e.change_type == DiffChangeType::Added).count(),
            entries.iter().filter(|e| e.change_type == DiffChangeType::Removed).count(),
            entries.iter().filter(|e| e.change_type == DiffChangeType::Modified).count(),
        );

        Self { entries, summary }
    }

    pub fn display(&self) -> String {
        let mut display = self.summary.clone();
        display.push_str("\n\n");

        for entry in &self.entries {
            display.push_str(&entry.display());
            display.push('\n');
        }

        display
    }

    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }
}

/// Stage for pending changes
pub struct Stage {
    pub changes: Vec<DiffEntry>,
    pub temp_dir: Option<std::path::PathBuf>,
}

impl Stage {
    pub fn new() -> Self {
        Self {
            changes: vec![],
            temp_dir: None,
        }
    }

    pub fn add_change(&mut self, change: DiffEntry) {
        self.changes.push(change);
    }

    pub fn create_preview(&self) -> DiffPreview {
        DiffPreview::new(self.changes.clone())
    }

    /// Apply staged changes (atomic commit)
    pub fn apply(&mut self) -> Result<(), String> {
        for change in &self.changes {
            match change.change_type {
                DiffChangeType::Added => {
                    if let Some(new_value) = &change.new_value {
                        std::fs::write(&change.path, new_value)
                            .map_err(|e| format!("Failed to write {}: {}", change.path, e))?;
                    }
                }
                DiffChangeType::Removed => {
                    std::fs::remove_file(&change.path)
                        .map_err(|e| format!("Failed to remove {}: {}", change.path, e))?;
                }
                DiffChangeType::Modified => {
                    if let Some(new_value) = &change.new_value {
                        std::fs::write(&change.path, new_value)
                            .map_err(|e| format!("Failed to write {}: {}", change.path, e))?;
                    }
                }
            }
        }

        self.changes.clear();
        Ok(())
    }

    /// Rollback staged changes
    pub fn rollback(&mut self) -> Result<(), String> {
        // Cleanup temp directory if exists
        if let Some(temp_dir) = &self.temp_dir {
            std::fs::remove_dir_all(temp_dir)
                .map_err(|e| format!("Failed to cleanup temp dir: {}", e))?;
        }

        self.changes.clear();
        Ok(())
    }
}

impl Default for Stage {
    fn default() -> Self {
        Self::new()
    }
}
```

### `src/safety/mod.rs`

```rust
pub mod classify;
pub mod diff;
pub mod prompt;

pub use classify::{OperationClassification, OperationClassifier, RiskLevel};
pub use diff::{DiffChangeType, DiffEntry, DiffPreview, Stage};
pub use prompt::ConfirmationPrompt;
```

---

## Implementation Steps

- [ ] **Step 1: Create test file**

- [ ] **Step 2: Implement classify.rs**

- [ ] **Step 3: Implement prompt.rs**

- [ ] **Step 4: Implement diff.rs**

- [ ] **Step 5: Implement mod.rs**

- [ ] **Step 6: Add safety to lib.rs**

- [ ] **Step 7: Run tests**

- [ ] **Step 8: Commit**

---

## Mock Strategy for Tests

Use in-memory stage (no actual file operations).
Mock user input for confirmation tests.
