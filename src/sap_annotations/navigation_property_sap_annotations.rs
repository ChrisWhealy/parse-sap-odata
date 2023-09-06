use serde::{Deserialize, Serialize};

use crate::utils::{de_str_to_bool, default_true};

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
/// SAP Annotations applicable to `edm:NavigationProperty`
///
/// See https://sap.github.io/odata-vocabularies/docs/v2-annotations.html#element-edmnavigationproperty
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct NavigationPropertySAPAnnotations {
    #[serde(
        rename = "sap:creatable",
        deserialize_with = "de_str_to_bool",
        default = "default_true"
    )]
    pub is_creatable: bool,

    #[serde(rename = "sap:creatable-path")]
    pub creatable_path: Option<String>,

    #[serde(
        rename = "sap:filterable",
        deserialize_with = "de_str_to_bool",
        default = "default_true"
    )]
    pub is_filterable: bool,
}
