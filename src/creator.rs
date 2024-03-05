use anyhow::{anyhow, Context, Ok, Result};
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    fs,
    io::Write,
    path::{Path, PathBuf},
};

#[derive(Debug, Deserialize, Serialize)]
pub struct FileStructure {
    pub file: String,
}

pub type SubStructure = HashMap<String, FileStructure>;
pub type MainStructure = HashMap<String, SubStructure>;

#[derive(Debug, Serialize, Deserialize)]
struct Data {
    creator: MainStructure,
}

pub struct Creator {
    config: PathBuf,
    pwd: PathBuf,
    data: Data,
}

impl Creator {
    pub fn from_config(config: PathBuf, pwd: PathBuf) -> Self {
        if fs::metadata(&config).is_ok() {
            let contents = fs::read_to_string(&config);
            let contents = contents.unwrap_or(String::from("{\"creator\":{}}"));
            let data = serde_json::from_str(&contents);
            let data = data.unwrap_or(default_data());

            return Creator { config, pwd, data };
        }

        Creator {
            config,
            pwd,
            data: default_data(),
        }
    }

    fn get_sub_structure(&self, key: &str) -> Result<&SubStructure> {
        if let Some(sub_structure) = self.data.creator.get(key) {
            return Ok(sub_structure);
        }

        Err(anyhow!(
            "creator get sub structure received an invalid key {}",
            key
        ))
    }

    pub fn create(&self, key: &str, main_folder_name: &str) -> Result<()> {
        let feature_path = PathBuf::from(key).join(main_folder_name);
        let folder_structure = self.get_sub_structure(key)?;

        create_folders(&feature_path, folder_structure)
    }

    pub fn log(&self) {
        println!("{:?}", &self.data);
    }
}

fn default_data() -> Data {
    Data {
        creator: HashMap::new(),
    }
}

// move this to a new file
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

#[cfg(test)]
mod test {}
