use anyhow::{Result, bail};
use base64::Engine;
use serde::{Deserialize, Serialize};
use std::path::Path;
use std::time::Instant;

use crate::models::ApiType;

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

// --- Gemini (Nano Banana) request/response types ---

#[derive(Serialize)]
struct GeminiRequest {
    contents: Vec<GeminiContent>,
    #[serde(rename = "generationConfig")]
    generation_config: GeminiGenerationConfig,
}

#[derive(Serialize)]
struct GeminiContent {
    parts: Vec<GeminiPart>,
}

#[derive(Serialize)]
#[serde(untagged)]
enum GeminiPart {
    Text {
        text: String,
    },
    InlineData {
        #[serde(rename = "inlineData")]
        inline_data: GeminiInlineData,
    },
}

#[derive(Serialize)]
struct GeminiInlineData {
    #[serde(rename = "mimeType")]
    mime_type: String,
    data: String,
}

#[derive(Serialize)]
struct GeminiGenerationConfig {
    #[serde(rename = "responseModalities")]
    response_modalities: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "imageConfig")]
    image_config: Option<GeminiImageConfig>,
}

#[derive(Serialize)]
struct GeminiImageConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "aspectRatio")]
    aspect_ratio: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "imageSize")]
    output_image_size: Option<String>,
}

#[derive(Deserialize)]
struct GeminiResponse {
    candidates: Option<Vec<GeminiCandidate>>,
    error: Option<ApiError>,
}

#[derive(Deserialize)]
struct GeminiCandidate {
    content: Option<GeminiCandidateContent>,
    #[serde(rename = "finishReason")]
    finish_reason: Option<String>,
}

#[derive(Deserialize)]
struct GeminiCandidateContent {
    parts: Option<Vec<GeminiResponsePart>>,
}

#[derive(Deserialize)]
struct GeminiResponsePart {
    text: Option<String>,
    #[serde(rename = "inlineData")]
    inline_data: Option<GeminiResponseInlineData>,
}

#[derive(Deserialize)]
struct GeminiResponseInlineData {
    #[serde(rename = "mimeType")]
    mime_type: String,
    data: String,
}

// --- Imagen 4 request/response types ---

#[derive(Serialize)]
struct ImagenRequest {
    instances: Vec<ImagenInstance>,
    parameters: ImagenParameters,
}

#[derive(Serialize)]
struct ImagenInstance {
    prompt: String,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct ImagenParameters {
    #[serde(rename = "sampleCount")]
    sample_count: u32,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "aspectRatio")]
    aspect_ratio: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "outputOptions")]
    output_options: Option<ImagenOutputOptions>,
}

#[derive(Serialize)]
struct ImagenOutputOptions {
    #[serde(rename = "mimeType")]
    mime_type: String,
}

#[derive(Deserialize)]
struct ImagenResponse {
    predictions: Option<Vec<ImagenPrediction>>,
    error: Option<ApiError>,
}

#[derive(Deserialize)]
struct ImagenPrediction {
    #[serde(rename = "bytesBase64Encoded")]
    bytes_base64_encoded: String,
    #[serde(rename = "mimeType")]
    mime_type: Option<String>,
}

