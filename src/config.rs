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
            toml_edit::de::from_str(&envim_config_content).unwrap()
        } else {
            EnvimConfig::new()
        }
    } else {
        EnvimConfig::new()
    }
}

pub fn get_toml_doc() -> toml_edit::DocumentMut {
    let config_path = PathBuf::from("envim.toml");
    if let Ok(metadata) = fs::metadata(&config_path) {
        if metadata.is_file() {
            let envim_config_content =
                fs::read_to_string(config_path).expect("Couldn't read envim.toml");
            if let Ok(doc) = envim_config_content.parse::<toml_edit::DocumentMut>() {
                return doc;
            };
        };
    };
    return toml_edit::DocumentMut::new();
}

// update the toml file

fn update_dependencies(doc: &mut toml_edit::DocumentMut, config: &EnvimConfig) {
    for (key, val) in &config.workspace.dependencies {
        if let Some(tag) = &val.tag {
            doc["workspace"]["dependencies"][key] = toml_edit::value(tag);
        } else if let Some(commit) = &val.commit {
            doc["workspace"]["dependencies"][key]["commit"] = toml_edit::value(commit);
        }
    }
    doc["workspace"]["dependencie"].as_table_mut().map(|t| t.fmt());
}

pub fn update_config(doc: &mut toml_edit::DocumentMut, config: &EnvimConfig) {
    update_dependencies(doc, config);
    println!("{}", doc.to_string());
    println!("{:#?}", config);
}
