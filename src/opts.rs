use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
#[clap(
    about = "CLI tool for creating cohesive module structures in projects",
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
    #[clap(about = "Create a new item in a module (module/item_type/name)")]
    Create {
        #[clap(help = "Path in format: module/item_type/name")]
        path: String,
    },

    #[clap(about = "List available modules and items from config")]
    List {
        #[clap(short = 'm', long = "module", help = "Show items for specific module")]
        category: Option<String>,
    },

    #[clap(about = "Initialize a new project with preset configuration")]
    Init {
        #[clap(short = 'p', long = "preset", help = "Preset configuration to use")]
        preset: Option<String>,
    },

    #[clap(about = "Run interactive mode for guided project setup")]
    Interactive,
}
