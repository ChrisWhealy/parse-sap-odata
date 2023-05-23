pub mod include;

use crate::xml::default_xml_namespace_oasis;
use include::Include;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Reference {
    #[serde(rename = "xmlns:edmx", default = "default_xml_namespace_oasis")]
    pub xml_namespace_edmx: String,

    pub uri: Option<String>,

    #[serde(rename = "edmx:Include")]
    pub include: Option<Include>,
}
