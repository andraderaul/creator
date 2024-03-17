use anyhow::{anyhow, Ok, Result};
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, fs, path::PathBuf};

use crate::{
    file_utils::{create_file, create_folder},
    generator::Generator,
};

#[derive(Debug, Deserialize, Serialize)]
pub struct FileStructure {
    pub template: String,
    pub file: String,
}

pub type SubStructure = HashMap<String, FileStructure>;
pub type MainStructure = HashMap<String, SubStructure>;

#[derive(Debug, Serialize, Deserialize)]
struct Data {
    creator: MainStructure,
}

pub struct Creator {
    source: PathBuf,
    data: Data,
}

impl Creator {
    pub fn from_config(config: PathBuf, source: PathBuf) -> Self {
        if fs::metadata(&config).is_ok() {
            let contents = fs::read_to_string(&config);
            let contents = contents.unwrap_or(String::from("{\"creator\":{}}"));
            let data = serde_json::from_str(&contents);
            let data = data.unwrap_or(default_data());

            return Creator { source, data };
        }

        Creator {
            source,
            data: default_data(),
        }
    }

    pub fn create_feature(&self, key: &str, main_folder_name: &str) -> Result<()> {
        let feature_path = PathBuf::from(self.source.as_path())
            .join(key)
            .join(main_folder_name);
        self.create(key, feature_path)
    }

    pub fn create_core(&self, key: &str) -> Result<()> {
        let core_path = PathBuf::from(self.source.as_path()).join(key);
        self.create(key, core_path)
    }

    pub fn create_application(&self, key: &str) -> Result<()> {
        let application_path = PathBuf::from(self.source.as_path()).join(key);
        self.create(key, application_path)
    }

    pub fn create_component_module(
        &self,
        main_key: &str,
        feature_name: &str,
        sub_key: &str,
        component_name: &str,
    ) -> Result<()> {
        let sub = self.get_sub_structure(main_key)?;
        let file = self.get_file_structure(sub, sub_key)?;

        let template_path = PathBuf::from(&file.template);
        let component = PathBuf::from(&self.source)
            .join(&main_key)
            .join(&feature_name)
            .join(&sub_key)
            .join(component_name)
            .with_extension("tsx");

        let template = Generator::generate(&template_path, component_name.to_string())?;

        create_file(&component, template)?;

        Ok(())
    }

    pub fn log(&self) {
        println!("{:?}", &self.data);
    }

    fn create(&self, key: &str, path: PathBuf) -> Result<()> {
        let folder_structure = self.get_sub_structure(key)?;

        for (folder_name, folder_config) in folder_structure {
            let folder_path = path.join(folder_name);
            create_folder(&folder_path)?;

            let file_path = folder_path.join(&folder_config.file);
            let template_path = PathBuf::from(&folder_config.template);
            let template = Generator::generate(&template_path, folder_name.to_string())?;

            create_file(&file_path, template)?;
        }

        Ok(())
    }

    fn get_sub_structure(&self, key: &str) -> Result<&SubStructure> {
        if let Some(sub_structure) = self.data.creator.get(key) {
            return Ok(sub_structure);
        }

        Err(anyhow!(
            "Failed to retrieve substructure from the Creator for key '{}'. The key may be invalid or missing.",
            key
        ))
    }

    fn get_file_structure<'a>(
        &self,
        sub_structure: &'a SubStructure,
        key: &str,
    ) -> Result<&'a FileStructure> {
        if let Some(file_structure) = sub_structure.get(key) {
            return Ok(file_structure);
        }

        Err(anyhow!(
            "Failed to retrieve file structure from the Creator for key '{}'. The key may be invalid or missing.",
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
