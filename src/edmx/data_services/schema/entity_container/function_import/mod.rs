use serde::{Deserialize, Serialize};

use crate::sap_annotations::function_import::*;

use parameter::Parameter;

pub mod parameter;

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
/// Represents a `<FunctionImport>` tag
///
/// # Child Nodes
/// `1:n Parameter`
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct FunctionImport {
    #[serde(rename = "@Name")]
    pub name: String,
    #[serde(rename = "@ReturnType")]
    pub return_type: String,
    #[serde(rename = "@EntitySet")]
    pub entity_set: Option<String>,
    #[serde(rename = "@HttpMethod")]
    pub http_method: String,
    #[serde(flatten)]
    pub sap_annotations: SAPAnnotationsFunctionImport,
    #[serde(rename = "Parameter")]
    pub parameters: Option<Vec<Parameter>>,
}
