use serde::{Deserialize, Serialize};

fn default_true() -> bool {
    true
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Property {
    pub name: String,

    // #[serde(flatten)]
    // pub inner: PropertyType,
    #[serde(rename = "Type")]
    pub property_type: String,

    #[serde(default = "default_true")]
    pub nullable: bool,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct NavigationProperty {
    pub name: String,
    pub relationship: String,
    pub to_role: String,
    pub from_role: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct PropertyRef {
    pub name: String,
}
