use anyhow::{Context, Result};
use futures::StreamExt;
use reqwest::Client;
use std::path::Path;
use std::time::Duration;

/// Base URL for HuggingFace model hub. Can be overridden for mirrors/enterprise registries.
const HF_BASE_URL: &str = "https://huggingface.co";

pub async fn download_model_from_hf(
    repo: &str,
    filename: &str,
    dest_path: &str,
) -> Result<u64> {
    let url = format!("{}/{}/resolve/main/{}", HF_BASE_URL, repo, filename);

    let client = Client::builder()
        .timeout(Duration::from_secs(600))
        .build()
        .context("Failed to create HTTP client")?;

    let response = client
        .get(&url)
        .send()
        .await
        .context("Failed to start download")?;

    if !response.status().is_success() {
        anyhow::bail!(
            "Download failed: {} - {}",
            response.status(),
            response.text().await.unwrap_or_default()
        );
    }

    let total_bytes = response.content_length().unwrap_or(0);
    let dest = Path::new(dest_path);
    if let Some(parent) = dest.parent() {
        tokio::fs::create_dir_all(parent)
            .await
            .context("Failed to create destination directory")?;
    }

    let mut file = tokio::fs::File::create(dest_path)
        .await
        .context("Failed to create destination file")?;

    let mut downloaded = 0u64;
    let mut stream = response.bytes_stream();

    let mut last_log_bytes = 0u64;
    let log_interval = 10 * 1024 * 1024;

    while let Some(chunk_result) = stream.next().await {
        let chunk = chunk_result.context("Failed to read download chunk")?;
        tokio::io::AsyncWriteExt::write_all(&mut file, &chunk)
            .await
            .context("Failed to write chunk")?;

        downloaded += chunk.len() as u64;

        if downloaded - last_log_bytes >= log_interval {
            let percent = if total_bytes > 0 {
                (downloaded as f64 / total_bytes as f64 * 100.0) as u64
            } else {
                0
            };
            tracing::info!(
                "Download progress: {}MB / {}MB ({}%)",
                downloaded / 1024 / 1024,
                total_bytes / 1024 / 1024,
                percent
            );
            last_log_bytes = downloaded;
        }
    }

    tracing::info!("Download complete: {} bytes", downloaded);
    Ok(downloaded)
}
