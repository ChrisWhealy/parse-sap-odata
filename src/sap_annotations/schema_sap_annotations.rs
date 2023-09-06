use serde::{Deserialize, Serialize};

use super::default_sap_schema_version;

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
/// SAP Annotations applicable to `edm:Schema`
///
/// See https://sap.github.io/odata-vocabularies/docs/v2-annotations.html#element-edmschema
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct SchemaSAPAnnotations {
    #[serde(rename = "sap:schema_version", default = "default_sap_schema_version")]
    pub schema_version: String,
}
