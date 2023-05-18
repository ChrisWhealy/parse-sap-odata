use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct AtomLink {
    pub rel: String,
    pub href: String,
}
