use serde::Deserialize;

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
#[derive(Debug, serde::Serialize, Deserialize)]
pub enum EntityTypeSAPSemantics {
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
