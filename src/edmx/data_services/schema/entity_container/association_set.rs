use crate::edmx::data_services::schema::association::End;
use crate::sap_annotations::{default_sap_creatable, default_sap_deletable, default_sap_updatable};
use serde::{Deserialize, Serialize};

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
// AssociationSet
//
// Child Nodes:
//   2:2 End
// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct AssociationSet {
    pub name: String,
    pub association: String,

    #[serde(rename = "sap:createable", default = "default_sap_creatable")]
    pub sap_creatable: bool,

    #[serde(rename = "sap:deletable", default = "default_sap_deletable")]
    pub sap_deletable: bool,

    #[serde(rename = "sap:updatable", default = "default_sap_updatable")]
    pub sap_updatable: bool,

    #[serde(rename = "End")]
    pub ends: [End; 2],
}
