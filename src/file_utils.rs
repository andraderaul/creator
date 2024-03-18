use anyhow::{anyhow, Result};
use std::{fs, io::Write, path::Path};

pub fn create_folder(folder_path: &Path) -> Result<()> {
    fs::create_dir_all(folder_path).map_err(|err| {
        anyhow!(
            "Failed to create folder '{}': {}",
            folder_path.display(),
            err
        )
    })
}

pub fn create_file(file_path: &Path, content: String) -> Result<usize> {
    let mut file = fs::File::create(file_path)
        .map_err(|err| anyhow!("Failed to create file '{}': {}", file_path.display(), err))?;

    file.write(content.as_bytes()).map_err(|err| {
        anyhow!(
            "Failed to write content to file '{}': {}",
            file_path.display(),
            err
        )
    })
}
