use serde::{Deserialize, Serialize};

use super::default_sap_content_version;

#[derive(Debug, Serialize, Deserialize)]
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

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
/// SAP Annotations applicable to `edm:EntityType`
///
/// See https://sap.github.io/odata-vocabularies/docs/v2-annotations.html#element-edmentitytype
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct EntityTypeSAPAnnotations {
    #[serde(rename = "sap:label")]
    pub sap_label: Option<String>,

    #[serde(rename = "sap:semantics")]
    pub sap_semantics: Option<EntityTypeSAPSemantics>,

    #[serde(rename = "sap:content-version", default = "default_sap_content_version")]
    pub sap_content_version: String,
}
