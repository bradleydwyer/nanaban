use anyhow::Result;
use base64::Engine;
use chrono::Local;
use serde::Serialize;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::path::{Path, PathBuf};

#[derive(Serialize)]
pub struct ImageOutput {
    pub path: String,
    pub width: u32,
    pub height: u32,
}

#[derive(Serialize)]
pub struct SuccessOutput {
    pub status: &'static str,
    pub images: Vec<ImageOutput>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    pub model: String,
    pub model_short: String,
    pub elapsed_seconds: f64,
    pub estimated_cost_usd: f64,
}

#[derive(Serialize)]
pub struct ErrorOutput {
    pub status: &'static str,
    pub error: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model_short: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub elapsed_seconds: Option<f64>,
}

pub fn auto_filename(prompt: &str) -> PathBuf {
    let now = Local::now();
    let timestamp = now.format("%Y%m%d_%H%M%S");

    let mut hasher = DefaultHasher::new();
    prompt.hash(&mut hasher);
    let hash = format!("{:08x}", hasher.finish());
    let hash8 = &hash[..8];

    PathBuf::from(format!("nanaban_{timestamp}_{hash8}.png"))
}

pub fn save_image(base64_data: &str, path: &Path) -> Result<(u32, u32)> {
    let bytes = base64::engine::general_purpose::STANDARD.decode(base64_data)?;

    // Validate it's actually an image and get dimensions
    let img = image::load_from_memory(&bytes)?;
    let (w, h) = (img.width(), img.height());

    if let Some(parent) = path.parent()
        && !parent.as_os_str().is_empty()
    {
        std::fs::create_dir_all(parent)?;
    }

    std::fs::write(path, &bytes)?;
    Ok((w, h))
}

pub fn emit_success_json(output: &SuccessOutput) {
    if let Ok(json) = serde_json::to_string(output) {
        println!("{json}");
    }
}

pub fn emit_error_json(output: &ErrorOutput) {
    if let Ok(json) = serde_json::to_string(output) {
        println!("{json}");
    }
}

pub fn open_image(path: &Path) {
    let _ = std::process::Command::new("open")
        .arg(path)
        .stdout(std::process::Stdio::null())
        .stderr(std::process::Stdio::null())
        .spawn();
}
