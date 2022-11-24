use std::path::{Path, PathBuf};

use crate::model::cargo_content::CargoContent;

use super::path::is_dir;

const CARGO_FILE: &str = "Cargo.toml";

pub fn project_root() -> PathBuf {
    Path::new(&env!("CARGO_MANIFEST_DIR"))
        .ancestors()
        .nth(1)
        .unwrap()
        .to_path_buf()
}

pub fn get_cargo_content(path: &PathBuf) -> Option<CargoContent> {
    let content = std::fs::read_to_string(path).ok()?;

    toml::from_str(&content).ok()
}

pub fn get_project_names(path: PathBuf) -> Vec<String> {
    if !is_dir(&path) {
        return vec![];
    }
    let cargo_path = path.join(CARGO_FILE);
    if let Some(cargo) = get_cargo_content(&cargo_path) {
        return vec![cargo.package.name];
    }

    let mut result = Vec::new();

    let dirs = std::fs::read_dir(&path).unwrap();

    for dir in dirs {
        let dir = dir.unwrap();
        let path = dir.path();
        if is_dir(&path) {
            result.append(&mut get_project_names(path))
        }
    }

    result
}
