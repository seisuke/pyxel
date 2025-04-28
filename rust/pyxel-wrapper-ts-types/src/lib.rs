use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct TsFunction {
    pub name: String,
    pub args: Vec<(String, String)>,
    pub return_type: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TsClass {
    pub name: String,
    pub methods: Vec<TsFunction>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TsModule {
    pub name: String,
    pub functions: Vec<TsFunction>,
    pub classes: Vec<TsClass>,
}
