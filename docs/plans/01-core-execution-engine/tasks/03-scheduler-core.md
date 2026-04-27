# Task 03: Scheduler Core

**Goal:** Implement scheduler core with worker pool management, job prioritization, execution loop, cancellation support, and retry with exponential backoff.

**Files:**
- Create: `src/scheduler/mod.rs`
- Create: `src/scheduler/core.rs`
- Create: `src/scheduler/worker.rs`
- Create: `src/scheduler/cancel.rs`
- Create: `tests/unit/scheduler_test.rs`

---

## Rust Definitions

### `src/scheduler/core.rs`

```rust
use crate::chat_session::SessionManager;
use crate::executor::step::StepExecutor;
use crate::queue::{Job, JobId, Priority, QueueStorage, QueueState, QueueEvent};
use async_trait::async_trait;
use tokio::sync::{mpsc, oneshot, RwLock};
use tokio::task::JoinHandle;
use tokio::time::{sleep, Duration, Instant};
use std::collections::BinaryHeap;
use std::sync::Arc;
use uuid::Uuid;

/// Job priority wrapper for BinaryHeap
#[derive(Debug, Eq, PartialEq)]
struct PriorityJob {
    job: Arc<RwLock<Job>>,
}

impl Ord for PriorityJob {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        // Reverse: higher priority comes first
        let self_job = self.job.read().await;
        let other_job = other.job.read().await;
        other_job.priority.cmp(&self_job.priority)
            .then_with(|| self_job.stats.created_at.cmp(&other_job.stats.created_at))
    }
}

impl PartialOrd for PriorityJob {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

/// Scheduler configuration
#[derive(Debug, Clone)]
pub struct SchedulerConfig {
    pub max_workers: usize,
    pub job_timeout_secs: u64,
    pub max_retries: u32,
    pub retry_backoff_base_ms: u64,
    pub retry_backoff_max_ms: u64,
    pub queue_check_interval_ms: u64,
}

impl Default for SchedulerConfig {
    fn default() -> Self {
        Self {
            max_workers: 4,
            job_timeout_secs: 3600, // 1 hour
            max_retries: 3,
            retry_backoff_base_ms: 1000,
            retry_backoff_max_ms: 60000,
            queue_check_interval_ms: 1000,
        }
    }
}

/// Scheduler commands
#[derive(Debug)]
pub enum SchedulerCommand {
    SubmitJob { job: Job, response_tx: oneshot::Sender<Result<JobId, String>> },
    CancelJob { job_id: JobId, response_tx: oneshot::Sender<Result<(), String>> },
    PauseJob { job_id: JobId, response_tx: oneshot::Sender<Result<(), String>> },
    ResumeJob { job_id: JobId, response_tx: oneshot::Sender<Result<(), String>> },
    GetJobStatus { job_id: JobId, response_tx: oneshot::Sender<Result<Job, String>> },
    Shutdown,
}

/// Scheduler state
pub struct Scheduler {
    storage: Arc<QueueStorage>,
    session_manager: Arc<SessionManager<QueueStorage>>,
    config: SchedulerConfig,
    command_rx: mpsc::UnboundedReceiver<SchedulerCommand>,
    command_tx: mpsc::UnboundedSender<SchedulerCommand>,

    // Job queues
    pending_queue: Arc<RwLock<BinaryHeap<PriorityJob>>>,
    running_jobs: Arc<RwLock<std::collections::HashMap<JobId, Arc<RwLock<Job>>>>>,

    // Worker management
    workers: Vec<JoinHandle<()>>,
    job_tx: mpsc::UnboundedSender<JobAssignment>,

    // Cancellation
    cancellation_rx: mpsc::UnboundedReceiver<CancelRequest>,

    // Shutdown
    shutdown_rx: oneshot::Receiver<()>,
    shutdown_tx: Option<oneshot::Sender<()>>,
}

/// Job assignment to worker
#[derive(Debug)]
pub struct JobAssignment {
    pub job: Arc<RwLock<Job>>,
    pub step_executor: Arc<dyn StepExecutor>,
}

/// Cancel request
#[derive(Debug)]
pub struct CancelRequest {
    pub job_id: JobId,
    pub reason: String,
}

/// Result type for scheduler operations
pub type SchedulerResult<T> = Result<T, String>;

impl Scheduler {
    pub fn new(
        storage: Arc<QueueStorage>,
        session_manager: Arc<SessionManager<QueueStorage>>,
        config: SchedulerConfig,
    ) -> Self {
        let (command_tx, command_rx) = mpsc::unbounded_channel();
        let (job_tx, job_rx) = mpsc::unbounded_channel();
        let (cancellation_tx, cancellation_rx) = mpsc::unbounded_channel();
        let (shutdown_tx, shutdown_rx) = oneshot::channel();

        let mut scheduler = Self {
            storage: storage.clone(),
            session_manager: session_manager.clone(),
            config,
            command_rx,
            command_tx: command_tx.clone(),
            pending_queue: Arc::new(RwLock::new(BinaryHeap::new())),
            running_jobs: Arc::new(RwLock::new(std::collections::HashMap::new())),
            workers: Vec::new(),
            job_tx,
            cancellation_rx,
            shutdown_rx,
            shutdown_tx: Some(shutdown_tx),
        };

        // Spawn worker pool
        for worker_id in 0..scheduler.config.max_workers {
            scheduler.spawn_worker(worker_id, job_rx.clone(), cancellation_tx.clone());
        }

        scheduler
    }

    /// Get command sender for external interaction
    pub fn command_sender(&self) -> mpsc::UnboundedSender<SchedulerCommand> {
        self.command_tx.clone()
    }

    /// Spawn a worker task
    fn spawn_worker(
        &mut self,
        worker_id: usize,
        mut job_rx: mpsc::UnboundedReceiver<JobAssignment>,
        cancellation_tx: mpsc::UnboundedSender<CancelRequest>,
    ) {
        let storage = self.storage.clone();
        let config = self.config.clone();
        let running_jobs = self.running_jobs.clone();

        let handle = tokio::spawn(async move {
            tracing::info!("Worker {} started", worker_id);

            while let Some(assignment) = job_rx.recv().await {
                let job_id = assignment.job.read().await.id.clone();

                tracing::info!("Worker {} assigned job {}", worker_id, job_id.as_str());

                // Execute job with timeout
                let result = tokio::time::timeout(
                    Duration::from_secs(config.job_timeout_secs),
                    execute_job(assignment, storage.clone(), cancellation_tx.clone()),
                )
                .await;

                match result {
                    Ok(Ok(())) => {
                        tracing::info!("Job {} completed successfully", job_id.as_str());
                    }
                    Ok(Err(e)) => {
                        tracing::error!("Job {} failed: {}", job_id.as_str(), e);
                    }
                    Err(_) => {
                        tracing::warn!("Job {} timed out", job_id.as_str());

                        // Mark job as timed out
                        let mut job_opt = running_jobs.write().await.remove(&job_id);
                        if let Some(job) = job_opt {
                            let mut job_write = job.write().await;
                            let _ = job_write.transition(QueueEvent::Timeout {
                                job_id: job_id.as_str().to_string(),
                                elapsed_secs: config.job_timeout_secs,
                            });
                            let _ = storage.save_job(&job_write);
                        }
                    }
                }

                // Remove from running jobs
                running_jobs.write().await.remove(&job_id);
            }

            tracing::info!("Worker {} stopped", worker_id);
        });

        self.workers.push(handle);
    }

    /// Run scheduler main loop
    pub async fn run(mut self) {
        tracing::info!("Scheduler started with {} workers", self.config.max_workers);

        // Recovery: check for running jobs
        self.recover_running_jobs().await;

        let mut check_interval = tokio::time::interval(Duration::from_millis(self.config.queue_check_interval_ms));

        loop {
            tokio::select! {
                // Handle commands
                Some(command) = self.command_rx.recv() => {
                    if !self.handle_command(command).await {
                        break;
                    }
                }

                // Check for pending jobs
                _ = check_interval.tick() => {
                    self.assign_jobs().await;
                }

                // Handle shutdown
                _ = &mut self.shutdown_rx => {
                    tracing::info!("Scheduler received shutdown signal");
                    self.shutdown().await;
                    break;
                }
            }
        }

        tracing::info!("Scheduler stopped");
    }

    /// Recover jobs that were running at shutdown
    async fn recover_running_jobs(&self) {
        match self.storage.recover_running_jobs() {
            Ok(jobs) => {
                tracing::info!("Found {} running jobs to recover", jobs.len());

                for job in jobs {
                    tracing::info!("Recovering job {}", job.id.as_str());

                    // Reset to pending for re-execution
                    let mut job = job;
                    job.state = QueueState::Pending;
                    job.worker_id = None;

                    if let Err(e) = self.storage.save_job(&job) {
                        tracing::error!("Failed to save recovered job {}: {}", job.id.as_str(), e);
                    }
                }
            }
            Err(e) => {
                tracing::error!("Failed to recover running jobs: {}", e);
            }
        }
    }

    /// Handle scheduler command
    async fn handle_command(&mut self, command: SchedulerCommand) -> bool {
        match command {
            SchedulerCommand::SubmitJob { job, response_tx } => {
                let result = self.submit_job(job).await;
                let _ = response_tx.send(result);
                true
            }
            SchedulerCommand::CancelJob { job_id, response_tx } => {
                let result = self.cancel_job(job_id, "User requested cancellation".to_string()).await;
                let _ = response_tx.send(result);
                true
            }
            SchedulerCommand::PauseJob { job_id, response_tx } => {
                let result = self.pause_job(job_id).await;
                let _ = response_tx.send(result);
                true
            }
            SchedulerCommand::ResumeJob { job_id, response_tx } => {
                let result = self.resume_job(job_id).await;
                let _ = response_tx.send(result);
                true
            }
            SchedulerCommand::GetJobStatus { job_id, response_tx } => {
                let result = self.get_job_status(job_id).await;
                let _ = response_tx.send(result);
                true
            }
            SchedulerCommand::Shutdown => {
                false
            }
        }
    }

    /// Submit job to queue
    async fn submit_job(&self, mut job: Job) -> SchedulerResult<JobId> {
        job.state = QueueState::Pending;

        let notification = job.transition(QueueEvent::Created {
            job_id: job.id.as_str().to_string(),
        })
        .map_err(|e| e.to_string())?;

        self.storage.save_job(&job).map_err(|e| e.to_string())?;

        tracing::info!("Job {} submitted: {:?}", job.id.as_str(), notification);

        Ok(job.id)
    }

    /// Assign pending jobs to workers
    async fn assign_jobs(&self) {
        let running_count = self.running_jobs.read().await.len();

        if running_count >= self.config.max_workers {
            return;
        }

        let mut pending_queue = self.pending_queue.write().await;

        while pending_queue.len() > 0 && self.running_jobs.read().await.len() < self.config.max_workers {
            if let Some(priority_job) = pending_queue.pop() {
                let job_id = priority_job.job.read().await.id.clone();

                // Transition to Scheduled
                let mut job = priority_job.job.write().await;
                let _ = job.transition(QueueEvent::Scheduled {
                    job_id: job.id.as_str().to_string(),
                    worker_id: format!("worker-{}", self.running_jobs.read().await.len()),
                });

                // Assign to worker
                let assignment = JobAssignment {
                    job: Arc::clone(&priority_job.job),
                    // TODO: Create step executor in Task 04
                    step_executor: todo!(),
                };

                let _ = self.job_tx.send(assignment);

                // Add to running jobs
                self.running_jobs.write().await.insert(job_id.clone(), Arc::clone(&priority_job.job));

                // Save job state
                let _ = self.storage.save_job(&job);
            }
        }
    }

    /// Cancel a job
    async fn cancel_job(&self, job_id: JobId, reason: String) -> SchedulerResult<()> {
        // Check if job is running
        if let Some(job) = self.running_jobs.write().await.remove(&job_id) {
            // Mark as cancelled
            let mut job_write = job.write().await;
            let _ = job_write.transition(QueueEvent::Cancelled {
                job_id: job_id.as_str().to_string(),
                reason: reason.clone(),
            });
            let _ = self.storage.save_job(&job_write);

            // Send cancel request to worker
            let _ = self.cancellation_rx.send(CancelRequest { job_id, reason });

            Ok(())
        } else {
            // Job not running, check storage
            if let Ok(Some(mut job)) = self.storage.load_job(&job_id) {
                if job.state.is_active() {
                    let _ = job.transition(QueueEvent::Cancelled {
                        job_id: job_id.as_str().to_string(),
                        reason,
                    });
                    let _ = self.storage.save_job(&job);
                    Ok(())
                } else {
                    Err(format!("Job {} is in terminal state {:?}", job_id.as_str(), job.state))
                }
            } else {
                Err(format!("Job {} not found", job_id.as_str()))
            }
        }
    }

    /// Pause a job
    async fn pause_job(&self, job_id: JobId) -> SchedulerResult<()> {
        if let Some(job) = self.running_jobs.read().await.get(&job_id) {
            let mut job_write = job.write().await;
            let _ = job_write.transition(QueueEvent::Paused {
                job_id: job_id.as_str().to_string(),
                reason: "User requested pause".to_string(),
            });
            let _ = self.storage.save_job(&job_write);
            Ok(())
        } else {
            Err(format!("Job {} is not running", job_id.as_str()))
        }
    }

    /// Resume a paused job
    async fn resume_job(&self, job_id: JobId) -> SchedulerResult<()> {
        if let Ok(Some(mut job)) = self.storage.load_job(&job_id) {
            if job.state == QueueState::Paused {
                let _ = job.transition(QueueEvent::Resumed {
                    job_id: job_id.as_str().to_string(),
                });

                // Re-add to pending queue
                let priority_job = PriorityJob {
                    job: Arc::new(RwLock::new(job.clone())),
                };

                self.pending_queue.write().await.push(priority_job);
                let _ = self.storage.save_job(&job);

                Ok(())
            } else {
                Err(format!("Job {} is not paused (state: {:?})", job_id.as_str(), job.state))
            }
        } else {
            Err(format!("Job {} not found", job_id.as_str()))
        }
    }

    /// Get job status
    async fn get_job_status(&self, job_id: JobId) -> SchedulerResult<Job> {
        self.storage
            .load_job(&job_id)
            .map_err(|e| e.to_string())?
            .ok_or_else(|| format!("Job {} not found", job_id.as_str()))
    }

    /// Graceful shutdown
    async fn shutdown(&mut self) {
        tracing::info!("Shutting down scheduler...");

        // Wait for running jobs to complete or timeout
        let timeout = Duration::from_secs(30);
        let start = Instant::now();

        while self.running_jobs.read().await.len() > 0 && start.elapsed() < timeout {
            tokio::time::sleep(Duration::from_millis(100)).await;
        }

        // Abort remaining workers
        for worker in &mut self.workers {
            worker.abort();
        }

        // Flush storage
        let _ = self.storage.flush();

        tracing::info!("Scheduler shutdown complete");
    }
}

/// Execute a job (worker task)
async fn execute_job(
    assignment: JobAssignment,
    storage: Arc<QueueStorage>,
    cancellation_tx: mpsc::UnboundedSender<CancelRequest>,
) -> Result<(), String> {
    let job_id = assignment.job.read().await.id.clone();

    // Transition to Running
    {
        let mut job = assignment.job.write().await;
        let _ = job.transition(QueueEvent::Started {
            job_id: job_id.as_str().to_string(),
        });
        let _ = storage.save_job(&job);
    }

    // TODO: Implement step execution in Task 04-07
    // This is a placeholder that will be implemented later
    tracing::info!("Executing job {}", job_id.as_str());

    Ok(())
}
```

