use crate::system::SystemInfo;
use handlebars::Handlebars;
use rust_embed::RustEmbed;
use serde_json::json;

/// Embed prompt assets
#[derive(RustEmbed)]
#[folder = "static/prompts/"]
struct PromptAssets;

/// Prompt builder structure
pub struct PromptBuilder {
    system_info: SystemInfo,
    handlebars: Handlebars<'static>,
}

impl PromptBuilder {
    /// Create new PromptBuilder
    pub fn new(system_info: SystemInfo) -> Self {
        Self {
            system_info,
            handlebars: Handlebars::new(),
        }
    }

    /// Load prompt file content
    fn load_prompt(name: &str) -> String {
        PromptAssets::get(name)
            .and_then(|file| {
                std::str::from_utf8(file.data.as_ref())
                    .ok()
                    .map(|s| s.to_string())
            })
            .unwrap_or_else(|| {
                eprintln!("Warning: Failed to load prompt file: {}", name);
                tracing::warn!("Failed to load prompt file: {}", name);
                String::new()
            })
    }

    /// Create common prompt section
    fn build_common_prompt(&self) -> String {
        let template = Self::load_prompt("common.md");

        let data = json!({
            "os": self.system_info.os.as_str(),
            "shell": self.system_info.shell.as_str(),
            "current_dir": self.system_info.current_dir.display().to_string(),
            "username": self.system_info.username.as_deref().unwrap_or("unknown"),
            "hostname": self.system_info.hostname.as_deref().unwrap_or("unknown"),
        });

        self.handlebars
            .render_template(&template, &data)
            .unwrap_or(template)
    }

    /// Create Ask mode message array
    pub fn build_ask(&self) -> String {
        let common_prompt = self.build_common_prompt();
        let ask_template = Self::load_prompt("ask.md");

        let prompt = format!("{}\n\n---\n\n{}", common_prompt, ask_template);
        prompt
    }

    /// Create Suggest mode message array
    pub fn build_suggest(&self) -> String {
        let common_prompt = self.build_common_prompt();
        let suggest_template = Self::load_prompt("suggest.md");

        let data = json!({
            "os": self.system_info.os.as_str(),
            "shell": self.system_info.shell.as_str(),
        });

        let suggest_prompt = self
            .handlebars
            .render_template(&suggest_template, &data)
            .unwrap_or(suggest_template);

        let prompt = format!("{}\n\n---\n\n{}", common_prompt, suggest_prompt);
        prompt
    }

    /// Create Mode Select message array
    pub fn build_mode_select(&self) -> String {
        let common_prompt = self.build_common_prompt();
        let mode_select_template = Self::load_prompt("mode_select.md");

        let prompt = format!("{}\n\n---\n\n{}", common_prompt, mode_select_template);
        prompt
    }
}
