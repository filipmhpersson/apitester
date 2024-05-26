use std::{collections::HashMap, str::FromStr};

use tui_scrollview::ScrollViewState;

use crate::{
    api::{Action, ApiDocs},
    apirunner::{fetch_url, ApiResponse},
};

#[derive(Debug)]
pub enum CurrentScreen {
    Main,
}

#[derive(Debug, PartialEq)]
pub enum CurrentPane {
    FilterApi,
    ApiPaths,
    Collections,
    HttpCalls,
    HttpResponse,
    HttpResult,
}

#[derive(Debug, Clone)]
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
    paths: Vec<AppApiPaths>,
    pub filtered_paths: Vec<AppApiPaths>,
    pub current_path: Option<AppApiPaths>,
    pub current_method: Option<(String, Action)>,
    pub current_methods: Option<Vec<(String, Action)>>,
    pub environments: Vec<ApiEnvironment>,
    pub filter: String,
    pub index_path: usize,
    pub index_environment: usize,
    pub scroll_view_state: ScrollViewState,
    pub index_method: usize,
    pub api_response: Option<ApiResponseResult>,
}

#[derive(Debug)]
pub enum ApiResponseResult {
    Success(ApiResponse),
    Failure(String),
}
impl App {
    pub fn new(docs: ApiDocs) -> App {
        let paths: Vec<AppApiPaths> = docs
            .paths
            .into_iter()
            .map(|p| AppApiPaths {
                path: p.0.to_string(),
                methods: p.1.clone(),
            })
            .collect();
        let filtered_paths = paths.clone();
        let current_path = filtered_paths[0].clone();
        let current_methods: Vec<(String, Action)> = current_path
            .methods
            .clone()
            .into_iter()
            .map(|m| (m.0.to_string(), m.1.clone()))
            .collect();
        let current_method = current_methods[0].clone();

        App {
            key_input: String::new(),
            value_input: String::new(),
            pairs: HashMap::new(),
            current_screen: CurrentScreen::Main,
            current_pane: CurrentPane::ApiPaths,
            index_path: 0,
            index_environment: 0,
            scroll_view_state: ScrollViewState::new(),
            index_method: 0,
            current_path: Some(filtered_paths[0].clone()),
            filter: "document".to_string(),
            filtered_paths,
            current_method: Some(current_method),
            current_methods: Some(current_methods),
            api_response: None,
            environments: vec![
                ApiEnvironment {
                    name: "localhost".to_string(),
                    url: "http://localhost:5035/api/documents".to_string(),
                },
                ApiEnvironment {
                    name: "Test".to_string(),
                    url: "https://test-my.ibinder.com/api/documents".to_string(),
                },
                ApiEnvironment {
                    name: "Prod".to_string(),
                    url: "https://my.ibinder.com/api/documents".to_string(),
                },
            ],
            paths,
        }
    }
    fn set_current(&mut self) {
        let current_path = self.filtered_paths[self.index_path].clone();
        let current_methods: Vec<(String, Action)> = current_path
            .methods
            .clone()
            .into_iter()
            .map(|m| (m.0.to_string(), m.1.clone()))
            .collect();
        let current_method = current_methods[self.index_method].clone();

        self.current_path = Some(current_path);
        self.current_methods = Some(current_methods);
        self.current_method = Some(current_method);
    }
    pub fn push_filter(&mut self, char: char) {
        self.filter.push(char);
        let paths: Vec<AppApiPaths> = self
            .filtered_paths
            .drain(..)
            .into_iter()
            .filter(|p| p.path.contains(&self.filter))
            .collect();
        self.filtered_paths = paths;
    }
    pub fn pop_filter(&mut self) {
        if self.filter.len() == 0 {
            return;
        }
        self.index_path = 0;
        self.filter.pop();
        self.filtered_paths = self
            .paths
            .clone()
            .into_iter()
            .filter(|p| p.path.contains(&self.filter))
            .collect();
    }
    pub fn clear_filter(&mut self) {
        self.filtered_paths = self.paths.clone();
        self.filter = "".to_string();
    }

    pub fn next_action(&mut self) {
        match &self.current_methods {
            Some(current_methods) => {
                if self.index_method >= current_methods.len() - 1 {
                    self.index_method = 0;
                } else {
                    self.index_method += 1;
                }
                match &self.current_methods {
                    Some(current_methods) => {
                        self.current_method = Some(current_methods[self.index_method].clone());
                    }
                    None => (),
                }
            }
            None => (),
        }
    }

    pub fn prev_action(&mut self) {
        match &self.current_methods {
            Some(current_methods) => {
                if self.index_method == 0 {
                    self.index_method = current_methods.len() - 1;
                } else {
                    self.index_method -= 1;
                }
                match &self.current_methods {
                    Some(current_methods) => {
                        self.current_method = Some(current_methods[self.index_method].clone());
                    }
                    None => (),
                }
            }
            None => (),
        }
    }

    pub fn scroll_down_selected_env(&mut self, items: usize) {
        if self.index_environment + items >= self.environments.len() {
        } else {
            self.index_environment += items;
        }
    }

    pub fn scroll_up_selected_env(&mut self, items: usize) {
        if items > self.index_environment {
            self.index_environment = 0;
        } else {
            self.index_environment -= items;
        }
    }
    pub fn scroll_down_cursor_path(&mut self, items: usize) {
        if self.index_path + items >= self.paths.len() {
            self.index_path = self.paths.len() - 1;
        } else {
            self.index_path += items;
        }
        self.set_current();
    }

    pub fn scroll_up_cursor_path(&mut self, items: usize) {
        if items > self.index_path {
            self.index_path = 0;
        } else {
            self.index_path -= items;
        }
        self.set_current();
    }

    pub fn print_json(&self) -> serde_json::Result<()> {
        let output = serde_json::to_string(&self.pairs)?;
        println!("{}", output);
        Ok(())
    }

    pub async fn send_apirequest(&mut self) {
        let environment = &self.environments[self.index_environment];
        match &self.current_path {
            Some(path) => {
                let st = hyper::Uri::from_str(&format!("{}{}", environment.url, &path.path));
                let result = fetch_url(st.expect("Valid URL")).await;
                match result {
                    Ok(result) => self.api_response = Some(ApiResponseResult::Success(result)),
                    Err(err) => {
                        self.api_response = Some(ApiResponseResult::Failure(err.to_string()))
                    }
                }
            }
            None => panic!("No selected path"),
        }
    }
}
