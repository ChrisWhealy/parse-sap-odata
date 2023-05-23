use crate::sap_annotations::SAPAnnotations;
use crate::xml::default_true;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Property {
    pub name: String,

    #[serde(rename = "Type")]
    pub property_type: String,

    #[serde(default = "default_true")]
    pub nullable: bool,

    pub max_length: Option<String>,

    pub precision: Option<String>,
    pub scale: Option<String>,
    pub concurrency_mode: Option<String>,

    // SAP Annotations
    #[serde(flatten)]
    pub sap_annotations: SAPAnnotations,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct PropertyRef {
    pub name: String,
}
