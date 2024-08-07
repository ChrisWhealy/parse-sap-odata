use crate::{
    sap_annotations::function_import::parameter::SAPAnnotationsFunctionImportParameter,
    utils::{de_str_to_bool, default_true},
};
use serde::{Deserialize, Serialize};

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
