// Provider Selection and Configuration Menu

use crate::system::error::{DoumError, Result};
use crate::system::config::{load_config, save_config, ProviderConfig};
use crate::cli::menu::{Menu, MenuItem};
use crate::cli::tui::ratatui_select;
use crate::cli::ui::{prompt_text_input, prompt_password_input};

/// Provider selection menu (just selects default provider)
pub fn run_provider_selection() -> Result<()> {
    let config = load_config()?;
    let current_provider = &config.llm.provider;
    
    let menu_items = vec![
        MenuItem::new("openai", "OpenAI", "Set OpenAI as default provider"),
        MenuItem::new("anthropic", "Anthropic", "Set Anthropic as default provider"),
    ];
    
    let selected = ratatui_select(
        "🔌 Select Default Provider",
        &menu_items,
        Some("↑↓: Navigate | Enter: Select | Esc/q: Cancel"),
        Some(current_provider)
    )?;
    
    if let Some(item) = selected {
        set_default_provider(&item.id)?;
    }
    
    Ok(())
}

/// Set default provider
fn set_default_provider(provider: &str) -> Result<()> {
    let mut config = load_config()?;
    
    if !config.llm.providers.contains_key(provider) {
        return Err(DoumError::Config(
            format!("Provider '{}' not found in configuration", provider)
        ));
    }
    
    config.llm.provider = provider.to_string();
    save_config(&config)?;
    
    println!("✅ Default provider set to '{}'", provider);
    
    Ok(())
}

/// Providers configuration menu (configure individual providers)
pub fn run_providers_menu() -> Result<()> {
    loop {
        let menu = Menu::builder("Providers Configuration")
            .with_back()
            .add_item("openai", "OpenAI", "Configure OpenAI provider (API key, model, etc.)")
            .add_item("anthropic", "Anthropic", "Configure Anthropic provider (API key, model, etc.)")
            .build();
        
        let selected = ratatui_select(
            "⚙️  Configure Providers",
            &menu.items,
            Some("↑↓: Navigate | Enter: Select | Esc/q: Cancel"),
            None
        )?;
        
        match selected {
            Some(item) => {
                match item.id.as_str() {
                    "back" => break,
                    "exit" => {
                        println!("\n👋 Goodbye!");
                        std::process::exit(0);
                    }
                    "openai" => {
                        if let Err(e) = run_openai_config() {
                            eprintln!("❌ OpenAI configuration failed: {}", e);
                        }
                    }
                    "anthropic" => {
                        if let Err(e) = run_anthropic_config() {
                            eprintln!("❌ Anthropic configuration failed: {}", e);
                        }
                    }
                    _ => {
                        println!("\n⚠️  Unknown provider");
                    }
                }
            }
            None => break,
        }
    }
    
    Ok(())
}

/// OpenAI configuration menu
fn run_openai_config() -> Result<()> {
    loop {
        let config = load_config()?;
        let provider_config = config.llm.get_provider("openai")?;
        
        let (model, api_key_display) = if let ProviderConfig::Openai(openai_config) = provider_config {
            let api_key_display = if openai_config.api_key.is_empty() {
                "(not set)".to_string()
            } else if openai_config.api_key.len() > 7 {
                format!("{}...", &openai_config.api_key[..7])
            } else {
                "***".to_string()
            };
            (openai_config.model.clone(), api_key_display)
        } else {
            return Err(DoumError::Config("OpenAI configuration not found".to_string()));
        };
        
        let menu = Menu::builder("OpenAI Configuration")
            .with_back()
            .add_item(
                "model",
                &format!("Model [current: {}]", model),
                &format!("Select GPT model (current: {})", model)
            )
            .add_item(
                "api_key",
                &format!("API Key [{}]", api_key_display),
                "Set your OpenAI API key (starts with sk-)"
            )
            .add_item(
                "organization",
                "Organization (optional)",
                "Set organization ID (optional)"
            )
            .add_item(
                "project",
                "Project (optional)",
                "Set project ID (optional)"
            )
            .build();
        
        let selected = ratatui_select(
            "🟢 OpenAI Configuration",
            &menu.items,
            Some("↑↓: Navigate | Enter: Select | Esc/q: Cancel"),
            None
        )?;
        
        match selected {
            Some(item) => {
                match item.id.as_str() {
                    "back" => break,
                    "exit" => {
                        println!("\n👋 Goodbye!");
                        std::process::exit(0);
                    }
                    "model" => {
                        edit_openai_model()?;
                    }
                    "api_key" => {
                        edit_openai_api_key()?;
                    }
                    "organization" => {
                        edit_openai_organization()?;
                    }
                    "project" => {
                        edit_openai_project()?;
                    }
                    _ => {
                        println!("\n⚠️  Unknown option");
                    }
                }
            }
            None => break,
        }
    }
    
    Ok(())
}

