pub mod category;
pub mod content;

use crate::feed::{entry::content::Content, AtomLink};
use serde::{Deserialize, Serialize};

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
/// Represents an Atom `<entry>` of type `<T>` where `<T>` is the entity type of this particular entity set
///
/// # Child Nodes
/// `1:1 id`<br>
/// `1:1 title`<br>
/// `1:1 updated`<br>
/// `1:1 category`<br>
/// `1:n link`<br>
/// `1:1 content`<br>
#[derive(Debug, Serialize, Deserialize)]
pub struct Entry<T> {
    #[serde(rename = "m:etag")]
    pub etag: Option<String>,

    pub id: String,
    pub title: String,
    pub updated: String,
    pub category: String,

    #[serde(rename = "link")]
    pub links: Vec<AtomLink>,

    pub content: Content<T>,
}
