use std::path::PathBuf;

pub fn is_dir(path: &PathBuf) -> bool {
    std::fs::metadata(path)
        .map(|meta| meta.is_dir())
        .unwrap_or(false)
}