/// Anthropic configuration menu
fn run_anthropic_config() -> Result<()> {
    loop {
        let config = load_config()?;
        let provider_config = config.llm.get_provider("anthropic")?;
        
        let (model, api_key_display) = if let ProviderConfig::Anthropic(anthropic_config) = provider_config {
            let api_key_display = if anthropic_config.api_key.is_empty() {
                "(not set)".to_string()
            } else if anthropic_config.api_key.len() > 10 {
                format!("{}...", &anthropic_config.api_key[..10])
            } else {
                "***".to_string()
            };
            (anthropic_config.model.clone(), api_key_display)
        } else {
            return Err(DoumError::Config("Anthropic configuration not found".to_string()));
        };
        
        let menu = Menu::builder("Anthropic Configuration")
            .with_back()
            .add_item(
                "model",
                &format!("Model [current: {}]", model),
                &format!("Select Claude model (current: {})", model)
            )
            .add_item(
                "api_key",
                &format!("API Key [{}]", api_key_display),
                "Set your Anthropic API key (starts with sk-ant-)"
            )
            .build();
        
        let selected = ratatui_select(
            "🟣 Anthropic Configuration",
            &menu.items,
            Some("↑↓: Navigate | Enter: Select | Esc/q: Cancel"),
            None
        )?;
        
        match selected {
            Some(item) => {
                match item.id.as_str() {
                    "back" => break,
                    "exit" => {
                        println!("\n👋 Goodbye!");
                        std::process::exit(0);
                    }
                    "model" => {
                        edit_anthropic_model()?;
                    }
                    "api_key" => {
                        edit_anthropic_api_key()?;
                    }
                    _ => {
                        println!("\n⚠️  Unknown option");
                    }
                }
            }
            None => break,
        }
    }
    
    Ok(())
}

/// Set active provider
/// Verify provider API key and settings
/// Edit OpenAI model
fn edit_openai_model() -> Result<()> {
    let mut config = load_config()?;
    
    let provider_config = config.llm.get_provider("openai")?;
    let current_model = if let ProviderConfig::Openai(openai_config) = provider_config {
        openai_config.model.clone()
    } else {
        return Err(DoumError::Config("OpenAI configuration not found".to_string()));
    };
    
    let models = vec![
        ("gpt-5.1", "GPT-5.1", "Best for complex coding and agent tasks"),
        ("gpt-5-mini", "GPT-5 Mini", "Faster and cheaper version of GPT-5"),
        ("gpt-5-nano", "GPT-5 Nano", "Fastest and most affordable for simple tasks"),
        ("gpt-4.1", "GPT-4.1", "Previous generation flagship model"),
        ("gpt-4.1-mini", "GPT-4.1 Mini", "Smaller version of GPT-4.1"),
        ("custom", "Custom", "Enter model name manually"),
    ];
    
    let menu = Menu::builder("Select OpenAI Model")
        .with_back()
        .build();
    
    let mut menu_with_models = menu;
    for (id, label, desc) in &models {
        menu_with_models.items.push(crate::cli::menu::MenuItem {
            id: id.to_string(),
            label: label.to_string(),
            description: desc.to_string(),
        });
    }
    
    let selected = ratatui_select(
        "🟢 Select OpenAI Model",
        &menu_with_models.items,
        Some("↑↓: Navigate | Enter: Select | Esc/q: Cancel"),
        Some(&current_model)
    )?;
    
    let model = match selected {
        Some(item) if item.id == "back" => return Ok(()),
        Some(item) if item.id == "exit" => {
            println!("\n👋 Goodbye!");
            std::process::exit(0);
        }
        Some(item) if item.id == "custom" => {
            prompt_text_input("Model name", Some(&current_model))?
        }
        Some(item) => item.id.clone(),
        None => return Ok(()),
    };
    
    let provider_config = config.llm.get_provider_mut("openai")?;
    if let ProviderConfig::Openai(openai_config) = provider_config {
        openai_config.model = model.clone();
    }
    
    save_config(&config)?;
    println!("✅ Model set to '{}'", model);
    
    Ok(())
}

