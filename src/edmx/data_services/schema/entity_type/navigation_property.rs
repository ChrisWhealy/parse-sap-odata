use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct NavigationProperty {
    pub name: String,
    pub relationship: String,
    pub to_role: String,
    pub from_role: String,
}
