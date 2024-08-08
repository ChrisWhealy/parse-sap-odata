use std::fmt::Formatter;

use crate::{
    edmx::data_services::schema::association::referential_constraint::ReferentialConstraint,
    parser::syntax_fragments::{CLOSE_CURLY, COLON, COMMA, LINE_FEED, OPEN_CURLY},
};

static MY_NAME: &[u8] = "ReferentialConstraint".as_bytes();

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
pub enum ReferentialConstraintMetadata {
    Principal,
    Dependent,
}

impl ReferentialConstraintMetadata {
    pub fn get_field_name(prop_name: ReferentialConstraintMetadata) -> Vec<u8> {
        let member = match prop_name {
            ReferentialConstraintMetadata::Principal => "principal",
            ReferentialConstraintMetadata::Dependent => "dependent",
        };

        member.as_bytes().to_vec()
    }
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
fn line_from_ref_con(prop_md: ReferentialConstraintMetadata, val: Vec<u8>) -> Vec<u8> {
    [
        &*ReferentialConstraintMetadata::get_field_name(prop_md),
        COLON,
        &val,
        COMMA,
        LINE_FEED,
    ]
    .concat()
}

impl std::fmt::Display for ReferentialConstraint {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let out_buffer: Vec<u8> = [
            MY_NAME,
            OPEN_CURLY,
            &*line_from_ref_con(
                ReferentialConstraintMetadata::Principal,
                format!("{}", &self.principal).into_bytes(),
            ),
            &*line_from_ref_con(
                ReferentialConstraintMetadata::Dependent,
                format!("{}", &self.dependent).into_bytes(),
            ),
            CLOSE_CURLY,
        ]
        .concat();

        write!(f, "{}", String::from_utf8(out_buffer).unwrap())
    }
}
