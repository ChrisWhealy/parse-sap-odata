use super::ComplexType;
use crate::parser::generate::gen_owned_string;
use crate::parser::generate::syntax_fragments::{
    CLOSE_SQR, COLON, COMMA, COMPLEX_TYPE, END_BLOCK, LINE_FEED, OPEN_CURLY, VEC_BANG,
};
use std::fmt::Formatter;

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
fn line_from(f: &mut Formatter<'_>, prop_md: ComplexTypeFieldNames, val: &[u8]) -> std::fmt::Result {
    for s in [ComplexTypeFieldNames::value(prop_md), COLON, val, COMMA, LINE_FEED] {
        write!(f, "{}", std::str::from_utf8(s).unwrap())?;
    }
    Ok(())
}

// Output a ComplexType instance as its own source code
impl std::fmt::Display for ComplexType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        // Start ComplexType declaration
        write!(f, "{}", std::str::from_utf8(COMPLEX_TYPE).unwrap())?;
        write!(f, "{}", std::str::from_utf8(OPEN_CURLY).unwrap())?;
        line_from(f, ComplexTypeFieldNames::Name, &gen_owned_string(&self.name))?;
        write!(f, "{}", std::str::from_utf8(ComplexTypeFieldNames::value(ComplexTypeFieldNames::Properties)).unwrap())?;
        write!(f, "{}", std::str::from_utf8(COLON).unwrap())?;
        write!(f, "{}", std::str::from_utf8(VEC_BANG).unwrap())?;
        write!(f, "{}", std::str::from_utf8(LINE_FEED).unwrap())?;

        for prop in &self.properties {
            write!(f, "{prop}")?;
            write!(f, "{}", std::str::from_utf8(COMMA).unwrap())?;
            write!(f, "{}", std::str::from_utf8(LINE_FEED).unwrap())?;
        }

        // End vector of properties and ComplexType declaration
        write!(f, "{}", std::str::from_utf8(CLOSE_SQR).unwrap())?;
        write!(f, "{}", std::str::from_utf8(END_BLOCK).unwrap())
    }
}
