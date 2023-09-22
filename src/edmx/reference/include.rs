use serde::{Deserialize, Serialize};

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
/// Represents an `<edmx:Include>` tag
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Include {
    #[serde(rename = "@Namespace")]
    pub namespace: String,
    #[serde(rename = "@Alias")]
    pub alias: String,
}
