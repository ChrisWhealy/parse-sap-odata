use crate::xml::default_xml_namespace_atom;
use serde::{Deserialize, Serialize};

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
/// Represents an `<atom:link>` tag
#[derive(Debug, Serialize, Deserialize)]
pub struct AtomLink {
    #[serde(rename = "xmlns:atom", default = "default_xml_namespace_atom")]
    pub xml_namespace_atom: Option<String>,

    #[serde(rename = "type")]
    pub mime_type: Option<String>,

    pub rel: String,
    pub href: String,
    pub title: Option<String>,
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
#[cfg(test)]
pub mod unit_tests;
