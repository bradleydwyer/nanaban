use anyhow::{Result, bail};
use std::env;

pub struct Config {
    pub api_key: String,
}

impl Config {
    pub fn load() -> Result<Self> {
        let api_key = env::var("GEMINI_API_KEY")
            .or_else(|_| env::var("GOOGLE_API_KEY"))
            .map_err(|_| {
                anyhow::anyhow!(
                    "No API key found. Set GEMINI_API_KEY or GOOGLE_API_KEY environment variable.\n\
                     Get a key at: https://aistudio.google.com/apikey"
                )
            })?;

        if api_key.trim().is_empty() {
            bail!("API key is empty. Set GEMINI_API_KEY or GOOGLE_API_KEY to a valid key.");
        }

        Ok(Config { api_key })
    }
}
