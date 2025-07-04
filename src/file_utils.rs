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

    #[test]
    fn test_is_valid_name() {
        // Valid names
        assert!(is_valid_name("user"));
        assert!(is_valid_name("user-profile"));
        assert!(is_valid_name("user_profile"));
        assert!(is_valid_name("UserProfile"));
        assert!(is_valid_name("user123"));
        assert!(is_valid_name("123user"));
        assert!(is_valid_name("a"));
        assert!(is_valid_name("ABC123_test-name"));

        // Invalid names
        assert!(!is_valid_name(""));  // Empty
        assert!(!is_valid_name("user profile"));  // Space
        assert!(!is_valid_name("user@profile"));  // @
        assert!(!is_valid_name("user.profile"));  // .
        assert!(!is_valid_name("user/profile"));  // /
        assert!(!is_valid_name("user\\profile"));  // \
        assert!(!is_valid_name("user#profile"));  // #
        assert!(!is_valid_name("user$profile"));  // $
        assert!(!is_valid_name("user%profile"));  // %
        assert!(!is_valid_name("user+profile"));  // +
        assert!(!is_valid_name("user=profile"));  // =
        assert!(!is_valid_name("user[profile]"));  // brackets
        assert!(!is_valid_name("user{profile}"));  // braces
        assert!(!is_valid_name("user(profile)"));  // parentheses
        assert!(!is_valid_name("user;profile"));  // semicolon
        assert!(!is_valid_name("user:profile"));  // colon
        assert!(!is_valid_name("user,profile"));  // comma
        assert!(!is_valid_name("user<profile>"));  // angle brackets
        assert!(!is_valid_name("user'profile"));  // quote
        assert!(!is_valid_name("user\"profile"));  // double quote
    }

    #[test]
    fn test_to_kebab_case_edge_cases() {
        // Empty and single characters
        assert_eq!(to_kebab_case(""), "");
        assert_eq!(to_kebab_case("a"), "a");
        assert_eq!(to_kebab_case("A"), "a");
        
        // Multiple spaces and underscores
        assert_eq!(to_kebab_case("   "), "");
        assert_eq!(to_kebab_case("a   b"), "a-b");
        assert_eq!(to_kebab_case("a___b"), "a-b");
        assert_eq!(to_kebab_case("a _ _ b"), "a-b");
        
        // Mixed separators
        assert_eq!(to_kebab_case("user_name space"), "user-name-space");
        assert_eq!(to_kebab_case(" user name "), "user-name");
        assert_eq!(to_kebab_case("_user_name_"), "user-name");
        
        // Numbers
        assert_eq!(to_kebab_case("api2Client"), "api2client");
        assert_eq!(to_kebab_case("user123Profile"), "user123profile");
        
        // Already kebab case
        assert_eq!(to_kebab_case("user-profile"), "user-profile");
        assert_eq!(to_kebab_case("already-kebab-case"), "already-kebab-case");
    }

    #[test]
    fn test_to_pascal_case_edge_cases() {
        // Empty and single characters
        assert_eq!(to_pascal_case(""), "");
        assert_eq!(to_pascal_case("a"), "A");
        assert_eq!(to_pascal_case("A"), "A");
        
        // Multiple separators
        assert_eq!(to_pascal_case("   "), "");
        assert_eq!(to_pascal_case("a   b"), "AB");
        assert_eq!(to_pascal_case("a---b"), "AB");
        assert_eq!(to_pascal_case("a___b"), "AB");
        
        // Mixed separators
        assert_eq!(to_pascal_case("user_name-space test"), "UserNameSpaceTest");
        assert_eq!(to_pascal_case(" user name "), "UserName");
        assert_eq!(to_pascal_case("_user_name_"), "UserName");
        
        // Already PascalCase
        assert_eq!(to_pascal_case("UserProfile"), "UserProfile");
        assert_eq!(to_pascal_case("APIClient"), "APIClient");
        
        // Complex camelCase
        assert_eq!(to_pascal_case("getUserProfile"), "GetUserProfile");
        assert_eq!(to_pascal_case("XMLHttpRequest"), "XMLHttpRequest");
        
        // Numbers
        assert_eq!(to_pascal_case("api2client"), "Api2client");
        assert_eq!(to_pascal_case("user123profile"), "User123profile");
        
        // Single word
        assert_eq!(to_pascal_case("user"), "User");
        assert_eq!(to_pascal_case("USER"), "USER");
    }

    #[test]
    fn test_to_camel_case_edge_cases() {
        // Empty and single characters
        assert_eq!(to_camel_case(""), "");
        assert_eq!(to_camel_case("a"), "a");
        assert_eq!(to_camel_case("A"), "a");
        
        // Single word
        assert_eq!(to_camel_case("user"), "user");
        assert_eq!(to_camel_case("USER"), "uSER");
        
        // Already camelCase
        assert_eq!(to_camel_case("userProfile"), "userProfile");
        assert_eq!(to_camel_case("getUserData"), "getUserData");
        
        // Complex cases
        assert_eq!(to_camel_case("XMLHttpRequest"), "xMLHttpRequest");
        assert_eq!(to_camel_case("user_name-space test"), "userNameSpaceTest");
    }

    #[test]
    fn test_generate_template_name_edge_cases() {
        // Empty strings
        assert_eq!(generate_template_name("", ""), "");
        assert_eq!(generate_template_name("components", ""), "");
        assert_eq!(generate_template_name("", "user"), "User");
        
        // Hooks edge cases
        assert_eq!(generate_template_name("hooks", "use-"), "");
        assert_eq!(generate_template_name("hooks", "use-use-user"), "UseUser");
        assert_eq!(generate_template_name("hooks", "useUser"), "UseUser");  // No prefix to remove
        assert_eq!(generate_template_name("HOOKS", "use-auth"), "Auth");  // Case insensitive
        
        // Case variations
        assert_eq!(generate_template_name("COMPONENTS", "user-profile"), "UserProfile");
        assert_eq!(generate_template_name("Services", "api-client"), "ApiClientService");
        assert_eq!(generate_template_name("TYPES", "user-data"), "UserDataType");
        
        // Unknown item types
        assert_eq!(generate_template_name("unknown", "user-profile"), "UserProfile");
        assert_eq!(generate_template_name("custom-type", "api-client"), "ApiClient");
        
        // Complex names
        assert_eq!(generate_template_name("services", "complex_API-client_name"), "ComplexAPIClientNameService");
        assert_eq!(generate_template_name("types", "XMLHttpRequest"), "XMLHttpRequestType");
        
        // Single character names
        assert_eq!(generate_template_name("components", "a"), "A");
        assert_eq!(generate_template_name("services", "x"), "XService");
    }

    #[test]
    fn test_unicode_handling() {
        // Unicode in names should be preserved
        assert!(is_valid_name("usuário"));  // Should pass basic alphanumeric check
        assert_eq!(to_pascal_case("usuário-perfil"), "UsuárioPerfil");
        assert_eq!(to_kebab_case("usuário perfil"), "usuário-perfil");
        assert_eq!(to_camel_case("usuário-perfil"), "usuárioPerfil");
        
        // Mixed unicode and ASCII
        assert_eq!(to_pascal_case("user-configuração"), "UserConfiguração");
        assert_eq!(generate_template_name("components", "página-usuário"), "PáginaUsuário");
    }

    #[test]
    fn test_file_operations() {
        use tempfile::TempDir;
        use std::fs;
        
        let temp_dir = TempDir::new().unwrap();
        
        // Test create_folder
        let folder_path = temp_dir.path().join("test_folder");
        create_folder(&folder_path).unwrap();
        assert!(folder_path.exists());
        assert!(folder_path.is_dir());
        
        // Test create_folder for existing folder (should not error)
        create_folder(&folder_path).unwrap();
        
        // Test create nested folders
        let nested_path = temp_dir.path().join("nested").join("deep").join("folder");
        create_folder(&nested_path).unwrap();
        assert!(nested_path.exists());
        assert!(nested_path.is_dir());
        
        // Test create_file
        let file_path = temp_dir.path().join("test_file.txt");
        let content = "Hello, World!";
        let bytes_written = create_file(&file_path, content.to_string()).unwrap();
        
        assert!(file_path.exists());
        assert!(file_path.is_file());
        assert_eq!(bytes_written, content.len());
        
        // Verify file contents
        let read_content = fs::read_to_string(&file_path).unwrap();
        assert_eq!(read_content, content);
        
        // Test create_file with unicode content
        let unicode_file_path = temp_dir.path().join("unicode_file.txt");
        let unicode_content = "Olá, Mundo! 🌍";
        create_file(&unicode_file_path, unicode_content.to_string()).unwrap();
        
        let read_unicode_content = fs::read_to_string(&unicode_file_path).unwrap();
        assert_eq!(read_unicode_content, unicode_content);
    }

    #[test]
    fn test_create_file_in_nested_directory() {
        use tempfile::TempDir;
        
        let temp_dir = TempDir::new().unwrap();
        let nested_dir = temp_dir.path().join("nested").join("directory");
        
        // Create the directory structure first
        create_folder(&nested_dir).unwrap();
        
        // Create file in nested directory
        let file_path = nested_dir.join("nested_file.txt");
        let content = "Nested file content";
        create_file(&file_path, content.to_string()).unwrap();
        
        assert!(file_path.exists());
        assert!(file_path.is_file());
    }
}
