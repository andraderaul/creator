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
    #[clap(about = "Create a new item in a category")]
    Create {
        #[clap(short = 'c', long = "category", help = "Category to create in")]
        category: Option<String>,

        #[clap(short = 'i', long = "item", help = "Item type to create")]
        item: Option<String>,

        #[clap(short = 'n', long = "name", help = "Name of the new item")]
        name: Option<String>,
    },

    #[clap(about = "List available categories and items from config")]
    List {
        #[clap(
            short = 'c',
            long = "category",
            help = "Show items for specific category"
        )]
        category: Option<String>,
    },

    #[clap(about = "Initialize a new project with preset configuration")]
    Init {
        #[clap(short = 'p', long = "preset", help = "Preset configuration to use")]
        preset: Option<String>,
    },
}

impl Commands {
    /// Get the primary action for this command
    pub fn action(&self) -> &'static str {
        match self {
            Commands::Create { .. } => "create",
            Commands::List { .. } => "list",
            Commands::Init { .. } => "init",
        }
    }

    /// Check if this is a create command
    pub fn is_create(&self) -> bool {
        matches!(self, Commands::Create { .. })
    }

    /// Check if this is a list command
    pub fn is_list(&self) -> bool {
        matches!(self, Commands::List { .. })
    }

    /// Check if this is an init command
    pub fn is_init(&self) -> bool {
        matches!(self, Commands::Init { .. })
    }
}
