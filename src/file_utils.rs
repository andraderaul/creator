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

pub fn to_kebab_case(input: &str) -> String {
    input
        .chars()
        .collect::<String>()
        .split(|c: char| c.is_whitespace() || c == '_')
        .map(|word| word.to_lowercase())
        .filter(|word| !word.is_empty())
        .collect::<Vec<_>>()
        .join("-")
}

pub fn to_pascal_case(input: &str) -> String {
    // Handle camelCase, kebab-case, snake_case, and spaces
    let mut result = String::new();
    let mut current_word = String::new();
    
    for ch in input.chars() {
        if ch.is_whitespace() || ch == '-' || ch == '_' {
            if !current_word.is_empty() {
                result.push_str(&capitalize_word(&current_word));
                current_word.clear();
            }
        } else if ch.is_uppercase() && !current_word.is_empty() {
            // Handle camelCase - when we hit uppercase, finish current word
            result.push_str(&capitalize_word(&current_word));
            current_word.clear();
            current_word.push(ch);
        } else {
            current_word.push(ch);
        }
    }
    
    // Handle the last word
    if !current_word.is_empty() {
        result.push_str(&capitalize_word(&current_word));
    }
    
    result
}

fn capitalize_word(word: &str) -> String {
    if word.is_empty() {
        return String::new();
    }
    let mut chars = word.chars();
    match chars.next() {
        None => String::new(),
        Some(first) => {
            let rest = chars.collect::<String>().to_lowercase();
            format!("{}{}", first.to_uppercase(), rest)
        }
    }
}

pub fn to_camel_case(input: &str) -> String {
    let pascal = to_pascal_case(input);
    if let Some(first_char) = pascal.chars().next() {
        format!("{}{}", first_char.to_lowercase(), &pascal[1..])
    } else {
        pascal
    }
}

/// Generate template name based on item type and name
pub fn generate_template_name(item_type: &str, name: &str) -> String {
    match item_type.to_lowercase().as_str() {
        "hooks" => {
            // For hooks: remove "use-" prefix if present, then PascalCase
            let clean_name = if name.starts_with("use-") {
                &name[4..] // Remove "use-" prefix
            } else {
                name
            };
            to_pascal_case(clean_name)
        }
        "components" | "containers" | "screens" | "pages" => {
            // For components: PascalCase
            to_pascal_case(name)
        }
        "services" => {
            // For services: PascalCaseService
            format!("{}Service", to_pascal_case(name))
        }
        "types" => {
            // For types: PascalCaseType
            format!("{}Type", to_pascal_case(name))
        }
        _ => {
            // Default: PascalCase
            to_pascal_case(name)
        }
    }
}

/// Validates if a name contains only valid characters for file/directory names
/// Allows: letters, numbers, hyphens, underscores
/// Rejects: special characters like @, #, $, /, \, etc.
pub fn is_valid_name(name: &str) -> bool {
    if name.is_empty() {
        return false;
    }
    
    name.chars().all(|c| c.is_alphanumeric() || c == '-' || c == '_')
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_to_kebab_case() {
        let inputs = vec![
            ("nav bar", "nav-bar"),
            ("Nav Bar", "nav-bar"),
            ("nav_bar", "nav-bar"),
            ("navBar", "navbar"),
            ("nav-bar", "nav-bar"),
        ];

        for (input, expected) in inputs {
            assert_eq!(to_kebab_case(input), expected);
        }
    }

    #[test]
    fn test_to_pascal_case() {
        let inputs = vec![
            ("nav bar", "NavBar"),
            ("Nav Bar", "NavBar"),
            ("nav_bar", "NavBar"),
            ("nav-bar", "NavBar"),
            ("navBar", "NavBar"),
            ("cat-list", "CatList"),
            ("user-auth", "UserAuth"),
        ];

        for (input, expected) in inputs {
            assert_eq!(to_pascal_case(input), expected);
        }
    }

    #[test]
    fn test_to_camel_case() {
        let inputs = vec![
            ("nav bar", "navBar"),
            ("Nav Bar", "navBar"),
            ("nav_bar", "navBar"),
            ("nav-bar", "navBar"),
            ("cat-list", "catList"),
        ];

        for (input, expected) in inputs {
            assert_eq!(to_camel_case(input), expected);
        }
    }

    #[test]
    fn test_generate_template_name() {
        // Test hooks
        assert_eq!(generate_template_name("hooks", "cat-list"), "CatList");
        assert_eq!(generate_template_name("hooks", "user-auth"), "UserAuth");
        assert_eq!(generate_template_name("hooks", "use-cats"), "Cats");
        assert_eq!(generate_template_name("hooks", "use-user-data"), "UserData");
        
        // Test components
        assert_eq!(generate_template_name("components", "cat-list"), "CatList");
        assert_eq!(generate_template_name("components", "user-profile"), "UserProfile");
        
        // Test services
        assert_eq!(generate_template_name("services", "cat-list"), "CatListService");
        assert_eq!(generate_template_name("services", "user-auth"), "UserAuthService");
        
        // Test types
        assert_eq!(generate_template_name("types", "user-data"), "UserDataType");
        
        // Test default
        assert_eq!(generate_template_name("utils", "api-client"), "ApiClient");
    }
}
