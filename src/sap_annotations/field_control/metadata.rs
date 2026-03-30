use super::SAPFieldControlProperty;

use crate::{
    parser::generate::{
        gen_some_value,
        syntax_fragments::NONE,
    },
    sap_annotations::{generate_fq_name, AnnotationType, OptionalAnnotationType},
};

static MY_NAME: &[u8] = "SAPFieldControlProperty".as_bytes();

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
impl AnnotationType for SAPFieldControlProperty {
    fn member_name(&self) -> &'static [u8] {
        match self {
            SAPFieldControlProperty::Hidden => b"Hidden",
            SAPFieldControlProperty::ReadOnly => b"ReadOnly",
            SAPFieldControlProperty::Optional => b"Optional",
            SAPFieldControlProperty::Mandatory => b"Mandatory",
        }
    }
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
impl OptionalAnnotationType for Option<SAPFieldControlProperty> {
    fn opt_anno_type(&self) -> Vec<u8> {
        if let Some(anno_type) = self {
            gen_some_value(&*generate_fq_name(MY_NAME, anno_type.member_name()))
        } else {
            NONE.to_vec()
        }
    }
}
