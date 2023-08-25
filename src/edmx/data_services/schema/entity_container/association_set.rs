use crate::edmx::data_services::schema::association::End;
use crate::sap_annotations::default_sap_content_version;
use crate::utils::{de_str_to_bool, default_true};
use serde::{Deserialize, Serialize};

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
/// Represents an `<AssociationSet>` tag
///
/// # Child Nodes
/// `2:2 End`
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct AssociationSet {
    pub name: String,
    pub association: String,

    #[serde(rename = "sap:content-version", default = "default_sap_content_version")]
    pub sap_content_version: String,

    #[serde(
        rename = "sap:creatable",
        deserialize_with = "de_str_to_bool",
        default = "default_true"
    )]
    pub sap_creatable: bool,

    #[serde(
        rename = "sap:deletable",
        deserialize_with = "de_str_to_bool",
        default = "default_true"
    )]
    pub sap_deletable: bool,

    #[serde(
        rename = "sap:updatable",
        deserialize_with = "de_str_to_bool",
        default = "default_true"
    )]
    pub sap_updatable: bool,

    #[serde(rename = "End")]
    pub ends: [End; 2],
}