### `src/scheduler/mod.rs`

```rust
pub mod cancel;
pub mod core;
pub mod worker;

pub use core::{Scheduler, SchedulerCommand, SchedulerConfig};
```

### `src/scheduler/cancel.rs`

```rust
use crate::queue::JobId;

/// Cancel request (placeholder, will be expanded)
#[derive(Debug, Clone)]
pub struct CancelRequest {
    pub job_id: JobId,
    pub reason: String,
}

### `src/scheduler/worker.rs`

```rust
/// Worker task (placeholder, will be expanded)
pub struct Worker {
    pub id: usize,
}

```

---

## Implementation Steps

- [ ] **Step 1: Create test file with failing tests**

```rust
// tests/unit/scheduler_test.rs
use agentsdk::chat_session::{SessionManager, SessionStorage};
use agentsdk::queue::{Job, JobId, Priority, QueueStorage, QueueState, Scheduler, SchedulerCommand, SchedulerConfig};
use std::collections::HashMap;
use tokio::sync::oneshot;

#[tokio::test]
async fn test_scheduler_creation() {
    let storage = Arc::new(QueueStorage::open_in_memory().unwrap());
    let session_manager = Arc::new(SessionManager::new(storage.clone()));
    let config = SchedulerConfig::default();

    let scheduler = Scheduler::new(storage, session_manager, config);

    assert_eq!(scheduler.config.max_workers, 4);
}

