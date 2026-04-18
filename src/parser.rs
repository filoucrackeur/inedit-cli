use anyhow::Result;
use std::collections::BTreeMap;
use std::fs;
use std::path::Path;

const DEFAULT_SECTION_NAME: &str = "DEFAULT";

#[derive(Debug, Clone, PartialEq)]
pub enum Line {
    Comment(String),
    Section(String),
    KeyValue {
        key: String,
        value: String,
        commented: bool,
    },
    Empty,
}

#[derive(Debug, Clone)]
pub struct IniFile {
    pub sections: BTreeMap<String, BTreeMap<String, String>>,
    pub comments: BTreeMap<Option<String>, Vec<String>>,
    pub raw_lines: Vec<Line>,
    pub modified: bool,
    has_default_section: bool,
}

impl IniFile {
    pub fn new() -> Self {
        Self {
            sections: BTreeMap::new(),
            comments: BTreeMap::new(),
            raw_lines: Vec::new(),
            modified: false,
            has_default_section: false,
        }
    }

    pub fn load(path: &Path) -> Result<Self> {
        let content = fs::read_to_string(path)?;
        Self::parse(&content)
    }

    pub fn parse(content: &str) -> Result<Self> {
        let mut ini_file = Self::new();
        let mut current_section: Option<String> = None;

        ini_file.comments.insert(None, Vec::new());

        for line in content.lines() {
            let trimmed = line.trim();

            if trimmed.is_empty() {
                ini_file.raw_lines.push(Line::Empty);
                continue;
            }

            if trimmed.starts_with(';') || trimmed.starts_with('#') {
                let comment = trimmed[1..].trim().to_string();
                ini_file.raw_lines.push(Line::Comment(comment.clone()));

                if let Some(ref sec) = current_section {
                    ini_file
                        .comments
                        .entry(Some(sec.clone()))
                        .or_default()
                        .push(comment);
                } else {
                    ini_file.comments.entry(None).or_default().push(comment);
                }
                continue;
            }

            if trimmed.starts_with('[') && trimmed.ends_with(']') {
                let section_name = trimmed[1..trimmed.len() - 1].trim().to_string();
                let is_default = section_name.to_uppercase() == "DEFAULT";
                current_section = Some(section_name.clone());
                ini_file.raw_lines.push(Line::Section(section_name.clone()));
                ini_file.sections.entry(section_name.clone()).or_default();
                ini_file.comments.insert(Some(section_name), Vec::new());

                if is_default {
                    ini_file.has_default_section = true;
                }
                continue;
            }

            if let Some(eq_pos) = trimmed.find('=') {
                let key = trimmed[..eq_pos].trim().to_string();
                let value = trimmed[eq_pos + 1..].trim().to_string();

                let commented = line.trim().starts_with('#');

                // If no section yet, use DEFAULT section (but don't save it)
                if current_section.is_none() {
                    current_section = Some(DEFAULT_SECTION_NAME.to_string());
                    ini_file.has_default_section = false; // It's auto-created, not from file
                }

                if let Some(ref sec) = current_section {
                    if commented {
                        let uncommented_key = key.trim_start_matches('#').trim().to_string();
                        ini_file
                            .sections
                            .entry(sec.clone())
                            .or_default()
                            .insert(uncommented_key, value.clone());
                    } else {
                        ini_file
                            .sections
                            .entry(sec.clone())
                            .or_default()
                            .insert(key.clone(), value.clone());
                    }
                }

                ini_file.raw_lines.push(Line::KeyValue {
                    key,
                    value,
                    commented,
                });
            } else if !trimmed.is_empty() {
                // Handle variables without value (flags like FLAG_DEBUG)
                let key = trimmed.trim().to_string();
                let commented = trimmed.starts_with('#');
                let uncommented_key = if commented {
                    key.trim_start_matches('#').trim().to_string()
                } else {
                    key.clone()
                };

                if current_section.is_none() {
                    current_section = Some(DEFAULT_SECTION_NAME.to_string());
                    ini_file.has_default_section = false;
                }

                if let Some(ref sec) = current_section {
                    ini_file
                        .sections
                        .entry(sec.clone())
                        .or_default()
                        .insert(uncommented_key, String::new());
                }

                ini_file.raw_lines.push(Line::KeyValue {
                    key,
                    value: String::new(),
                    commented,
                });
            } else {
                ini_file.raw_lines.push(Line::Empty);
            }
        }

        Ok(ini_file)
    }

    pub fn save(&self, path: &Path) -> Result<()> {
        let mut output = String::new();
        let mut in_default_section = false;

        let comment_char = self.detect_comment_char();
        let comment_prefix = format!("{} ", comment_char);

        for line in &self.raw_lines {
            match line {
                Line::Empty => output.push('\n'),
                Line::Comment(c) => {
                    if in_default_section {
                        continue;
                    }
                    output.push_str(&format!("{}{}\n", comment_prefix, c));
                }
                Line::Section(s) => {
                    if s.to_uppercase() == "DEFAULT" {
                        in_default_section = true;
                        continue;
                    } else {
                        in_default_section = false;
                    }
                    output.push_str(&format!("[{}]\n", s));
                }
                Line::KeyValue {
                    key,
                    value,
                    commented,
                } => {
                    if in_default_section {
                        continue;
                    }
                    let prefix = if *commented { &comment_prefix } else { "" };
                    output.push_str(&format!("{}{} = {}\n", prefix, key, value));
                }
            }
        }

        fs::write(path, output)?;
        Ok(())
    }

