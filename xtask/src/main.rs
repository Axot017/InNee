use clap::Parser;
use command::{build_lambda::build_lambda, Cli, Command};
use model::error::DynError;

mod command;
mod model;
mod util;

fn main() {
    if let Err(e) = try_main() {
        eprintln!("{}", e);
        std::process::exit(-1);
    }
}

fn try_main() -> Result<(), DynError> {
    let cli = Cli::parse();

    match &cli.command {
        Command::BuildLambda(args) => build_lambda(args),
    }
}
