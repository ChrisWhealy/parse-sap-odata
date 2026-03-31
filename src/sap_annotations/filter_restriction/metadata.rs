use super::SAPFilterRestrictionProperty;

use crate::{
    parser::generate::{
        gen_some_value,
        syntax_fragments::NONE,
    },
    sap_annotations::{generate_fq_name, AnnotationType, OptionalAnnotationType},
};

static MY_NAME: &str = "SAPFilterRestrictionProperty";

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
impl AnnotationType for SAPFilterRestrictionProperty {
    fn member_name(&self) -> &'static str {
        match self {
            SAPFilterRestrictionProperty::SingleValue => "SingleValue",
            SAPFilterRestrictionProperty::MultiValue => "MultiValue",
            SAPFilterRestrictionProperty::Interval => "Interval",
        }
    }
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
impl OptionalAnnotationType for Option<SAPFilterRestrictionProperty> {
    fn opt_anno_type(&self) -> String {
        if let Some(anno_type) = self {
            gen_some_value(&generate_fq_name(MY_NAME, anno_type.member_name()))
        } else {
            NONE.to_string()
        }
    }
}
