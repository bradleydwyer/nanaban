mod cli;
mod client;
mod config;
mod models;
mod output;

use anyhow::Result;
use clap::Parser;
use cli::{Cli, Commands};
use std::path::Path;
use std::process;

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    let result = match cli.command {
        Commands::Models => {
            models::print_table();
            Ok(())
        }
        Commands::Generate(args) => run_generate(args, cli.json, cli.verbose).await,
        Commands::Edit(args) => run_edit(args, cli.json, cli.verbose).await,
    };

    if let Err(e) = result {
        if cli.json {
            output::emit_error_json(&output::ErrorOutput {
                status: "error",
                error: e.to_string(),
                model: None,
                model_short: None,
                elapsed_seconds: None,
            });
        } else {
            eprintln!("Error: {e}");
        }
        let msg = e.to_string();
        if msg.contains("safety filter") {
            process::exit(3);
        } else if msg.contains("API")
            || msg.contains("Rate limited")
            || msg.contains("Authentication")
        {
            process::exit(2);
        } else {
            process::exit(1);
        }
    }
}

async fn run_generate(args: cli::GenerateArgs, json_mode: bool, verbose: u8) -> Result<()> {
    let prompt = cli::resolve_prompt(&args.prompt, &args.prompt_flag)?;
    let model_info = models::lookup(&args.model).ok_or_else(|| {
        anyhow::anyhow!(
            "Unknown model '{}'. Run `nanaban models` to see options.",
            args.model
        )
    })?;

    if let Some(ref aspect) = args.aspect {
        cli::validate_aspect(aspect)?;
    }
    cli::validate_size(&args.size)?;

    for r in &args.refs {
        if !r.exists() {
            anyhow::bail!("Reference image not found: {}", r.display());
        }
    }
    if args.refs.len() > 14 {
        anyhow::bail!(
            "Maximum 14 reference images allowed, got {}.",
            args.refs.len()
        );
    }

    if args.dry_run {
        if json_mode {
            println!(
                "{}",
                serde_json::json!({
                    "status": "dry_run",
                    "model": model_info.api_id,
                    "model_short": model_info.short_name,
                    "estimated_cost_usd": model_info.cost_per_image,
                })
            );
        } else {
            eprintln!("Model: {} ({})", model_info.short_name, model_info.api_id);
            eprintln!("Estimated cost: ~${:.2}", model_info.cost_per_image);
        }
        return Ok(());
    }

    let config = config::Config::load()?;
    let client = client::GeminiClient::new(config.api_key);

    if verbose > 0 {
        eprintln!("Model: {} ({})", model_info.short_name, model_info.api_id);
        eprintln!("Prompt: {}", &prompt[..prompt.len().min(100)]);
    }

    let ref_paths: Vec<&Path> = args.refs.iter().map(|p| p.as_path()).collect();

    if verbose > 0 {
        eprintln!("Sending request...");
    }

    let result = client
        .generate(
            &prompt,
            model_info.api_id,
            model_info.api_type,
            &ref_paths,
            args.aspect.as_deref(),
            &args.size,
        )
        .await?;

    let first_mime = result
        .images
        .first()
        .map(|i| i.mime_type.as_str())
        .unwrap_or("image/png");
    let output_path = args
        .output
        .clone()
        .unwrap_or_else(|| output::auto_filename(&prompt, first_mime));

    let image_outputs = save_result_images(&result, &output_path, verbose)?;

    eprintln!(
        "Estimated cost: ~${:.2} ({})",
        model_info.cost_per_image, model_info.short_name
    );

    if json_mode {
        output::emit_success_json(&output::SuccessOutput {
            status: "success",
            images: image_outputs,
            text: result.text,
            model: model_info.api_id.to_string(),
            model_short: model_info.short_name.to_string(),
            elapsed_seconds: result.elapsed_seconds,
            estimated_cost_usd: model_info.cost_per_image,
        });
    } else {
        let abs = std::fs::canonicalize(&output_path).unwrap_or(output_path.clone());
        println!("{}", abs.display());

        if output_path.exists() {
            output::show_image(&output_path, args.open, args.copy);
        }
    }

    Ok(())
}

