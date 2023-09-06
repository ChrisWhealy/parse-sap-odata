use crate::sap_annotations::entity_set_sap_annotations::EntitySetSAPAnnotations;
use serde::{Deserialize, Serialize};

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
/// Represents an `<EntitySet>` tag
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct EntitySet {
    pub name: String,
    pub entity_type: String,

    #[serde(flatten)]
    pub sap_annotations: EntitySetSAPAnnotations,
}

impl EntitySet {
    pub fn to_enum_entry(&self) -> &[u8] {
        if let Some(idx) = self.entity_type.find('.') {
            let (_prefix, enum_entry) = self.entity_type.split_at(idx);
            enum_entry.as_bytes()
        } else {
            self.entity_type.as_bytes()
        }
    }
}
