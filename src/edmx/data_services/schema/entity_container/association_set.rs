use crate::default_true;
use crate::edmx::data_services::schema::association::End;
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

    #[serde(rename = "sap:createable", default = "default_true")]
    pub sap_creatable: bool,

    #[serde(rename = "sap:deletable", default = "default_true")]
    pub sap_deletable: bool,

    #[serde(rename = "sap:updatable", default = "default_true")]
    pub sap_updatable: bool,

    #[serde(rename = "End")]
    pub ends: [End; 2],
}
