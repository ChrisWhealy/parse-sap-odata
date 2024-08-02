pub mod schema;

use schema::Schema;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum DataServiceVersion {
    #[serde(rename = "2.0")]
    V2_0,
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
/// Represents an `<edmx:DataServices>` tag
///
/// # Child Nodes
/// `1:1 Schema`
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct DataServices {
    #[serde(rename = "@DataServiceVersion")]
    pub data_service_version: DataServiceVersion,
    #[serde(rename = "Schema", default)]
    pub schemas: Vec<Schema>,
}

impl DataServices {
    pub fn fetch_schema(&self, namespace: &str) -> Option<&Schema> {
        self.schemas.iter().find(|schema| schema.namespace == namespace)
    }

    pub fn default_schema(&self) -> Option<&Schema> {
        self.fetch_schema("Default")
    }
}
