use std::fmt::Formatter;

use crate::{
    parser::generate::{
        gen_owned_string,
        syntax_fragments::{CLOSE_CURLY, COLON, LINE_FEED, OPEN_CURLY},
    },
    property::property_ref::PropertyRef,
    utils::odata_name_to_rust_safe_name,
};

static MY_NAME: &str = "PropertyRef";

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
impl std::fmt::Display for PropertyRef {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let name_val = gen_owned_string(&odata_name_to_rust_safe_name(&self.name));
        write!(f, "{MY_NAME}{OPEN_CURLY}{LINE_FEED}name{COLON}{name_val}{CLOSE_CURLY}")
    }
}
