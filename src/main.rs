use serde::Deserialize;
use std::collections::HashMap;
use std::error::Error;
use std::fs;
mod api;
mod app;
mod ui;
fn main() -> Result<(), Box<dyn Error>> {
    println!("Hello, world!");
    let file = fs::read_to_string("sample.json")?;
    let json: api::ApiDocs = serde_json::from_str(&file).unwrap();
    dbg!("{}", json);
    let app = app::App::new();

    Ok(())
}
