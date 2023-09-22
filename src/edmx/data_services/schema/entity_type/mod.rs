mod navigation_property;

use crate::property::{Property, PropertyRef};
use crate::sap_annotations::default_sap_content_version;
use crate::utils::{de_str_to_bool, default_false};
use navigation_property::NavigationProperty;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum EntityTypeSAPSemantics {
    #[serde(rename = "vcard")]
    VCard,

    #[serde(rename = "vevent")]
    VEvent,

    #[serde(rename = "vtodo")]
    VToDo,

    #[serde(rename = "parameters")]
    Paramaters,

    #[serde(rename = "aggregate")]
    Aggregate,

    #[serde(rename = "variant")]
    Variant,
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
/// Represents a `<Key>` tag
///
/// # Child Nodes
/// `1:n PropertyRef`
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Key {
    #[serde(rename = "PropertyRef")]
    pub property_refs: Vec<PropertyRef>,
}

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

impl EntityType {
    pub fn key_property(&self) -> Option<&Property> {
        self.properties.iter().find(|property| {
            self.key
                .property_refs
                .iter()
                .any(|prop_ref| prop_ref.name == property.odata_name)
        })
    }
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
#[cfg(test)]
pub mod unit_tests;
