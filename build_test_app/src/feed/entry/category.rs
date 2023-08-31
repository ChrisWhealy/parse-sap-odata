use serde::{Deserialize, Serialize};

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
/// Represents an Atom `<category>`
#[derive(Debug, Serialize, Deserialize)]
pub struct Category {
    pub term: String,
    pub scheme: String,
}
