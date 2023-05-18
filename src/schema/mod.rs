// pub mod atom_link;
pub mod association;
pub mod complex_type;
pub mod entity_container;
pub mod entity_type;

use association::Association;
// use atom_link::AtomLink;
use complex_type::ComplexType;
use entity_container::EntityContainer;
use entity_type::EntityType;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Schema {
    #[serde(rename = "Namespace", default)]
    pub namespace: String,

    #[serde(rename = "EntityType", default)]
    pub entity_types: Vec<EntityType>,

    #[serde(rename = "ComplexType", default)]
    pub complex_types: Option<Vec<ComplexType>>,

    #[serde(rename = "Association", default)]
    pub associations: Vec<Association>,

    #[serde(rename = "EntityContainer", default)]
    pub entity_container: Option<EntityContainer>,
    // quick-xml is not able to handle tagnames containing a colon character :-(
    // #[serde(rename = "atom:link")]
    // pub atom_links: Vec<AtomLink>,
}
