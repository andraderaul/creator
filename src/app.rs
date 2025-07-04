use anyhow::{anyhow, Result};
use inquire::Text;
use std::path::PathBuf;

use crate::cli_engine::CliEngine;
use crate::config::ProjectConfig;
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
        // Commands provided via CLI - validate config for all commands except Interactive
        match &c {
            Commands::Interactive => {
                // Interactive mode - validate config later when running
                return Ok(c);
            }
            _ => {
                // Other commands - validate config early
                let _project_config =
                    ProjectConfig::load_and_validate(config_path).map_err(|e| {
                        anyhow!(
                            "Config validation failed: {}. Please fix {} and try again.",
                            e,
                            config_path.display()
                        )
                    })?;
            }
        }

        return Ok(c);
    }

    // No commands provided - return error with helpful suggestions
    // This is now CLI-first: no automatic interactive mode
    Err(anyhow!(
        "No command specified. Creator requires explicit commands for automation-friendly operation.\n\n💡 Available commands:\n   creator create <path>        # Create new item\n   creator list                 # List available modules\n   creator init                 # Initialize configuration\n   creator interactive          # Run interactive mode\n   creator --help               # Show detailed help"
    ))
}

fn get_config_path(config: Option<PathBuf>) -> Result<PathBuf> {
    if let Some(c) = config {
        return Ok(c);
    }

    // Try to find config automatically
    let default_configs = [
        "config.json",
        "config-clean-architecture.json",
        "config-module-based.json",
    ];

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
        Commands::Interactive => {
            cli_engine.handle_interactive()?;
        }
    }

    Ok(())
}

/// Handle init command (create new config)
fn handle_init(preset: Option<&str>, config_path: &PathBuf) -> Result<()> {
    println!("🚀 Initializing new Creator project...");

    let template_config = match preset {
        Some("clean-architecture") => std::fs::read_to_string("config-clean-architecture.json")
            .map_err(|_| anyhow!("Clean architecture template not found"))?,
        Some("module-based") => std::fs::read_to_string("config-module-based.json")
            .map_err(|_| anyhow!("Module-based template not found"))?,
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
    fn test_cli_first_behavior_no_commands() {
        // Test that no commands results in helpful error, not interactive mode
        use std::fs;
        use tempfile::TempDir;

        let temp_dir = TempDir::new().unwrap();
        let config_path = temp_dir.path().join("config.json");

        // Create minimal valid config
        let config_content = r#"
        {
            "project": {
                "name": "test-project",
                "version": "1.0",
                "structure": {
                    "modules": {
                        "allow_dynamic_children": true,
                        "default_structure": {
                            "components": {
                                "template": "templates/components.hbs",
                                "file_extension": "tsx"
                            }
                        }
                    }
                }
            }
        }
        "#;
        fs::write(&config_path, config_content).unwrap();

        // Test CLI-first behavior: no commands = error, not interactive
        let result = get_commands(None, &config_path);
        assert!(result.is_err());

        let error_msg = result.unwrap_err().to_string();
        assert!(error_msg.contains("No command specified"));
        assert!(error_msg.contains("automation-friendly"));
        assert!(error_msg.contains("creator interactive"));
    }

    #[test]
    fn test_explicit_interactive_command() {
        use std::fs;
        use tempfile::TempDir;

        let temp_dir = TempDir::new().unwrap();
        let config_path = temp_dir.path().join("config.json");

        // Create minimal valid config
        let config_content = r#"
        {
            "project": {
                "name": "test-project",
                "version": "1.0",
                "structure": {
                    "modules": {
                        "allow_dynamic_children": true,
                        "default_structure": {
                            "components": {
                                "template": "templates/components.hbs",
                                "file_extension": "tsx"
                            }
                        }
                    }
                }
            }
        }
        "#;
        fs::write(&config_path, config_content).unwrap();

        // Test explicit interactive command
        let interactive_cmd = Some(Commands::Interactive);
        let result = get_commands(interactive_cmd, &config_path);
        assert!(result.is_ok());

        if let Ok(Commands::Interactive) = result {
            // Success - interactive command was recognized
        } else {
            panic!("Expected Interactive command");
        }
    }

    #[test]
    fn test_cli_commands_still_work() {
        use std::fs;
        use tempfile::TempDir;

        let temp_dir = TempDir::new().unwrap();
        let config_path = temp_dir.path().join("config.json");

        // Create minimal valid config
        let config_content = r#"
        {
            "project": {
                "name": "test-project",
                "version": "1.0",
                "structure": {
                    "modules": {
                        "allow_dynamic_children": true,
                        "default_structure": {
                            "components": {
                                "template": "templates/components.hbs",
                                "file_extension": "tsx"
                            }
                        }
                    }
                }
            }
        }
        "#;
        fs::write(&config_path, config_content).unwrap();

        // Test create command
        let create_cmd = Some(Commands::Create {
            path: "users/components/test".to_string(),
        });
        let result = get_commands(create_cmd, &config_path);
        assert!(result.is_ok());

        // Test list command
        let list_cmd = Some(Commands::List { category: None });
        let result = get_commands(list_cmd, &config_path);
        assert!(result.is_ok());

        // Test init command
        let init_cmd = Some(Commands::Init { preset: None });
        let result = get_commands(init_cmd, &config_path);
        assert!(result.is_ok());
    }

    #[test]
    fn test_automation_friendly_behavior() {
        use std::fs;
        use tempfile::TempDir;

        let temp_dir = TempDir::new().unwrap();
        let config_path = temp_dir.path().join("config.json");

        // Create minimal valid config
        let config_content = r#"
        {
            "project": {
                "name": "test-project",
                "version": "1.0",
                "structure": {
                    "modules": {
                        "allow_dynamic_children": true,
                        "default_structure": {
                            "components": {
                                "template": "templates/components.hbs",
                                "file_extension": "tsx"
                            }
                        }
                    }
                }
            }
        }
        "#;
        fs::write(&config_path, config_content).unwrap();

        // Simulate CI/CD scenario - script calls creator without commands
        let result = get_commands(None, &config_path);

        // Should fail fast with helpful error, not hang waiting for input
        assert!(result.is_err());

        // Error should be deterministic and automation-friendly
        let error_msg = result.unwrap_err().to_string();
        assert!(error_msg.contains("creator interactive")); // Should mention explicit interactive command
        assert!(error_msg.contains("automation-friendly")); // Should explain why it's designed this way
    }

    #[test]
    fn test_config_validation_still_works() {
        use std::fs;
        use tempfile::TempDir;

        let temp_dir = TempDir::new().unwrap();
        let config_path = temp_dir.path().join("invalid-config.json");

        // Create invalid config
        let invalid_config = r#"{ "invalid": "json" }"#;
        fs::write(&config_path, invalid_config).unwrap();

        // All commands except Interactive should validate config early
        let create_cmd = Some(Commands::Create {
            path: "users/components/test".to_string(),
        });
        let result = get_commands(create_cmd, &config_path);
        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("Config validation failed"));

        // Interactive command should not validate config early (deferred validation)
        let interactive_cmd = Some(Commands::Interactive);
        let result = get_commands(interactive_cmd, &config_path);
        assert!(result.is_ok()); // Should pass because validation is deferred
    }
}
