use super::collection::Collection;
use crate::xml::default_false;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct PropertyValue {
    pub property: String,

    pub string: Option<String>,

    #[serde(rename = "Bool", default = "default_false")]
    pub is_bool: bool,

    pub collection: Option<Collection>,
}
