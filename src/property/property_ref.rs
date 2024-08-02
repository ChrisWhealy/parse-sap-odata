#[cfg(feature = "parser")]
use std::fmt::Formatter;

use serde::{Deserialize, Serialize};

#[cfg(feature = "parser")]
use crate::{
    parser::syntax_fragments::{
        COLON, END_BLOCK, fragment_generators::gen_owned_string, LINE_FEED, OPEN_CURLY, PROPERTYREF,
    },
    utils::odata_name_to_rust_safe_name,
};

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
/// Represents a `<PropertyRef>` tag
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct PropertyRef {
    #[serde(rename = "@Name")]
    pub name: String,
}

#[cfg(feature = "parser")]
impl std::fmt::Display for PropertyRef {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let out_buffer: Vec<u8> = [
            PROPERTYREF,
            OPEN_CURLY,
            LINE_FEED,
            "name".as_bytes(),
            COLON,
            &*gen_owned_string(&odata_name_to_rust_safe_name(&self.name)),
            LINE_FEED,
            END_BLOCK
        ]
            .concat();

        write!(f, "{}", String::from_utf8(out_buffer).unwrap())
    }
}
