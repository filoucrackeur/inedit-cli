use crate::i18n::{self, AVAILABLE_LANGUAGES};
use crate::parser::{IniFile, Line};
use ratatui::widgets::{ListState, TableState};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FocusArea {
    Sections,
    Variables,
    Search,
}

#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq)]
pub enum ValueType {
    Bool,
    Number,
    String,
}

pub struct AppState {
    pub ini_file: IniFile,
    pub file_path: String,
    #[allow(dead_code)]
    pub modified: bool,
    pub focus: FocusArea,
    pub section_list_state: ListState,
    pub variable_list_state: TableState,
    pub editing_key: Option<String>,
    pub editing_key_name: bool,
    pub editing_section: bool,
    pub input_buffer: String,
    pub show_help: bool,
    pub search_mode: bool,
    pub search_query: String,
    pub search_results: Vec<(usize, usize)>,
    pub search_index: usize,
    pub dropdown_options: Vec<String>,
    pub dropdown_index: usize,
    pub show_dropdown: bool,
    pub confirm_delete: Option<String>,
    pub show_language_menu: bool,
    pub language_menu_index: usize,
    pub status_message: Option<(String, bool)>,
}

impl AppState {
    pub fn new(ini_file: IniFile, file_path: String) -> Self {
        let mut section_list_state = ListState::default();
        section_list_state.select_first();

        let mut variable_list_state = TableState::default();

        let sections_count = ini_file.sections.len();
        if sections_count > 0 {
            variable_list_state.select(Some(0));
        }

        Self {
            ini_file,
            file_path,
            modified: false,
            focus: FocusArea::Sections,
            section_list_state,
            variable_list_state,
            editing_key: None,
            editing_key_name: false,
            editing_section: false,
            input_buffer: String::new(),
            show_help: false,
            search_mode: false,
            search_query: String::new(),
            search_results: Vec::new(),
            search_index: 0,
            dropdown_options: Vec::new(),
            dropdown_index: 0,
            show_dropdown: false,
            confirm_delete: None,
            show_language_menu: false,
            language_menu_index: 0,
            status_message: None,
        }
    }

    pub fn selected_section(&self) -> Option<String> {
        self.section_list_state
            .selected()
            .and_then(|idx| self.ini_file.sections.keys().nth(idx))
            .cloned()
    }

    pub fn selected_key(&self) -> Option<String> {
        let section = self.selected_section()?;
        let keys: Vec<_> = self.ini_file.sections.get(&section)?.keys().collect();
        self.variable_list_state
            .selected()
            .and_then(|idx| keys.get(idx).map(|s| (*s).clone()))
    }

    pub fn selected_value(&self) -> Option<String> {
        let section = self.selected_section()?;
        let key = self.selected_key()?;
        self.ini_file.sections.get(&section)?.get(&key).cloned()
    }

    pub fn current_variables(&self) -> Vec<(String, String, bool)> {
        let section = match self.selected_section() {
            Some(s) => s,
            None => return Vec::new(),
        };
        let vars = match self.ini_file.sections.get(&section) {
            Some(v) => v,
            None => return Vec::new(),
        };

        vars.iter()
            .map(|(k, v)| {
                let commented = self.ini_file.raw_lines.iter().any(|line| {
                    if let Line::KeyValue {
                        key, commented: c, ..
                    } = line
                    {
                        key == k && *c
                    } else {
                        false
                    }
                });
                (k.clone(), v.clone(), commented)
            })
            .collect()
    }

    pub fn perform_search(&mut self) {
        self.search_results.clear();
        let query = self.search_query.to_lowercase();

        if query.is_empty() {
            return;
        }

        for (sec_idx, (sec_name, vars)) in self.ini_file.sections.iter().enumerate() {
            if sec_name.to_lowercase().contains(&query) {
                self.search_results.push((sec_idx, usize::MAX)); // MAX means section match
            }

            for (key_idx, (key, value)) in vars.iter().enumerate() {
                if key.to_lowercase().contains(&query) || value.to_lowercase().contains(&query) {
                    self.search_results.push((sec_idx, key_idx));
                }
            }
        }

        self.search_index = 0;

        if let Some((sec_idx, key_idx)) = self.search_results.first() {
            self.section_list_state.select(Some(*sec_idx));
            if *key_idx != usize::MAX {
                self.variable_list_state.select(Some(*key_idx));
            } else {
                self.variable_list_state.select_first();
            }
        }
    }

    pub fn next_search_result(&mut self) {
        if self.search_results.is_empty() {
            return;
        }

        self.search_index = (self.search_index + 1) % self.search_results.len();

        if let Some((sec_idx, key_idx)) = self.search_results.get(self.search_index) {
            self.section_list_state.select(Some(*sec_idx));
            if *key_idx != usize::MAX {
                self.variable_list_state.select(Some(*key_idx));
            } else {
                self.variable_list_state.select_first();
            }
        }
    }

    pub fn prev_search_result(&mut self) {
        if self.search_results.is_empty() {
            return;
        }

        if self.search_index == 0 {
            self.search_index = self.search_results.len() - 1;
        } else {
            self.search_index -= 1;
        }

        if let Some((sec_idx, key_idx)) = self.search_results.get(self.search_index) {
            self.section_list_state.select(Some(*sec_idx));
            if *key_idx != usize::MAX {
                self.variable_list_state.select(Some(*key_idx));
            } else {
                self.variable_list_state.select_first();
            }
        }
    }

