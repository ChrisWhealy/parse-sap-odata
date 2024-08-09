use std::fmt::Formatter;

use crate::{
    edmx::data_services::schema::association::referential_constraint::principal::Principal,
    parser::syntax_fragments::{
        fragment_generators::gen_owned_string, CLOSE_CURLY, CLOSE_SQR, COLON, COMMA, LINE_FEED, OPEN_CURLY, VEC_BANG,
    },
};

static MY_NAME: &[u8] = "Principal".as_bytes();

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
enum PrincipalFieldNames {
    Role,
    PropertyRefs,
}

impl PrincipalFieldNames {
    fn value(prop_name: PrincipalFieldNames) -> Vec<u8> {
        let member = match prop_name {
            PrincipalFieldNames::Role => "role",
            PrincipalFieldNames::PropertyRefs => "property_refs",
        };

        member.as_bytes().to_vec()
    }
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
fn line_from_dependent(prop_md: PrincipalFieldNames, val: Vec<u8>) -> Vec<u8> {
    [&*PrincipalFieldNames::value(prop_md), COLON, &val, COMMA, LINE_FEED].concat()
}

impl std::fmt::Display for Principal {
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
            &*line_from_dependent(PrincipalFieldNames::Role, gen_owned_string(&self.role)),
            &*line_from_dependent(
                PrincipalFieldNames::PropertyRefs,
                [VEC_BANG, &*prop_refs_str, CLOSE_SQR].concat(),
            ),
            CLOSE_CURLY,
        ]
        .concat();

        write!(f, "{}", String::from_utf8(out_buffer).unwrap())
    }
}
