#[cfg(feature = "parser")]
pub mod metadata;

use serde::{Deserialize, Serialize};

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Debug, Serialize, Deserialize)]
pub enum SAPFilterRestrictionProperty {
    #[serde(rename = "single-value")]
    SingleValue,
    #[serde(rename = "multi-value")]
    MultiValue,
    #[serde(rename = "interval")]
    Interval,
}
