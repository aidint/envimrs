use std::{fs, path::PathBuf};

pub mod lazy_base;
pub mod lazyvim;

fn ensure_paths(paths: &[PathBuf]) {
    paths.into_iter().for_each(|path| {
        if !path.exists() {
            fs::create_dir_all(path).expect("Creating directory `{path}`: File system error");
        }
    });
}
