use super::ComplexType;
use crate::parser::syntax_fragments::{
    fragment_generators::gen_owned_string, CLOSE_SQR, COLON, COMMA, COMPLEX_TYPE, END_BLOCK, LINE_FEED, OPEN_CURLY,
    VEC_BANG,
};
use std::fmt::Formatter;

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
pub enum ComplexTypeFieldNames {
    Name,
    Properties,
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
impl ComplexTypeFieldNames {
    pub fn value(prop_name: ComplexTypeFieldNames) -> Vec<u8> {
        let member = match prop_name {
            ComplexTypeFieldNames::Name => "name",
            ComplexTypeFieldNames::Properties => "properties",
        };

        member.as_bytes().to_vec()
    }
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
fn line_from(prop_md: ComplexTypeFieldNames, val: Vec<u8>) -> Vec<u8> {
    [&*ComplexTypeFieldNames::value(prop_md), COLON, &val, COMMA, LINE_FEED].concat()
}

// Output a ComplexType instance as its own source code
impl std::fmt::Display for ComplexType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut out_buffer: Vec<u8> = [
            // Start ComplexType declaration
            COMPLEX_TYPE,
            OPEN_CURLY,
            &*line_from(ComplexTypeFieldNames::Name, gen_owned_string(&self.name)),
            // Start vector of properties
            &*ComplexTypeFieldNames::value(ComplexTypeFieldNames::Properties),
            COLON,
            VEC_BANG,
            LINE_FEED,
        ]
        .concat();

        for prop in &self.properties {
            out_buffer.append(&mut [format!("{prop}").as_bytes(), COMMA, LINE_FEED].concat());
        }

        // End vector of properties and ComplexType declaration
        out_buffer.append(&mut [CLOSE_SQR, END_BLOCK].concat());

        write!(f, "{}", String::from_utf8(out_buffer).unwrap())
    }
}
