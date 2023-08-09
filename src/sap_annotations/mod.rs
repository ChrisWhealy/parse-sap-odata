use serde::{Deserialize, Serialize};

use crate::utils::de_str_to_bool;
use crate::utils::{default_false, default_true};

pub fn default_sap_schema_version() -> String {
    "1".to_string()
}
pub fn default_sap_content_version() -> String {
    "1".to_string()
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct SAPAnnotations {
    // - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
    // Boolean annotation properties
    // - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
    #[serde(
        rename = "sap:unicode",
        deserialize_with = "de_str_to_bool",
        default = "default_true"
    )]
    pub sap_unicode: bool,

    #[serde(
        rename = "sap:createable",
        deserialize_with = "de_str_to_bool",
        default = "default_true"
    )]
    pub sap_creatable: bool,

    #[serde(
        rename = "sap:deletable",
        deserialize_with = "de_str_to_bool",
        default = "default_false"
    )]
    pub sap_deletatable: bool,

    #[serde(
        rename = "sap:pageable",
        deserialize_with = "de_str_to_bool",
        default = "default_true"
    )]
    pub sap_pageable: bool,

    #[serde(
        rename = "sap:addressable",
        deserialize_with = "de_str_to_bool",
        default = "default_true"
    )]
    pub sap_addressable: bool,

    #[serde(rename = "sap:content-version", default = "default_sap_content_version")]
    pub sap_content_version: String,

    #[serde(
        rename = "sap:filterable",
        deserialize_with = "de_str_to_bool",
        default = "default_true"
    )]
    pub sap_filterable: bool,

    #[serde(
        rename = "sap:sortable",
        deserialize_with = "de_str_to_bool",
        default = "default_true"
    )]
    pub sap_sortable: bool,

    #[serde(
        rename = "sap:updatable",
        deserialize_with = "de_str_to_bool",
        default = "default_true"
    )]
    pub sap_updatable: bool,

    // - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
    // Optional annotation properties
    // - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
    #[serde(rename = "sap:display-format")]
    pub sap_display_format: Option<String>,

    #[serde(rename = "sap:label")]
    pub sap_label: Option<String>,

    #[serde(rename = "sap:quickinfo")]
    pub sap_quick_info: Option<String>,

    #[serde(rename = "sap:semantics")]
    pub sap_semantics: Option<String>,

    #[serde(rename = "sap:text")]
    pub sap_text: Option<String>,

    #[serde(rename = "sap:unit")]
    pub sap_unit: Option<String>,
}
