#[cfg(feature = "parser")]
pub mod metadata;

use serde::{Deserialize, Serialize};

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
/// Represents an `<End>` tag
#[derive(Clone, Debug, Serialize, Deserialize, Ord, Eq, PartialOrd, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct End {
    #[serde(rename = "@Role")]
    pub role: String,
    #[serde(rename = "@EntitySet")]
    pub entity_set: Option<String>,
    #[serde(rename = "@Type")]
    pub end_type: Option<String>,
    #[serde(rename = "@Multiplicity")]
    pub multiplicity: Option<String>,
}
