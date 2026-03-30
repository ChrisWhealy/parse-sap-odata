use std::fmt::Formatter;

use crate::edmx::data_services::schema::association::referential_constraint::ReferentialConstraint;
use crate::parser::generate::syntax_fragments::{CLOSE_CURLY, COLON, COMMA, LINE_FEED, OPEN_CURLY};

static MY_NAME: &[u8] = "ReferentialConstraint".as_bytes();

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
enum ReferentialConstraintFieldNames {
    Principal,
    Dependent,
}

impl ReferentialConstraintFieldNames {
    pub fn value(prop_name: ReferentialConstraintFieldNames) -> &'static [u8] {
        match prop_name {
            ReferentialConstraintFieldNames::Principal => b"principal",
            ReferentialConstraintFieldNames::Dependent => b"dependent",
        }
    }
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
fn line_from_ref_con(f: &mut Formatter<'_>, prop_md: ReferentialConstraintFieldNames, val: &[u8]) -> std::fmt::Result {
    for s in [ReferentialConstraintFieldNames::value(prop_md), COLON, val, COMMA, LINE_FEED] {
        write!(f, "{}", std::str::from_utf8(s).unwrap())?;
    }
    Ok(())
}

impl std::fmt::Display for ReferentialConstraint {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let principal_val = format!("{}", &self.principal);
        let dependent_val = format!("{}", &self.dependent);

        write!(f, "{}", std::str::from_utf8(MY_NAME).unwrap())?;
        write!(f, "{}", std::str::from_utf8(OPEN_CURLY).unwrap())?;
        line_from_ref_con(f, ReferentialConstraintFieldNames::Principal, principal_val.as_bytes())?;
        line_from_ref_con(f, ReferentialConstraintFieldNames::Dependent, dependent_val.as_bytes())?;
        write!(f, "{}", std::str::from_utf8(CLOSE_CURLY).unwrap())
    }
}
