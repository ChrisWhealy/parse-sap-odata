use super::SAPParameterProperty;

use crate::{
    parser::generate::{gen_some_value, syntax_fragments::NONE},
    sap_annotations::{generate_fq_name, AnnotationType, OptionalAnnotationType},
};

static MY_NAME: &str = "SAPParameterProperty";

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
impl AnnotationType for SAPParameterProperty {
    fn member_name(&self) -> &'static str {
        match self {
            SAPParameterProperty::Mandatory => "Mandatory",
            SAPParameterProperty::Optional => "Optional",
        }
    }
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
impl OptionalAnnotationType for Option<SAPParameterProperty> {
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
