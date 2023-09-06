use serde::{Deserialize, Serialize};

use super::default_sap_content_version;
use crate::utils::{de_str_to_bool, default_true};

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
/// SAP Annotations applicable to `edm:AssociationSet`
///
/// See https://sap.github.io/odata-vocabularies/docs/v2-annotations.html#element-edmassociationset
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct AssociationSetSAPAnnotations {
    #[serde(rename = "sap:content-version", default = "default_sap_content_version")]
    pub content_version: String,

    #[serde(
        rename = "sap:creatable",
        deserialize_with = "de_str_to_bool",
        default = "default_true"
    )]
    pub is_creatable: bool,

    #[serde(
        rename = "sap:deletable",
        deserialize_with = "de_str_to_bool",
        default = "default_true"
    )]
    pub is_deletable: bool,

    #[serde(
        rename = "sap:updatable",
        deserialize_with = "de_str_to_bool",
        default = "default_true"
    )]
    pub is_updatable: bool,
}
