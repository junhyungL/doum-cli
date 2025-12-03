use crate::cli::args::ConfigAction;
use crate::core::ConfigManager;
use anyhow::Result;

pub fn handle_config_command(action: Option<ConfigAction>) -> Result<()> {
    // if no action is provided, default to Show
    let action = action.unwrap_or(ConfigAction::Show);

    match action {
        ConfigAction::Show => {
            let config_path = ConfigManager::get_config_path()?;
            let toml_str = ConfigManager::get_all_as_toml()?;
            println!("Config file location: {}\n", config_path.display());
            println!("{}", toml_str);
            Ok(())
        }
        ConfigAction::Reset => {
            ConfigManager::reset()?;
            println!("✅ Configuration reset to default");
            Ok(())
        }
        ConfigAction::Set { key, value } => {
            ConfigManager::set_value(&key, &value)?;
            println!("✅ Config {} = {}", key, value);
            Ok(())
        }
        ConfigAction::Get { key } => {
            let value = ConfigManager::get_value(&key)?;
            println!("{}", value);
            Ok(())
        }
        ConfigAction::Unset { key } => {
            ConfigManager::unset_value(&key)?;
            println!("✅ Config {} reset to default", key);
            Ok(())
        }
    }
}
