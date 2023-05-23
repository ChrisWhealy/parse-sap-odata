use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Include {
    pub namespace: String,
    pub alias: String,
}
