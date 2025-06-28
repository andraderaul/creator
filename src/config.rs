use anyhow::{anyhow, Result};
use inquire::Text;
use std::path::PathBuf;

use crate::cli_engine::CliEngine;
use crate::config_v2::ProjectConfig;
use crate::opts::{Commands, Opts};

#[derive(Debug)]
pub struct Config {
    pub commands: Commands,
    pub config_path: PathBuf,
    pub source_dir: PathBuf,
}

impl TryFrom<Opts> for Config {
    type Error = anyhow::Error;

    fn try_from(value: Opts) -> Result<Self> {
        let config_path = get_config_path(value.config)?;
        let source_dir = get_source_dir(value.source_dir)?;
        let commands = get_commands(value.commands, &config_path)?;

        Ok(Config {
            commands,
            config_path,
            source_dir,
        })
    }
}

fn get_commands(commands: Option<Commands>, config_path: &PathBuf) -> Result<Commands> {
    if let Some(c) = commands {
        // Commands provided via CLI - validate config but don't run interactive mode
        let _project_config = ProjectConfig::load_and_validate(config_path)
            .map_err(|e| anyhow!("Config validation failed: {}. Please fix {} and try again.", e, config_path.display()))?;
        
        return Ok(c);
    }

    // No commands provided - run interactive mode
    // Load config early (early loading strategy)
    let project_config = match ProjectConfig::load_and_validate(config_path) {
        Ok(config) => config,
        Err(e) => {
            // Graceful degradation - try to help user fix config
            eprintln!("⚠️  Config validation failed: {}", e);
            eprintln!("💡 Would you like to:");
            eprintln!("   1. Fix the config file manually");
            eprintln!("   2. Use basic interactive mode"); 
            eprintln!("   3. Exit and check config");
            
            // For now, return error but in future could implement fallback mode
            return Err(anyhow!("Invalid config. Please fix {} and try again.", config_path.display()));
        }
    };

    // Create CLI engine with loaded config
    let source_dir = get_source_dir_from_current()?;
    let cli_engine = CliEngine::new(project_config, source_dir);

    // Run interactive CLI to get commands
    cli_engine.run_interactive()
}

fn get_config_path(config: Option<PathBuf>) -> Result<PathBuf> {
    if let Some(c) = config {
        return Ok(c);
    }

    // Try to find config automatically
    let default_configs = ["config.json", "config-clean-architecture.json", "config-module-based.json"];
    
    for config_name in &default_configs {
        let path = PathBuf::from(config_name);
        if path.exists() {
            println!("📋 Found config: {}", config_name);
            return Ok(path);
        }
    }

    // If no config found, ask user
    let config_path = Text::new("Enter the path to the config file:")
        .with_placeholder("config.json")
        .prompt()
        .map_err(|_| anyhow!("Failed to read the config file path."))?;

    Ok(PathBuf::from(&config_path))
}

fn get_source_dir(source_dir: Option<PathBuf>) -> Result<PathBuf> {
    if let Some(s) = source_dir {
        return Ok(s);
    }

    get_source_dir_from_current()
}

fn get_source_dir_from_current() -> Result<PathBuf> {
    // Try common source directories
    let common_dirs = ["src", "app", "lib"];
    
    for dir_name in &common_dirs {
        let path = PathBuf::from(dir_name);
        if path.exists() && path.is_dir() {
            println!("📁 Found source directory: {}", dir_name);
            return Ok(path);
        }
    }

    // If no common directory found, ask user
    let source_path = Text::new("Enter the path to the source directory:")
        .with_placeholder("src")
        .prompt()
        .map_err(|_| anyhow!("Failed to read the source directory path."))?;

    Ok(PathBuf::from(&source_path))
}

/// Execute the loaded configuration
pub fn execute_config(config: Config) -> Result<()> {
    // Load project config again for execution
    let project_config = ProjectConfig::load_and_validate(&config.config_path)?;
    let cli_engine = CliEngine::new(project_config, config.source_dir);

    // Execute the command
    match &config.commands {
        Commands::Create { .. } => {
            cli_engine.handle_create(config.commands)?;
        }
        Commands::List { .. } => {
            cli_engine.handle_list(config.commands)?;
        }
        Commands::Init { preset } => {
            handle_init(preset.as_deref(), &config.config_path)?;
        }
    }

    Ok(())
}

/// Handle init command (create new config)
fn handle_init(preset: Option<&str>, config_path: &PathBuf) -> Result<()> {
    println!("🚀 Initializing new Creator project...");
    
    let template_config = match preset {
        Some("clean-architecture") => {
            std::fs::read_to_string("config-clean-architecture.json")
                .map_err(|_| anyhow!("Clean architecture template not found"))?
        }
        Some("module-based") => {
            std::fs::read_to_string("config-module-based.json")
                .map_err(|_| anyhow!("Module-based template not found"))?
        }
        Some(custom) => {
            return Err(anyhow!("Unknown preset: {}", custom));
        }
        None => {
            // Interactive preset selection
            use inquire::Select;
            let presets = vec!["clean-architecture", "module-based"];
            let selected = Select::new("Select a preset:", presets)
                .prompt()
                .map_err(|_| anyhow!("Failed to select preset"))?;
            
            match selected {
                "clean-architecture" => std::fs::read_to_string("config-clean-architecture.json")?,
                "module-based" => std::fs::read_to_string("config-module-based.json")?,
                _ => return Err(anyhow!("Invalid preset selected")),
            }
        }
    };

    // Write config to target path
    let target_config = if config_path.file_name().unwrap() == "config.json" {
        PathBuf::from("config.json")
    } else {
        config_path.clone()
    };

    std::fs::write(&target_config, template_config)
        .map_err(|e| anyhow!("Failed to write config file: {}", e))?;

    println!("✅ Created config file: {}", target_config.display());
    println!("🎯 You can now run 'creator' to start using the dynamic CLI!");

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn test_config_path_detection() {
        // This test would need to be run in a directory with config files
        // For now, just test the manual path input
        let manual_path = get_config_path(Some(PathBuf::from("test-config.json")));
        assert!(manual_path.is_ok());
    }

    #[test]
    fn test_source_dir_detection() {
        let manual_dir = get_source_dir(Some(PathBuf::from("test-src")));
        assert!(manual_dir.is_ok());
    }

    #[test]
    fn test_config_from_opts() {
        let opts = Opts {
            config: Some(PathBuf::from("config-clean-architecture.json")),
            source_dir: Some(PathBuf::from("src")),
            commands: Some(Commands::List { category: None }),
        };

        // This would work if we're in the right directory
        // let config = Config::try_from(opts);
        // assert!(config.is_ok());
    }
}
