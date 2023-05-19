use crate::xml::default_xml_namespace_atom;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct AtomLink {
    #[serde(rename = "xmlns:atom", default = "default_xml_namespace_atom")]
    pub xml_namespace_atom: String,

    pub rel: String,
    pub href: String,

    #[serde(rename = "type")]
    pub mime_type: Option<String>,

    pub title: Option<String>,
}
