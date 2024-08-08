#[cfg(feature = "parser")]
pub mod metadata;

use serde::{Deserialize, Serialize};

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Debug, Serialize, Deserialize)]
pub enum SAPFieldControlProperty {
    #[serde(rename = "0")]
    Hidden,
    #[serde(rename = "1")]
    ReadOnly,
    #[serde(rename = "3")]
    Optional,
    #[serde(rename = "7")]
    Mandatory,
}
