use clap::{Args, Parser, Subcommand};

pub mod build_lambda;

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
#[clap(propagate_version = true)]
pub struct Cli {
    #[clap(subcommand)]
    pub command: Command,
}

#[derive(Subcommand)]
pub enum Command {
    BuildLambda(BuildLambdaArgs),
}

#[derive(Args, Debug)]
pub struct BuildLambdaArgs {
    #[clap(short, long, default_value_t = String::from("aarch64-unknown-linux-gnu"))]
    pub target: String,
    #[clap(short, long, default_value_t = String::from("bootstrap"))]
    pub entrypoint: String,
    #[clap(short, long)]
    pub use_cross: bool,
}
