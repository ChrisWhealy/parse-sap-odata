use super::SAPFieldControlProperty;

use crate::{
    parser::generate::{
        gen_some_value,
        syntax_fragments::NONE,
    },
    sap_annotations::{generate_fq_name, AnnotationType, OptionalAnnotationType},
};

static MY_NAME: &str = "SAPFieldControlProperty";

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
impl AnnotationType for SAPFieldControlProperty {
    fn member_name(&self) -> &'static str {
        match self {
            SAPFieldControlProperty::Hidden => "Hidden",
            SAPFieldControlProperty::ReadOnly => "ReadOnly",
            SAPFieldControlProperty::Optional => "Optional",
            SAPFieldControlProperty::Mandatory => "Mandatory",
        }
    }
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
impl OptionalAnnotationType for Option<SAPFieldControlProperty> {
    fn opt_anno_type(&self) -> String {
        let mut out = String::new();

        if let Some(anno_type) = self {
            gen_some_value(&mut out, &generate_fq_name(MY_NAME, anno_type.member_name()))
        } else {
            out.push_str(NONE)
        }
        
        out
    }
}
