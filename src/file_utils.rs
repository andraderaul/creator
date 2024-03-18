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
        .split_whitespace()
        .map(|word| word.to_lowercase())
        .collect::<Vec<_>>()
        .join("-")
}

pub fn to_pascal_case(input: &str) -> String {
    input
        .split_whitespace()
        .map(|word| {
            let mut chars = word.chars();
            match chars.next() {
                None => String::new(),
                Some(first) => {
                    let mut rest = chars.collect::<String>();
                    rest.make_ascii_lowercase();
                    format!("{}{}", first.to_uppercase(), rest)
                }
            }
        })
        .collect::<Vec<_>>()
        .join("")
}

#[cfg(test)]
mod test {
    use super::*;

    //TODO: test case with "nav-bar", "NavBar"

    #[test]
    fn test_to_kebab_case() {
        let inputs = vec!["nav bar", "Nav Bar", "Nav bar", "nAV bAR"];

        for input in inputs {
            assert_eq!(to_kebab_case(input), "nav-bar");
        }

        let inputs = vec!["components", "cOMPONENTS", "COMPONENTS", "Components"];

        for input in inputs {
            assert_eq!(to_kebab_case(input), "components");
        }
    }

    #[test]
    fn test_to_pascal_case() {
        let inputs = vec!["nav bar", "Nav Bar", "Nav bar", "nAV bAR"];

        for input in inputs {
            assert_eq!(to_pascal_case(input), "NavBar");
        }

        let inputs = vec!["components", "cOMPONENTS", "COMPONENTS", "Components"];

        for input in inputs {
            assert_eq!(to_pascal_case(input), "Components");
        }
    }
}
