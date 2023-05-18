pub mod data_services;

use crate::xml::{default_xml_namespace_edmx, default_xml_namespace_m, default_xml_namespace_sap};
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

    #[serde(rename = "xmlns:edmx", default = "default_xml_namespace_edmx")]
    pub namespace_edmx: String,

    #[serde(rename = "xmlns:m", default = "default_xml_namespace_m")]
    pub namespace_m: String,

    #[serde(rename = "xmlns:sap", default = "default_xml_namespace_sap")]
    pub namespace_sap: String,

    pub data_services: DataServices,
}

impl std::str::FromStr for Edmx {
    type Err = quick_xml::DeError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        quick_xml::de::from_str(s)
    }
}
