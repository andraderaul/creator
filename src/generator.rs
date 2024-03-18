use anyhow::{anyhow, Result};
use handlebars::Handlebars;
use std::{collections::BTreeMap, fs, path::PathBuf};

pub struct Generator {}

impl Generator {
    pub fn generate(path: &PathBuf, name: String) -> Result<String> {
        let source = match fs::read_to_string(&path) {
            Ok(s) => s,
            Err(err) => {
                println!(
                    "[warn] Failed to read the source directory path '{}': {}",
                    path.display(),
                    err
                );

                String::from("export function {{templateName}}(){}")
            }
        };

        let mut handlebars = Handlebars::new();

        handlebars
            .register_template_string("template", &source)
            .map_err(|err| {
                anyhow!(
                    "Cannot register template string '{}' in Handlebars: {}",
                    source,
                    err
                )
            })?;

        let mut data = BTreeMap::new();
        data.insert("templateName".to_string(), name);

        let result = handlebars
            .render("template", &data)
            .map_err(|err| anyhow!("Cannot render the template string: '{:?}' {}", data, err))?;

        return Ok(result);
    }
}
