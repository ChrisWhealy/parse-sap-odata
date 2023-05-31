use crate::utils::de_str_to_bool;
use crate::utils::default_true;
use serde::{Deserialize, Serialize};

pub fn default_sap_schema_version() -> String {
    "1".to_string()
}
pub fn default_sap_content_version() -> String {
    "1".to_string()
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct SAPAnnotations {
    #[serde(
        rename = "sap:unicode",
        deserialize_with = "de_str_to_bool",
        default = "default_true"
    )]
    pub sap_unicode: bool,

    #[serde(rename = "sap:display-format")]
    pub sap_display_format: Option<String>,

    #[serde(
        rename = "sap:createable",
        deserialize_with = "de_str_to_bool",
        default = "default_true"
    )]
    pub sap_creatable: bool,

    #[serde(
        rename = "sap:filterable",
        deserialize_with = "de_str_to_bool",
        default = "default_true"
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
        default = "default_true"
    )]
    pub sap_sortable: bool,

    #[serde(rename = "sap:text")]
    pub sap_text: Option<String>,

    #[serde(rename = "sap:unit")]
    pub sap_unit: Option<String>,

    #[serde(
        rename = "sap:updatable",
        deserialize_with = "de_str_to_bool",
        default = "default_true"
    )]
    pub sap_updatable: bool,
}
