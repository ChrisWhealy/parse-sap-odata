pub mod association;
pub mod complex_type;
pub mod entity_container;
pub mod entity_set;
pub mod entity_type;

use association::{Association, AssociationSet};
use complex_type::ComplexType;
use entity_container::EntityContainer;
use entity_set::EntitySet;
use entity_type::EntityType;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Schema {
    pub namespace: String,

    #[serde(rename = "EntityType", default)]
    pub entities: Vec<EntityType>,

    #[serde(rename = "ComplexType", default)]
    pub complex_types: Option<Vec<ComplexType>>,

    #[serde(rename = "Association", default)]
    pub associations: Vec<Association>,

    pub entity_container: Option<EntityContainer>,
}

impl Schema {
    pub fn entity_sets(&self) -> Option<&Vec<EntitySet>> {
        self.entity_container
            .as_ref()
            .map(|container| &container.entity_sets)
    }

    pub fn association_sets(&self) -> Option<&Vec<AssociationSet>> {
        self.entity_container
            .as_ref()
            .map(|container| &container.association_sets)
    }
}
