use crate::property::property_ref::PropertyRef;
use serde::{Deserialize, Serialize};

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
/// Represents a `<Principal>` tag
//
// # Child Nodes
// `1:n PropertyRef`
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Principal {
    #[serde(rename = "@Role")]
    pub role: String,

    #[serde(rename = "PropertyRef", default)]
    pub property_refs: Vec<PropertyRef>,
}
