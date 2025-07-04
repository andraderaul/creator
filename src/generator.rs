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

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::TempDir;

    #[test]
    fn test_generate_with_valid_template() {
        let temp_dir = TempDir::new().unwrap();
        let template_path = temp_dir.path().join("test_template.hbs");
        
        // Create a simple template
        let template_content = "import React from 'react';\n\nexport function {{templateName}}() {\n  return <div>{{templateName}}</div>;\n}";
        fs::write(&template_path, template_content).unwrap();

        let result = Generator::generate(&template_path, "UserProfile".to_string()).unwrap();
        
        assert!(result.contains("export function UserProfile()"));
        assert!(result.contains("return <div>UserProfile</div>"));
        assert!(result.contains("import React from 'react'"));
    }

    #[test]
    fn test_generate_with_hooks_template() {
        let temp_dir = TempDir::new().unwrap();
        let template_path = temp_dir.path().join("hooks_template.hbs");
        
        // Simulate hooks template pattern
        let template_content = "import { useState, useEffect } from 'react';\n\nexport function use{{templateName}}() {\n  return {};\n}";
        fs::write(&template_path, template_content).unwrap();

        let result = Generator::generate(&template_path, "UserData".to_string()).unwrap();
        
        assert!(result.contains("export function useUserData()"));
        assert!(result.contains("import { useState, useEffect }"));
    }

    #[test]
    fn test_generate_with_nonexistent_template() {
        let nonexistent_path = PathBuf::from("/nonexistent/template.hbs");
        
        let result = Generator::generate(&nonexistent_path, "TestComponent".to_string()).unwrap();
        
        // Should fallback to default template
        assert_eq!(result, "export function TestComponent(){}");
    }

    #[test]
    fn test_generate_with_invalid_handlebars_template() {
        let temp_dir = TempDir::new().unwrap();
        let template_path = temp_dir.path().join("invalid_template.hbs");
        
        // Create invalid handlebars syntax
        let invalid_template = "export function {{templateName}() { // Missing closing brace in handlebars";
        fs::write(&template_path, invalid_template).unwrap();

        let result = Generator::generate(&template_path, "TestComponent".to_string());
        
        // Should return error for invalid template syntax
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Cannot register template string"));
    }

    #[test]
    fn test_generate_with_empty_name() {
        let temp_dir = TempDir::new().unwrap();
        let template_path = temp_dir.path().join("test_template.hbs");
        
        let template_content = "export function {{templateName}}() {}";
        fs::write(&template_path, template_content).unwrap();

        let result = Generator::generate(&template_path, "".to_string()).unwrap();
        
        // Should handle empty name gracefully
        assert_eq!(result, "export function () {}");
    }

    #[test]
    fn test_generate_with_special_characters_in_name() {
        let temp_dir = TempDir::new().unwrap();
        let template_path = temp_dir.path().join("test_template.hbs");
        
        let template_content = "export function {{templateName}}() {}";
        fs::write(&template_path, template_content).unwrap();

        let result = Generator::generate(&template_path, "User-Profile_Component".to_string()).unwrap();
        
        // Should preserve the exact name passed
        assert_eq!(result, "export function User-Profile_Component() {}");
    }

    #[test]
    fn test_generate_with_unicode_name() {
        let temp_dir = TempDir::new().unwrap();
        let template_path = temp_dir.path().join("test_template.hbs");
        
        let template_content = "export function {{templateName}}() {}";
        fs::write(&template_path, template_content).unwrap();

        let result = Generator::generate(&template_path, "Usuário".to_string()).unwrap();
        
        // Should handle unicode characters
        assert_eq!(result, "export function Usuário() {}");
    }

    #[test]
    fn test_generate_with_complex_template() {
        let temp_dir = TempDir::new().unwrap();
        let template_path = temp_dir.path().join("complex_template.hbs");
        
        let template_content = r#"import React, { useState, useEffect } from 'react';

interface {{templateName}}Props {
  id: string;
}

export const {{templateName}}: React.FC<{{templateName}}Props> = ({ id }) => {
  const [loading, setLoading] = useState(false);

  useEffect(() => {
    // {{templateName}} logic here
  }, [id]);

  return (
    <div className="{{templateName}}">
      <h1>{{templateName}}</h1>
    </div>
  );
};

export default {{templateName}};"#;
        
        fs::write(&template_path, template_content).unwrap();

        let result = Generator::generate(&template_path, "UserDashboard".to_string()).unwrap();
        
        // Verify multiple template replacements
        assert!(result.contains("interface UserDashboardProps"));
        assert!(result.contains("export const UserDashboard: React.FC<UserDashboardProps>"));
        assert!(result.contains("className=\"UserDashboard\""));
        assert!(result.contains("<h1>UserDashboard</h1>"));
        assert!(result.contains("export default UserDashboard;"));
        assert!(result.contains("// UserDashboard logic here"));
    }

    #[test]
    fn test_generate_handlebars_escaping() {
        let temp_dir = TempDir::new().unwrap();
        let template_path = temp_dir.path().join("escape_template.hbs");
        
        // Template with special characters
        let template_content = "export const {{templateName}} = () => '<div>{{templateName}}</div>';";
        fs::write(&template_path, template_content).unwrap();

        let result = Generator::generate(&template_path, "Test<script>".to_string()).unwrap();
        
        // Handlebars escapes HTML by default for security
        assert!(result.contains("Test&lt;script&gt;"));
        assert!(result.contains("export const Test&lt;script&gt; = ()"));
    }
}
