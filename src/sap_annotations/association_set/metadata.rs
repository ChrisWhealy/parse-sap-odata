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
    pub fn value(prop_name: SAPAnnotationsAssociationSetFieldNames) -> Vec<u8> {
        let member = match prop_name {
            SAPAnnotationsAssociationSetFieldNames::ContentVersion => "content_version",
            SAPAnnotationsAssociationSetFieldNames::IsCreatable => "is_creatable",
            SAPAnnotationsAssociationSetFieldNames::IsUpdatable => "is_updatable",
            SAPAnnotationsAssociationSetFieldNames::IsDeletable => "is_deletable",
        };

        member.as_bytes().to_vec()
    }
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
fn line_from(prop_md: SAPAnnotationsAssociationSetFieldNames, val: Vec<u8>) -> Vec<u8> {
    [
        SAPAnnotationsAssociationSetFieldNames::value(prop_md),
        COLON.to_vec(),
        val,
        COMMA.to_vec(),
        LINE_FEED.to_vec(),
    ]
    .concat()
}

impl std::fmt::Display for SAPAnnotationsAssociationSet {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let out_buffer: Vec<u8> = [
            MY_NAME,
            OPEN_CURLY,
            &*line_from(
                SAPAnnotationsAssociationSetFieldNames::ContentVersion,
                gen_owned_string(&self.content_version),
            ),
            &*line_from(
                SAPAnnotationsAssociationSetFieldNames::IsCreatable,
                gen_bool_string(self.is_creatable),
            ),
            &*line_from(
                SAPAnnotationsAssociationSetFieldNames::IsUpdatable,
                gen_bool_string(self.is_updatable),
            ),
            &*line_from(
                SAPAnnotationsAssociationSetFieldNames::IsDeletable,
                gen_bool_string(self.is_deletable),
            ),
            CLOSE_CURLY,
        ]
        .concat();

        write!(f, "{}", String::from_utf8(out_buffer).unwrap())
    }
}
