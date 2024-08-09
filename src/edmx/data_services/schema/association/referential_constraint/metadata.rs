use std::fmt::Formatter;

use crate::{
    edmx::data_services::schema::association::referential_constraint::ReferentialConstraint,
    parser::syntax_fragments::{CLOSE_CURLY, COLON, COMMA, LINE_FEED, OPEN_CURLY},
};

static MY_NAME: &[u8] = "ReferentialConstraint".as_bytes();

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
enum ReferentialConstraintFieldNames {
    Principal,
    Dependent,
}

impl ReferentialConstraintFieldNames {
    pub fn value(prop_name: ReferentialConstraintFieldNames) -> Vec<u8> {
        let member = match prop_name {
            ReferentialConstraintFieldNames::Principal => "principal",
            ReferentialConstraintFieldNames::Dependent => "dependent",
        };

        member.as_bytes().to_vec()
    }
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
fn line_from_ref_con(prop_md: ReferentialConstraintFieldNames, val: Vec<u8>) -> Vec<u8> {
    [
        &*ReferentialConstraintFieldNames::value(prop_md),
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
                ReferentialConstraintFieldNames::Principal,
                format!("{}", &self.principal).into_bytes(),
            ),
            &*line_from_ref_con(
                ReferentialConstraintFieldNames::Dependent,
                format!("{}", &self.dependent).into_bytes(),
            ),
            CLOSE_CURLY,
        ]
        .concat();

        write!(f, "{}", String::from_utf8(out_buffer).unwrap())
    }
}
