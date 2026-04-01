use std::fmt::Formatter;

use crate::parser::generate::{
    gen_bool_string, gen_owned_string_src,
    syntax_fragments::{CLOSE_CURLY, COLON, COMMA, LINE_FEED, OPEN_CURLY},
};

use super::SAPAnnotationsAssociationSet;

static MY_NAME: &str = "SAPAnnotationsAssociationSet";

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
pub enum SAPAnnotationsAssociationSetFieldNames {
    ContentVersion,
    IsCreatable,
    IsUpdatable,
    IsDeletable,
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
impl SAPAnnotationsAssociationSetFieldNames {
    pub fn value(prop_name: SAPAnnotationsAssociationSetFieldNames) -> &'static str {
        match prop_name {
            SAPAnnotationsAssociationSetFieldNames::ContentVersion => "content_version",
            SAPAnnotationsAssociationSetFieldNames::IsCreatable => "is_creatable",
            SAPAnnotationsAssociationSetFieldNames::IsUpdatable => "is_updatable",
            SAPAnnotationsAssociationSetFieldNames::IsDeletable => "is_deletable",
        }
    }
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
fn line_into(f: &mut Formatter<'_>, prop_md: SAPAnnotationsAssociationSetFieldNames, val: &str) -> std::fmt::Result {
    write!(
        f,
        "{}{}{}{}{}",
        SAPAnnotationsAssociationSetFieldNames::value(prop_md),
        COLON,
        val,
        COMMA,
        LINE_FEED
    )
}

impl std::fmt::Display for SAPAnnotationsAssociationSet {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{MY_NAME}")?;
        write!(f, "{OPEN_CURLY}")?;
        line_into(
            f,
            SAPAnnotationsAssociationSetFieldNames::ContentVersion,
            &gen_owned_string_src(&self.content_version),
        )?;
        line_into(
            f,
            SAPAnnotationsAssociationSetFieldNames::IsCreatable,
            gen_bool_string(self.is_creatable),
        )?;
        line_into(
            f,
            SAPAnnotationsAssociationSetFieldNames::IsUpdatable,
            gen_bool_string(self.is_updatable),
        )?;
        line_into(
            f,
            SAPAnnotationsAssociationSetFieldNames::IsDeletable,
            gen_bool_string(self.is_deletable),
        )?;
        write!(f, "{CLOSE_CURLY}")
    }
}
