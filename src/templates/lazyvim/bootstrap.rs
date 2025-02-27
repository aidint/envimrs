use crate::get_config_dir;
use git2::Repository;
use std::fs;

fn deploy_lazyvim() {
    let url = "https://github.com/LazyVim/starter";
    let config_dir = get_config_dir();
    Repository::clone(url, &config_dir).expect("Cloning: Git error");
    fs::remove_dir_all(config_dir.join(".git")).expect("Removing .git: File system error");
}

pub fn deploy() {
    deploy_lazyvim();
}
