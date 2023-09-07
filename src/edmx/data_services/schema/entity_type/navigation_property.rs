use serde::{Deserialize, Serialize};

use crate::sap_annotations::NavigationPropertySAPAnnotations;

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
// Represents a `<NavigationProperty>` tag
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct NavigationProperty {
    pub name: String,
    pub relationship: String,
    pub to_role: String,
    pub from_role: String,

    #[serde(flatten)]
    pub sap_annotations: NavigationPropertySAPAnnotations,
}
