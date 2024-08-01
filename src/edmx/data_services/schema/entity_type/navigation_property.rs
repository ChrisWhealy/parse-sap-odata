use serde::{Deserialize, Serialize};

use crate::sap_annotations::navigation_property::SAPAnnotationsNavigationProperty;

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
// Represents a `<NavigationProperty>` tag
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct NavigationProperty {
    #[serde(rename = "@Name")]
    pub name: String,

    #[serde(rename = "@Relationship")]
    pub relationship: String,

    #[serde(rename = "@ToRole")]
    pub to_role: String,

    #[serde(rename = "@FromRole")]
    pub from_role: String,

    #[serde(flatten)]
    pub sap_annotations: SAPAnnotationsNavigationProperty,
}
