use std::{env, process};

use crate::model::error::{DynError, Error};

use super::project::project_root;

pub fn run_cargo_build(target: &str, use_cross: bool) -> Result<(), DynError> {
    let cargo = if use_cross {
        "cross".to_owned()
    } else {
        env::var("CARGO").unwrap_or_else(|_| "cargo".to_owned())
    };
    let status = process::Command::new(cargo)
        .current_dir(project_root())
        .args([
            "build",
            "--release",
            "--target",
            target,
            "--exclude",
            "xtask",
            "--workspace",
        ])
        .status()?;

    if status.success() {
        Ok(())
    } else {
        Err(Error::BuildFailed)?
    }
}
