use serde::{Deserialize, Serialize};

use crate::{
    sap_annotations::function_import::*,
    utils::{de_str_to_bool, default_true},
};

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
/// Represents a `<Parameter>` tag
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Parameter {
    #[serde(rename = "@Name")]
    pub parameter_name: String,

    #[serde(rename = "@Type")]
    pub parameter_type: String,

    #[serde(rename = "@Mode")]
    pub mode: String,

    #[serde(rename = "@MaxLength")]
    pub max_length: Option<String>,

    #[serde(
        rename = "@Nullable",
        deserialize_with = "de_str_to_bool",
        default = "default_true"
    )]
    pub nullable: bool,

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
