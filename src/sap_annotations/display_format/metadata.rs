use super::SAPDisplayFormatProperty;

use crate::{
    parser::generate::{
        gen_some_value,
        syntax_fragments::NONE,
    },
    sap_annotations::{generate_fq_name, AnnotationType, OptionalAnnotationType},
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
            gen_some_value(&*generate_fq_name(MY_NAME, anno_type.member_name()))
        } else {
            NONE.to_vec()
        }
    }
}
