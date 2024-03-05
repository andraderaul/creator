mod cli;
mod feature;

use anyhow::{Context, Result};
use cli::{Command, Opt};
use feature::{create_feature, read_feature_config};
use std::path::PathBuf;
use structopt::StructOpt;

fn main() -> Result<()> {
    let opt = Opt::from_args();

    match opt.command {
        Command::NewFeature { feature_name } => {
            let config_path = PathBuf::from(opt.config_file);
            let feature_config = read_feature_config(&config_path)
                .with_context(|| format!("Failed to read feature config from {:?}", config_path))?;

            create_feature(&feature_name, &feature_config)
                .with_context(|| format!("Failed to create feature '{}'", feature_name))?;
            println!("Feature '{}' created successfully!", feature_name);
        }
    }

    Ok(())
}
