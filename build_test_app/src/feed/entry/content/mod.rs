use crate::xml::{default_xml_namespace_d, default_xml_namespace_m};
use serde::{Deserialize, Serialize};

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
/// Represents an Atom `<content>` tag
///
/// # Child Nodes
/// `1:1 m:properties: <T>` where `<T>` is the entity type of this particular entity set
#[derive(Debug, Serialize, Deserialize)]
pub struct Content<T> {
    #[serde(rename = "xmlns:m", default = "default_xml_namespace_m")]
    pub namespace_m: String,

    #[serde(rename = "xmlns:d", default = "default_xml_namespace_d")]
    pub namespace_d: String,

    // quick_xml strips the namespace "m:" from the start of the XML tag name!
    pub properties: T,
}
