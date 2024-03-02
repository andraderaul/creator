use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};

pub type FeatureStructure = HashMap<String, FolderConfig>;

#[derive(Debug, Deserialize, Serialize)]
pub struct FeatureConfig {
    #[serde(rename = "feature_structure")]
    pub feature_structure: FeatureStructure,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct FolderConfig {
    pub file: String,
}

pub fn create_feature(feature_name: &str, feature_config: &FeatureConfig) -> Result<()> {
    let feature_path = PathBuf::from("features").join(feature_name);
    println!("create_feature feature_config {:?}", feature_config);

    create_folders(&feature_path, &feature_config.feature_structure)
        .with_context(|| format!("Failed to create folders for feature '{}'", feature_name))?;

    Ok(())
}

pub fn create_folders(path: &Path, config: &FeatureStructure) -> Result<()> {
    for (folder_name, folder_config) in config {
        let folder_path = path.join(folder_name);
        fs::create_dir_all(&folder_path)
            .with_context(|| format!("Failed to create folder '{}'", folder_path.display()))?;
        let file = &folder_config.file;
        create_files(&folder_path.join(file)).with_context(|| {
            format!(
                "Failed to create file in folder '{}'",
                folder_path.display()
            )
        })?;
    }

    Ok(())
}

pub fn create_files(file_path: &Path) -> Result<()> {
    let mut file = fs::File::create(file_path)
        .with_context(|| format!("Failed to create file '{}'", file_path.display()))?;
    let content = "Hello, Rust!";
    file.write_all(content.as_bytes())
        .with_context(|| format!("Failed to write content to file '{}'", file_path.display()))?;

    Ok(())
}

pub fn read_feature_config(config_path: &Path) -> Result<FeatureConfig> {
    let config_content = fs::read_to_string(config_path).with_context(|| {
        format!(
            "Failed to read content from file '{}'",
            config_path.display()
        )
    })?;
    let feature_config: FeatureConfig =
        serde_json::from_str(&config_content).with_context(|| {
            format!(
                "Failed to deserialize JSON from file '{}'",
                config_path.display()
            )
        })?;

    Ok(feature_config)
}
