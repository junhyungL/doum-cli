use crate::cli::args::ConfigAction;
use crate::core::{get_all_as_str, get_value, reset, set_value, unset_value};
use crate::system::get_config_path;
use anyhow::Result;

pub fn handle_config_command(action: Option<ConfigAction>) -> Result<()> {
    // if no action is provided, default to Show
    let action = action.unwrap_or(ConfigAction::Show);

    match action {
        ConfigAction::Show => {
            let config_path = get_config_path()?;
            let toml_str = get_all_as_str()?;
            println!("Config file location: {}\n", config_path.display());
            println!("{}", toml_str);
            Ok(())
        }
        ConfigAction::Reset => {
            reset()?;
            println!("✅ Configuration reset to default");
            Ok(())
        }
        ConfigAction::Set { key, value } => {
            set_value(&key, &value)?;
            println!("✅ Config {} = {}", key, value);
            Ok(())
        }
        ConfigAction::Get { key } => {
            let value = get_value(&key)?;
            println!("{}", value);
            Ok(())
        }
        ConfigAction::Unset { key } => {
            unset_value(&key)?;
            println!("✅ Config {} reset to default", key);
            Ok(())
        }
    }
}
