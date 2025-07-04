use anyhow::{anyhow, Result};
use inquire::{validator::Validation, Select, Text};
use std::path::PathBuf;

use crate::config::ProjectConfig;
use crate::file_utils::{generate_template_name, to_kebab_case, is_valid_name};
use crate::generator::Generator;
use crate::opts::Commands;

pub struct CliEngine {
    config: ProjectConfig,
    source_dir: PathBuf,
}

impl CliEngine {
    /// Create new CLI engine with loaded config
    pub fn new(config: ProjectConfig, source_dir: PathBuf) -> Self {
        Self { config, source_dir }
    }

    /// Run interactive CLI to get user commands
    pub fn run_interactive(&self) -> Result<Commands> {
        println!("🚀 Creator v1.0 - Dynamic Config Loaded");
        println!("📋 Project: {}", self.config.project.name);

        // Discover available categories
        let categories = self.config.get_categories();
        if categories.is_empty() {
            return Err(anyhow!("No categories found in config"));
        }

        // Main action selection
        let actions = vec!["Create new item", "List structure", "Exit"];
        let selected_action = Select::new("What would you like to do?", actions)
            .prompt()
            .map_err(|_| anyhow!("Failed to select action"))?;

        match selected_action {
            "Create new item" => self.interactive_create(),
            "List structure" => Ok(Commands::List { category: None }),
            "Exit" => std::process::exit(0),
            _ => Err(anyhow!("Invalid action selected")),
        }
    }

    /// Interactive create flow - unified cohesive module API
    fn interactive_create(&self) -> Result<Commands> {
        println!("🏗️  Creating new item in cohesive module structure...");
        println!("💡 Format: module/item_type/name");
        println!();

        // Step 1: Get module name
        let module_name = Text::new("Enter module name:")
            .with_placeholder("e.g., cats, users, auth")
            .with_validator(|input: &str| {
                if input.trim().is_empty() {
                    Ok(Validation::Invalid("Module name cannot be empty".into()))
                } else if input.chars().any(|c| !c.is_alphanumeric() && c != '_' && c != '-') {
                    Ok(Validation::Invalid("Module name can only contain alphanumeric characters, underscore, and dash".into()))
                } else {
                    Ok(Validation::Valid)
                }
            })
            .prompt()
            .map_err(|_| anyhow!("Failed to get module name"))?;

        // Step 2: Collect all available item types from all categories
        let mut all_item_types = Vec::new();
        
        for category_name in self.config.get_categories() {
            if let Some(category) = self.config.get_category(&category_name) {
                // Add static items
                let static_items = category.get_item_names();
                for item in static_items {
                    all_item_types.push(format!("{} ({})", item, category_name));
                }
                
                // Add dynamic items
                if category.supports_dynamic_children() {
                    if let Some(default_structure) = category.get_default_structure() {
                        for item in default_structure.keys() {
                            all_item_types.push(format!("{} ({})", item, category_name));
                        }
                    }
                }
            }
        }

        if all_item_types.is_empty() {
            return Err(anyhow!("No item types found in any module"));
        }

        all_item_types.sort();

        // Step 3: Select item type
        let selected_item_display = Select::new("Select item type:", all_item_types)
            .prompt()
            .map_err(|_| anyhow!("Failed to select item type"))?;

        // Extract item type from "item_type (module)" format
        let item_type = selected_item_display
            .split(" (")
            .next()
            .ok_or_else(|| anyhow!("Invalid item type format"))?;

        // Step 4: Get item name
        let item_name = Text::new(&format!("Enter name for {}:", item_type))
            .with_placeholder("e.g., cat-list, user-profile")
            .with_validator(|input: &str| {
                if input.trim().is_empty() {
                    Ok(Validation::Invalid("Item name cannot be empty".into()))
                } else if input.chars().any(|c| !c.is_alphanumeric() && c != '_' && c != '-') {
                    Ok(Validation::Invalid("Item name can only contain alphanumeric characters, underscore, and dash".into()))
                } else {
                    Ok(Validation::Valid)
                }
            })
            .prompt()
            .map_err(|_| anyhow!("Failed to get item name"))?;

        // Build path in module/item_type/name format
        let path = format!("{}/{}/{}", module_name, item_type, item_name);

        println!();
        println!("📁 Will create: {}", path);

        Ok(Commands::Create { path })
    }

