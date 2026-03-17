use anyhow::{bail, Result};
use base64::Engine;
use serde::{Deserialize, Serialize};
use std::path::Path;
use std::time::Instant;

const API_BASE: &str = "https://generativelanguage.googleapis.com/v1beta/models";

pub struct GeminiClient {
    http: reqwest::Client,
    api_key: String,
}

#[derive(Debug)]
pub struct GenerationResult {
    pub images: Vec<ImageData>,
    pub text: Option<String>,
    pub elapsed_seconds: f64,
}

#[derive(Debug)]
pub struct ImageData {
    pub base64: String,
    #[allow(dead_code)]
    pub mime_type: String,
}

// Request types
#[derive(Serialize)]
struct GenerateRequest {
    contents: Vec<Content>,
    #[serde(rename = "generationConfig")]
    generation_config: GenerationConfig,
}

#[derive(Serialize)]
struct Content {
    parts: Vec<Part>,
}

#[derive(Serialize)]
#[serde(untagged)]
enum Part {
    Text {
        text: String,
    },
    InlineData {
        #[serde(rename = "inlineData")]
        inline_data: InlineData,
    },
}

#[derive(Serialize)]
struct InlineData {
    #[serde(rename = "mimeType")]
    mime_type: String,
    data: String,
}

#[derive(Serialize)]
struct GenerationConfig {
    #[serde(rename = "responseModalities")]
    response_modalities: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "imageConfig")]
    image_config: Option<ImageConfig>,
}

#[derive(Serialize)]
struct ImageConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "aspectRatio")]
    aspect_ratio: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "outputImageSize")]
    output_image_size: Option<String>,
}

// Response types
#[derive(Deserialize)]
struct GenerateResponse {
    candidates: Option<Vec<Candidate>>,
    error: Option<ApiError>,
}

#[derive(Deserialize)]
struct Candidate {
    content: Option<CandidateContent>,
    #[serde(rename = "finishReason")]
    finish_reason: Option<String>,
}

#[derive(Deserialize)]
struct CandidateContent {
    parts: Option<Vec<ResponsePart>>,
}

#[derive(Deserialize)]
struct ResponsePart {
    text: Option<String>,
    #[serde(rename = "inlineData")]
    inline_data: Option<ResponseInlineData>,
}

#[derive(Deserialize)]
struct ResponseInlineData {
    #[serde(rename = "mimeType")]
    mime_type: String,
    data: String,
}

#[derive(Deserialize)]
struct ApiError {
    message: String,
    #[allow(dead_code)]
    code: Option<i32>,
}

impl GeminiClient {
    pub fn new(api_key: String) -> Self {
        let http = reqwest::Client::builder()
            .timeout(std::time::Duration::from_secs(120))
            .build()
            .expect("Failed to build HTTP client");
        GeminiClient { http, api_key }
    }

    pub async fn generate(
        &self,
        prompt: &str,
        model_api_id: &str,
        input_images: &[&Path],
        aspect: Option<&str>,
        size: &str,
    ) -> Result<GenerationResult> {
        let mut parts: Vec<Part> = Vec::new();

        // Add input images (edit image + reference images)
        for img_path in input_images {
            let data = std::fs::read(img_path)?;
            let mime = mime_for_path(img_path);
            let b64 = base64::engine::general_purpose::STANDARD.encode(&data);
            parts.push(Part::InlineData {
                inline_data: InlineData {
                    mime_type: mime.to_string(),
                    data: b64,
                },
            });
        }

        // Add text prompt
        parts.push(Part::Text {
            text: prompt.to_string(),
        });

        let image_config = if aspect.is_some() || size != "1K" {
            Some(ImageConfig {
                aspect_ratio: aspect.map(|a| a.replace(':', ":")),
                output_image_size: match size {
                    "512" => Some("512".to_string()),
                    "2K" => Some("2048".to_string()),
                    "4K" => Some("4096".to_string()),
                    _ => None, // 1K is default, don't send
                },
            })
        } else {
            None
        };

        let request = GenerateRequest {
            contents: vec![Content { parts }],
            generation_config: GenerationConfig {
                response_modalities: vec!["TEXT".to_string(), "IMAGE".to_string()],
                image_config,
            },
        };

        let url = format!("{API_BASE}/{model_api_id}:generateContent?key={}", self.api_key);

        let start = Instant::now();
        let response = self.http.post(&url).json(&request).send().await?;
        let elapsed = start.elapsed().as_secs_f64();

        let status = response.status();
        let body = response.text().await?;

        if !status.is_success() {
            // Try to parse error
            if let Ok(parsed) = serde_json::from_str::<GenerateResponse>(&body) {
                if let Some(err) = parsed.error {
                    match status.as_u16() {
                        401 | 403 => bail!("Authentication failed: {}. Check your GEMINI_API_KEY.", err.message),
                        429 => bail!("Rate limited: {}. Try again in a few seconds.", err.message),
                        _ => bail!("API error ({}): {}", status, err.message),
                    }
                }
            }
            bail!("API returned {}: {}", status, &body[..body.len().min(200)]);
        }

        let parsed: GenerateResponse = serde_json::from_str(&body)
            .map_err(|e| anyhow::anyhow!("Failed to parse API response: {}", e))?;

        if let Some(err) = parsed.error {
            bail!("API error: {}", err.message);
        }

        let candidates = parsed.candidates.unwrap_or_default();
        if candidates.is_empty() {
            bail!("No candidates in response. The model may have refused the request.");
        }

        let candidate = &candidates[0];

        // Check for safety blocks
        if let Some(reason) = &candidate.finish_reason {
            if reason == "SAFETY" {
                bail!("Request blocked by safety filter. Try rephrasing your prompt.");
            }
        }

        let parts = candidate
            .content
            .as_ref()
            .and_then(|c| c.parts.as_ref())
            .map(|p| p.as_slice())
            .unwrap_or(&[]);

        let mut images = Vec::new();
        let mut text_parts = Vec::new();

        for part in parts {
            if let Some(t) = &part.text {
                text_parts.push(t.clone());
            }
            if let Some(data) = &part.inline_data {
                images.push(ImageData {
                    base64: data.data.clone(),
                    mime_type: data.mime_type.clone(),
                });
            }
        }

        if images.is_empty() {
            let text_hint = if text_parts.is_empty() {
                String::new()
            } else {
                format!(" Model said: {}", text_parts.join(" "))
            };
            bail!("No image returned by the model.{text_hint}");
        }

        Ok(GenerationResult {
            images,
            text: if text_parts.is_empty() {
                None
            } else {
                Some(text_parts.join("\n"))
            },
            elapsed_seconds: elapsed,
        })
    }
}

fn mime_for_path(path: &Path) -> &'static str {
    match path.extension().and_then(|e| e.to_str()) {
        Some("png") => "image/png",
        Some("jpg" | "jpeg") => "image/jpeg",
        Some("webp") => "image/webp",
        Some("gif") => "image/gif",
        _ => "image/png",
    }
}
