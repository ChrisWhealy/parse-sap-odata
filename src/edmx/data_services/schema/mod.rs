use serde::{Deserialize, Serialize};

use association::Association;
use complex_type::ComplexType;
use entity_container::EntityContainer;
use entity_type::EntityType;

use crate::{
    atom::AtomLink,
    oasis::annotations::Annotations,
    sap_annotations::schema::SAPAnnotationsSchema,
    xml::{default_xml_language, default_xml_namespace},
};

pub mod association;
pub mod complex_type;
pub mod entity_container;
pub mod entity_type;
#[cfg(feature = "parser")]
pub mod metadata;

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
/// Represents a `<Schema>` tag
///
/// # Child Nodes
///
/// `1:n EntityType`<br>
/// `0:n Association`<br>
/// `0:n ComplexType`<br>
/// `1:1 EntityContainer`
#[derive(Debug, Serialize, Deserialize)]
pub struct Schema {
    #[serde(rename = "@xmlns", default = "default_xml_namespace")]
    pub xml_namespace: String,
    #[serde(rename = "@Namespace", default)]
    pub namespace: String,
    #[serde(rename = "@xml:lang", default = "default_xml_language")]
    pub xml_lang: String,
    #[serde(flatten)]
    pub sap_annotations: SAPAnnotationsSchema,
    #[serde(rename = "EntityType", default)]
    pub entity_types: Vec<EntityType>,
    #[serde(rename = "ComplexType", default)]
    pub complex_types: Option<Vec<ComplexType>>,
    #[serde(rename = "Association", default)]
    pub associations: Vec<Association>,
    #[serde(rename = "EntityContainer", default)]
    pub entity_container: Option<EntityContainer>,
    #[serde(rename = "Annotations", default)]
    pub annotation_list: Option<Vec<Annotations>>,
    // Appears in the original XML as the tagname "atom:link"
    #[serde(rename = "link")]
    pub atom_links: Vec<AtomLink>,
}
