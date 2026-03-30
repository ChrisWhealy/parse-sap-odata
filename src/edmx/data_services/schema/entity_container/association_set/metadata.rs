use super::AssociationSet;
use std::fmt::Formatter;

use crate::parser::generate::gen_owned_string;
use crate::parser::generate::syntax_fragments::{
    CLOSE_CURLY, CLOSE_SQR, COLON, COMMA, LINE_FEED, OPEN_CURLY, OPEN_SQR,
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
    pub fn value(prop_name: AssociationSetFieldNames) -> &'static [u8] {
        match prop_name {
            AssociationSetFieldNames::Name => b"name",
            AssociationSetFieldNames::Association => b"association",
            AssociationSetFieldNames::SapAnnotations => b"sap_annotations",
            AssociationSetFieldNames::Ends => b"ends",
        }
    }
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
fn line_from_end(f: &mut Formatter<'_>, prop_md: AssociationSetFieldNames, val: &[u8]) -> std::fmt::Result {
    for s in [AssociationSetFieldNames::value(prop_md), COLON, val, COMMA, LINE_FEED] {
        write!(f, "{}", std::str::from_utf8(s).unwrap())?;
    }
    Ok(())
}

impl std::fmt::Display for AssociationSet {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let ends_str = format!("{}{}{}{}{}",
            std::str::from_utf8(OPEN_SQR).unwrap(),
            self.ends[0],
            std::str::from_utf8(COMMA).unwrap(),
            self.ends[1],
            std::str::from_utf8(CLOSE_SQR).unwrap());
        let sap_annotations_str = self.sap_annotations.to_string();

        write!(f, "{}", std::str::from_utf8(MY_NAME).unwrap())?;
        write!(f, "{}", std::str::from_utf8(OPEN_CURLY).unwrap())?;
        line_from_end(f, AssociationSetFieldNames::Name, &gen_owned_string(&self.name))?;
        line_from_end(f, AssociationSetFieldNames::Association, &gen_owned_string(&self.association))?;
        line_from_end(f, AssociationSetFieldNames::Ends, ends_str.as_bytes())?;
        line_from_end(f, AssociationSetFieldNames::SapAnnotations, sap_annotations_str.as_bytes())?;
        write!(f, "{}", std::str::from_utf8(CLOSE_CURLY).unwrap())
    }
}
