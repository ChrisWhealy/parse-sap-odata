use crate::sap_annotations::{
    default_sap_creatable, default_sap_filterable, default_sap_sortable, default_sap_unicode,
    default_sap_updatable,
};
use crate::xml::default_true;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Property {
    pub name: String,

    #[serde(rename = "Type")]
    pub property_type: String,

    #[serde(default = "default_true")]
    pub nullable: bool,

    pub max_length: Option<String>,

    pub precision: Option<String>,
    pub scale: Option<String>,
    pub concurrency_mode: Option<String>,

    // SAP Annotations
    #[serde(rename = "sap:unicode", default = "default_sap_unicode")]
    pub sap_unicode: bool,

    #[serde(rename = "sap:display-format")]
    pub sap_display_format: Option<String>,

    #[serde(rename = "sap:createable", default = "default_sap_creatable")]
    pub sap_creatable: bool,

    #[serde(rename = "sap:filterable", default = "default_sap_filterable")]
    pub sap_filterable: bool,

    #[serde(rename = "sap:label")]
    pub sap_label: Option<String>,

    #[serde(rename = "sap:quickinfo")]
    pub sap_quick_info: Option<String>,

    #[serde(rename = "sap:semantics")]
    pub sap_semantics: Option<String>,

    #[serde(rename = "sap:sortable", default = "default_sap_sortable")]
    pub sap_sortable: bool,

    #[serde(rename = "sap:text")]
    pub sap_text: Option<String>,

    #[serde(rename = "sap:unit")]
    pub sap_unit: Option<String>,

    #[serde(rename = "sap:updatable", default = "default_sap_updatable")]
    pub sap_updatable: bool,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct PropertyRef {
    pub name: String,
}
