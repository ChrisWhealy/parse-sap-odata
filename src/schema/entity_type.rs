use crate::property::{NavigationProperty, Property, PropertyRef};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Key {
    #[serde(rename = "PropertyRef")]
    pub property_refs: Vec<PropertyRef>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct EntityType {
    pub name: String,
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
                .any(|prop_ref| prop_ref.name == property.name)
        })
    }
}
