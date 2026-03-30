use std::fmt::Formatter;

use crate::parser::generate::{
    gen_bool_string, gen_owned_string,
    syntax_fragments::{CLOSE_CURLY, COLON, COMMA, LINE_FEED, OPEN_CURLY},
};

use super::SAPAnnotationsAssociationSet;

static MY_NAME: &[u8] = "SAPAnnotationsAssociationSet".as_bytes();

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
pub enum SAPAnnotationsAssociationSetFieldNames {
    ContentVersion,
    IsCreatable,
    IsUpdatable,
    IsDeletable,
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
impl SAPAnnotationsAssociationSetFieldNames {
    pub fn value(prop_name: SAPAnnotationsAssociationSetFieldNames) -> &'static [u8] {
        match prop_name {
            SAPAnnotationsAssociationSetFieldNames::ContentVersion => b"content_version",
            SAPAnnotationsAssociationSetFieldNames::IsCreatable => b"is_creatable",
            SAPAnnotationsAssociationSetFieldNames::IsUpdatable => b"is_updatable",
            SAPAnnotationsAssociationSetFieldNames::IsDeletable => b"is_deletable",
        }
    }
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
fn line_into(f: &mut Formatter<'_>, prop_md: SAPAnnotationsAssociationSetFieldNames, val: &[u8]) -> std::fmt::Result {
    for s in [SAPAnnotationsAssociationSetFieldNames::value(prop_md), COLON, val, COMMA, LINE_FEED] {
        write!(f, "{}", std::str::from_utf8(s).unwrap())?;
    }
    Ok(())
}

impl std::fmt::Display for SAPAnnotationsAssociationSet {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", std::str::from_utf8(MY_NAME).unwrap())?;
        write!(f, "{}", std::str::from_utf8(OPEN_CURLY).unwrap())?;
        line_into(f, SAPAnnotationsAssociationSetFieldNames::ContentVersion, &gen_owned_string(&self.content_version))?;
        line_into(f, SAPAnnotationsAssociationSetFieldNames::IsCreatable, &gen_bool_string(self.is_creatable))?;
        line_into(f, SAPAnnotationsAssociationSetFieldNames::IsUpdatable, &gen_bool_string(self.is_updatable))?;
        line_into(f, SAPAnnotationsAssociationSetFieldNames::IsDeletable, &gen_bool_string(self.is_deletable))?;
        write!(f, "{}", std::str::from_utf8(CLOSE_CURLY).unwrap())
    }
}