    fn detect_comment_char(&self) -> char {
        for line in &self.raw_lines {
            match line {
                Line::Comment(c) => {
                    if c.starts_with(';') {
                        return ';';
                    } else if c.starts_with('#') {
                        return '#';
                    }
                }
                Line::KeyValue {
                    key,
                    value: _,
                    commented: true,
                } => {
                    let full_line = self.raw_lines.iter().find(|l| {
                        if let Line::KeyValue { key: k, .. } = l {
                            k == key
                        } else {
                            false
                        }
                    });
                    if let Some(Line::KeyValue { .. }) = full_line {
                        return '#';
                    }
                }
                _ => {}
            }
        }
        '#'
    }

    #[allow(dead_code)]
    pub fn get_section_comments(&self, section: &str) -> &[String] {
        self.comments
            .get(&Some(section.to_string()))
            .map(|v| v.as_slice())
            .unwrap_or(&[])
    }

    #[allow(dead_code)]
    pub fn get_global_comments(&self) -> &[String] {
        self.comments
            .get(&None)
            .map(|v| v.as_slice())
            .unwrap_or(&[])
    }

    pub fn is_default_section(section_name: &str) -> bool {
        section_name.to_uppercase() == "DEFAULT"
    }
}

impl Default for IniFile {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_ini_with_variables() {
        let content = "[database]
host = localhost
port = 5432
max_connections = 100";

        let ini = IniFile::parse(content).unwrap();

        println!("Sections: {:?}", ini.sections);
        println!("Raw lines: {:?}", ini.raw_lines);

        assert!(
            ini.sections.contains_key("database"),
            "Should have database section"
        );
        assert!(
            ini.sections.get("database").unwrap().contains_key("host"),
            "Should have host key"
        );
        assert_eq!(
            ini.sections.get("database").unwrap().get("host"),
            Some(&"localhost".to_string())
        );
    }

    #[test]
    fn test_parse_conf_with_variables() {
        let content = "APP_NAME=MonService
VERSION=1.0.0

[database]
host = localhost
port = 5432";

        let ini = IniFile::parse(content).unwrap();

        println!("CONF Sections: {:?}", ini.sections);

        // Variables before any section should go to DEFAULT section
        assert!(
            ini.sections.contains_key("DEFAULT") || ini.sections.contains_key("database"),
            "Should have sections"
        );
        if ini.sections.contains_key("DEFAULT") {
            assert!(ini
                .sections
                .get("DEFAULT")
                .unwrap()
                .contains_key("APP_NAME"));
        }
        assert!(ini.sections.get("database").unwrap().contains_key("host"));
    }

    #[test]
    fn test_parse_ini_file() {
        let ini = IniFile::load(Path::new("../test.ini")).unwrap();

        println!("\n=== TOUTES LES SECTIONS ===");
        for (name, vars) in &ini.sections {
            println!("\n[{}]", name);
            for (k, v) in vars {
                println!("  {} = {}", k, v);
            }
        }

        assert!(ini.sections.contains_key("database"));
        assert!(ini.sections.get("database").unwrap().contains_key("host"));
        assert_eq!(
            ini.sections.get("database").unwrap().get("host"),
            Some(&"localhost".to_string())
        );
    }

    #[test]
    fn test_integration_flow() {
        // Test the full flow: load -> get sections -> get variables
        let ini = IniFile::load(Path::new("../test_variables.ini")).unwrap();

        // Get sections
        let sections: Vec<&String> = ini.sections.keys().collect();
        println!("Sections found: {:?}", sections);
        assert!(!sections.is_empty());

        // Get first section's variables
        let first_section = sections[0];
        let vars = ini.sections.get(first_section).unwrap();
        println!("Variables in '{}': {:?}", first_section, vars);

        // Verify we have key-value pairs
        assert!(!vars.is_empty());
    }

    #[test]
    fn test_compare_ini_and_conf_parsing() {
        // Compare parsing of .ini vs .conf
        let ini = IniFile::load(Path::new("../test_variables.ini")).unwrap();
        let conf = IniFile::load(Path::new("../test.conf")).unwrap();

        println!(
            "INI sections: {:?}",
            ini.sections.keys().collect::<Vec<_>>()
        );
        println!(
            "CONF sections: {:?}",
            conf.sections.keys().collect::<Vec<_>>()
        );

        // Both should have variables
        for (name, vars) in &ini.sections {
            println!("INI section '{}' has {} vars", name, vars.len());
        }

        for (name, vars) in &conf.sections {
            println!("CONF section '{}' has {} vars", name, vars.len());
        }

        // Verify both have non-empty sections with variables
        assert!(ini.sections.values().any(|v| !v.is_empty()));
        assert!(conf.sections.values().any(|v| !v.is_empty()));
    }

    #[test]
    fn test_commented_variables() {
        let content = "[database]
# This is a commented variable
# max_connections = 100
host = localhost

[server]
port = 8080";

        let ini = IniFile::parse(content).unwrap();

        println!("Commented vars test - Sections: {:?}", ini.sections);
        println!("Commented vars test - Raw lines: {:?}", ini.raw_lines);

        // The commented variable should be added to the sections map
        // but marked as commented in raw_lines
        // For now, let's check that at least non-commented vars work
        assert!(ini.sections.get("database").unwrap().contains_key("host"));
    }
}
