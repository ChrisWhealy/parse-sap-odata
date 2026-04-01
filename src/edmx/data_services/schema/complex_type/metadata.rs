use super::ComplexType;

use crate::parser::generate::{
    gen_owned_string_src,
    syntax_fragments::{CLOSE_SQR, COLON, COMMA, COMPLEX_TYPE, END_BLOCK, LINE_FEED, OPEN_CURLY, VEC_BANG},
};
use std::fmt::Formatter;

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
pub enum ComplexTypeFieldNames {
    Name,
    Properties,
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
impl ComplexTypeFieldNames {
    pub fn value(prop_name: ComplexTypeFieldNames) -> &'static str {
        match prop_name {
            ComplexTypeFieldNames::Name => "name",
            ComplexTypeFieldNames::Properties => "properties",
        }
    }
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
fn line_from(f: &mut Formatter<'_>, prop_md: ComplexTypeFieldNames, val: &str) -> std::fmt::Result {
    write!(f, "{}{COLON}{val}{COMMA}{LINE_FEED}", ComplexTypeFieldNames::value(prop_md))
}

// Output a ComplexType instance as its own source code
impl std::fmt::Display for ComplexType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        // Start ComplexType declaration
        write!(f, "{COMPLEX_TYPE}{OPEN_CURLY}")?;
        line_from(f, ComplexTypeFieldNames::Name, &gen_owned_string_src(&self.name))?;
        write!(f, "{}", ComplexTypeFieldNames::value(ComplexTypeFieldNames::Properties))?;
        write!(f, "{COLON}{VEC_BANG}{LINE_FEED}")?;

        for prop in &self.properties {
            write!(f, "{prop}{COMMA}{LINE_FEED}")?;
        }

        // End vector of properties and ComplexType declaration
        write!(f, "{CLOSE_SQR}{END_BLOCK}")
    }
}
