use super::record::Record;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Collection {
    #[serde(rename = "Record")]
    pub records: Option<Vec<Record>>,

    #[serde(rename = "PropertyPath")]
    pub property_paths: Option<Vec<String>>,
}
