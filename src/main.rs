mod opts;

use anyhow::Result;
use creator::creator::Creator;
use opts::{Command, Opts};
use structopt::StructOpt;

fn main() -> Result<()> {
    let opt = Opts::from_args();
    let creator = Creator::from_config(opt.config, opt.sourc_dir);

    match opt.command {
        Command::NewFeature { feature_name } => {
            creator.create_feature("features", &feature_name)?;

            println!("Feature '{}' created successfully!", feature_name);
        }
        Command::NewCore {} => {
            creator.create_core("core")?;
            println!("Core created successfully!",);
        }
        Command::NewApplication {} => {
            creator.create_application("application")?;
            println!("Application created successfully!",);
        }
    }

    Ok(())
}
