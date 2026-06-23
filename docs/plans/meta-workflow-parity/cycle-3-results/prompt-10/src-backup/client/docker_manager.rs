//! Docker container lifecycle management via docker compose.

use anyhow::{Context, Result};
use std::process::Command;
use std::time::Duration;
use tokio::time::sleep;

/// Docker compose container manager.
pub struct DockerManager {
    project_name: String,
    compose_file: Option<String>,
}

impl DockerManager {
    /// Create a new Docker manager.
    pub fn new(project_name: impl Into<String>) -> Self {
        Self {
            project_name: project_name.into(),
            compose_file: None,
        }
    }

    /// Set a custom docker-compose file path.
    pub fn with_compose_file(mut self, path: impl Into<String>) -> Self {
        self.compose_file = Some(path.into());
        self
    }

    fn compose_cmd(&self) -> Command {
        let mut cmd = Command::new("docker");
        cmd.args(["compose"]);
        if let Some(ref f) = self.compose_file {
            cmd.args(["-f", f]);
        }
        cmd.arg("-p").arg(&self.project_name);
        cmd
    }

    /// Start containers (`docker compose up -d`).
    pub async fn start(&self) -> Result<()> {
        println!("Starting Docker container…");
        let output = self
            .compose_cmd()
            .args(["up", "-d"])
            .output()
            .context("Failed to run docker compose up")?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            anyhow::bail!("docker compose up failed: {}", stderr);
        }
        println!("Container started.");
        Ok(())
    }

    /// Stop containers (`docker compose down`).
    pub async fn stop(&self) -> Result<()> {
        println!("Stopping Docker container…");
        let output = self
            .compose_cmd()
            .args(["down"])
            .output()
            .context("Failed to run docker compose down")?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            anyhow::bail!("docker compose down failed: {}", stderr);
        }
        println!("Container stopped.");
        Ok(())
    }

    /// Restart containers.
    pub async fn restart(&self) -> Result<()> {
        println!("Restarting Docker container…");
        let output = self
            .compose_cmd()
            .args(["restart"])
            .output()
            .context("Failed to run docker compose restart")?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            anyhow::bail!("docker compose restart failed: {}", stderr);
        }
        println!("Container restarted.");
        Ok(())
    }

    /// Get container status.
    pub async fn status(&self) -> Result<String> {
        let output = self
            .compose_cmd()
            .args(["ps", "--format", "{{.State}}"])
            .output()
            .context("Failed to get container status")?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            anyhow::bail!("docker compose ps failed: {}", stderr);
        }

        Ok(String::from_utf8_lossy(&output.stdout).trim().into())
    }

    /// Wait until container reports healthy.
    pub async fn wait_for_healthy(&self, max_wait: Duration) -> Result<()> {
        println!("Waiting for container healthy…");
        let start = std::time::Instant::now();
        while start.elapsed() < max_wait {
            let s = self.status().await?;
            if s.contains("healthy") {
                println!("Container healthy.");
                return Ok(());
            }
            sleep(Duration::from_secs(2)).await;
        }
        anyhow::bail!("Container not healthy within {:?}", max_wait);
    }

    /// Print container logs.
    pub async fn logs(&self) -> Result<String> {
        let output = self
            .compose_cmd()
            .args(["logs"])
            .output()
            .context("Failed to get container logs")?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            anyhow::bail!("docker compose logs failed: {}", stderr);
        }

        Ok(String::from_utf8_lossy(&output.stdout).into())
    }
}