async fn run_edit(args: cli::EditArgs, json_mode: bool, verbose: u8) -> Result<()> {
    let prompt = cli::resolve_prompt(&args.prompt, &args.prompt_flag)?;
    let model_info = models::lookup(&args.model).ok_or_else(|| {
        anyhow::anyhow!(
            "Unknown model '{}'. Run `nanaban models` to see options.",
            args.model
        )
    })?;

    if model_info.api_type == models::ApiType::Imagen {
        anyhow::bail!(
            "Imagen models only support generation, not editing. Use flash or pro instead."
        );
    }
    if !args.image.exists() {
        anyhow::bail!("Input image not found: {}", args.image.display());
    }
    if let Some(ref aspect) = args.aspect {
        cli::validate_aspect(aspect)?;
    }
    if let Some(ref size) = args.size {
        cli::validate_size(size)?;
    }
    for r in &args.refs {
        if !r.exists() {
            anyhow::bail!("Reference image not found: {}", r.display());
        }
    }
    if args.refs.len() + 1 > 14 {
        anyhow::bail!(
            "Maximum 14 total images (input + refs), got {}.",
            args.refs.len() + 1
        );
    }

    if args.dry_run {
        if json_mode {
            println!(
                "{}",
                serde_json::json!({
                    "status": "dry_run",
                    "model": model_info.api_id,
                    "model_short": model_info.short_name,
                    "estimated_cost_usd": model_info.cost_per_image,
                })
            );
        } else {
            eprintln!("Model: {} ({})", model_info.short_name, model_info.api_id);
            eprintln!("Estimated cost: ~${:.2}", model_info.cost_per_image);
        }
        return Ok(());
    }

    let config = config::Config::load()?;
    let client = client::GeminiClient::new(config.api_key);

    if verbose > 0 {
        eprintln!("Model: {} ({})", model_info.short_name, model_info.api_id);
        eprintln!("Input: {}", args.image.display());
        eprintln!("Prompt: {}", &prompt[..prompt.len().min(100)]);
    }

    let mut all_images: Vec<&Path> = vec![args.image.as_path()];
    all_images.extend(args.refs.iter().map(|p| p.as_path()));

    if verbose > 0 {
        eprintln!("Sending request...");
    }

    let size = args.size.as_deref().unwrap_or("1K");
    let result = client
        .generate(
            &prompt,
            model_info.api_id,
            model_info.api_type,
            &all_images,
            args.aspect.as_deref(),
            size,
        )
        .await?;

    let first_mime = result
        .images
        .first()
        .map(|i| i.mime_type.as_str())
        .unwrap_or("image/png");
    let output_path = args
        .output
        .clone()
        .unwrap_or_else(|| output::auto_filename(&prompt, first_mime));

    let image_outputs = save_result_images(&result, &output_path, verbose)?;

    eprintln!(
        "Estimated cost: ~${:.2} ({})",
        model_info.cost_per_image, model_info.short_name
    );

    if json_mode {
        output::emit_success_json(&output::SuccessOutput {
            status: "success",
            images: image_outputs,
            text: result.text,
            model: model_info.api_id.to_string(),
            model_short: model_info.short_name.to_string(),
            elapsed_seconds: result.elapsed_seconds,
            estimated_cost_usd: model_info.cost_per_image,
        });
    } else {
        let abs = std::fs::canonicalize(&output_path).unwrap_or(output_path.clone());
        println!("{}", abs.display());

        if output_path.exists() {
            output::show_image(&output_path, args.open, args.copy);
        }
    }

    Ok(())
}

fn save_result_images(
    result: &client::GenerationResult,
    output_path: &Path,
    verbose: u8,
) -> Result<Vec<output::ImageOutput>> {
    let mut image_outputs = Vec::new();
    for (i, img) in result.images.iter().enumerate() {
        let path = if i == 0 {
            output_path.to_path_buf()
        } else {
            let stem = output_path
                .file_stem()
                .unwrap_or_default()
                .to_str()
                .unwrap_or("image");
            let ext = output::ext_for_mime(&img.mime_type);
            output_path.with_file_name(format!("{stem}_{i}.{ext}"))
        };
        let (w, h) = output::save_image(&img.base64, &path)?;
        let abs = std::fs::canonicalize(&path).unwrap_or(path.clone());
        image_outputs.push(output::ImageOutput {
            path: abs.to_string_lossy().to_string(),
            width: w,
            height: h,
        });
        if verbose > 0 {
            eprintln!("Saved: {} ({}x{})", path.display(), w, h);
        }
    }
    Ok(image_outputs)
}
