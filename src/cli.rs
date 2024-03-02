use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(
    name = "feature-cli",
    about = "CLI tool for creating React Native feature folders"
)]
pub struct Opt {
    #[structopt(subcommand)]
    pub command: Command,
}

#[derive(StructOpt, Debug)]
pub enum Command {
    #[structopt(about = "Create a new feature")]
    NewFeature {
        #[structopt(help = "Feature name")]
        feature_name: String,
    },
}
