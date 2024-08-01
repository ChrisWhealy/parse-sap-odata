use serde::{Deserialize, Serialize};

use crate::sap_annotations::default_sap_schema_version;

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
/// SAP Annotations applicable to `edm:Schema`
///
/// See https://sap.github.io/odata-vocabularies/docs/v2-annotations.html#element-edmschema
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct SAPAnnotationsSchema {
    #[serde(rename = "@schema_version", default = "default_sap_schema_version")]
    pub schema_version: String,
}
