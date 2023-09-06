use serde::{Deserialize, Serialize};

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
/// SAP Annotations applicable to `edm:FunctionImport`
///
/// See https://sap.github.io/odata-vocabularies/docs/v2-annotations.html#element-edmfunctionimport
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct FunctionImportSAPAnnotations {
    #[serde(rename = "sap:label")]
    pub label: Option<String>,

    #[serde(rename = "sap:action-for")]
    pub action_for: String,

    #[serde(rename = "sap:applicable-path")]
    pub creatable_path: Option<String>,

    #[serde(rename = "sap:planning-function")]
    pub planning_function: Option<String>,
}
