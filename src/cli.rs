use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser)]
#[command(
    name = "nanaban",
    version,
    about = "Gemini image generation CLI (Nano Banana 2 / Pro)"
)]
pub struct Cli {
    /// Increase verbosity (-v, -vv, -vvv)
    #[arg(short, long, action = clap::ArgAction::Count, global = true)]
    pub verbose: u8,

    /// Output structured JSON to stdout
    #[arg(long, global = true)]
    pub json: bool,

    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Generate an image from a text prompt
    Generate(GenerateArgs),
    /// Edit an image with a text prompt
    Edit(EditArgs),
    /// List available models and costs
    Models,
}

#[derive(Parser)]
pub struct GenerateArgs {
    /// Text prompt (positional)
    pub prompt: Option<String>,

    /// Text prompt (alternative to positional, use @file.txt to read from file)
    #[arg(short, long)]
    pub prompt_flag: Option<String>,

    /// Model to use
    #[arg(short, long, default_value = "flash")]
    pub model: String,

    /// Output file path
    #[arg(short, long)]
    pub output: Option<PathBuf>,

    /// Aspect ratio (e.g. 1:1, 16:9, 9:16)
    #[arg(short, long)]
    pub aspect: Option<String>,

    /// Resolution: 512, 1K, 2K, 4K
    #[arg(short, long, default_value = "1K")]
    pub size: String,

    /// Reference image(s), repeatable
    #[arg(short, long = "ref")]
    pub refs: Vec<PathBuf>,

    /// Show cost estimate without calling the API
    #[arg(long)]
    pub dry_run: bool,

    /// Also open image in system viewer (Preview on macOS)
    #[arg(long)]
    pub open: bool,

    /// Copy image to clipboard
    #[arg(long)]
    pub copy: bool,
}

#[derive(Parser)]
pub struct EditArgs {
    /// Input image to edit
    pub image: PathBuf,

    /// Edit instructions (positional)
    pub prompt: Option<String>,

    /// Edit instructions (alternative to positional, use @file.txt to read from file)
    #[arg(short, long)]
    pub prompt_flag: Option<String>,

    /// Model to use
    #[arg(short, long, default_value = "flash")]
    pub model: String,

    /// Output file path
    #[arg(short, long)]
    pub output: Option<PathBuf>,

    /// Aspect ratio
    #[arg(short, long)]
    pub aspect: Option<String>,

    /// Resolution: 512, 1K, 2K, 4K
    #[arg(short, long)]
    pub size: Option<String>,

    /// Additional reference image(s), repeatable
    #[arg(short, long = "ref")]
    pub refs: Vec<PathBuf>,

    /// Show cost estimate without calling the API
    #[arg(long)]
    pub dry_run: bool,

    /// Also open image in system viewer (Preview on macOS)
    #[arg(long)]
    pub open: bool,

    /// Copy image to clipboard
    #[arg(long)]
    pub copy: bool,
}

const VALID_ASPECTS: &[&str] = &[
    "1:1", "16:9", "9:16", "4:3", "3:4", "3:2", "2:3", "4:5", "5:4", "21:9",
];

const VALID_SIZES: &[&str] = &["512", "1K", "2K", "4K"];

pub fn validate_aspect(aspect: &str) -> anyhow::Result<()> {
    if !VALID_ASPECTS.contains(&aspect) {
        anyhow::bail!(
            "Invalid aspect ratio '{}'. Valid options: {}",
            aspect,
            VALID_ASPECTS.join(", ")
        );
    }
    Ok(())
}

pub fn validate_size(size: &str) -> anyhow::Result<()> {
    if !VALID_SIZES.contains(&size) {
        anyhow::bail!(
            "Invalid size '{}'. Valid options: {}",
            size,
            VALID_SIZES.join(", ")
        );
    }
    Ok(())
}

/// Resolve prompt from flag, positional arg, or stdin.
pub fn resolve_prompt(
    positional: &Option<String>,
    flag: &Option<String>,
) -> anyhow::Result<String> {
    // Flag takes priority
    if let Some(p) = flag {
        if positional.is_some() {
            anyhow::bail!(
                "Cannot provide both a positional prompt and --prompt-flag. Use one or the other."
            );
        }
        // @file syntax
        if let Some(path) = p.strip_prefix('@') {
            let content = std::fs::read_to_string(path)
                .map_err(|e| anyhow::anyhow!("Failed to read prompt file '{}': {}", path, e))?;
            return Ok(content.trim().to_string());
        }
        return Ok(p.clone());
    }

    if let Some(p) = positional {
        return Ok(p.clone());
    }

    // Try stdin
    if !atty::is(atty::Stream::Stdin) {
        let mut input = String::new();
        std::io::Read::read_to_string(&mut std::io::stdin(), &mut input)?;
        let trimmed = input.trim().to_string();
        if trimmed.is_empty() {
            anyhow::bail!("Empty prompt from stdin.");
        }
        return Ok(trimmed);
    }

    anyhow::bail!(
        "No prompt provided. Usage:\n  \
         nanaban generate \"your prompt\"\n  \
         nanaban generate -p \"your prompt\"\n  \
         nanaban generate -p @prompt.txt\n  \
         echo \"your prompt\" | nanaban generate"
    );
}
