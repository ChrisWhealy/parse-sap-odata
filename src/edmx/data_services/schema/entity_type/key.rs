use serde::Deserialize;
use crate::property::property_ref::PropertyRef;

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
/// Represents a `<Key>` tag
///
/// # Child Nodes
/// `1:n PropertyRef`
#[derive(Debug, serde::Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Key {
    #[serde(rename = "PropertyRef")]
    pub property_refs: Vec<PropertyRef>,
}

