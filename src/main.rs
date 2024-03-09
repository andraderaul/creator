use anyhow::Result;
use clap::Parser;
use creator::{
    config::Config,
    creator::Creator,
    opts::{Commands, Opts},
};

fn main() -> Result<()> {
    let config: Config = Opts::parse().try_into()?;

    println!("avemaria {:?}", config);
    let creator = Creator::from_config(config.config, config.source_dir);

    match config.commands {
        Commands::NewFeature { feature_name } => {
            creator.create_feature("features", &feature_name)?;

            println!("Feature '{}' created successfully!", feature_name);
        }
        Commands::NewCore {} => {
            creator.create_core("core")?;

            println!("Core created successfully!",);
        }
        Commands::NewApplication {} => {
            creator.create_application("application")?;

            println!("Application created successfully!",);
        }
    }

    Ok(())
}
