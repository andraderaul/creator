use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectConfig {
    pub project: ProjectInfo,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectInfo {
    pub name: String,
    pub version: String,
    pub structure: HashMap<String, Category>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Category {
    pub description: Option<String>,
    pub children: Option<HashMap<String, Item>>,
    pub allow_dynamic_children: Option<bool>,
    pub default_structure: Option<HashMap<String, Item>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Item {
    pub template: String,
    pub file_extension: String,
}

impl ProjectConfig {
    /// Load and validate project configuration from file
    pub fn load_and_validate(config_path: &PathBuf) -> Result<Self> {
        // Check if file exists
        if !config_path.exists() {
            return Err(anyhow!(
                "Config file not found at path: {}",
                config_path.display()
            ));
        }

        // Read file contents
        let contents = fs::read_to_string(config_path)
            .map_err(|e| anyhow!("Failed to read config file: {}", e))?;

        // Parse JSON
        let config: ProjectConfig = serde_json::from_str(&contents)
            .map_err(|e| anyhow!("Failed to parse config JSON: {}", e))?;

        // Validate configuration
        config.validate()?;

        Ok(config)
    }

    /// Validate the entire configuration
    pub fn validate(&self) -> Result<()> {
        // Validate project info
        if self.project.name.is_empty() {
            return Err(anyhow!("Project name cannot be empty"));
        }

        if self.project.version.is_empty() {
            return Err(anyhow!("Project version cannot be empty"));
        }

        if self.project.structure.is_empty() {
            return Err(anyhow!("Project structure cannot be empty"));
        }

        // Validate each category
        for (category_name, category) in &self.project.structure {
            category.validate(category_name)?;
        }

        Ok(())
    }

    /// Get available category names
    pub fn get_categories(&self) -> Vec<String> {
        self.project.structure.keys().cloned().collect()
    }

    /// Get category by name
    pub fn get_category(&self, name: &str) -> Option<&Category> {
        self.project.structure.get(name)
    }
}

impl Category {
    /// Validate category configuration
    pub fn validate(&self, category_name: &str) -> Result<()> {
        match (
            &self.children,
            &self.allow_dynamic_children,
            &self.default_structure,
        ) {
            // Static children only
            (Some(children), None, None) => {
                if children.is_empty() {
                    return Err(anyhow!(
                        "Category '{}' has empty children but no dynamic support",
                        category_name
                    ));
                }
                self.validate_items(children, category_name)?;
            }
            // Dynamic children with default structure
            (None, Some(true), Some(default_structure)) => {
                if default_structure.is_empty() {
                    return Err(anyhow!(
                        "Category '{}' allows dynamic children but has empty default structure",
                        category_name
                    ));
                }
                self.validate_items(default_structure, category_name)?;
            }
            // Static children + dynamic support
            (Some(children), Some(true), Some(default_structure)) => {
                self.validate_items(children, category_name)?;
                self.validate_items(default_structure, category_name)?;
            }
            // Invalid configurations
            (None, None, None) => {
                return Err(anyhow!(
                    "Category '{}' must have either children or dynamic support",
                    category_name
                ));
            }
            (None, Some(false), _) => {
                return Err(anyhow!(
                    "Category '{}' has dynamic children disabled but no static children",
                    category_name
                ));
            }
            (None, Some(true), None) => {
                return Err(anyhow!(
                    "Category '{}' allows dynamic children but has no default structure",
                    category_name
                ));
            }
            _ => {
                return Err(anyhow!(
                    "Category '{}' has invalid configuration",
                    category_name
                ));
            }
        }

        Ok(())
    }

    /// Validate items within a category
    fn validate_items(&self, items: &HashMap<String, Item>, category_name: &str) -> Result<()> {
        for (item_name, item) in items {
            item.validate(category_name, item_name)?;
        }
        Ok(())
    }

    /// Get available item names (static children only)
    pub fn get_item_names(&self) -> Vec<String> {
        self.children
            .as_ref()
            .map(|children| children.keys().cloned().collect())
            .unwrap_or_default()
    }

    /// Get item by name
    pub fn get_item(&self, name: &str) -> Option<&Item> {
        self.children.as_ref()?.get(name)
    }

    /// Check if category supports dynamic children
    pub fn supports_dynamic_children(&self) -> bool {
        self.allow_dynamic_children.unwrap_or(false)
    }

    /// Get default structure for dynamic children
    pub fn get_default_structure(&self) -> Option<&HashMap<String, Item>> {
        self.default_structure.as_ref()
    }
}

impl Item {
    /// Validate item configuration
    pub fn validate(&self, category_name: &str, item_name: &str) -> Result<()> {
        if self.template.is_empty() {
            return Err(anyhow!(
                "Item '{}' in category '{}' has empty template path",
                item_name,
                category_name
            ));
        }

        if self.file_extension.is_empty() {
            return Err(anyhow!(
                "Item '{}' in category '{}' has empty file extension",
                item_name,
                category_name
            ));
        }

        // Validate template file exists (optional - can be skipped for performance)
        // let template_path = PathBuf::from(&self.template);
        // if !template_path.exists() {
        //     return Err(anyhow!(
        //         "Template file '{}' for item '{}' in category '{}' does not exist",
        //         self.template,
        //         item_name,
        //         category_name
        //     ));
        // }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::NamedTempFile;

    #[test]
    fn test_valid_config_parsing() {
        let config_json = r#"
        {
            "project": {
                "name": "test-project",
                "version": "2.0",
                "structure": {
                    "features": {
                        "description": "Business features",
                        "children": {
                            "modules": {
                                "template": "templates/module.hbs",
                                "file_extension": "tsx"
                            }
                        }
                    }
                }
            }
        }
        "#;

        let config: ProjectConfig = serde_json::from_str(config_json).unwrap();
        assert_eq!(config.project.name, "test-project");
        assert_eq!(config.project.version, "2.0");
        assert!(config.project.structure.contains_key("features"));
        
        // Test validation
        config.validate().unwrap();
    }

    #[test]
    fn test_invalid_config_empty_name() {
        let config_json = r#"
        {
            "project": {
                "name": "",
                "version": "2.0",
                "structure": {
                    "features": {
                        "children": {
                            "modules": {
                                "template": "templates/module.hbs",
                                "file_extension": "tsx"
                            }
                        }
                    }
                }
            }
        }
        "#;

        let config: ProjectConfig = serde_json::from_str(config_json).unwrap();
        assert!(config.validate().is_err());
    }

    #[test]
    fn test_dynamic_category_validation() {
        let config_json = r#"
        {
            "project": {
                "name": "test-project",
                "version": "2.0",
                "structure": {
                    "features": {
                        "description": "Dynamic features",
                        "allow_dynamic_children": true,
                        "default_structure": {
                            "modules": {
                                "template": "templates/module.hbs",
                                "file_extension": "tsx"
                            }
                        }
                    }
                }
            }
        }
        "#;

        let config: ProjectConfig = serde_json::from_str(config_json).unwrap();
        config.validate().unwrap();
        
        let features = config.get_category("features").unwrap();
        assert!(features.supports_dynamic_children());
        assert!(features.get_default_structure().is_some());
    }

    #[test]
    fn test_mixed_category_validation() {
        let config_json = r#"
        {
            "project": {
                "name": "test-project",
                "version": "2.0",
                "structure": {
                    "features": {
                        "description": "Mixed features",
                        "children": {
                            "existing": {
                                "template": "templates/existing.hbs",
                                "file_extension": "tsx"
                            }
                        },
                        "allow_dynamic_children": true,
                        "default_structure": {
                            "modules": {
                                "template": "templates/module.hbs",
                                "file_extension": "tsx"
                            }
                        }
                    }
                }
            }
        }
        "#;

        let config: ProjectConfig = serde_json::from_str(config_json).unwrap();
        config.validate().unwrap();
    }

    #[test]
    fn test_file_loading() {
        let config_json = r#"
        {
            "project": {
                "name": "test-project",
                "version": "2.0",
                "structure": {
                    "features": {
                        "children": {
                            "modules": {
                                "template": "templates/module.hbs",
                                "file_extension": "tsx"
                            }
                        }
                    }
                }
            }
        }
        "#;

        let temp_file = NamedTempFile::new().unwrap();
        fs::write(temp_file.path(), config_json).unwrap();

        let config = ProjectConfig::load_and_validate(&temp_file.path().to_path_buf()).unwrap();
        assert_eq!(config.project.name, "test-project");
    }

    #[test]
    fn test_clean_architecture_config_example() {
        let config = ProjectConfig::load_and_validate(&PathBuf::from("config-clean-architecture.json")).unwrap();
        
        // Validate basic project info
        assert_eq!(config.project.name, "my-react-native-clean-app");
        assert_eq!(config.project.version, "2.0");
        
        // Test categories
        let categories = config.get_categories();
        assert!(categories.contains(&"infra".to_string()));
        assert!(categories.contains(&"features".to_string()));
        assert!(categories.contains(&"pages".to_string()));
        assert!(categories.contains(&"core".to_string()));
        
        // Test features category (dynamic)
        let features = config.get_category("features").unwrap();
        assert!(features.supports_dynamic_children());
        assert!(features.get_default_structure().is_some());
        assert_eq!(features.description, Some("Business features with dynamic creation support".to_string()));
        
        // Test pages category (static)
        let pages = config.get_category("pages").unwrap();
        assert!(!pages.supports_dynamic_children());
        let page_items = pages.get_item_names();
        assert!(page_items.contains(&"dashboard".to_string()));
        assert!(page_items.contains(&"login".to_string()));
        assert!(page_items.contains(&"profile".to_string()));
    }

    #[test]
    fn test_module_based_config_example() {
        let config = ProjectConfig::load_and_validate(&PathBuf::from("config-module-based.json")).unwrap();
        
        // Validate basic project info
        assert_eq!(config.project.name, "my-react-native-modular-app");
        assert_eq!(config.project.version, "2.0");
        
        // Test categories
        let categories = config.get_categories();
        assert!(categories.contains(&"application".to_string()));
        assert!(categories.contains(&"modules".to_string()));
        assert!(categories.contains(&"shared".to_string()));
        assert!(categories.contains(&"external".to_string()));
        
        // Test modules category (fully dynamic)
        let modules = config.get_category("modules").unwrap();
        assert!(modules.supports_dynamic_children());
        assert!(modules.get_default_structure().is_some());
        
        // Test external category (mixed: static + dynamic)
        let external = config.get_category("external").unwrap();
        assert!(external.supports_dynamic_children());
        assert!(external.get_default_structure().is_some());
        let external_items = external.get_item_names();
        assert!(external_items.contains(&"apis".to_string()));
        assert!(external_items.contains(&"clients".to_string()));
    }

    #[test]
    fn test_config_api_usage_patterns() {
        let config = ProjectConfig::load_and_validate(&PathBuf::from("config-clean-architecture.json")).unwrap();
        
        // Test typical CLI usage patterns
        
        // 1. List available categories
        let categories = config.get_categories();
        assert!(!categories.is_empty());
        
        // 2. Get category details
        let features = config.get_category("features").unwrap();
        assert!(features.description.is_some());
        
        // 3. Check if dynamic children are supported
        if features.supports_dynamic_children() {
            let default_structure = features.get_default_structure().unwrap();
            assert!(!default_structure.is_empty());
        }
        
        // 4. Get static items if available
        let pages = config.get_category("pages").unwrap();
        let page_items = pages.get_item_names();
        if !page_items.is_empty() {
            let dashboard = pages.get_item("dashboard").unwrap();
            assert!(!dashboard.template.is_empty());
            assert!(!dashboard.file_extension.is_empty());
        }
    }
} 