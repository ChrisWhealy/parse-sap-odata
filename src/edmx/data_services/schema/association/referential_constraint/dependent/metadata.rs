use std::fmt::Formatter;

use crate::{
    edmx::data_services::schema::association::referential_constraint::dependent::Dependent,
    parser::syntax_fragments::{
        fragment_generators::gen_owned_string, CLOSE_CURLY, CLOSE_SQR, COLON, COMMA, LINE_FEED, OPEN_CURLY, VEC_BANG,
    },
};

static MY_NAME: &[u8] = "Dependent".as_bytes();

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
pub enum DependentMetadata {
    Role,
    PropertyRefs,
}

impl DependentMetadata {
    fn get_field_name(prop_name: DependentMetadata) -> Vec<u8> {
        let member = match prop_name {
            DependentMetadata::Role => "role",
            DependentMetadata::PropertyRefs => "property_refs",
        };

        member.as_bytes().to_vec()
    }
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
fn line_from_dependent(prop_md: DependentMetadata, val: Vec<u8>) -> Vec<u8> {
    [&*DependentMetadata::get_field_name(prop_md), COLON, &val, COMMA, LINE_FEED].concat()
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
            &*line_from_dependent(DependentMetadata::Role, gen_owned_string(&self.role)),
            &*line_from_dependent(DependentMetadata::PropertyRefs, [VEC_BANG, &*prop_refs_str, CLOSE_SQR].concat()),
            CLOSE_CURLY,
        ]
        .concat();

        write!(f, "{}", String::from_utf8(out_buffer).unwrap())
    }
}
