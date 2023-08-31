pub mod author;
pub mod entry;
pub mod link;

use crate::xml::{default_xml_namespace_atom, default_xml_namespace_d, default_xml_namespace_m};
use author::Author;
use entry::Entry;
use link::AtomLink;

use serde::{de::DeserializeOwned, Deserialize, Serialize};

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
/// Represents an Atom `<feed>`
///
/// # Child Nodes
/// `1:1 author`<br>
/// `1:1 link`<br>
/// `0:n entry`
///
/// ***WARNING:***<br>`quick-xml` strips the namespace from XML tag names, but not attribute names!
///
/// Consequently, tag names such as `<m:properties>` and `<d:Address>` will appear simply as `<properties>` and
/// `<address>` etc, but attribute names such as `xmlns:d` will appear without modification
#[derive(Debug, Serialize, Deserialize)]
pub struct Feed<T> {
    #[serde(rename = "xmlns", default = "default_xml_namespace_atom")]
    pub namespace: String,

    #[serde(rename = "xmlns:m", default = "default_xml_namespace_m")]
    pub namespace_m: String,

    #[serde(rename = "xmlns:d", default = "default_xml_namespace_d")]
    pub namespace_d: String,

    #[serde(rename = "xml:base")]
    pub xml_base: Option<String>,

    pub id: String,
    pub title: String,
    pub updated: String,
    pub author: Author,

    #[serde(rename = "link")]
    pub links: Vec<AtomLink>,

    #[serde(rename = "entry")]
    pub entries: Option<Vec<Entry<T>>>,
}

impl<T> std::str::FromStr for Feed<T>
where
    T: DeserializeOwned,
{
    type Err = quick_xml::DeError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        quick_xml::de::from_str(s)
    }
}