    /// Handle create command execution - unified API for cohesive modules
    pub fn handle_create(&self, cmd: Commands) -> Result<()> {
        if let Commands::Create { path } = cmd {
            println!("🏗️  Creating item from path: {}", path);
            
            // Parse path: module/item_type/name
            let parts: Vec<&str> = path.split('/').collect();
            if parts.len() != 3 {
                return Err(anyhow!(
                    "Invalid path format. Expected: module/item_type/name, got: {}\n💡 Example: cats/components/cat-list",
                    path
                ));
            }

            let first_part = parts[0];
            let second_part = parts[1];
            let third_part = parts[2];

            // Check if first part is a static category
            let (category_name, category, module_name, item_type, item_name) = 
                if let Some(category) = self.config.get_category(first_part) {
                    if !category.supports_dynamic_children() {
                        // Static category: category/item_type/item_name
                        (first_part.to_string(), category, first_part, second_part, third_part)
                    } else {
                        // Dynamic category specified: treat as module_name/item_type/item_name
                        let item_type = second_part;
                        let (cat_name, cat) = self.find_category_for_item_type(item_type)?;
                        (cat_name, cat, first_part, second_part, third_part)
                    }
                } else {
                    // Normal case: module_name/item_type/item_name
                    let item_type = second_part;
                    let (cat_name, cat) = self.find_category_for_item_type(item_type)?;
                    (cat_name, cat, first_part, second_part, third_part)
                };

            // Validate names contain only valid characters
            if !is_valid_name(module_name) {
                return Err(anyhow!(
                    "Invalid module name '{}'. Use only letters, numbers, hyphens, and underscores.",
                    module_name
                ));
            }
            
            if !is_valid_name(item_name) {
                return Err(anyhow!(
                    "Invalid item name '{}'. Use only letters, numbers, hyphens, and underscores.",
                    item_name
                ));
            }

            // Get item configuration
            let item_config = if let Some(static_item) = category.get_item(item_type) {
                static_item
            } else if category.supports_dynamic_children() {
                let default_structure = category.get_default_structure().ok_or_else(|| {
                    anyhow!("Category '{}' supports dynamic children but has no default structure", category_name)
                })?;

                default_structure.get(item_type).ok_or_else(|| {
                    anyhow!("Item type '{}' not found in default structure", item_type)
                })?
            } else {
                return Err(anyhow!(
                    "Item type '{}' not found in category '{}'",
                    item_type,
                    category_name
                ));
            };

            // Create the item using the appropriate structure
            if category.supports_dynamic_children() {
                // Dynamic category: category/module_name/item_type/item_name.ext
                self.create_cohesive_module_item(&category_name, module_name, item_type, item_name, item_config)?;
            } else {
                // Static category: category/item_type/item_name.ext
                self.create_static_category_item(&category_name, item_type, item_name, item_config)?;
            }

            println!(
                "✅ Successfully created {} '{}' in module '{}'",
                item_type, item_name, module_name
            );
        } else {
            return Err(anyhow!("Invalid command for create handler"));
        }

        Ok(())
    }

    /// Handle list command execution
    pub fn handle_list(&self, cmd: Commands) -> Result<()> {
        if let Commands::List { category } = cmd {
            if let Some(category_name) = category {
                // List specific category
                self.list_category(&category_name)?;
            } else {
                // List all categories
                self.list_all_categories()?;
            }
        } else {
            return Err(anyhow!("Invalid command for list handler"));
        }

        Ok(())
    }



    /// List all available categories
    fn list_all_categories(&self) -> Result<()> {
        println!("📋 Available modules in '{}':", self.config.project.name);
        println!();

        for category_name in self.config.get_categories() {
            if let Some(category) = self.config.get_category(&category_name) {
                println!("📁 {}", category_name);

                if let Some(description) = &category.description {
                    println!("   {}", description);
                }

                let static_items = category.get_item_names();
                if !static_items.is_empty() {
                    println!("   Static items: {}", static_items.join(", "));
                }

                if category.supports_dynamic_children() {
                    if let Some(default_structure) = category.get_default_structure() {
                        let dynamic_types: Vec<String> =
                            default_structure.keys().cloned().collect();
                        println!("   Dynamic types: {}", dynamic_types.join(", "));
                    }
                }

                println!();
            }
        }

        Ok(())
    }

