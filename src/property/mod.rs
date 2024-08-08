use serde::{Deserialize, Serialize};

use crate::{
    sap_annotations::property::SAPAnnotationsProperty,
    utils::{de_str_to_bool, default_false, default_true},
};

#[cfg(feature = "parser")]
pub mod metadata;

pub mod property_ref;

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
/// Represents an `edm:Property` element
#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Property {
    #[serde(rename = "@Name")]
    pub odata_name: String,
    #[serde(rename = "@Type")]
    pub edm_type: String,
    #[serde(rename = "@Nullable", default = "default_true")]
    pub nullable: bool,
    #[serde(rename = "@MaxLength")]
    pub max_length: Option<u16>,
    #[serde(rename = "@Precision")]
    pub precision: Option<u16>,
    #[serde(rename = "@Scale")]
    pub scale: Option<u16>,
    #[serde(rename = "@ConcurrencyMode")]
    pub concurrency_mode: Option<String>,
    // Microsoft Annotations
    #[serde(
        rename = "@FC_KeepInContent",
        deserialize_with = "de_str_to_bool",
        default = "default_false"
    )]
    pub fc_keep_in_content: bool,
    #[serde(rename = "@FC_TargetPath")]
    pub fc_target_path: Option<String>,
    // SAP Annotations
    #[serde(flatten)]
    pub sap_annotations: SAPAnnotationsProperty,
    #[serde(skip, default)]
    pub deserializer_fn: &'static str,
    #[serde(skip, default)]
    pub deserializer_module: &'static str,
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
#[cfg(test)]
pub mod unit_tests;
