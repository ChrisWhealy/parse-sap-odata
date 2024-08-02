use serde::{Deserialize, Serialize};

use crate::{
    edmx::data_services::schema::association::End, sap_annotations::association_set::SAPAnnotationsAssociationSet,
};

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
/// Represents an `<AssociationSet>` tag
///
/// # Child Nodes
/// `2:2 End`
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct AssociationSet {
    #[serde(rename = "@Name")]
    pub name: String,
    #[serde(rename = "@Association")]
    pub association: String,
    #[serde(flatten)]
    pub sap_annotations: SAPAnnotationsAssociationSet,
    #[serde(rename = "End")]
    pub ends: [End; 2],
}
