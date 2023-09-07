use serde::{Deserialize, Serialize};

use crate::sap_annotations::{SAPAnnotationsFunctionImport, SAPAnnotationsFunctionImportParameter};

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
/// Represents a `<Parameter>` tag
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Parameter {
    #[serde(rename = "Name")]
    pub parameter_name: String,

    #[serde(rename = "Type")]
    pub parameter_type: String,

    pub mode: String,
    pub max_length: Option<String>,

    #[serde(flatten)]
    pub sap_annotations: SAPAnnotationsFunctionImportParameter,
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
/// Represents a `<FunctionImport>` tag
///
/// # Child Nodes
/// `1:n Parameter`
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct FunctionImport {
    pub name: String,
    pub return_type: String,
    pub entity_set: Option<String>,

    #[serde(rename = "m:HttpMethod")]
    pub http_method: String,

    #[serde(flatten)]
    pub sap_annotations: SAPAnnotationsFunctionImport,

    #[serde(rename = "Parameter")]
    pub parameters: Option<Vec<Parameter>>,
}
