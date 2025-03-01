use std::env;
use std::error::Error;
use std::{collections::HashMap, fs, io, path::PathBuf, process::Command};

use regex::Regex;

pub mod cli;
mod templates;

const TEMPLATES: [&str; 2] = ["lazyvim", "lazy"];

fn create_envim_dir() {
    match fs::exists(".envim").expect("Directory check: File system error") {
        true => panic!("Directory already exists"),
        false => fs::create_dir(".envim").expect("Directory creation: File system error"),
    };
}

fn get_current_config_dir() -> PathBuf {
    let envim_dir = PathBuf::from(".envim");
    envim_dir.join("config").join("nvim")
}

fn get_data_dir() -> PathBuf {
    if false {
        let Some(envim_dir) = homedir::my_home().unwrap() else {
            panic!("Home directory not accessible");
        };

        let data_dir = envim_dir.join(".local").join("share").join(".envim");
        if !fs::exists(&data_dir).unwrap() {
            fs::create_dir_all(&data_dir).expect("Couldn't create data directory");
        }

        return data_dir;
    }

    PathBuf::from(".")
}

fn deploy_template(template: &str) {
    if template == "default" || template == "lazy" {
        templates::lazy_base::bootstrap::deploy();
    } else if template == "lazyvim" {
        templates::lazyvim::bootstrap::deploy();
    }
}

fn run_nvim(args: &Vec<String>) -> io::Result<()> {
    let variables = vec![
        ("XDG_CONFIG_HOME", "config"),
        ("XDG_DATA_HOME", "data"),
        ("XDG_RUNTIME_DIR", "runtime"),
        ("XDG_STATE_HOME", "state"),
        ("XDG_CACHE_HOME", "cache"),
        ("XDG_LOGFILE", "log"),
    ];

    let pwd = std::env::current_dir().unwrap();
    let envs = variables
        .iter()
        .fold(HashMap::new(), |mut acc, (key, value)| {
            let path = pwd.join(".envim").join(value);
            acc.insert(key, path);
            acc
        });
    let _ = args;
    Command::new("nvim")
        .envs(envs)
        .args(args)
        .spawn()?
        .wait_with_output()?;
    Ok(())
}

fn create_symlink() {
    let symlink = get_current_config_dir().join("lua").join("plugins");
    let nvim_plugins = env::current_dir().unwrap().join(".nvim").join("plugins");
    match std::os::unix::fs::symlink(nvim_plugins, symlink) {
        io::Result::Err(e) => {
            if e.kind() == io::ErrorKind::AlreadyExists {
                println!("Plugins symlink already exists.");
            } else {
                panic!("Error creating symlink: {e}");
            }
        }
        Ok(_) => {}
    }
}

fn run_init(template: &str) {
    if TEMPLATES.contains(&template) || template == "default" {
        create_envim_dir();
        deploy_template(template);
        create_symlink();
    } else {
        panic!("Template not found");
    }
}

fn add_plugin(plugin: &str) {
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

    let spec = spec_v.iter().fold(get_data_dir(), |acc, x| acc.join(x));

    let file_path = plugins_path.join(format!("{}.lua", plugin_name));
    let plugin_file_content;

    if fs::exists(&spec).unwrap() {
        plugin_file_content = fs::read_to_string(&spec).unwrap();
        println!("Using default spec for {plugin}, setup with lazy");
    } else {
        let plugin_config = r#"return {
              "%name",
              opts = {},
              lazy = false
            }
        "#;
        plugin_file_content = plugin_config.replace("%name", plugin)
    }

    fs::write(file_path, plugin_file_content).unwrap_or_else(|e| {
        panic!("Error creating plugin config file: {}", e);
    });
}

pub fn run(args: &cli::ClArgs) -> Result<(), Box<dyn Error>> {
    match &args.command {
        Some(cli::Commands::Init { template }) => {
            let default_template = String::from("default");
            let template = template.as_ref().unwrap_or(&default_template);
            run_init(&template);
        }
        Some(cli::Commands::Run { extra_args }) => {
            run_nvim(extra_args)?;
        }
        Some(cli::Commands::Add { plugin }) => {
            add_plugin(plugin);
        }
        None => {}
    }
    Ok(())
}
