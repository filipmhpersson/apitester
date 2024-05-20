use std::collections::HashMap;

use crate::api::{Action, ApiDocs};

#[derive(Debug)]
pub enum CurrentScreen {
    Main,
    Editing,
    Exiting,
}

#[derive(Debug, PartialEq)]
pub enum CurrentPane {
    FilterApi,
    ApiPaths,
    Collections,
    HttpCalls,
}
#[derive(Debug)]
pub enum CurrentlyEditing {
    Key,
    Value,
}

#[derive(Debug)]
pub struct AppApiPaths {
    pub path: String,
    pub methods: HashMap<String, Action>,
}
#[derive(Debug)]
pub struct ApiEnvironment {
    pub name: String,
    pub url: String,
}
#[derive(Debug)]
pub struct App {
    pub key_input: String,
    pub value_input: String,
    pub pairs: HashMap<String, String>,
    pub current_screen: CurrentScreen,
    pub current_pane: CurrentPane,
    pub currently_editing: Option<CurrentlyEditing>,
    paths: Vec<AppApiPaths>,
    pub environments: Vec<ApiEnvironment>,
    pub filter: String,
    pub cursor_path: usize,
    pub selected_environment: usize,
    pub selected_method: usize,
}

impl App {
    pub fn new(docs: ApiDocs) -> App {
        App {
            key_input: String::new(),
            value_input: String::new(),
            pairs: HashMap::new(),
            current_screen: CurrentScreen::Main,
            currently_editing: None,
            current_pane: CurrentPane::ApiPaths,
            cursor_path: 0,
            selected_environment: 0,
            selected_method: 0,
            filter: "document".to_string(),
            environments: vec![
                ApiEnvironment {
                    name: "localhost".to_string(),
                    url: "localhost:5035/api/documents".to_string(),
                },
                ApiEnvironment {
                    name: "Test".to_string(),
                    url: "test-my.ibinder.com/api/documents".to_string(),
                },
                ApiEnvironment {
                    name: "Prod".to_string(),
                    url: "my.ibinder.com/api/documents".to_string(),
                },
            ],
            paths: docs
                .paths
                .into_iter()
                .map(|p| AppApiPaths {
                    path: p.0.to_string(),
                    methods: p.1.clone(),
                })
                .collect(),
        }
    }
    pub fn save_key_value(&mut self) {
        self.pairs
            .insert(self.key_input.clone(), self.value_input.clone());
        self.key_input = String::new();
        self.value_input = String::new();
        self.currently_editing = None;
    }

    pub fn filter(&self) -> Vec<&AppApiPaths> {
        match &self.filter.len() {
            0 => self.paths.iter().collect(),
            _ => self
                .paths
                .iter()
                .filter(|p| p.path.contains(&self.filter))
                .collect(),
        }
    }

    pub fn scroll_down_selected_env(&mut self, items: usize) {
        if self.selected_environment + items >= self.paths.len() {
            self.selected_environment = self.paths.len() - 1;
        } else {
            self.selected_environment += items;
        }
    }

    pub fn scroll_up_selected_env(&mut self, items: usize) {
        if items > self.selected_environment {
            self.selected_environment = 0;
        } else {
            self.selected_environment -= items;
        }
    }
    pub fn scroll_down_cursor_path(&mut self, items: usize) {
        if self.cursor_path + items >= self.paths.len() {
            self.cursor_path = self.paths.len() - 1;
        } else {
            self.cursor_path += items;
        }
    }

    pub fn scroll_up_cursor_path(&mut self, items: usize) {
        if items > self.cursor_path {
            self.cursor_path = 0;
        } else {
            self.cursor_path -= items;
        }
    }
    pub fn next_tab(&mut self, items: usize) {
        if items > self.cursor_path {
            self.cursor_path = 0;
        } else {
            self.cursor_path -= items;
        }
    }
    pub fn toggle_editing(&mut self) {
        if let Some(edit_mode) = &self.currently_editing {
            match edit_mode {
                CurrentlyEditing::Key => self.currently_editing = Some(CurrentlyEditing::Value),
                CurrentlyEditing::Value => self.currently_editing = Some(CurrentlyEditing::Key),
            }
        } else {
            self.currently_editing = Some(CurrentlyEditing::Key);
        }
    }

    pub fn print_json(&self) -> serde_json::Result<()> {
        let output = serde_json::to_string(&self.pairs)?;
        println!("{}", output);
        Ok(())
    }
}
