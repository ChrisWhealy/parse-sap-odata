mod dependent;
mod principal;

use dependent::Dependent;
use principal::Principal;

use serde::{Deserialize, Serialize};

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
/// Represents a `<ReferentialConstraint>` tag
///
/// # Child Nodes
/// `1:1 Principal`<br>
/// `1:1 Dependent`
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ReferentialConstraint {
    pub principal: Principal,
    pub dependent: Dependent,
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
#[cfg(test)]
pub mod unit_tests;
