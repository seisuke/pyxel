use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TsFunction {
    pub name: String,
    pub args: Vec<(String, String)>,
    pub return_type: String,

    #[serde(skip_serializing_if = "String::is_empty", default)]
    pub body: String,

    #[serde(skip_serializing_if = "HashMap::is_empty", default)]
    pub meta: HashMap<String, Value>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TsClass {
    pub name: String,
    pub methods: Vec<TsFunction>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TsModule {
    pub name: String,
    pub functions: Vec<TsFunction>,
    pub classes: Vec<TsClass>,
}
