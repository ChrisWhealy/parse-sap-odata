use super::SAPParameterProperty;

use crate::{
    parser::generate::{
        gen_some_value,
        syntax_fragments::NONE,
    },
    sap_annotations::{generate_fq_name, AnnotationType, OptionalAnnotationType},
};

static MY_NAME: &[u8] = "SAPParameterProperty".as_bytes();

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
impl AnnotationType for SAPParameterProperty {
    fn member_name(&self) -> &'static [u8] {
        match self {
            SAPParameterProperty::Mandatory => b"Mandatory",
            SAPParameterProperty::Optional => b"Optional",
        }
    }
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
impl OptionalAnnotationType for Option<SAPParameterProperty> {
    fn opt_anno_type<T: AnnotationType>(&self, opt_self: &Option<T>) -> Vec<u8> {
        if let Some(anno_type) = opt_self {
            gen_some_value(&*generate_fq_name(MY_NAME, anno_type.member_name()))
        } else {
            NONE.to_vec()
        }
    }
}
