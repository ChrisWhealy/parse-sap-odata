pub mod schema;

use schema::Schema;
use serde::{Deserialize, Serialize};

use crate::ms_annotations::MSAnnotationsDataServices;

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
// Data Services
//
// Child Nodes:
//   1:1 Schema
// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct DataServices {
    #[serde(flatten)]
    pub ms_annotations: MSAnnotationsDataServices,

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
