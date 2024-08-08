use serde::{Deserialize, Serialize};

use association_set::AssociationSet;
use entity_set::EntitySet;
use function_import::FunctionImport;

use crate::{
    sap_annotations::entity_container::SAPAnnotationsEntityContainer,
    utils::{de_str_to_bool, default_false},
};

pub mod association_set;
pub mod entity_set;
pub mod function_import;
#[cfg(feature = "parser")]
pub mod metadata;

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
/// Represents an `<EntityContainer>` tag
///
/// # Child Nodes
/// `1:n EntitySet`<br>
/// `1:n AssociationSet`<br>
/// `0:n FunctionImport`
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct EntityContainer {
    #[serde(rename = "@Name")]
    pub name: String,
    #[serde(
        rename = "@IsDefaultEntityContainer",
        deserialize_with = "de_str_to_bool",
        default = "default_false"
    )]
    pub is_default_entity_container: bool,
    #[serde(flatten)]
    pub sap_annotations: SAPAnnotationsEntityContainer,
    #[serde(rename = "EntitySet", default)]
    pub entity_sets: Vec<EntitySet>,
    #[serde(rename = "AssociationSet", default)]
    pub association_sets: Vec<AssociationSet>,
    #[serde(rename = "FunctionImport", default)]
    pub function_imports: Option<Vec<FunctionImport>>,
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
#[cfg(test)]
pub mod unit_tests;
