use super::End;
use std::fmt::Formatter;

use crate::parser::generate::syntax_fragments::{CLOSE_CURLY, COLON, COMMA, LINE_FEED, OPEN_CURLY};
use crate::{
    parser::generate::{gen_opt_string_src, gen_owned_string_src},
    utils::to_upper_camel_case,
};

static MY_NAME: &str = "End";

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
enum EndFieldNames {
    Role,
    EntitySet,
    EndType,
    Multiplicity,
}

impl EndFieldNames {
    pub fn value(prop_name: EndFieldNames) -> &'static str {
        match prop_name {
            EndFieldNames::Role => "role",
            EndFieldNames::EntitySet => "entity_set",
            EndFieldNames::EndType => "end_type",
            EndFieldNames::Multiplicity => "multiplicity",
        }
    }
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
fn line_from_end(f: &mut Formatter<'_>, prop_md: EndFieldNames, val: &str) -> std::fmt::Result {
    write!(f, "{}{}{}{}{}", EndFieldNames::value(prop_md), COLON, val, COMMA, LINE_FEED)
}

impl std::fmt::Display for End {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let entity_set = if let Some(es) = &self.entity_set {
            Some(to_upper_camel_case(es)) // Convert to struct name
        } else {
            None
        };

        let end_type = if let Some(et) = &self.end_type {
            let et_parts = et.split(".").collect::<Vec<&str>>();

            Some(if et_parts.len() == 2 {
                to_upper_camel_case(et_parts[1])
            } else {
                // This branch should never be used because SAP should always generate a fully qualified name...
                et.to_owned()
            })
        } else {
            None
        };

        write!(f, "{MY_NAME}")?;
        write!(f, "{OPEN_CURLY}")?;
        line_from_end(f, EndFieldNames::Role, &gen_owned_string_src(&self.role))?;
        line_from_end(f, EndFieldNames::EntitySet, &gen_opt_string_src(&entity_set))?;
        line_from_end(f, EndFieldNames::EndType, &gen_opt_string_src(&end_type))?;
        line_from_end(f, EndFieldNames::Multiplicity, &gen_opt_string_src(&self.multiplicity))?;
        write!(f, "{CLOSE_CURLY}")
    }
}
