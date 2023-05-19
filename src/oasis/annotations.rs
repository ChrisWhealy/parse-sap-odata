use super::annotation::Annotation;
use crate::xml::default_xml_namespace_oasis;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Annotations {
    #[serde(rename = "xmlns", default = "default_xml_namespace_oasis")]
    pub xml_namespace: String,

    pub target: String,

    #[serde(rename = "Annotation")]
    pub annotation_list: Option<Vec<Annotation>>,
}
