pub mod data_services;

use data_services::DataServices;
use serde::{Deserialize, Serialize};

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
// Entity Data Model
//
// WARNING: quick-xml strips the namespace from XML tag names, but not attribute names!
//
// Consequently, tag names such as "edmx:DataServices" and "atom:link" will appear simply as "DataServices" and "link"
// etc, but attribute names such as "sap:schema-version" will appear without modification
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

impl std::str::FromStr for Edmx {
    type Err = quick_xml::DeError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        quick_xml::de::from_str(s)
    }
}
