use serde::{Deserialize, Serialize};

use end::End;
use referential_constraint::ReferentialConstraint;

use crate::sap_annotations::default_sap_content_version;

pub mod end;
#[cfg(feature = "parser")]
pub mod metadata;
pub mod referential_constraint;

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
/// Represents an `<Association>` tag
///
/// # Child Nodes
/// `2:2 End`<br>
/// `0:1 ReferentialConstraint`
#[derive(Clone, Debug, Serialize, Ord, Eq, PartialOrd, PartialEq, Deserialize)]
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
#[cfg(test)]
pub mod unit_tests;
