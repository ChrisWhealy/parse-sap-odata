use super::ComplexType;
use crate::parser::generate::gen_owned_string;
use crate::parser::generate::syntax_fragments::{
    CLOSE_SQR, COLON, COMMA, COMPLEX_TYPE, END_BLOCK, LINE_FEED, OPEN_CURLY, VEC_BANG,
};
use std::{fmt::Formatter, io::Write};

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
pub enum ComplexTypeFieldNames {
    Name,
    Properties,
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
impl ComplexTypeFieldNames {
    pub fn value(prop_name: ComplexTypeFieldNames) -> &'static [u8] {
        match prop_name {
            ComplexTypeFieldNames::Name => b"name",
            ComplexTypeFieldNames::Properties => b"properties",
        }
    }
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
fn line_from(prop_md: ComplexTypeFieldNames, val: Vec<u8>) -> Vec<u8> {
    let mut line: Vec<u8> = Vec::new();

    line.extend_from_slice(ComplexTypeFieldNames::value(prop_md));
    line.extend_from_slice(COLON);
    line.extend_from_slice(&*val);
    line.extend_from_slice(COMMA);
    line.extend_from_slice(LINE_FEED);
    line
}

// Output a ComplexType instance as its own source code
impl std::fmt::Display for ComplexType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        // Start ComplexType declaration
        let mut out_buffer: Vec<u8> = Vec::new();
        out_buffer.extend_from_slice(COMPLEX_TYPE);
        out_buffer.extend_from_slice(OPEN_CURLY);
        out_buffer.extend_from_slice(&line_from(ComplexTypeFieldNames::Name, gen_owned_string(&self.name)));
        out_buffer.extend_from_slice(ComplexTypeFieldNames::value(ComplexTypeFieldNames::Properties));
        out_buffer.extend_from_slice(COLON);
        out_buffer.extend_from_slice(VEC_BANG);
        out_buffer.extend_from_slice(LINE_FEED);

        for prop in &self.properties {
            write!(out_buffer, "{prop}").unwrap();
            out_buffer.extend_from_slice(COMMA);
            out_buffer.extend_from_slice(LINE_FEED);
        }

        // End vector of properties and ComplexType declaration
        out_buffer.extend_from_slice(CLOSE_SQR);
        out_buffer.extend_from_slice(END_BLOCK);

        write!(f, "{}", String::from_utf8(out_buffer).unwrap())
    }
}
