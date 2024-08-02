use serde::{Deserialize, Serialize};

use super::record::Record;
use crate::utils::default_false;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Annotation {
    #[serde(rename = "@xmlns")]
    pub xmlns: Option<String>,
    #[serde(rename = "@Term")]
    pub term: Option<String>,
    #[serde(rename = "@String")]
    pub string: Option<String>,
    #[serde(rename = "@Bool", default = "default_false")]
    pub is_boolean: bool,
    #[serde(rename = "@EnumMember")]
    pub enum_member: Option<String>,
    pub record: Option<Record>,
}
