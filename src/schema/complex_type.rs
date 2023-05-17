use crate::property::Property;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct ComplexType {
    pub name: String,

    #[serde(rename = "Property", default)]
    pub properties: Vec<Property>,
}
