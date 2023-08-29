use serde::{Deserialize, Serialize};

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
/// Represents an Atom <author>
#[derive(Debug, Serialize, Deserialize)]
pub struct Author {
    pub name: Option<String>,
    pub uri: Option<String>,
    pub email: Option<String>,
}
