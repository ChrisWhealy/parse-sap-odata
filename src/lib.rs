mod property;
mod schema;

use schema::Schema;
use serde::{Deserialize, Serialize};
use std::str::FromStr;

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
// Entity Data Model
// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
#[derive(Debug, Serialize, Deserialize)]
pub enum EdmxVersion {
    #[serde(rename = "1.0")]
    V1_0,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Edmx {
    pub version: EdmxVersion,
    pub data_services: DataServices,
}

impl Edmx {
    pub fn default_schema(&self) -> Option<&Schema> {
        self.data_services.default_schema()
    }
}

impl FromStr for Edmx {
    type Err = quick_xml::DeError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        quick_xml::de::from_str(s)
    }
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
// edmx:DataServices
// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct DataServices {
    #[serde(rename = "Schema", default)]
    pub schemas: Vec<Schema>,
}

impl DataServices {
    pub fn default_schema(&self) -> Option<&Schema> {
        self.schemas
            .iter()
            .find(|schema| schema.namespace == "Default")
    }
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
// Tests
// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
#[cfg(test)]
mod unit_tests;
