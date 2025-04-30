use std::{fs, io, path::PathBuf};

use regex::Regex;

use crate::config::{get_config, Dependency, EnvimConfig};

mod git;

#[derive(Debug)]
pub struct PluginAdditionInfo<'a> {
    plugin_name: &'a str,
    commit: Option<String>,
    tag: Option<String>,
}

#[must_use]
pub fn add_plugin(plugin: &str) -> PluginAdditionInfo {
    // create the plugins path if it doesn't exist
    let plugins_path = PathBuf::from(".nvim").join("plugins");
    match fs::create_dir_all(&plugins_path) {
        io::Result::Err(e) => {
            if e.kind() == io::ErrorKind::AlreadyExists {
                // TODO: there is no need for this println!
                // remove it
                println!("Directory already exists");
            } else {
                panic!("Error creating directory: {e}");
            }
        }
        Ok(_) => {}
    }

    // find the spec file in the index given the author and plugin name
    // TODO: add version and commit to this

    let re = Regex::new(r"(?<author>.+)/(?<plugin_name>.+)").unwrap();
    let Some(caps) = re.captures(plugin) else {
        panic!("Plugin name doesn't conform with {{author}}/{{plugin-name}}")
    };

    let plugin_name = &caps["plugin_name"];
    let author = &caps["author"];

    // find the latest commit
    let git::PackageVersion::Commit(commit) = git::get_package_latest_version(plugin);

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

    // save the spec as the {plugin_name}.lua in the plugins_path
    let file_path = plugins_path.join(if plugin_name.ends_with(".lua") {
        format!("{}", plugin_name)
    } else {
        format!("{}.lua", plugin_name)
    });
    let plugin_config_content;

    if fs::exists(&spec).unwrap() {
        plugin_config_content = fs::read_to_string(&spec).unwrap();
        println!("Using default spec for {plugin}, setup with lazy");
    } else {
        plugin_config_content = r#"return {
  "%name",
  commit = "%commit",
  opts = {},
  lazy = false
}"#
        .to_string();
    }

    // replace placeholders
    let plugin_config_content = plugin_config_content
        .replace("%name", plugin)
        .replace("%commit", &commit);

    fs::write(&file_path, plugin_config_content).unwrap_or_else(|e| {
        panic!("Error creating plugin config file: {}", e);
    });

    PluginAdditionInfo {
        plugin_name: plugin,
        commit: Some(commit),
        tag: None,
    }
}

pub fn config_with_plugin(info: PluginAdditionInfo) -> EnvimConfig {
    let mut config = get_config();
    let dep = config
        .workspace
        .dependencies
        .entry(info.plugin_name.to_string())
        .or_insert(Dependency::new());
    dep.commit = info.commit;
    dep.tag = info.tag;

    config
}

