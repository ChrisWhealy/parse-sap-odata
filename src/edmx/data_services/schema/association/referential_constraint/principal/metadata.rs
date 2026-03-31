use std::fmt::Formatter;

use crate::parser::generate::syntax_fragments::{
    CLOSE_CURLY, CLOSE_SQR, COLON, COMMA, LINE_FEED, OPEN_CURLY, VEC_BANG,
};
use crate::{
    edmx::data_services::schema::association::referential_constraint::principal::Principal,
    parser::generate::gen_owned_string,
};

static MY_NAME: &str = "Principal";

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
enum PrincipalFieldNames {
    Role,
    PropertyRefs,
}

impl PrincipalFieldNames {
    fn value(prop_name: PrincipalFieldNames) -> &'static str {
        match prop_name {
            PrincipalFieldNames::Role => "role",
            PrincipalFieldNames::PropertyRefs => "property_refs",
        }
    }
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
fn line_from_dependent(f: &mut Formatter<'_>, prop_md: PrincipalFieldNames, val: &str) -> std::fmt::Result {
    write!(f, "{}{}{}{}{}", PrincipalFieldNames::value(prop_md), COLON, val, COMMA, LINE_FEED)
}

impl std::fmt::Display for Principal {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let prop_refs_str = self
            .property_refs
            .iter()
            .map(|pr| format!("{},", pr))
            .collect::<String>();
        let prop_refs_val = [VEC_BANG, &prop_refs_str, CLOSE_SQR].concat();

        write!(f, "{MY_NAME}")?;
        write!(f, "{OPEN_CURLY}")?;
        line_from_dependent(f, PrincipalFieldNames::Role, &gen_owned_string(&self.role))?;
        line_from_dependent(f, PrincipalFieldNames::PropertyRefs, &prop_refs_val)?;
        write!(f, "{CLOSE_CURLY}")
    }
}
