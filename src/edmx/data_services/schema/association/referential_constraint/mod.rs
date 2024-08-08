use serde::{Deserialize, Serialize};

use dependent::Dependent;
use principal::Principal;

pub mod dependent;
#[cfg(feature = "parser")]
pub mod metadata;
pub mod principal;

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
/// Represents a `<ReferentialConstraint>` tag
///
/// # Child Nodes
/// `1:1 Principal`<br>
/// `1:1 Dependent`
#[derive(Clone, Debug, Serialize, Deserialize, Ord, Eq, PartialOrd, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct ReferentialConstraint {
    pub principal: Principal,
    pub dependent: Dependent,
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
#[cfg(test)]
pub mod unit_tests;
