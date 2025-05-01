pub mod toml_config;

use std::{fs, path::PathBuf};

pub use crate::add_plugin::config_with_plugin;
pub use toml_config::{Dependency, EnvimConfig};

// get config from toml file

pub fn get_config() -> EnvimConfig {
    let config_path = PathBuf::from("envim.toml");
    if let Ok(metadata) = fs::metadata(&config_path) {
        if metadata.is_file() {
            let envim_config_content =
                fs::read_to_string(config_path).expect("Couldn't read envim.toml");
            let mut config: EnvimConfig = toml_edit::de::from_str(&envim_config_content).unwrap();
            // TODO: later we need to add support for keeping the decors and comments
            config.set_doc(
                envim_config_content
                    .parse::<toml_edit::DocumentMut>()
                    .unwrap_or(toml_edit::DocumentMut::new()),
            );
            config
        } else {
            EnvimConfig::new()
        }
    } else {
        EnvimConfig::new()
    }
}
