use crate::property::PropertyRef;
use serde::{Deserialize, Serialize};

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
/// Represents a `<Dependent>` tag
///
/// # Child Nodes
/// `1:n PropertyRef`
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Dependent {
    #[serde(rename = "@Role")]
    pub role: String,

    #[serde(rename = "PropertyRef", default)]
    pub property_refs: Vec<PropertyRef>,
}
