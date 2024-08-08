use serde::{Deserialize, Serialize};

use entity_type_semantics::EntityTypeSAPSemantics;
use navigation_property::NavigationProperty;
use key::Key;

use crate::{
    property::Property,
    sap_annotations::default_sap_content_version,
    utils::{de_str_to_bool, default_false},
};

pub mod entity_type_semantics;
pub mod key;
pub mod navigation_property;

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
/// Represents an `<EntityType>`
///
/// # Child Nodes
/// `1:1 Key`
/// `1:n Property`
/// `0:n NavigationProperty`
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct EntityType {
    #[serde(rename = "@Name")]
    pub name: String,
    #[serde(rename = "@label")]
    pub sap_label: Option<String>,
    #[serde(rename = "@semantics")]
    pub sap_semantics: Option<EntityTypeSAPSemantics>,
    #[serde(rename = "@content-version", default = "default_sap_content_version")]
    pub sap_content_version: String,
    #[serde(
        rename = "@HasStream",
        deserialize_with = "de_str_to_bool",
        default = "default_false"
    )]
    pub has_stream: bool,
    pub key: Key,
    #[serde(rename = "Property", default)]
    pub properties: Vec<Property>,
    #[serde(rename = "NavigationProperty", default)]
    pub navigations: Vec<NavigationProperty>,
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
#[cfg(test)]
pub mod unit_tests;
