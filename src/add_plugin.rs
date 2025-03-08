use std::{fs, io, path::PathBuf};

use regex::Regex;

use crate::toml_config;

pub struct PluginAdditionInfo<'a> {
    plugin_name: &'a str,
    config_path: PathBuf,
}

#[must_use]
pub fn add_plugin(plugin: &str) -> PluginAdditionInfo {
    let plugins_path = PathBuf::from(".nvim").join("plugins");
    match fs::create_dir_all(&plugins_path) {
        io::Result::Err(e) => {
            if e.kind() == io::ErrorKind::AlreadyExists {
                println!("Directory already exists");
            } else {
                panic!("Error creating directory: {e}");
            }
        }
        Ok(_) => {}
    }

    let re = Regex::new(r"(?<author>.+)/(?<plugin_name>.+)").unwrap();
    let Some(caps) = re.captures(plugin) else {
        panic!("Plugin name doesn't conform with {{author}}/{{plugin-name (without any dots)}}(.nvim)*")
    };

    let plugin_name = &caps["plugin_name"];
    let author = &caps["author"];

    let spec_v = vec![
        "index",
        "plugins",
        author,
        plugin_name,
        "default",
        "lazy",
        "spec.lua",
    ];

    let spec = spec_v
        .iter()
        .fold(crate::get_data_dir(), |acc, x| acc.join(x));

    let file_path = plugins_path.join(format!("{}.lua", plugin_name));
    let plugin_config_content;

    if fs::exists(&spec).unwrap() {
        plugin_config_content = fs::read_to_string(&spec).unwrap();
        println!("Using default spec for {plugin}, setup with lazy");
    } else {
        let config_template = r#"return {
              "%name",
              opts = {},
              lazy = false
            }
        "#;
        plugin_config_content = config_template.replace("%name", plugin)
    }

    fs::write(&file_path, plugin_config_content).unwrap_or_else(|e| {
        panic!("Error creating plugin config file: {}", e);
    });

    PluginAdditionInfo {
        plugin_name: plugin,
        config_path: file_path,
    }
}

pub fn update_config(info: &PluginAdditionInfo) {
    let config_path = PathBuf::from("envim.toml");
    let mut config: toml_config::EnvimConfig;

    if let Ok(metadata) = fs::metadata(&config_path) {
        if metadata.is_file() {
            let envim_config_content =
                fs::read_to_string(config_path).expect("Couldn't read envim.toml");
            config = toml::from_str(&envim_config_content).unwrap();
        } else {
            panic!("envim.toml is not a file");
        }
    }
    // Otherwise, create the file
    config.workspace.dependencies
}