    #[allow(dead_code)]
    pub fn detect_value_type(value: &str) -> ValueType {
        let lower = value.to_lowercase();
        if lower == "true"
            || lower == "false"
            || lower == "yes"
            || lower == "no"
            || lower == "on"
            || lower == "off"
        {
            return ValueType::Bool;
        }
        if value.parse::<f64>().is_ok() {
            return ValueType::Number;
        }
        ValueType::String
    }

    pub fn toggle_dropdown(&mut self) {
        if self.show_dropdown {
            self.show_dropdown = false;
            self.dropdown_options.clear();
            self.dropdown_index = 0;
            return;
        }

        let current_value = self.selected_value().unwrap_or_default();
        let lower = current_value.to_lowercase();

        let is_conf = self.file_path.to_lowercase().ends_with(".conf");

        if lower == "true" || lower == "false" {
            if is_conf {
                self.dropdown_options = vec!["yes".to_string(), "no".to_string()];
            } else {
                self.dropdown_options = vec!["true".to_string(), "false".to_string()];
            }
        } else if lower == "yes" || lower == "no" {
            self.dropdown_options = vec!["yes".to_string(), "no".to_string()];
        } else if lower == "on" || lower == "off" {
            self.dropdown_options = vec!["on".to_string(), "off".to_string()];
        }

        if !self.dropdown_options.is_empty() {
            self.dropdown_index = 0;
            for (i, opt) in self.dropdown_options.iter().enumerate() {
                if opt.to_lowercase() == lower {
                    self.dropdown_index = i;
                    break;
                }
            }
            self.show_dropdown = true;
        }
    }

    pub fn dropdown_select_next(&mut self) {
        if !self.dropdown_options.is_empty() {
            self.dropdown_index = (self.dropdown_index + 1) % self.dropdown_options.len();
        }
    }

    pub fn dropdown_select_prev(&mut self) {
        if !self.dropdown_options.is_empty() {
            if self.dropdown_index == 0 {
                self.dropdown_index = self.dropdown_options.len() - 1;
            } else {
                self.dropdown_index -= 1;
            }
        }
    }

    pub fn dropdown_select(&mut self) {
        if !self.dropdown_options.is_empty() {
            if let Some(key) = self.selected_key() {
                let key_clone = key.clone();
                if let Some(section) = self.selected_section() {
                    if let Some(vars) = self.ini_file.sections.get_mut(&section) {
                        let new_value = self.dropdown_options[self.dropdown_index].clone();
                        vars.insert(key_clone.clone(), new_value.clone());
                        self.ini_file.modified = true;

                        for line in &mut self.ini_file.raw_lines {
                            if let Line::KeyValue {
                                key: k, value: v, ..
                            } = line
                            {
                                if k == &key_clone {
                                    *v = new_value;
                                    break;
                                }
                            }
                        }
                    }
                }
            }
        }
        self.show_dropdown = false;
    }

    pub fn confirm_delete_section(&mut self, section: String) {
        if section.to_uppercase() != "DEFAULT" {
            self.confirm_delete = Some(format!("section:{}", section));
        }
    }

    pub fn execute_delete_confirmed(&mut self) {
        if let Some(ref what) = self.confirm_delete {
            if what.starts_with("section:") {
                let section = what.trim_start_matches("section:");
                self.ini_file.sections.remove(section);
                self.ini_file.raw_lines.retain(|line| match line {
                    Line::Section(s) => s != section,
                    Line::KeyValue { .. } => false,
                    _ => true,
                });
                self.ini_file.modified = true;
            }
        }
        self.confirm_delete = None;
    }

    pub fn cancel_delete(&mut self) {
        self.confirm_delete = None;
    }

    pub fn toggle_language_menu(&mut self) {
        if self.show_language_menu {
            self.show_language_menu = false;
        } else {
            let current_lang = i18n::get_current_language();
            self.language_menu_index = AVAILABLE_LANGUAGES
                .iter()
                .position(|l| l.code == current_lang)
                .unwrap_or(0);
            self.show_language_menu = true;
        }
    }

    pub fn language_menu_next(&mut self) {
        let count = AVAILABLE_LANGUAGES.len();
        self.language_menu_index = (self.language_menu_index + 1) % count;
    }

    pub fn language_menu_prev(&mut self) {
        let count = AVAILABLE_LANGUAGES.len();
        if self.language_menu_index == 0 {
            self.language_menu_index = count - 1;
        } else {
            self.language_menu_index -= 1;
        }
    }

    pub fn language_menu_select(&mut self) {
        let lang = &AVAILABLE_LANGUAGES[self.language_menu_index];
        i18n::set_language(lang.code);
        self.show_language_menu = false;
    }

    pub fn set_status_success(&mut self, message: String) {
        self.status_message = Some((message, true));
    }

    pub fn set_status_error(&mut self, message: String) {
        self.status_message = Some((message, false));
    }

    pub fn clear_status(&mut self) {
        self.status_message = None;
    }
}
