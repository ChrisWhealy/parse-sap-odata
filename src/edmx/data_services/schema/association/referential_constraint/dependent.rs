use crate::property::PropertyRef;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Dependent {
    pub role: String,

    #[serde(rename = "PropertyRef", default)]
    pub property_refs: Vec<PropertyRef>,
}
