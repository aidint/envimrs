use std::env;
use std::error::Error;
use std::{collections::HashMap, fs, io, path::PathBuf, process::Command};

mod add_plugin;
pub mod cli;
mod config;
mod templates;

use add_plugin::add_plugin;
use config::config_with_plugin;

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
    let Some(envim_dir) = homedir::my_home().unwrap() else {
        panic!("Home directory not accessible");
    };

    let data_dir = envim_dir.join(".local").join("share").join("envim");
    return data_dir;
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
            let add_info = add_plugin(plugin);
            let mut config = config_with_plugin(add_info);
            config.edit_toml();
            fs::write(PathBuf::from("envim.toml"), config.toml_to_string())?
        }
        Some(cli::Commands::Test) => {
            // let file = fs::read_to_string("/Users/aidin/Documents/code/mine/nvim/envim-rust-try/envim-cli/test_dir/envim.toml")?;
            // let mut doc = file.parse::<toml_edit::DocumentMut>()?;
            let mut doc = toml_edit::DocumentMut::new();

            let mut hasher = HashMap::<&'_ str, i64>::new();
            hasher.insert("hi", 1);
            hasher.insert("bye", 2);

            doc["workspace"] = toml_edit::Item::Value(toml_edit::Value::from_iter(hasher));

            // doc["workspace"].as_table_mut().map(|t| t.set_implicit(true));
            // doc["workspace"]["dependencies"] = toml_edit::table();
            // doc["workspace"]["a"] = toml_edit::value(1);
            // let table = doc["workspace"]["dependencies"]
            //     .clone()
            //     .into_table()
            //     .unwrap();
            //
            // let (key, _) = table.get_key_value("a").unwrap();
            //
            // println!("{key}");
            // println!("it is a value: {:#?}", doc["workspace"]["dependencies"]["a"].as_table_mut().unwrap().key_mut("b"));
            // doc["workspace"]["dependencies"]["a"].as_table_mut().map(|t| t.fmt());
            println!("{doc}")
        }
        None => {}
    }
    Ok(())
}
