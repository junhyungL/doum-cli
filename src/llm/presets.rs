use rust_embed::RustEmbed;
use serde::Deserialize;

#[derive(RustEmbed)]
#[folder = "static/presets/"]
struct PresetModels;

#[derive(Debug, Deserialize, Clone)]
pub struct ModelInfo {
    pub id: String,
    pub name: String,
    pub description: String,
}

#[derive(Debug, Deserialize)]
struct ModelList {
    models: Vec<ModelInfo>,
}

/// Load model presets for a provider
pub fn load_presets(provider: &str) -> Vec<ModelInfo> {
    let filename = format!("{}.toml", provider);

    if let Some(content) = PresetModels::get(&filename)
        && let Ok(data) = std::str::from_utf8(content.data.as_ref())
        && let Ok(list) = toml::from_str::<ModelList>(data)
    {
        return list.models;
    }

    vec![]
}
