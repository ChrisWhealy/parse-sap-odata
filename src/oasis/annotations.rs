use serde::{Deserialize, Serialize};

use super::annotation::Annotation;
use crate::xml::default_xml_namespace_oasis;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Annotations {
    #[serde(rename = "@xmlns", default = "default_xml_namespace_oasis")]
    pub xml_namespace: String,
    #[serde(rename = "@Target")]
    pub target: Option<String>,
    #[serde(rename = "Annotation")]
    pub annotation_list: Option<Vec<Annotation>>,
}
