use std::fmt::Formatter;

use crate::{
    edmx::data_services::schema::association::referential_constraint::ReferentialConstraint,
    parser::generate::syntax_fragments::{CLOSE_CURLY, COLON, COMMA, LINE_FEED, OPEN_CURLY},
};

static MY_NAME: &str = "ReferentialConstraint";

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
enum ReferentialConstraintFieldNames {
    Principal,
    Dependent,
}

impl ReferentialConstraintFieldNames {
    pub fn value(prop_name: ReferentialConstraintFieldNames) -> &'static str {
        match prop_name {
            ReferentialConstraintFieldNames::Principal => "principal",
            ReferentialConstraintFieldNames::Dependent => "dependent",
        }
    }
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
fn line_from_ref_con(f: &mut Formatter<'_>, prop_md: ReferentialConstraintFieldNames, val: &str) -> std::fmt::Result {
    write!(
        f,
        "{}{}{}{}{}",
        ReferentialConstraintFieldNames::value(prop_md),
        COLON,
        val,
        COMMA,
        LINE_FEED
    )
}

impl std::fmt::Display for ReferentialConstraint {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{MY_NAME}")?;
        write!(f, "{OPEN_CURLY}")?;
        line_from_ref_con(f, ReferentialConstraintFieldNames::Principal, &self.principal.to_string())?;
        line_from_ref_con(f, ReferentialConstraintFieldNames::Dependent, &self.dependent.to_string())?;
        write!(f, "{CLOSE_CURLY}")
    }
}
