use crate::sap_annotations::{de_str_to_bool, default_false};
use serde::{Deserialize, Serialize};

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
/// SAP Annotations applicable to `edm:FunctionImport`
///
/// See https://sap.github.io/odata-vocabularies/docs/v2-annotations.html#element-edmfunctionimport
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct SAPAnnotationsFunctionImportParameter {
    #[serde(rename = "@label")]
    pub label: Option<String>,
    #[serde(
        rename = "@variable-scale",
        deserialize_with = "de_str_to_bool",
        default = "default_false"
    )]
    pub has_variable_scale: bool,
}
