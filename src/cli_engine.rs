use anyhow::{anyhow, Result};
use inquire::{validator::Validation, Select, Text};
use std::path::PathBuf;

use crate::config::ProjectConfig;
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
        println!("🚀 Creator v2.0 - Dynamic Config Loaded");
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

    /// Interactive create flow
    fn interactive_create(&self) -> Result<Commands> {
        // Step 1: Select category
        let categories = self.config.get_categories();
        let category_name = Select::new("Select category:", categories)
            .prompt()
            .map_err(|_| anyhow!("Failed to select category"))?;

        let category = self
            .config
            .get_category(&category_name)
            .ok_or_else(|| anyhow!("Category '{}' not found", category_name))?;

        // Step 2: Determine if static or dynamic
        let available_items = category.get_item_names();
        let supports_dynamic = category.supports_dynamic_children();

        let mut options = available_items.clone();
        if supports_dynamic {
            options.push("Create new (dynamic)".to_string());
        }

        if options.is_empty() {
            return Err(anyhow!(
                "Category '{}' has no available items",
                category_name
            ));
        }

        // Step 3: Select item type
        let selected_item = Select::new("Select item type:", options)
            .prompt()
            .map_err(|_| anyhow!("Failed to select item type"))?;

        // Step 4: Handle dynamic creation or get name
        let (item_type, item_name) = if selected_item == "Create new (dynamic)" {
            // Dynamic creation - get both type and name
            if let Some(default_structure) = category.get_default_structure() {
                let default_types: Vec<String> = default_structure.keys().cloned().collect();

                let item_type = if default_types.len() == 1 {
                    default_types[0].clone()
                } else {
                    Select::new("Select default item type:", default_types)
                        .prompt()
                        .map_err(|_| anyhow!("Failed to select default item type"))?
                };

                let item_name = Text::new("Enter name for the new item:")
                    .with_validator(|input: &str| {
                        if input.trim().is_empty() {
                            Ok(Validation::Invalid("Name cannot be empty".into()))
                        } else if input.chars().any(|c| !c.is_alphanumeric() && c != '_' && c != '-') {
                            Ok(Validation::Invalid("Name can only contain alphanumeric characters, underscore, and dash".into()))
                        } else {
                            Ok(Validation::Valid)
                        }
                    })
                    .prompt()
                    .map_err(|_| anyhow!("Failed to get item name"))?;

                (item_type, item_name)
            } else {
                return Err(anyhow!(
                    "Category '{}' supports dynamic children but has no default structure",
                    category_name
                ));
            }
        } else {
            // Static item - just get name
            let item_name = Text::new(&format!("Enter name for {}:", selected_item))
                .with_validator(|input: &str| {
                    if input.trim().is_empty() {
                        Ok(Validation::Invalid("Name cannot be empty".into()))
                    } else if input
                        .chars()
                        .any(|c| !c.is_alphanumeric() && c != '_' && c != '-')
                    {
                        Ok(Validation::Invalid(
                            "Name can only contain alphanumeric characters, underscore, and dash"
                                .into(),
                        ))
                    } else {
                        Ok(Validation::Valid)
                    }
                })
                .prompt()
                .map_err(|_| anyhow!("Failed to get item name"))?;

            (selected_item, item_name)
        };

        Ok(Commands::Create {
            category: Some(category_name),
            item: Some(item_type),
            name: Some(item_name),
        })
    }

    /// Handle create command execution
    pub fn handle_create(&self, cmd: Commands) -> Result<()> {
        if let Commands::Create {
            category,
            item,
            name,
        } = cmd
        {
            let category_name = category.ok_or_else(|| anyhow!("Category is required"))?;
            let item_type = item.ok_or_else(|| anyhow!("Item type is required"))?;
            let item_name = name.ok_or_else(|| anyhow!("Item name is required"))?;

            println!(
                "🏗️  Creating {} '{}' in category '{}'...",
                item_type, item_name, category_name
            );

            // Get category and validate
            let category = self
                .config
                .get_category(&category_name)
                .ok_or_else(|| anyhow!("Category '{}' not found", category_name))?;

            // Determine item template
            let item_config = if let Some(static_item) = category.get_item(&item_type) {
                // Static item
                static_item
            } else if category.supports_dynamic_children() {
                // Dynamic item - use default structure
                let default_structure = category.get_default_structure().ok_or_else(|| {
                    anyhow!(
                        "Category '{}' supports dynamic children but has no default structure",
                        category_name
                    )
                })?;

                default_structure.get(&item_type).ok_or_else(|| {
                    anyhow!("Item type '{}' not found in default structure", item_type)
                })?
            } else {
                return Err(anyhow!(
                    "Item type '{}' not found in category '{}'",
                    item_type,
                    category_name
                ));
            };

            // Create the item using Creator
            self.create_item_with_config(&category_name, &item_type, &item_name, &item_config)?;

            println!(
                "✅ Successfully created {} '{}' in {}/{}",
                item_type, item_name, category_name, item_type
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
        println!("📋 Available categories in '{}':", self.config.project.name);
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

        println!("📁 Category: {}", category_name);

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

    /// Create item using the Creator with dynamic config
    fn create_item_with_config(
        &self,
        category: &str,
        item_type: &str,
        name: &str,
        item_config: &crate::config::Item,
    ) -> Result<()> {
        use crate::file_utils::{create_file, create_folder, to_kebab_case, to_pascal_case};
        use crate::generator::Generator;

        // Build path: source_dir/category/name/item_type
        let item_path = self
            .source_dir
            .join(category)
            .join(to_kebab_case(name))
            .join(item_type);

        // Create folder structure
        create_folder(&item_path)?;

        // Generate file from template
        let template_path = PathBuf::from(&item_config.template);
        let file_path = item_path
            .join("index")
            .with_extension(&item_config.file_extension);

        let template_content = Generator::generate(&template_path, to_pascal_case(name))?;
        create_file(&file_path, template_content)?;

        Ok(())
    }
}
