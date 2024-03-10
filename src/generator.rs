use anyhow::{anyhow, Result};
use handlebars::Handlebars;
use std::{collections::BTreeMap, fs, path::PathBuf};

pub struct Generator {}

impl Generator {
    pub fn generate(path: &PathBuf, name: String) -> Result<String> {
        //need improvements
        if fs::metadata(&path).is_ok() {
            let mut handlebars = Handlebars::new();

            let source = fs::read_to_string(&path);
            let source = source.unwrap();
            handlebars
                .register_template_string("component", source)
                .unwrap();

            let mut data = BTreeMap::new();
            data.insert("componentName".to_string(), name);
            let result = handlebars.render("component", &data)?;

            return Ok(result);
        }

        Err(anyhow!("Failed to read the source directory path."))
    }
}
