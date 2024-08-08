#[cfg(feature = "parser")]
pub mod metadata;

use serde::{Deserialize, Serialize};

use crate::{
    sap_annotations::default_sap_content_version,
    utils::{de_str_to_bool, default_true},
};

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
/// SAP Annotations applicable to `edm:AssociationSet`
///
/// See https://sap.github.io/odata-vocabularies/docs/v2-annotations.html#element-edmassociationset
#[derive(Clone, Debug, Serialize, Deserialize, Ord, Eq, PartialOrd, PartialEq, )]
#[serde(rename_all = "PascalCase")]
pub struct SAPAnnotationsAssociationSet {
    #[serde(rename = "@content-version", default = "default_sap_content_version")]
    pub content_version: String,
    #[serde(
        rename = "@creatable",
        deserialize_with = "de_str_to_bool",
        default = "default_true"
    )]
    pub is_creatable: bool,
    #[serde(
        rename = "@deletable",
        deserialize_with = "de_str_to_bool",
        default = "default_true"
    )]
    pub is_deletable: bool,
    #[serde(
        rename = "@updatable",
        deserialize_with = "de_str_to_bool",
        default = "default_true"
    )]
    pub is_updatable: bool,
}
