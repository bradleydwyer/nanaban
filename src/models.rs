pub struct ModelInfo {
    pub short_name: &'static str,
    pub api_id: &'static str,
    pub cost_per_image: f64,
    pub description: &'static str,
}

pub const MODELS: &[ModelInfo] = &[
    ModelInfo {
        short_name: "flash",
        api_id: "gemini-3.1-flash-image-preview",
        cost_per_image: 0.04,
        description: "Fast, cheap — Nano Banana 2",
    },
    ModelInfo {
        short_name: "pro",
        api_id: "gemini-3-pro-image-preview",
        cost_per_image: 0.13,
        description: "High quality — Nano Banana Pro",
    },
    ModelInfo {
        short_name: "2.5-flash",
        api_id: "gemini-2.5-flash-image",
        cost_per_image: 0.04,
        description: "Speed optimized — Gemini 2.5",
    },
];

pub const DEFAULT_MODEL: &str = "flash";

pub fn lookup(short_name: &str) -> Option<&'static ModelInfo> {
    MODELS.iter().find(|m| m.short_name == short_name)
}

pub fn print_table() {
    println!("{:<12} {:<38} {:>10}  Description", "Name", "API Identifier", "Cost");
    println!("{}", "-".repeat(90));
    for m in MODELS {
        let default_marker = if m.short_name == DEFAULT_MODEL { " (default)" } else { "" };
        println!(
            "{:<12} {:<38} {:>8}  {}{}",
            m.short_name, m.api_id,
            format!("~${:.2}", m.cost_per_image),
            m.description, default_marker,
        );
    }
}
