use std::io::Write;

use std::io::Read;
use zip::write::FileOptions;

use crate::command::BuildLambdaArgs;
use crate::model::error::DynError;
use crate::util::cargo::run_cargo_build;
use crate::util::hash::are_files_the_same;
use crate::util::project::get_project_names;
use crate::util::project::project_root;

pub fn build_lambda(args: &BuildLambdaArgs) -> Result<(), DynError> {
    run_cargo_build(&args.target, args.use_cross)?;
    let lambdas_names =
        get_project_names(project_root().join("backend").join("api").join("lambda"));
    let target_path = project_root().join("target");
    let release_path = target_path.join(&args.target).join("release");
    let lambdas_dir = target_path.join("lambdas");

    std::fs::create_dir_all(&lambdas_dir).ok();

    for lambda in lambdas_names {
        let lambda_executable_path = release_path.join(&lambda);
        let lambda_target_path = lambdas_dir.join(&lambda);

        let zip_name = format!("{}.zip", lambda);
        let zip_path = lambdas_dir.join(zip_name);

        if are_files_the_same(&lambda_executable_path, &lambda_target_path) {
            continue;
        } else {
            std::fs::remove_file(&lambda_target_path).ok();
        }

        std::fs::copy(&lambda_executable_path, &lambda_target_path).unwrap();

        let mut lambda_executable = std::fs::File::open(&lambda_target_path)?;
        let file = std::fs::File::create(zip_path)?;

        let mut zip = zip::ZipWriter::new(file);
        zip.start_file(&args.entrypoint, FileOptions::default())?;
        let metadata = std::fs::metadata(&lambda_executable_path)?;
        let mut buffer = vec![0; metadata.len() as usize];
        lambda_executable.read_exact(&mut buffer)?;
        zip.write_all(&buffer)?;
        zip.finish()?;
        std::fs::copy(lambda_executable_path, lambdas_dir.join(lambda))?;
    }

    Ok(())
}