#[tokio::test]
async fn test_job_submission() {
    let storage = Arc::new(QueueStorage::open_in_memory().unwrap());
    let session_manager = Arc::new(SessionManager::new(storage.clone()));
    let config = SchedulerConfig::default();

    let scheduler = Scheduler::new(storage.clone(), session_manager, config);
    let command_tx = scheduler.command_sender();

    let session_id = agentsdk::chat_session::SessionId::new();
    let job = Job::new(
        session_id,
        "workflow-123".to_string(),
        HashMap::new(),
        5,
    );

    let (response_tx, response_rx) = oneshot::channel();
    let _ = command_tx.send(SchedulerCommand::SubmitJob { job, response_tx });

    let job_id = response_rx.await.unwrap().unwrap();

    let (response_tx, response_rx) = oneshot::channel();
    let _ = command_tx.send(SchedulerCommand::GetJobStatus { job_id, response_tx });

    let status = response_rx.await.unwrap().unwrap();

    assert_eq!(status.state, QueueState::Pending);
}

// More tests will be added after StepExecutor is implemented
```

- [ ] **Step 2: Run tests to verify they fail**

Run: `cargo test scheduler --lib`
Expected: FAIL with compilation errors (scheduler module doesn't exist yet)

- [ ] **Step 3: Implement core.rs**

```rust
// src/scheduler/core.rs (complete code from above)
```

- [ ] **Step 4: Implement mod.rs**

```rust
// src/scheduler/mod.rs (complete code from above)
```

- [ ] **Step 5: Implement placeholder cancel.rs**

```rust
// src/scheduler/cancel.rs (complete code from above)
```

- [ ] **Step 6: Implement placeholder worker.rs**

```rust
// src/scheduler/worker.rs (complete code from above)
```

- [ ] **Step 7: Add scheduler module to lib.rs**

```rust
// src/lib.rs
pub mod chat_session;
pub mod queue;
pub mod scheduler;
```

- [ ] **Step 8: Run tests to verify they pass**

Run: `cargo test scheduler --lib`
Expected: PASS for basic tests (step execution tests will fail until Task 04)

- [ ] **Step 9: Commit**

```bash
git add src/scheduler/ tests/unit/scheduler_test.rs
git commit -m "feat: implement scheduler core with worker pool and job prioritization"
```

---

## Mock Strategy for Tests

Use in-memory QueueStorage for scheduler tests.
Mock StepExecutor with a placeholder that returns success:

```rust
struct MockStepExecutor;
#[async_trait::async_trait]
impl StepExecutor for MockStepExecutor {
    async fn execute_step(&self, _step: &Step, _context: &ExecutionContext) -> Result<StepOutput, String> {
        Ok(StepOutput {
            success: true,
            data: serde_json::json!({}),
            logs: vec![],
        })
    }
}
```

Full integration with StepExecutor will be completed in Task 04.

---

## QA Cross-References

- **QA Criteria**: [QA-01-04](../../qa/phase-01/QA-CRITERIA.md)
- **Test Cases**: [P01-007](../../qa/phase-01/QA-TEST-CASES.md), [P01-008](../../qa/phase-01/QA-TEST-CASES.md), [P01-009](../../qa/phase-01/QA-TEST-CASES.md)
- **Schema Ref**: N/A (execution engine component)
