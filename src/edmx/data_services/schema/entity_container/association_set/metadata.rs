use super::AssociationSet;
use std::fmt::Formatter;

use crate::parser::generate::{
    gen_owned_string_src,
    syntax_fragments::{CLOSE_CURLY, CLOSE_SQR, COLON, COMMA, LINE_FEED, OPEN_CURLY, OPEN_SQR},
};

static MY_NAME: &str = "AssociationSet";

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
enum AssociationSetFieldNames {
    Name,
    Association,
    SapAnnotations,
    Ends,
}

impl AssociationSetFieldNames {
    pub fn value(prop_name: AssociationSetFieldNames) -> &'static str {
        match prop_name {
            AssociationSetFieldNames::Name => "name",
            AssociationSetFieldNames::Association => "association",
            AssociationSetFieldNames::SapAnnotations => "sap_annotations",
            AssociationSetFieldNames::Ends => "ends",
        }
    }
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
fn line_from_end(f: &mut Formatter<'_>, prop_md: AssociationSetFieldNames, val: &str) -> std::fmt::Result {
    write!(f, "{}{COLON}{val}{COMMA}{LINE_FEED}", AssociationSetFieldNames::value(prop_md))
}

impl std::fmt::Display for AssociationSet {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{MY_NAME}{OPEN_CURLY}")?;
        line_from_end(f, AssociationSetFieldNames::Name, &gen_owned_string_src(&self.name))?;
        line_from_end(
            f,
            AssociationSetFieldNames::Association,
            &gen_owned_string_src(&self.association),
        )?;
        line_from_end(
            f,
            AssociationSetFieldNames::Ends,
            &format!("{OPEN_SQR}{}{COMMA}{}{CLOSE_SQR}", self.ends[0], self.ends[1]),
        )?;
        line_from_end(f, AssociationSetFieldNames::SapAnnotations, &self.sap_annotations.to_string())?;
        write!(f, "{CLOSE_CURLY}")
    }
}
