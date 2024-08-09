use std::fmt::Formatter;

use super::AssociationSet;
use crate::parser::syntax_fragments::{
    fragment_generators::gen_owned_string, CLOSE_CURLY, CLOSE_SQR, COLON, COMMA, LINE_FEED, OPEN_CURLY, OPEN_SQR,
};

static MY_NAME: &[u8] = "AssociationSet".as_bytes();

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
enum AssociationSetFieldNames {
    Name,
    Association,
    SapAnnotations,
    Ends,
}

impl AssociationSetFieldNames {
    pub fn value(prop_name: AssociationSetFieldNames) -> Vec<u8> {
        let member = match prop_name {
            AssociationSetFieldNames::Name => "name",
            AssociationSetFieldNames::Association => "association",
            AssociationSetFieldNames::SapAnnotations => "sap_annotations",
            AssociationSetFieldNames::Ends => "ends",
        };

        member.as_bytes().to_vec()
    }
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
fn line_from_end(prop_md: AssociationSetFieldNames, val: Vec<u8>) -> Vec<u8> {
    [
        &*AssociationSetFieldNames::value(prop_md),
        COLON,
        &val,
        COMMA,
        LINE_FEED,
    ]
    .concat()
}

impl std::fmt::Display for AssociationSet {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let ends = [
            OPEN_SQR,
            format!("{}", self.ends[0]).as_bytes(),
            COMMA,
            format!("{}", self.ends[1]).as_bytes(),
            CLOSE_SQR,
        ]
        .concat();

        let out_buffer: Vec<u8> = [
            MY_NAME,
            OPEN_CURLY,
            &*line_from_end(AssociationSetFieldNames::Name, gen_owned_string(&self.name)),
            &*line_from_end(AssociationSetFieldNames::Association, gen_owned_string(&self.association)),
            &*line_from_end(AssociationSetFieldNames::Ends, ends),
            &*line_from_end(
                AssociationSetFieldNames::SapAnnotations,
                format!("{}", &self.sap_annotations).as_bytes().to_vec(),
            ),
            CLOSE_CURLY,
        ]
        .concat();

        write!(f, "{}", String::from_utf8(out_buffer).unwrap())
    }
}
