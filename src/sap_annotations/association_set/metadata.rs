use std::fmt::Formatter;

use crate::parser::syntax_fragments::{
    fragment_generators::{gen_bool_string, gen_owned_string},
    CLOSE_CURLY, COLON, COMMA, LINE_FEED, OPEN_CURLY,
};

use super::SAPAnnotationsAssociationSet;

static MY_NAME: &[u8] = "SAPAnnotationsAssociationSet".as_bytes();

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
pub enum SAPAnnotationsAssociationSetMetadata {
    ContentVersion,
    IsCreatable,
    IsUpdatable,
    IsDeletable,
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
impl SAPAnnotationsAssociationSetMetadata {
    pub fn get_field_name(prop_name: SAPAnnotationsAssociationSetMetadata) -> Vec<u8> {
        let member = match prop_name {
            SAPAnnotationsAssociationSetMetadata::ContentVersion => "content_version",
            SAPAnnotationsAssociationSetMetadata::IsCreatable => "is_creatable",
            SAPAnnotationsAssociationSetMetadata::IsUpdatable => "is_updatable",
            SAPAnnotationsAssociationSetMetadata::IsDeletable => "is_deletable",
        };

        member.as_bytes().to_vec()
    }
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
fn line_from(prop_md: SAPAnnotationsAssociationSetMetadata, val: Vec<u8>) -> Vec<u8> {
    [
        SAPAnnotationsAssociationSetMetadata::get_field_name(prop_md),
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
                SAPAnnotationsAssociationSetMetadata::ContentVersion,
                gen_owned_string(&self.content_version),
            ),
            &*line_from(
                SAPAnnotationsAssociationSetMetadata::IsCreatable,
                gen_bool_string(self.is_creatable),
            ),
            &*line_from(
                SAPAnnotationsAssociationSetMetadata::IsUpdatable,
                gen_bool_string(self.is_updatable),
            ),
            &*line_from(
                SAPAnnotationsAssociationSetMetadata::IsDeletable,
                gen_bool_string(self.is_deletable),
            ),
            CLOSE_CURLY,
        ]
        .concat();

        write!(f, "{}", String::from_utf8(out_buffer).unwrap())
    }
}