/// Edit OpenAI API key
fn edit_openai_api_key() -> Result<()> {
    let mut config = load_config()?;
    
    let api_key = prompt_password_input("OpenAI API Key")?;
    
    let provider_config = config.llm.get_provider_mut("openai")?;
    if let ProviderConfig::Openai(openai_config) = provider_config {
        openai_config.api_key = api_key;
    }
    
    save_config(&config)?;
    println!("✅ API key saved");
    
    Ok(())
}

/// Edit OpenAI Organization
fn edit_openai_organization() -> Result<()> {
    let mut config = load_config()?;
    
    let provider_config = config.llm.get_provider("openai")?;
    let current_org = if let ProviderConfig::Openai(openai_config) = provider_config {
        openai_config.organization.clone().unwrap_or_default()
    } else {
        String::new()
    };
    
    let org = prompt_text_input(
        "Organization ID (leave empty for none)",
        if current_org.is_empty() { None } else { Some(&current_org) }
    )?;
    
    let provider_config = config.llm.get_provider_mut("openai")?;
    if let ProviderConfig::Openai(openai_config) = provider_config {
        openai_config.organization = if org.is_empty() { None } else { Some(org) };
    }
    
    save_config(&config)?;
    println!("✅ Organization ID saved");
    
    Ok(())
}

/// Edit OpenAI Project
fn edit_openai_project() -> Result<()> {
    let mut config = load_config()?;
    
    let provider_config = config.llm.get_provider("openai")?;
    let current_proj = if let ProviderConfig::Openai(openai_config) = provider_config {
        openai_config.project.clone().unwrap_or_default()
    } else {
        String::new()
    };
    
    let proj = prompt_text_input(
        "Project ID (leave empty for none)",
        if current_proj.is_empty() { None } else { Some(&current_proj) }
    )?;
    
    let provider_config = config.llm.get_provider_mut("openai")?;
    if let ProviderConfig::Openai(openai_config) = provider_config {
        openai_config.project = if proj.is_empty() { None } else { Some(proj) };
    }
    
    save_config(&config)?;
    println!("✅ Project ID saved");
    
    Ok(())
}

/// Edit Anthropic model
fn edit_anthropic_model() -> Result<()> {
    let mut config = load_config()?;
    
    let provider_config = config.llm.get_provider("anthropic")?;
    let current_model = if let ProviderConfig::Anthropic(anthropic_config) = provider_config {
        anthropic_config.model.clone()
    } else {
        return Err(DoumError::Config("Anthropic configuration not found".to_string()));
    };
    
    let models = vec![
        ("claude-sonnet-4-5", "Claude Sonnet 4.5", "Best for complex agents and coding"),
        ("claude-haiku-4-5", "Claude Haiku 4.5", "Fastest with near-frontier intelligence"),
        ("claude-opus-4-1", "Claude Opus 4.1", "Exceptional for specialized reasoning"),
        ("custom", "Custom", "Enter model name manually"),
    ];
    
    let menu = Menu::builder("Select Anthropic Model")
        .with_back()
        .build();
    
    let mut menu_with_models = menu;
    for (id, label, desc) in &models {
        menu_with_models.items.push(crate::cli::menu::MenuItem {
            id: id.to_string(),
            label: label.to_string(),
            description: desc.to_string(),
        });
    }
    
    let selected = ratatui_select(
        "🟣 Select Anthropic Model",
        &menu_with_models.items,
        Some("↑↓: Navigate | Enter: Select | Esc/q: Cancel"),
        Some(&current_model)
    )?;
    
    let model = match selected {
        Some(item) if item.id == "back" => return Ok(()),
        Some(item) if item.id == "exit" => {
            println!("\n👋 Goodbye!");
            std::process::exit(0);
        }
        Some(item) if item.id == "custom" => {
            prompt_text_input("Model name", Some(&current_model))?
        }
        Some(item) => item.id.clone(),
        None => return Ok(()),
    };
    
    let provider_config = config.llm.get_provider_mut("anthropic")?;
    if let ProviderConfig::Anthropic(anthropic_config) = provider_config {
        anthropic_config.model = model.clone();
    }
    
    save_config(&config)?;
    println!("✅ Model set to '{}'", model);
    
    Ok(())
}

/// Edit Anthropic API key
fn edit_anthropic_api_key() -> Result<()> {
    let mut config = load_config()?;
    
    let api_key = prompt_password_input("Anthropic API Key")?;
    
    let provider_config = config.llm.get_provider_mut("anthropic")?;
    if let ProviderConfig::Anthropic(anthropic_config) = provider_config {
        anthropic_config.api_key = api_key;
    }
    
    save_config(&config)?;
    println!("✅ API key saved");
    
    Ok(())
}
