#[cfg(feature = "parser")]
pub mod metadata;

use serde::{Deserialize, Serialize};

use crate::property::Property;

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
/// Represents a `<ComplexType>` tag
///
/// # Child Nodes
/// `1:n Property`
#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ComplexType {
    #[serde(rename = "@Name")]
    pub name: String,
    #[serde(rename = "Property", default)]
    pub properties: Vec<Property>,
}
