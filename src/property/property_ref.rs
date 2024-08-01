use serde::{Deserialize, Serialize};

#[cfg(feature = "parser")]
use std::fmt::Formatter;

#[cfg(feature = "parser")]
use crate::{
    utils::odata_name_to_rust_safe_name,
    parser::syntax_fragments::{
        COLON, END_BLOCK, fragment_generators::gen_owned_string, LINE_FEED, OPEN_CURLY, PROPERTYREF
        }
};

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
/// Represents a `<PropertyRef>` tag
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct PropertyRef {
    #[serde(rename = "@Name")]
    pub name: String,
}

#[cfg(feature= "parser")]
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
