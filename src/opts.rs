use std::path::PathBuf;

use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(
    name = "creator-cli",
    about = "CLI tool for creating React Native structure folders"
)]
pub struct Opts {
    #[structopt(subcommand)]
    pub command: Command,

    #[structopt(
        short = "c",
        long = "config",
        default_value = "config.json",
        parse(from_os_str)
    )]
    pub config: PathBuf,

    #[structopt(
        short = "s",
        long = "source_dir",
        default_value = ".",
        parse(from_os_str)
    )]
    pub sourc_dir: PathBuf,
}

#[derive(StructOpt, Debug)]
pub enum Command {
    #[structopt(about = "Create a new feature")]
    NewFeature {
        #[structopt(help = "Feature name")]
        feature_name: String,
    },
    #[structopt(about = "Create a new core")]
    NewCore {},
    #[structopt(about = "Create a new core")]
    NewApplication {},
}
