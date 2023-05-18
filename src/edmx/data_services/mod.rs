pub mod schema;

use schema::Schema;
use serde::{Deserialize, Serialize};

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
// Data Services
// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
#[derive(Debug, Serialize, Deserialize)]
pub enum DataServiceVersion {
    #[serde(rename = "2.0")]
    V2_0,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct DataServices {
    #[serde(rename = "m:DataServiceVersion")]
    pub version: DataServiceVersion,

    #[serde(rename = "Schema", default)]
    pub schemas: Vec<Schema>,
}

impl DataServices {
    pub fn fetch_schema(&self, schema_name: &str) -> Option<&Schema> {
        self.schemas
            .iter()
            .find(|schema| schema.namespace == schema_name)
    }

    pub fn default_schema(&self) -> Option<&Schema> {
        self.fetch_schema("Default")
    }
}
