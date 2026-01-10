use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub struct LexModule {
    #[serde(rename = "$schema")]
    pub schema: String,
    pub version: String,
    pub kind: String,
    pub metadata: Metadata,
    pub spec: Spec,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Metadata {
    pub name: String,
    pub namespace: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Spec {
    pub fields: Vec<Field>,
    pub capabilities: Capabilities,
    pub hooks: Vec<Hook>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Field {
    pub id: String,
    #[serde(rename = "type")]
    pub field_type: FieldType,
    pub required: bool,
    pub default: Option<serde_json::Value>,
    pub constraints: Option<HashMap<String, serde_json::Value>>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum FieldType {
    String,
    Number,
    Boolean,
    Datetime,
    Binary,
    Enum,
    Reference,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Capabilities {
    pub persistence: String,
    pub features: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Hook {
    pub on: String,
    pub action: String,
}