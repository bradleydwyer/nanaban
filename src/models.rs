#[derive(Clone, Copy, PartialEq)]
pub enum ApiType {
    Gemini,
    Imagen,
}

pub struct ModelInfo {
    pub short_name: &'static str,
    pub api_id: &'static str,
    pub api_type: ApiType,
    pub cost_per_image: f64,
    pub description: &'static str,
}

pub const MODELS: &[ModelInfo] = &[
    ModelInfo {
        short_name: "flash",
        api_id: "gemini-3.1-flash-image-preview",
        api_type: ApiType::Gemini,
        cost_per_image: 0.04,
        description: "Fast, cheap — Nano Banana 2",
    },
    ModelInfo {
        short_name: "pro",
        api_id: "gemini-3-pro-image-preview",
        api_type: ApiType::Gemini,
        cost_per_image: 0.13,
        description: "High quality — Nano Banana Pro",
    },
    ModelInfo {
        short_name: "imagen-fast",
        api_id: "imagen-4.0-fast-generate-001",
        api_type: ApiType::Imagen,
        cost_per_image: 0.02,
        description: "Fastest generation — Imagen 4 Fast",
    },
    ModelInfo {
        short_name: "imagen",
        api_id: "imagen-4.0-generate-001",
        api_type: ApiType::Imagen,
        cost_per_image: 0.04,
        description: "Balanced quality/speed — Imagen 4",
    },
    ModelInfo {
        short_name: "imagen-ultra",
        api_id: "imagen-4.0-ultra-generate-001",
        api_type: ApiType::Imagen,
        cost_per_image: 0.06,
        description: "Highest quality — Imagen 4 Ultra",
    },
];

pub const DEFAULT_MODEL: &str = "flash";

pub fn lookup(short_name: &str) -> Option<&'static ModelInfo> {
    MODELS.iter().find(|m| m.short_name == short_name)
}

pub fn print_table() {
    println!(
        "{:<14} {:<38} {:>8}  Description",
        "Name", "API Identifier", "Cost"
    );
    println!("{}", "-".repeat(95));
    for m in MODELS {
        let default_marker = if m.short_name == DEFAULT_MODEL {
            " (default)"
        } else {
            ""
        };
        println!(
            "{:<14} {:<38} {:>8}  {}{}",
            m.short_name,
            m.api_id,
            format!("~${:.2}", m.cost_per_image),
            m.description,
            default_marker,
        );
    }
}
