use std::fmt::Formatter;

use serde::{Deserialize, Serialize};

use crate::{
    parser::syntax_fragments::{
        fragment_generators::gen_owned_string, COLON, END_BLOCK, LINE_FEED, OPEN_CURLY, PROPERTYREF,
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

impl std::fmt::Display for PropertyRef {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut src_str: Vec<u8> = vec![];

        src_str.append(&mut [PROPERTYREF, OPEN_CURLY, LINE_FEED].concat());
        src_str.append(
            &mut [
                "name".as_bytes(),
                COLON,
                &*gen_owned_string(&odata_name_to_rust_safe_name(&self.name)),
                LINE_FEED,
            ]
            .concat(),
        );
        src_str.append(&mut END_BLOCK.to_vec());

        write!(f, "{}", String::from_utf8(src_str).unwrap())
    }
}
