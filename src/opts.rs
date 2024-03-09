use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
#[clap(
    about = "CLI tool for creating React Native structure folders",
    author = "Raul Andrade"
)]
pub struct Opts {
    #[clap(short = 'c', long = "config")]
    pub config: Option<PathBuf>,

    #[clap(short = 's', long = "source_dir")]
    pub source_dir: Option<PathBuf>,

    #[command(subcommand)]
    pub commands: Option<Commands>,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    #[clap(about = "Create a new feature")]
    NewFeature { feature_name: String },
    #[clap(about = "Create a new core")]
    NewCore {},
    #[clap(about = "Create a new application")]
    NewApplication {},
}
