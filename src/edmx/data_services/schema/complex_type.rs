use crate::property::Property;
use serde::{Deserialize, Serialize};

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
/// Represents a `<ComplexType>` tag
///
/// # Child Nodes
/// `1:n Property`
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ComplexType {
    #[serde(rename = "@Name")]
    pub name: String,

    #[serde(rename = "Property", default)]
    pub properties: Vec<Property>,
}
