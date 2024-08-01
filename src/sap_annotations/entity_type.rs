use serde::{Deserialize, Serialize};

use crate::{sap_annotations::default_sap_content_version, sap_semantics::SAPSemanticsEntityType};

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct SAPAnnotationsEntityType {
    #[serde(rename = "@label")]
    pub sap_label: Option<String>,

    #[serde(rename = "@semantics")]
    pub sap_semantics: Option<SAPSemanticsEntityType>,

    #[serde(rename = "@content-version", default = "default_sap_content_version")]
    pub sap_content_version: String,
}
