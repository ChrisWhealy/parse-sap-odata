use super::SAPFilterRestrictionProperty;

use crate::{
    parser::generate::{
        gen_some_value,
        syntax_fragments::NONE,
    },
    sap_annotations::{generate_fq_name, AnnotationType, OptionalAnnotationType},
};

static MY_NAME: &[u8] = "SAPFilterRestrictionProperty".as_bytes();

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
impl AnnotationType for SAPFilterRestrictionProperty {
    fn member_name(&self) -> &'static [u8] {
        match self {
            SAPFilterRestrictionProperty::SingleValue => b"SingleValue",
            SAPFilterRestrictionProperty::MultiValue => b"MultiValue",
            SAPFilterRestrictionProperty::Interval => b"Interval",
        }
    }
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
impl OptionalAnnotationType for Option<SAPFilterRestrictionProperty> {
    fn opt_anno_type(&self) -> Vec<u8> {
        if let Some(anno_type) = self {
            gen_some_value(&*generate_fq_name(MY_NAME, anno_type.member_name()))
        } else {
            NONE.to_vec()
        }
    }
}
