use super::SAPDisplayFormatProperty;

use crate::{
    parser::generate::{
        gen_some_value,
        syntax_fragments::NONE,
    },
    sap_annotations::{generate_fq_name, AnnotationType, OptionalAnnotationType},
};

static MY_NAME: &str = "SAPDisplayFormatProperty";

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
impl AnnotationType for SAPDisplayFormatProperty {
    fn member_name(&self) -> &'static str {
        match self {
            SAPDisplayFormatProperty::Date => "Date",
            SAPDisplayFormatProperty::NonNegative => "NonNegative",
            SAPDisplayFormatProperty::UpperCase => "UpperCase",
        }
    }
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
impl OptionalAnnotationType for Option<SAPDisplayFormatProperty> {
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
