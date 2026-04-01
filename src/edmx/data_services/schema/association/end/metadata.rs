use super::End;
use std::fmt::Formatter;

use crate::{
    parser::generate::{
        gen_opt_string_src, gen_owned_string_src,
        syntax_fragments::{CLOSE_CURLY, COLON, COMMA, LINE_FEED, OPEN_CURLY},
    },
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
        let entity_set = self.entity_set.as_ref().map(|es| to_upper_camel_case(es));

        let end_type = self.end_type.as_ref().map(|et| if let Some((_, part2)) = et.split_once('.') {
                to_upper_camel_case(part2)
            } else {
                // This branch should never be used because SAP should always generate a fully qualified name...
                et.to_owned()
            });

        write!(f, "{MY_NAME}")?;
        write!(f, "{OPEN_CURLY}")?;
        line_from_end(f, EndFieldNames::Role, &gen_owned_string_src(&self.role))?;
        line_from_end(f, EndFieldNames::EntitySet, &gen_opt_string_src(&entity_set))?;
        line_from_end(f, EndFieldNames::EndType, &gen_opt_string_src(&end_type))?;
        line_from_end(f, EndFieldNames::Multiplicity, &gen_opt_string_src(&self.multiplicity))?;
        write!(f, "{CLOSE_CURLY}")
    }
}
