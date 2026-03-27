use super::SAPDisplayFormatProperty;

use crate::{
    parser::generate::{
        gen_some_value,
        syntax_fragments::{COLON2, DOUBLE_QUOTE, NONE},
    },
    sap_annotations::{AnnotationType, OptionalAnnotationType},
};

static MY_NAME: &[u8] = "SAPDisplayFormatProperty".as_bytes();

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
impl AnnotationType for SAPDisplayFormatProperty {
    fn member_name(&self) -> &'static [u8] {
        match self {
            SAPDisplayFormatProperty::Date => b"Date",
            SAPDisplayFormatProperty::NonNegative => b"NonNegative",
            SAPDisplayFormatProperty::UpperCase => b"UpperCase",
        }
    }
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
impl OptionalAnnotationType for Option<SAPDisplayFormatProperty> {
    fn opt_anno_type<T: AnnotationType>(&self, opt_self: &Option<T>) -> Vec<u8> {
        if let Some(anno_type) = opt_self {
            let fq_name = [DOUBLE_QUOTE, MY_NAME, COLON2, anno_type.member_name(), DOUBLE_QUOTE].concat();
            gen_some_value(&fq_name)
        } else {
            NONE.to_vec()
        }
    }
}
