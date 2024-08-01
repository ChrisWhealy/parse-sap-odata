use serde::{Deserialize, Serialize};

use crate::xml::default_xml_namespace_atom;

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
/// Represents an `<atom:link>` tag
#[derive(Debug, Serialize, Deserialize)]
pub struct AtomLink {
    #[serde(rename = "@atom", default = "default_xml_namespace_atom")]
    pub xml_namespace_atom: Option<String>,

    #[serde(rename = "@type")]
    pub mime_type: Option<String>,

    #[serde(rename = "@rel")]
    pub rel: String,

    #[serde(rename = "@href")]
    pub href: String,

    #[serde(rename = "@title")]
    pub title: Option<String>,
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
#[cfg(test)]
pub mod unit_tests;
