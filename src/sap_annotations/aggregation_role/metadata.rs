use super::SAPAggregationRoleProperty;

use crate::{
    parser::generate::{
        gen_some_value,
        syntax_fragments::NONE,
    },
    sap_annotations::{generate_fq_name, AnnotationType, OptionalAnnotationType},
};

static MY_NAME: &[u8] = "SAPAggregationRoleProperty".as_bytes();

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
impl AnnotationType for SAPAggregationRoleProperty {
    fn member_name(&self) -> &'static [u8] {
        match self {
            SAPAggregationRoleProperty::Dimension => b"Dimension",
            SAPAggregationRoleProperty::Measure => b"Measure",
            SAPAggregationRoleProperty::TotalPropertiesList => b"TotalPropertiesList",
        }
    }
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
impl OptionalAnnotationType for Option<SAPAggregationRoleProperty> {
    fn opt_anno_type<T: AnnotationType>(&self, opt_self: &Option<T>) -> Vec<u8> {
        if let Some(anno_type) = opt_self {
            gen_some_value(&*generate_fq_name(MY_NAME, anno_type.member_name()))
        } else {
            NONE.to_vec()
        }
    }
}
