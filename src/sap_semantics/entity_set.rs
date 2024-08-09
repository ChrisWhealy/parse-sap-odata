use serde::{Deserialize, Serialize};

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
/// SAP Annotations applicable to `edm:EntitySet`
///
/// See https://sap.github.io/odata-vocabularies/docs/v2-annotations.html#element-edmentityset
#[derive(Debug, Serialize, Deserialize)]
pub enum SAPSemanticsEntitySet {
    #[serde(rename = "aggregate")]
    Aggregate,
    #[serde(rename = "timeseries")]
    TimeSeries,
}
