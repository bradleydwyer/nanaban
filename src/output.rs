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

pub fn ext_for_mime(mime: &str) -> &'static str {
    if mime.contains("jpeg") || mime.contains("jpg") {
        "jpg"
    } else {
        "png"
    }
}

pub fn auto_filename(prompt: &str, mime: &str) -> PathBuf {
    let now = Local::now();
    let timestamp = now.format("%Y%m%d_%H%M%S");

    let mut hasher = DefaultHasher::new();
    prompt.hash(&mut hasher);
    let hash = format!("{:08x}", hasher.finish());
    let hash8 = &hash[..8];

    let ext = ext_for_mime(mime);
    PathBuf::from(format!("nanaban_{timestamp}_{hash8}.{ext}"))
}

pub fn save_image(base64_data: &str, path: &Path) -> Result<(u32, u32)> {
    let bytes = base64::engine::general_purpose::STANDARD.decode(base64_data)?;

    let img = image::load_from_memory(&bytes)?;
    let (w, h) = (img.width(), img.height());

    if let Some(parent) = path.parent()
        && !parent.as_os_str().is_empty()
    {
        std::fs::create_dir_all(parent)?;
    }

    let target_format = match path.extension().and_then(|e| e.to_str()) {
        Some("jpg" | "jpeg") => Some(image::ImageFormat::Jpeg),
        Some("png") => Some(image::ImageFormat::Png),
        Some("webp") => Some(image::ImageFormat::WebP),
        _ => None,
    };

    let source_is_jpeg = bytes.starts_with(&[0xFF, 0xD8, 0xFF]);
    let source_is_png = bytes.starts_with(&[0x89, 0x50, 0x4E, 0x47]);

    let needs_conversion = match target_format {
        Some(image::ImageFormat::Jpeg) => !source_is_jpeg,
        Some(image::ImageFormat::Png) => !source_is_png,
        Some(_) => true,
        None => false,
    };

    if needs_conversion {
        if let Some(fmt) = target_format {
            img.save_with_format(path, fmt)?;
        } else {
            std::fs::write(path, &bytes)?;
        }
    } else {
        std::fs::write(path, &bytes)?;
    }

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

/// Returns true if the terminal supports the Kitty graphics protocol (inline images).
pub fn supports_inline_images() -> bool {
    if let Ok(term) = std::env::var("TERM_PROGRAM") {
        return matches!(term.as_str(), "ghostty" | "kitty" | "WezTerm");
    }
    if std::env::var("GHOSTTY_RESOURCES_DIR").is_ok() {
        return true;
    }
    if let Ok(term) = std::env::var("TERM") {
        return term.contains("kitty") || term == "xterm-ghostty";
    }
    false
}

/// Display image inline in the terminal using Kitty graphics protocol.
pub fn display_inline(path: &Path) {
    let conf = viuer::Config {
        width: Some(80),
        absolute_offset: false,
        ..Default::default()
    };
    let _ = viuer::print_from_file(path, &conf);
}

/// Show the image: inline if supported, otherwise open in system viewer.
/// If force_open is true, also open in system viewer regardless.
/// If copy is true, copy to clipboard.
pub fn show_image(path: &Path, force_open: bool, copy: bool) {
    if supports_inline_images() {
        display_inline(path);
        if force_open {
            open_image(path);
        }
    } else {
        open_image(path);
    }
    if copy {
        copy_to_clipboard(path);
    }
}

pub fn copy_to_clipboard(path: &Path) {
    let abs = std::fs::canonicalize(path).unwrap_or(path.to_path_buf());
    let class = match path.extension().and_then(|e| e.to_str()) {
        Some("jpg" | "jpeg") => "JPEG",
        _ => "PNGf",
    };
    let script = format!(
        "set the clipboard to (read (POSIX file \"{}\") as «class {class}»)",
        abs.display()
    );
    let result = std::process::Command::new("osascript")
        .arg("-e")
        .arg(&script)
        .output();
    match result {
        Ok(output) if output.status.success() => {
            eprintln!("Copied to clipboard");
        }
        _ => {
            eprintln!("Warning: failed to copy to clipboard");
        }
    }
}

pub fn open_image(path: &Path) {
    let _ = std::process::Command::new("open")
        .arg(path)
        .stdout(std::process::Stdio::null())
        .stderr(std::process::Stdio::null())
        .spawn();
}
