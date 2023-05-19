use super::record::Record;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Annotation {
    pub term: String,
    pub enum_member: Option<String>,

    pub record: Option<Record>,
}
