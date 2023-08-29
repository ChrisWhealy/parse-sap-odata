use serde::{Deserialize, Serialize};

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
/// Represents an Atom `<link>` tag within a `<Feed>`
#[derive(Debug, Serialize, Deserialize)]
pub struct AtomLink {
    pub href: String,
    pub rel: String,
    pub title: Option<String>,
}
