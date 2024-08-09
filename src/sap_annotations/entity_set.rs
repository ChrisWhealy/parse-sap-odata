use serde::{Deserialize, Serialize};

use crate::{
    sap_annotations::{de_str_to_bool, default_false, default_sap_content_version, default_true},
    sap_semantics::entity_set::SAPSemanticsEntitySet,
};

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct SAPAnnotationsEntitySet {
    #[serde(rename = "@content-version", default = "default_sap_content_version")]
    pub content_version: String,
    #[serde(rename = "@semantics")]
    pub semantics: Option<SAPSemanticsEntitySet>,
    #[serde(rename = "@label")]
    pub label: Option<String>,
    #[serde(
        rename = "@creatable",
        deserialize_with = "de_str_to_bool",
        default = "default_true"
    )]
    pub is_creatable: bool,
    #[serde(
        rename = "@updatable",
        deserialize_with = "de_str_to_bool",
        default = "default_true"
    )]
    pub is_updatable: bool,
    #[serde(
        rename = "@deletable",
        deserialize_with = "de_str_to_bool",
        default = "default_true"
    )]
    pub is_deletable: bool,
    #[serde(
        rename = "@searchable",
        deserialize_with = "de_str_to_bool",
        default = "default_false"
    )]
    pub is_searchable: bool,
    #[serde(
        rename = "@pageable",
        deserialize_with = "de_str_to_bool",
        default = "default_true"
    )]
    pub is_pageable: bool,
    #[serde(
        rename = "@topable",
        deserialize_with = "de_str_to_bool",
        default = "default_true"
    )]
    pub is_topable: bool,
    #[serde(
        rename = "@countable",
        deserialize_with = "de_str_to_bool",
        default = "default_true"
    )]
    pub is_countable: bool,
    #[serde(
        rename = "@addressable",
        deserialize_with = "de_str_to_bool",
        default = "default_true"
    )]
    pub is_addressable: bool,
    #[serde(
        rename = "@requires-filter",
        deserialize_with = "de_str_to_bool",
        default = "default_false"
    )]
    pub requires_filter: bool,
    #[serde(
        rename = "@change-tracking",
        deserialize_with = "de_str_to_bool",
        default = "default_false"
    )]
    pub change_tracking_enabled: bool,
    #[serde(rename = "@maxpagesize")]
    pub max_page_size: Option<u32>,
    #[serde(rename = "@delta-link-validity")]
    pub delta_link_validity: Option<u32>,
}
