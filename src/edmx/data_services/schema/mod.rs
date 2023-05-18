pub mod association;
pub mod atom_link;
pub mod complex_type;
pub mod entity_container;
pub mod entity_type;

use crate::default_xml_language;
use crate::sap_annotations::default_sap_schema_version;
use association::Association;
use atom_link::AtomLink;
use complex_type::ComplexType;
use entity_container::EntityContainer;
use entity_type::EntityType;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Schema {
    #[serde(rename = "Namespace", default)]
    pub namespace: String,

    #[serde(rename = "xml:lang", default = "default_xml_language")]
    pub lang: String,

    #[serde(rename = "sap:schema_version", default = "default_sap_schema_version")]
    pub sap_schema_version: String,

    #[serde(rename = "EntityType", default)]
    pub entity_types: Vec<EntityType>,

    #[serde(rename = "ComplexType", default)]
    pub complex_types: Option<Vec<ComplexType>>,

    #[serde(rename = "Association", default)]
    pub associations: Vec<Association>,

    #[serde(rename = "EntityContainer", default)]
    pub entity_container: Option<EntityContainer>,

    // Appears in the XML as "atom:link"
    #[serde(rename = "link")]
    pub atom_links: Vec<AtomLink>,
}
