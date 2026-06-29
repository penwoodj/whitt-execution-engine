use crate::error::{Error, Result};
use super::types::ArtifactType;
use super::Verifier;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

/// Registry for managing verifiers
pub struct VerifierRegistry {
    verifiers: RwLock<HashMap<String, Arc<dyn Verifier>>>,
    by_artifact_type: RwLock<HashMap<ArtifactType, Vec<String>>>,
}

impl VerifierRegistry {
    pub fn new() -> Self {
        Self {
            verifiers: RwLock::new(HashMap::new()),
            by_artifact_type: RwLock::new(HashMap::new()),
        }
    }

    /// Register a verifier
    pub async fn register(&self, verifier: Arc<dyn Verifier>) -> Result<()> {
        let name = verifier.name().to_string();
        let artifact_types = verifier.capabilities().artifact_types.clone();

        // Store verifier
        {
            let mut verifiers = self.verifiers.write().await;
            verifiers.insert(name.clone(), verifier);
        }

        // Index by artifact type
        {
            let mut by_type = self.by_artifact_type.write().await;
            for artifact_type in &artifact_types {
                by_type
                    .entry(*artifact_type)
                    .or_insert_with(Vec::new)
                    .push(name.clone());
            }
        }

        Ok(())
    }

    /// Get a verifier by name
    pub async fn get(&self, name: &str) -> Result<Arc<dyn Verifier>> {
        let verifiers = self.verifiers.read().await;
        verifiers
            .get(name)
            .cloned()
            .ok_or_else(|| Error::verification(format!("Verifier not found: {}", name)))
    }

    /// Get all verifiers for an artifact type
    pub async fn get_for_artifact_type(
        &self,
        artifact_type: ArtifactType,
    ) -> Result<Vec<Arc<dyn Verifier>>> {
        let by_type = self.by_artifact_type.read().await;
        let verifiers = self.verifiers.read().await;

        let names = by_type
            .get(&artifact_type)
            .ok_or_else(|| Error::verification(format!("No verifiers for artifact type: {:?}", artifact_type)))?;

        Ok(names
            .iter()
            .filter_map(|name| verifiers.get(name).cloned())
            .collect())
    }

    /// List all registered verifiers
    pub async fn list(&self) -> Vec<String> {
        let verifiers = self.verifiers.read().await;
        verifiers.keys().cloned().collect()
    }
}

impl Default for VerifierRegistry {
    fn default() -> Self {
        Self::new()
    }
}
