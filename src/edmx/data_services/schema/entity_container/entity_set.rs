use crate::sap_annotations::{
    default_sap_content_version, default_sap_creatable, default_sap_deletable,
    default_sap_pageable, default_sap_updatable,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct EntitySet {
    pub name: String,
    pub entity_type: String,

    #[serde(
        rename = "sap:content-version",
        default = "default_sap_content_version"
    )]
    pub sap_content_version: String,

    #[serde(rename = "sap:createable", default = "default_sap_creatable")]
    pub sap_creatable: bool,

    #[serde(rename = "sap:deletable", default = "default_sap_deletable")]
    pub sap_deletable: bool,

    #[serde(rename = "sap:updatable", default = "default_sap_updatable")]
    pub sap_updatable: bool,

    #[serde(rename = "sap:pageable", default = "default_sap_pageable")]
    pub sap_pageable: bool,
}
