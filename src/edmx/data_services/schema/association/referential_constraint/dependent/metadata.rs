use std::fmt::Formatter;

use crate::{
    edmx::data_services::schema::association::referential_constraint::dependent::Dependent,
    parser::generate::gen_owned_string,
};
use crate::parser::generate::syntax_fragments::{CLOSE_CURLY, CLOSE_SQR, COLON, COMMA, LINE_FEED, OPEN_CURLY, VEC_BANG};

static MY_NAME: &[u8] = "Dependent".as_bytes();

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
enum DependentFieldNames {
    Role,
    PropertyRefs,
}

impl DependentFieldNames {
    fn value(prop_name: DependentFieldNames) -> Vec<u8> {
        let member = match prop_name {
            DependentFieldNames::Role => "role",
            DependentFieldNames::PropertyRefs => "property_refs",
        };

        member.as_bytes().to_vec()
    }
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
fn line_from_dependent(prop_md: DependentFieldNames, val: Vec<u8>) -> Vec<u8> {
    [&*DependentFieldNames::value(prop_md), COLON, &val, COMMA, LINE_FEED].concat()
}

impl std::fmt::Display for Dependent {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let prop_refs_str = self
            .property_refs
            .clone()
            .into_iter()
            .map(|pr| format!("{},", pr))
            .collect::<String>()
            .into_bytes();

        let out_buffer: Vec<u8> = [
            MY_NAME,
            OPEN_CURLY,
            &*line_from_dependent(DependentFieldNames::Role, gen_owned_string(&self.role)),
            &*line_from_dependent(
                DependentFieldNames::PropertyRefs,
                [VEC_BANG, &*prop_refs_str, CLOSE_SQR].concat(),
            ),
            CLOSE_CURLY,
        ]
        .concat();

        write!(f, "{}", String::from_utf8(out_buffer).unwrap())
    }
}
