#[cfg(feature = "parser")]
pub mod metadata;

use serde::{Deserialize, Serialize};

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
// This enum acts as its own metadata
#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Debug, Serialize, Deserialize)]
pub enum SAPAggregationRoleProperty {
    #[serde(rename = "dimension")]
    Dimension,
    #[serde(rename = "measure")]
    Measure,
    #[serde(rename = "totaled-properties-list")]
    TotalPropertiesList,
}
