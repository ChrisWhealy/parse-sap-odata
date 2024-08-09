use std::fmt::Formatter;

use crate::parser::syntax_fragments::{
    fragment_generators::{gen_opt_string, gen_owned_string, gen_type_name_upper_camel},
    CLOSE_CURLY, COLON, COMMA, LINE_FEED, OPEN_CURLY,
};

use super::End;

static MY_NAME: &[u8] = "End".as_bytes();

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
enum EndFieldNames {
    Role,
    EntitySet,
    EndType,
    Multiplicity,
}

impl EndFieldNames {
    pub fn value(prop_name: EndFieldNames) -> Vec<u8> {
        let member = match prop_name {
            EndFieldNames::Role => "role",
            EndFieldNames::EntitySet => "entity_set",
            EndFieldNames::EndType => "end_type",
            EndFieldNames::Multiplicity => "multiplicity",
        };

        member.as_bytes().to_vec()
    }
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
fn line_from_end(prop_md: EndFieldNames, val: Vec<u8>) -> Vec<u8> {
    [&*EndFieldNames::value(prop_md), COLON, &val, COMMA, LINE_FEED].concat()
}

impl std::fmt::Display for End {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let entity_set = if let Some(es) = &self.entity_set {
            Some(gen_type_name_upper_camel(es)) // Convert to struct name
        } else {
            None
        };

        let end_type = if let Some(et) = &self.end_type {
            let et_parts = et.split(".").collect::<Vec<&str>>();

            Some(if et_parts.len() == 2 {
                format!("{}", gen_type_name_upper_camel(et_parts[1]))
            } else {
                // This branch should never be used because SAP should always generate a fully qualified name...
                et.to_owned()
            })
        } else {
            None
        };

        let out_buffer: Vec<u8> = [
            MY_NAME,
            OPEN_CURLY,
            &*line_from_end(EndFieldNames::Role, gen_owned_string(&self.role)),
            &*line_from_end(EndFieldNames::EntitySet, gen_opt_string(&entity_set)),
            &*line_from_end(EndFieldNames::EndType, gen_opt_string(&end_type)),
            &*line_from_end(EndFieldNames::Multiplicity, gen_opt_string(&self.multiplicity)),
            CLOSE_CURLY,
        ]
        .concat();

        write!(f, "{}", String::from_utf8(out_buffer).unwrap())
    }
}
