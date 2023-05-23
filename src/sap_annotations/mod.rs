use serde::{Deserialize, Deserializer, Serialize};
use std::str::FromStr;

pub fn default_sap_schema_version() -> String {
    "1".to_string()
}
pub fn default_sap_content_version() -> String {
    "1".to_string()
}

pub fn default_sap_unicode() -> bool {
    true
}
pub fn default_sap_addressable() -> bool {
    true
}
pub fn default_sap_createable() -> bool {
    true
}
pub fn default_sap_deletable() -> bool {
    true
}
pub fn default_sap_filterable() -> bool {
    true
}
pub fn default_sap_pageable() -> bool {
    true
}
pub fn default_sap_searchable() -> bool {
    true
}
pub fn default_sap_sortable() -> bool {
    true
}
pub fn default_sap_updatable() -> bool {
    true
}

fn de_str_to_bool<'de, D>(deserializer: D) -> Result<bool, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    bool::from_str(&s).or_else(|_| Ok(false))
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct SAPAnnotations {
    #[serde(
        rename = "sap:unicode",
        deserialize_with = "de_str_to_bool",
        default = "default_sap_unicode"
    )]
    pub sap_unicode: bool,

    #[serde(rename = "sap:display-format")]
    pub sap_display_format: Option<String>,

    #[serde(
        rename = "sap:createable",
        deserialize_with = "de_str_to_bool",
        default = "default_sap_createable"
    )]
    pub sap_creatable: bool,

    #[serde(
        rename = "sap:filterable",
        deserialize_with = "de_str_to_bool",
        default = "default_sap_filterable"
    )]
    pub sap_filterable: bool,

    #[serde(rename = "sap:label")]
    pub sap_label: Option<String>,

    #[serde(rename = "sap:quickinfo")]
    pub sap_quick_info: Option<String>,

    #[serde(rename = "sap:semantics")]
    pub sap_semantics: Option<String>,

    #[serde(
        rename = "sap:sortable",
        deserialize_with = "de_str_to_bool",
        default = "default_sap_sortable"
    )]
    pub sap_sortable: bool,

    #[serde(rename = "sap:text")]
    pub sap_text: Option<String>,

    #[serde(rename = "sap:unit")]
    pub sap_unit: Option<String>,

    #[serde(
        rename = "sap:updatable",
        deserialize_with = "de_str_to_bool",
        default = "default_sap_updatable"
    )]
    pub sap_updatable: bool,
}
