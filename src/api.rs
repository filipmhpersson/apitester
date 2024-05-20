use std::collections::{BTreeMap, HashMap};

use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
pub struct ApiDocs {
    pub paths: BTreeMap<String, HashMap<String, Action>>,
}
#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Action {
    pub operation_id: Option<String>,
    pub parameters: Vec<Parameters>,
}
#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Parameters {
    pub name: String,
    pub r#in: String,
    pub schema: Option<ParameterSchema>,
}
#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ParameterSchema {
    pub r#type: ParameterType,
}
#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub enum ParameterType {
    Integer,
    String,
    Boolean,
}
