use super::SAPAggregationRoleProperty;

use crate::{
    parser::generate::{
        gen_some_value_into,
        syntax_fragments::NONE,
    },
    sap_annotations::{generate_fq_name, AnnotationType, OptionalAnnotationType},
};

static MY_NAME: &str = "SAPAggregationRoleProperty";

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
impl AnnotationType for SAPAggregationRoleProperty {
    fn member_name(&self) -> &'static str {
        match self {
            SAPAggregationRoleProperty::Dimension => "Dimension",
            SAPAggregationRoleProperty::Measure => "Measure",
            SAPAggregationRoleProperty::TotalPropertiesList => "TotalPropertiesList",
        }
    }
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
impl OptionalAnnotationType for Option<SAPAggregationRoleProperty> {
    fn opt_anno_type(&self) -> String {
        let mut out = String::new();
        
        if let Some(anno_type) = self {
            gen_some_value_into(&mut out, &generate_fq_name(MY_NAME, anno_type.member_name()))
        } else {
            out.push_str(NONE)
        }
        
        out
    }
}