    /// List items in specific category
    fn list_category(&self, category_name: &str) -> Result<()> {
        let category = self
            .config
            .get_category(category_name)
            .ok_or_else(|| anyhow!("Category '{}' not found", category_name))?;

        println!("📁 Module: {}", category_name);

        if let Some(description) = &category.description {
            println!("   {}", description);
        }

        println!();

        // List static items
        let static_items = category.get_item_names();
        if !static_items.is_empty() {
            println!("📄 Static items:");
            for item_name in static_items {
                if let Some(item) = category.get_item(&item_name) {
                    println!(
                        "   • {} (template: {}, ext: {})",
                        item_name, item.template, item.file_extension
                    );
                }
            }
            println!();
        }

        // List dynamic support
        if category.supports_dynamic_children() {
            println!("🔄 Dynamic creation supported");
            if let Some(default_structure) = category.get_default_structure() {
                println!("   Default types:");
                for (type_name, item) in default_structure {
                    println!(
                        "   • {} (template: {}, ext: {})",
                        type_name, item.template, item.file_extension
                    );
                }
            }
        }

        Ok(())
    }



    /// Create item in cohesive module structure: category/module_name/item_type/item_name.ext
    fn create_cohesive_module_item(
        &self,
        category: &str,
        module_name: &str,
        item_type: &str,
        item_name: &str,
        item_config: &crate::config::Item,
    ) -> Result<()> {
        use crate::file_utils::{create_file, create_folder};

        // Build path: source_dir/category/module_name/item_type/
        let item_path = self
            .source_dir
            .join(category)
            .join(to_kebab_case(module_name))
            .join(item_type);

        // Create folder structure
        create_folder(&item_path)?;

        // Generate file from template
        let template_path = PathBuf::from(&item_config.template);
        let file_path = item_path
            .join(to_kebab_case(item_name))
            .with_extension(&item_config.file_extension);

        let template_name = generate_template_name(item_type, item_name);
        let template_content = Generator::generate(&template_path, template_name)?;
        create_file(&file_path, template_content)?;

        Ok(())
    }

    /// Create item in static category structure: category/item_type/item_name.ext
    fn create_static_category_item(
        &self,
        category: &str,
        item_type: &str,
        item_name: &str,
        item_config: &crate::config::Item,
    ) -> Result<()> {
        use crate::file_utils::{create_file, create_folder};

        // Build path: source_dir/category/item_type/
        let item_path = self
            .source_dir
            .join(category)
            .join(item_type);

        // Create folder structure
        create_folder(&item_path)?;

        // Generate file from template
        let template_path = PathBuf::from(&item_config.template);
        let file_path = item_path
            .join(to_kebab_case(item_name))
            .with_extension(&item_config.file_extension);

        let template_name = generate_template_name(item_type, item_name);
        let template_content = Generator::generate(&template_path, template_name)?;
        create_file(&file_path, template_content)?;

        Ok(())
    }

    /// Find category that contains the specified item type
    fn find_category_for_item_type(&self, item_type: &str) -> Result<(String, &crate::config::Category)> {
        // First, check dynamic categories (they have priority for cohesive modules)
        for category_name in self.config.get_categories() {
            if let Some(category) = self.config.get_category(&category_name) {
                if category.supports_dynamic_children() {
                    if let Some(default_structure) = category.get_default_structure() {
                        if default_structure.contains_key(item_type) {
                            return Ok((category_name, category));
                        }
                    }
                }
            }
        }
        
        // Then check static categories
        for category_name in self.config.get_categories() {
            if let Some(category) = self.config.get_category(&category_name) {
                // Skip dynamic categories (already checked)
                if category.supports_dynamic_children() {
                    continue;
                }
                
                // Check static items
                if category.get_item(item_type).is_some() {
                    return Ok((category_name, category));
                }
            }
        }

        // Build helpful error message with available types
        let mut available_types = Vec::new();
        for category_name in self.config.get_categories() {
            if let Some(category) = self.config.get_category(&category_name) {
                let static_items = category.get_item_names();
                for item in static_items {
                    available_types.push(format!("{} (in {})", item, category_name));
                }
                
                if category.supports_dynamic_children() {
                    if let Some(default_structure) = category.get_default_structure() {
                        for item in default_structure.keys() {
                            available_types.push(format!("{} (in {})", item, category_name));
                        }
                    }
                }
            }
        }

        Err(anyhow!(
            "Item type '{}' not found in any module.\n💡 Available types:\n  {}",
            item_type,
            available_types.join("\n  ")
        ))
    }
}
