use anyhow::{anyhow, Context, Result};
use std::{fs, io::Write, path::Path};

use crate::creator::SubStructure;

// TODO: let this function generic with generic type
pub fn create_folders(path: &Path, config: &SubStructure) -> Result<()> {
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

pub fn create_file(file_path: &Path, content: String) -> Result<usize> {
    let mut file = match fs::File::create(&file_path) {
        Ok(f) => f,
        Err(_) => {
            return Err(anyhow!(format!(
                "Failed to create file '{}'",
                file_path.display()
            )))
        }
    };

    let result = match file.write(content.as_bytes()) {
        Ok(r) => r,
        Err(_) => {
            return Err(anyhow!(format!(
                "Failed to write content to file '{}'",
                file_path.display()
            )));
        }
    };

    Ok(result)
}
