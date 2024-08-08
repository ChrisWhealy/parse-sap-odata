#[cfg(feature = "parser")]
pub mod metadata;

use serde::{Deserialize, Serialize};

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
/// Represents a `<PropertyRef>` tag
#[derive(Clone, Debug, Serialize, Deserialize, Ord, Eq, PartialOrd, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct PropertyRef {
    #[serde(rename = "@Name")]
    pub name: String,
}
