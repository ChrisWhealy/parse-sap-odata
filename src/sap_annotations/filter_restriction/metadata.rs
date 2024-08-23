use super::SAPFilterRestrictionProperty;

use crate::{
    parser::generate::{
        gen_some_value,
        syntax_fragments::{COLON2, DOUBLE_QUOTE, NONE},
    },
    sap_annotations::{AnnotationType, OptionalAnnotationType},
};

static MY_NAME: &[u8] = "SAPFilterRestrictionProperty".as_bytes();

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
impl AnnotationType for SAPFilterRestrictionProperty {
    fn member_name(&self) -> Vec<u8> {
        let member = match self {
            SAPFilterRestrictionProperty::SingleValue => "SingleValue",
            SAPFilterRestrictionProperty::MultiValue => "MultiValue",
            SAPFilterRestrictionProperty::Interval => "Interval",
        };

        member.as_bytes().to_vec()
    }
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
impl OptionalAnnotationType for Option<SAPFilterRestrictionProperty> {
    fn opt_anno_type<T: AnnotationType>(&self, opt_self: &Option<T>) -> Vec<u8> {
        if let Some(anno_type) = opt_self {
            let fq_name = [DOUBLE_QUOTE, MY_NAME, COLON2, &*anno_type.member_name(), DOUBLE_QUOTE].concat();
            gen_some_value(fq_name)
        } else {
            NONE.to_vec()
        }
    }
}
