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
fn line_into(out: &mut Vec<u8>, prop_md: SAPAnnotationsAssociationSetFieldNames, val: Vec<u8>) {
    out.extend_from_slice(SAPAnnotationsAssociationSetFieldNames::value(prop_md));
    out.extend_from_slice(COLON);
    out.extend_from_slice(&val);
    out.extend_from_slice(COMMA);
    out.extend_from_slice(LINE_FEED);
}

impl std::fmt::Display for SAPAnnotationsAssociationSet {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut out_buffer: Vec<u8> = Vec::new();
        out_buffer.extend_from_slice(MY_NAME);
        out_buffer.extend_from_slice(OPEN_CURLY);
        line_into(&mut out_buffer, SAPAnnotationsAssociationSetFieldNames::ContentVersion, gen_owned_string(&self.content_version));
        line_into(&mut out_buffer, SAPAnnotationsAssociationSetFieldNames::IsCreatable, gen_bool_string(self.is_creatable));
        line_into(&mut out_buffer, SAPAnnotationsAssociationSetFieldNames::IsUpdatable, gen_bool_string(self.is_updatable));
        line_into(&mut out_buffer, SAPAnnotationsAssociationSetFieldNames::IsDeletable, gen_bool_string(self.is_deletable));
        out_buffer.extend_from_slice(CLOSE_CURLY);
        write!(f, "{}", String::from_utf8(out_buffer).unwrap())
    }
}
