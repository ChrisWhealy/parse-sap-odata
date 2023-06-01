use serde::{Deserialize, Serialize};

use crate::utils::de_str_to_bool;
use crate::utils::default_false;

#[derive(Debug, Serialize, Deserialize)]
pub enum DataServiceVersion {
    #[serde(rename = "2.0")]
    V2_0,
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
// Annotations specific to an EntityType
// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct MSAnnotationsEntityType {
    #[serde(
        rename = "m:HasStream",
        deserialize_with = "de_str_to_bool",
        default = "default_false"
    )]
    pub has_stream: bool,
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
// Annotations specific to a Property
// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct MSAnnotationsProperty {
    #[serde(
        rename = "m:FC_KeepInContent",
        deserialize_with = "de_str_to_bool",
        default = "default_false"
    )]
    pub fc_keep_in_content: bool,

    #[serde(rename = "m:FC_TargetPath")]
    pub fc_target_path: Option<String>,
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
// Annotations specific to a FunctionImport
// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct MSAnnotationsFunctionImport {
    #[serde(rename = "m:HttpMethod")]
    pub http_method: Option<String>,
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
// Annotations specific to DataServices
// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct MSAnnotationsDataServices {
    #[serde(rename = "m:DataServiceVersion")]
    pub data_service_version: DataServiceVersion,
}
