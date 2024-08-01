pub mod referential_constraint;

use serde::{Deserialize, Serialize};

use referential_constraint::ReferentialConstraint;

use crate::sap_annotations::default_sap_content_version;

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
/// Represents an `<Association>` tag
///
/// # Child Nodes
/// `2:2 End`<br>
/// `0:1 ReferentialConstraint`
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Association {
    #[serde(rename = "@Name")]
    pub name: String,

    #[serde(rename = "@content-version", default = "default_sap_content_version")]
    pub sap_content_version: String,

    #[serde(rename = "End")]
    pub ends: [End; 2],

    pub referential_constraint: Option<ReferentialConstraint>,
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
/// Represents an `<End>` tag
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct End {
    #[serde(rename = "@Role")]
    pub role: String,

    #[serde(rename = "@EntitySet")]
    pub entity_set: Option<String>,

    #[serde(rename = "@Type")]
    pub end_type: Option<String>,

    #[serde(rename = "@Multiplicity")]
    pub multiplicity: Option<String>,
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
#[cfg(test)]
pub mod unit_tests;
