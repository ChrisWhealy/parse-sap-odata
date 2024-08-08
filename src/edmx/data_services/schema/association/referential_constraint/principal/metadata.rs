use std::fmt::Formatter;

use crate::{
    edmx::data_services::schema::association::referential_constraint::principal::Principal,
    parser::syntax_fragments::{
        fragment_generators::gen_owned_string, CLOSE_CURLY, CLOSE_SQR, COLON, COMMA, LINE_FEED, OPEN_CURLY, VEC_BANG,
    },
};

static MY_NAME: &[u8] = "Principal".as_bytes();

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
pub enum PrincipalMetadata {
    Role,
    PropertyRefs,
}

impl PrincipalMetadata {
    fn get_field_name(prop_name: PrincipalMetadata) -> Vec<u8> {
        let member = match prop_name {
            PrincipalMetadata::Role => "role",
            PrincipalMetadata::PropertyRefs => "property_refs",
        };

        member.as_bytes().to_vec()
    }
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
fn line_from_dependent(prop_md: PrincipalMetadata, val: Vec<u8>) -> Vec<u8> {
    [&*PrincipalMetadata::get_field_name(prop_md), COLON, &val, COMMA, LINE_FEED].concat()
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
            &*line_from_dependent(PrincipalMetadata::Role, gen_owned_string(&self.role)),
            &*line_from_dependent(PrincipalMetadata::PropertyRefs, [VEC_BANG, &*prop_refs_str, CLOSE_SQR].concat()),
            CLOSE_CURLY,
        ]
        .concat();

        write!(f, "{}", String::from_utf8(out_buffer).unwrap())
    }
}
