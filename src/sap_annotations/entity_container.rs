use serde::{Deserialize, Serialize};

use crate::{
    sap_annotations::default_entity_container_supported_formats,
    utils::{de_str_to_bool, de_str_to_list, default_false},
};

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
/// SAP Annotations applicable to `edm:EntityContainer`
///
/// See https://sap.github.io/odata-vocabularies/docs/v2-annotations.html#element-edmentitycontainer
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct SAPAnnotationsEntityContainer {
    #[serde(
        rename = "@message-scope-supported",
        deserialize_with = "de_str_to_bool",
        default = "default_false"
    )]
    pub message_scope_supported: bool,

    #[serde(
        rename = "@use-batch",
        deserialize_with = "de_str_to_bool",
        default = "default_false"
    )]
    pub use_batch: bool,

    #[serde(
        rename = "@supported-formats",
        deserialize_with = "de_str_to_list",
        default = "default_entity_container_supported_formats"
    )]
    pub supported_formats: Vec<String>,
}
