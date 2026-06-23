mod types;
mod registry;
pub mod builtin;

pub use types::*;
pub use registry::*;

use crate::error::Result;
use async_trait::async_trait;

/// Core verifier trait for artifact verification
#[async_trait]
pub trait Verifier: Send + Sync {
    /// Get verifier capabilities
    fn capabilities(&self) -> &VerifierCapabilities;

    /// Get verifier name
    fn name(&self) -> &str;

    /// Verify an artifact
    async fn verify(&self, artifact: &Artifact) -> Result<VerificationResult>;

    /// Check if verifier can handle this artifact
    fn can_verify(&self, artifact: &Artifact) -> bool {
        self.capabilities().can_handle(artifact.artifact_type)
    }

    /// Optional: Warm-up verifier for performance
    async fn warmup(&self) -> Result<()> {
        Ok(())
    }

    /// Optional: Cleanup verifier resources
    async fn cleanup(&self) -> Result<()> {
        Ok(())
    }
}
