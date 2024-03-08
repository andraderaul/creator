use anyhow::{anyhow, Result};
use inquire::Select;
use inquire::{validator::Validation, Text};

use std::path::PathBuf;

use crate::opts::{Commands, Opts};

#[derive(Debug)]
pub struct Config {
    pub commands: Commands,
    pub config: PathBuf,
    pub source_dir: PathBuf,
}

impl TryFrom<Opts> for Config {
    type Error = anyhow::Error;

    fn try_from(value: Opts) -> Result<Self> {
        let config = get_config(value.config)?;
        let source_dir = get_source_dir(value.source_dir)?;
        let commands = get_commands(value.commands)?;

        Ok(Config {
            config,
            source_dir,
            commands,
        })
    }
}

fn get_commands(commands: Option<Commands>) -> Result<Commands> {
    if let Some(c) = commands {
        return Ok(c);
    }

    // Fetch main command options (TODO: in the future, read from config.json)
    let options = get_main_commands_options();

    let selected_command = match Select::new("Select a command:", options).prompt() {
        Ok(command) => command,
        Err(_) => return Err(anyhow!("Failed to select a command. Please try again.")),
    };

    let ans = match selected_command {
        "new-feature" => get_feature_commands(),
        "new-core" => get_core_commands(),
        "new-application" => get_application_commands(),

        // This should not happen, as we're using Select with predefined options
        _ => return Err(anyhow!("Invalid command selected.")),
    };

    Ok(ans)
}

fn get_config(config: Option<PathBuf>) -> Result<PathBuf> {
    println!("aaaaaa {:?} aaaaas", config);
    if let Some(c) = config {
        return Ok(c);
    }

    //TODO: in the future, add autocomplete
    let config_path = match Text::new("Enter the path to the config file:")
        .with_placeholder("config.json")
        .prompt()
    {
        Ok(path) => PathBuf::from(&path),
        Err(_) => return Err(anyhow!("Failed to read the config file path.")),
    };

    Ok(config_path)
}

fn get_source_dir(source_dir: Option<PathBuf>) -> Result<PathBuf> {
    if let Some(s) = source_dir {
        return Ok(s);
    }

    //TODO: in the future, add autocomplete
    let source_path = match Text::new("Enter the path to the source directory:")
        .with_placeholder("src")
        .prompt()
    {
        Ok(path) => PathBuf::from(&path),
        Err(_) => return Err(anyhow!("Failed to read the source directory path.")),
    };

    Ok(source_path)
}

fn get_main_commands_options() -> Vec<&'static str> {
    vec!["new-feature", "new-core", "new-application"]
}

fn get_feature_commands() -> Commands {
    //TODO: in the future, add autocomplete
    let feature_name = match Text::new("Enter the name of the feature: ")
        .with_validator(|input: &str| {
            if input.chars().count() > 0 {
                Ok(Validation::Valid)
            } else {
                Ok(Validation::Invalid("Feature name cannot be empty.".into()))
            }
        })
        .with_placeholder("feature_name")
        .prompt()
    {
        Ok(name) => name,
        Err(_) => {
            eprintln!("Failed to read the feature name.");
            std::process::exit(1);
        }
    };

    Commands::NewFeature { feature_name }
}

fn get_core_commands() -> Commands {
    Commands::NewCore {}
}

fn get_application_commands() -> Commands {
    Commands::NewApplication {}
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_get_main_commands_options() {
        let options = get_main_commands_options();
        assert_eq!(options.len(), 3);
        assert!(options.contains(&"new-feature"));
        assert!(options.contains(&"new-core"));
        assert!(options.contains(&"new-application"));
    }

    #[test]
    fn test_get_config_ok() {
        let path = PathBuf::from("test_config_path".to_string());
        let arg = Some(path.clone());
        let ans = get_config(arg).unwrap();

        assert_eq!(path, ans);
    }

    #[test]
    fn get_source_dir_ok() {
        let path = PathBuf::from("source_dir_config_path".to_string());
        let arg = Some(path.clone());
        let ans = get_source_dir(arg).unwrap();

        assert_eq!(path, ans);
    }
}
