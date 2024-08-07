use serde::{Deserialize, Serialize};

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
/// SAP Annotations applicable to `edm:EntityType`
///
/// See https://sap.github.io/odata-vocabularies/docs/v2-annotations.html#element-edmentitytype
#[derive(Debug, Serialize, Deserialize)]
pub enum SAPSemanticsEntityType {
    #[serde(rename = "vcard")]
    VCard,
    #[serde(rename = "vevent")]
    VEvent,
    #[serde(rename = "vtodo")]
    VToDo,
    #[serde(rename = "parameters")]
    Paramaters,
    #[serde(rename = "aggregate")]
    Aggregate,
    #[serde(rename = "variant")]
    Variant,
}
