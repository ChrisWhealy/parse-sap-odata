use super::SAPAggregationRoleProperty;

use crate::{
    parser::generate::{
        gen_some_value,
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
        if let Some(anno_type) = self {
            gen_some_value(&generate_fq_name(MY_NAME, anno_type.member_name()))
        } else {
            NONE.to_string()
        }
    }
}
