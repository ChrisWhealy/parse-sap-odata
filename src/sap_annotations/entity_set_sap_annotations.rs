use serde::{Deserialize, Serialize};

use super::default_sap_content_version;
use crate::utils::{de_str_to_bool, default_false, default_true};

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
/// SAP Annotations applicable to `edm:EntitySet`
///
/// See https://sap.github.io/odata-vocabularies/docs/v2-annotations.html#element-edmentityset
#[derive(Debug, Serialize, Deserialize)]
pub enum EntitySetSAPSemantics {
    #[serde(rename = "aggregate")]
    Aggregate,

    #[serde(rename = "timeseries")]
    TimeSeries,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct EntitySetSAPAnnotations {
    #[serde(rename = "sap:content-version", default = "default_sap_content_version")]
    pub content_version: String,

    #[serde(rename = "sap:semantics")]
    pub semantics: Option<EntitySetSAPSemantics>,

    #[serde(rename = "sap:label")]
    pub label: Option<String>,

    #[serde(
        rename = "sap:creatable",
        deserialize_with = "de_str_to_bool",
        default = "default_true"
    )]
    pub is_creatable: bool,

    #[serde(
        rename = "sap:updatable",
        deserialize_with = "de_str_to_bool",
        default = "default_true"
    )]
    pub is_updatable: bool,

    #[serde(
        rename = "sap:deletable",
        deserialize_with = "de_str_to_bool",
        default = "default_true"
    )]
    pub is_deletable: bool,

    #[serde(
        rename = "sap:searchable",
        deserialize_with = "de_str_to_bool",
        default = "default_false"
    )]
    pub is_searchable: bool,

    #[serde(
        rename = "sap:pageable",
        deserialize_with = "de_str_to_bool",
        default = "default_true"
    )]
    pub is_pageable: bool,

    #[serde(
        rename = "sap:topable",
        deserialize_with = "de_str_to_bool",
        default = "default_true"
    )]
    pub is_topable: bool,

    #[serde(
        rename = "sap:countable",
        deserialize_with = "de_str_to_bool",
        default = "default_true"
    )]
    pub is_countable: bool,

    #[serde(
        rename = "sap:addressable",
        deserialize_with = "de_str_to_bool",
        default = "default_true"
    )]
    pub is_addressable: bool,

    #[serde(
        rename = "sap:requires-filter",
        deserialize_with = "de_str_to_bool",
        default = "default_false"
    )]
    pub requires_filter: bool,

    #[serde(
        rename = "sap:change-tracking",
        deserialize_with = "de_str_to_bool",
        default = "default_false"
    )]
    pub change_tracking_enabled: bool,

    #[serde(rename = "sap:maxpagesize")]
    pub max_page_size: Option<u32>,

    #[serde(rename = "sap:delta-link-validity")]
    pub delta_link_validity: Option<u32>,
}
