use std::fmt::Formatter;

use crate::{
    parser::generate::{
        gen_owned_string_src,
        syntax_fragments::{CLOSE_CURLY, COLON, OPEN_CURLY},
    },
    property::property_ref::PropertyRef,
    utils::odata_name_to_rust_safe_name,
};

static MY_NAME: &str = "PropertyRef";

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
impl std::fmt::Display for PropertyRef {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{MY_NAME}{OPEN_CURLY}name{COLON}{}{CLOSE_CURLY}",
            &gen_owned_string_src(&odata_name_to_rust_safe_name(&self.name))
        )
    }
}
