#[cfg(feature = "parser")]
pub mod metadata;

use serde::{Deserialize, Serialize};

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Debug, Serialize, Deserialize)]
pub enum SAPDisplayFormatProperty {
    #[serde(rename = "Date")]
    Date,
    #[serde(rename = "NonNegative")]
    NonNegative,
    #[serde(rename = "UpperCase")]
    UpperCase,
}
