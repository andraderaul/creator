use anyhow::{anyhow, Ok, Result};
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, fs, path::PathBuf};

use crate::file_utils::create_folders;

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

    pub fn create_feature(&self, key: &str, main_folder_name: &str) -> Result<()> {
        let feature_path = PathBuf::from(key).join(main_folder_name);
        self.create(key, feature_path)
    }

    pub fn create_core(&self, key: &str) -> Result<()> {
        let core_path = PathBuf::from(key);
        self.create(key, core_path)
    }

    pub fn create_application(&self, key: &str) -> Result<()> {
        let application_path = PathBuf::from(key);
        self.create(key, application_path)
    }

    pub fn log(&self) {
        println!("{:?}", &self.data);
    }

    fn create(&self, key: &str, path: PathBuf) -> Result<()> {
        let folder_structure = self.get_sub_structure(key)?;

        create_folders(&path, folder_structure)
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
}

fn default_data() -> Data {
    Data {
        creator: HashMap::new(),
    }
}

#[cfg(test)]
mod test {}
