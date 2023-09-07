use crate::{edmx::data_services::schema::association::End, sap_annotations::SAPAnnotationsAssociationSet};
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

    #[serde(flatten)]
    pub sap_annotations: SAPAnnotationsAssociationSet,

    #[serde(rename = "End")]
    pub ends: [End; 2],
}
