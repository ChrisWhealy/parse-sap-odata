pub mod parameter;

use serde::{Deserialize, Serialize};

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
/// SAP Annotations applicable to `edm:FunctionImport`
///
/// See https://sap.github.io/odata-vocabularies/docs/v2-annotations.html#element-edmfunctionimport
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct SAPAnnotationsFunctionImport {
    #[serde(rename = "@label")]
    pub label: Option<String>,
    #[serde(rename = "@action-for")]
    pub action_for: Option<String>,
    #[serde(rename = "@applicable-path")]
    pub creatable_path: Option<String>,
    #[serde(rename = "@planning-function")]
    pub planning_function: Option<String>,
}
