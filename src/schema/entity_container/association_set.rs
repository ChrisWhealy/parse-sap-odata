use crate::schema::association::End;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct AssociationSet {
    pub name: String,
    pub association: String,

    #[serde(rename = "End")]
    pub ends: [End; 2],
}
