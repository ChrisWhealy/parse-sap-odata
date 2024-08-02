use serde::{Deserialize, Serialize};

use crate::sap_annotations::entity_set::SAPAnnotationsEntitySet;

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
/// Represents an `<EntitySet>` tag
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct EntitySet {
    #[serde(rename = "@Name")]
    pub name: String,
    #[serde(rename = "@EntityType")]
    pub entity_type: String,
    #[serde(flatten)]
    pub sap_annotations: SAPAnnotationsEntitySet,
}

impl EntitySet {
    pub fn to_enum_entry(&self) -> &[u8] {
        // This is safe because an EntitySet type name always follows the pattern <Schema>.<EntityType>
        self.entity_type.split(".").collect::<Vec<&str>>()[1].as_bytes()
    }
}
