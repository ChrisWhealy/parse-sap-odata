use std::fmt::Formatter;

use crate::{
    parser::generate::{
        gen_owned_string,
        syntax_fragments::{CLOSE_CURLY, COLON, LINE_FEED, OPEN_CURLY},
    },
    property::property_ref::PropertyRef,
    utils::odata_name_to_rust_safe_name,
};

static MY_NAME: &[u8] = "PropertyRef".as_bytes();

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
impl std::fmt::Display for PropertyRef {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let out_buffer: Vec<u8> = [
            MY_NAME,
            OPEN_CURLY,
            LINE_FEED,
            "name".as_bytes(),
            COLON,
            &*gen_owned_string(&odata_name_to_rust_safe_name(&self.name)),
            CLOSE_CURLY,
        ]
        .concat();

        write!(f, "{}", String::from_utf8(out_buffer).unwrap())
    }
}
