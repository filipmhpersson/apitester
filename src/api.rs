use std::collections::HashMap;

use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct ApiDocs {
    pub paths: HashMap<String, Path>,
}
#[derive(Deserialize, Debug)]
pub struct Path {
    pub set: Option<Action>,
    pub post: Option<Action>,
    pub put: Option<Action>,
    pub delete: Option<Action>,
}
#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Action {
    pub operation_id: Option<String>,
    pub parameters: Vec<Parameters>,
}
#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Parameters {
    pub name: String,
    pub r#in: String,
    pub schema: Option<ParameterSchema>,
}
#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ParameterSchema {
    pub r#type: ParameterType,
}
#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub enum ParameterType {
    Integer,
    String,
    Boolean,
}
