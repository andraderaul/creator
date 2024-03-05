use std::path::PathBuf;

use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(
    name = "feature-cli",
    about = "CLI tool for creating React Native feature folders"
)]
pub struct Opt {
    #[structopt(subcommand)]
    pub command: Command,

    #[structopt(
        short = "c",
        long = "config",
        default_value = "config.json",
        parse(from_os_str)
    )]
    pub config_file: PathBuf,
}

#[derive(StructOpt, Debug)]
pub enum Command {
    #[structopt(about = "Create a new feature")]
    NewFeature {
        #[structopt(help = "Feature name")]
        feature_name: String,
    },
}