// --- Shared ---

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
        api_type: ApiType,
        input_images: &[&Path],
        aspect: Option<&str>,
        size: &str,
    ) -> Result<GenerationResult> {
        match api_type {
            ApiType::Gemini => {
                self.generate_gemini(prompt, model_api_id, input_images, aspect, size)
                    .await
            }
            ApiType::Imagen => {
                if !input_images.is_empty() {
                    bail!(
                        "Imagen 4 models only support text-to-image generation, not editing. Use flash or pro for image editing."
                    );
                }
                self.generate_imagen(prompt, model_api_id, aspect).await
            }
        }
    }

    async fn generate_gemini(
        &self,
        prompt: &str,
        model_api_id: &str,
        input_images: &[&Path],
        aspect: Option<&str>,
        size: &str,
    ) -> Result<GenerationResult> {
        let mut parts: Vec<GeminiPart> = Vec::new();

        for img_path in input_images {
            let data = std::fs::read(img_path)?;
            let mime = mime_for_path(img_path);
            let b64 = base64::engine::general_purpose::STANDARD.encode(&data);
            parts.push(GeminiPart::InlineData {
                inline_data: GeminiInlineData {
                    mime_type: mime.to_string(),
                    data: b64,
                },
            });
        }

        parts.push(GeminiPart::Text {
            text: prompt.to_string(),
        });

        let image_config = if aspect.is_some() || size != "1K" {
            Some(GeminiImageConfig {
                aspect_ratio: aspect.map(|a| a.to_string()),
                output_image_size: match size {
                    "512" => Some("512".to_string()),
                    "2K" => Some("2K".to_string()),
                    "4K" => Some("4K".to_string()),
                    _ => None,
                },
            })
        } else {
            None
        };

        let request = GeminiRequest {
            contents: vec![GeminiContent { parts }],
            generation_config: GeminiGenerationConfig {
                response_modalities: vec!["TEXT".to_string(), "IMAGE".to_string()],
                image_config,
            },
        };

        let url = format!(
            "{API_BASE}/{model_api_id}:generateContent?key={}",
            self.api_key
        );

        let start = Instant::now();
        let response = self.http.post(&url).json(&request).send().await?;
        let elapsed = start.elapsed().as_secs_f64();

        let status = response.status();
        let body = response.text().await?;

        if !status.is_success() {
            if let Ok(parsed) = serde_json::from_str::<GeminiResponse>(&body)
                && let Some(err) = parsed.error
            {
                match status.as_u16() {
                    401 | 403 => bail!(
                        "Authentication failed: {}. Check your GEMINI_API_KEY.",
                        err.message
                    ),
                    429 => bail!("Rate limited: {}. Try again in a few seconds.", err.message),
                    _ => bail!("API error ({}): {}", status, err.message),
                }
            }
            bail!("API returned {}: {}", status, &body[..body.len().min(200)]);
        }

        let parsed: GeminiResponse = serde_json::from_str(&body)
            .map_err(|e| anyhow::anyhow!("Failed to parse API response: {}", e))?;

        if let Some(err) = parsed.error {
            bail!("API error: {}", err.message);
        }

        let candidates = parsed.candidates.unwrap_or_default();
        if candidates.is_empty() {
            bail!("No candidates in response. The model may have refused the request.");
        }

        let candidate = &candidates[0];

        if let Some(reason) = &candidate.finish_reason
            && reason == "SAFETY"
        {
            bail!("Request blocked by safety filter. Try rephrasing your prompt.");
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

    async fn generate_imagen(
        &self,
        prompt: &str,
        model_api_id: &str,
        aspect: Option<&str>,
    ) -> Result<GenerationResult> {
        let request = ImagenRequest {
            instances: vec![ImagenInstance {
                prompt: prompt.to_string(),
            }],
            parameters: ImagenParameters {
                sample_count: 1,
                aspect_ratio: aspect.map(|a| a.to_string()),
                output_options: Some(ImagenOutputOptions {
                    mime_type: "image/png".to_string(),
                }),
            },
        };

        let url = format!("{API_BASE}/{model_api_id}:predict");

        let start = Instant::now();
        let response = self
            .http
            .post(&url)
            .header("x-goog-api-key", &self.api_key)
            .json(&request)
            .send()
            .await?;
        let elapsed = start.elapsed().as_secs_f64();

        let status = response.status();
        let body = response.text().await?;

        if !status.is_success() {
            if let Ok(parsed) = serde_json::from_str::<ImagenResponse>(&body)
                && let Some(err) = parsed.error
            {
                match status.as_u16() {
                    401 | 403 => bail!(
                        "Authentication failed: {}. Check your GEMINI_API_KEY.",
                        err.message
                    ),
                    429 => bail!("Rate limited: {}. Try again in a few seconds.", err.message),
                    _ => bail!("API error ({}): {}", status, err.message),
                }
            }
            bail!("API returned {}: {}", status, &body[..body.len().min(200)]);
        }

        let parsed: ImagenResponse = serde_json::from_str(&body)
            .map_err(|e| anyhow::anyhow!("Failed to parse Imagen response: {}", e))?;

        if let Some(err) = parsed.error {
            bail!("API error: {}", err.message);
        }

        let predictions = parsed.predictions.unwrap_or_default();
        if predictions.is_empty() {
            bail!("No images returned by Imagen. The request may have been filtered.");
        }

        let images: Vec<ImageData> = predictions
            .into_iter()
            .map(|p| ImageData {
                base64: p.bytes_base64_encoded,
                mime_type: p.mime_type.unwrap_or_else(|| "image/png".to_string()),
            })
            .collect();

        Ok(GenerationResult {
            images,
            text: None,
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
