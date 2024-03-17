use anyhow::{anyhow, Result};
use std::{fs, io::Write, path::Path};

pub fn create_folder(folder_path: &Path) -> Result<()> {
    match fs::create_dir_all(&folder_path) {
        Ok(r) => r,
        Err(_) => {
            return Err(anyhow!(format!(
                "Failed to create folder '{}'",
                folder_path.display()
            )))
        }
    };

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
