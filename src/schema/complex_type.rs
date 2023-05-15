use crate::property::Property;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ComplexType {
    #[serde(rename = "ComplexType", default)]
    pub properties: Vec<Property>,
}
