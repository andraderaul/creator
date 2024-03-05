mod opts;

use anyhow::Result;
use creator::creator::Creator;
use opts::{Command, Opts};
use structopt::StructOpt;

fn main() -> Result<()> {
    let opt = Opts::from_args();
    let creator = Creator::from_config(opt.config, opt.pwd);

    match opt.command {
        Command::NewFeature { feature_name } => {
            creator.create("features", &feature_name)?;
            println!("Feature '{}' created successfully!", feature_name);
        }
    }

    Ok(())
}
