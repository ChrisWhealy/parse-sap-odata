pub mod include;

use crate::xml::default_xml_namespace_oasis;
use include::Include;
use serde::{Deserialize, Serialize};

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
/// Represents an optional `<edmx:Reference>` tag
///
/// # Child Nodes
/// `1:1 edmx:Include`

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Reference {
    #[serde(rename = "@xmlns:edmx", default = "default_xml_namespace_oasis")]
    pub xml_namespace_edmx: String,

    #[serde(rename = "@Uri")]
    pub uri: Option<String>,

    pub include: Option<Include>,
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
#[cfg(test)]
pub mod unit_tests;
