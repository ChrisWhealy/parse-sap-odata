use crate::parser::syntax_fragments::{CLOSE_PAREN, COLON2, DOUBLE_QUOTE, NONE, OPEN_PAREN, SOME};

use super::SAPAggregationRoleProperty;

static MY_NAME: &[u8] = "SAPAggregationRoleProperty".as_bytes();

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
impl SAPAggregationRoleProperty {
    pub fn opt_anno_type(opt_self: &Option<SAPAggregationRoleProperty>) -> Vec<u8> {
        if let Some(anno_type) = opt_self {
            [
                SOME,
                OPEN_PAREN,
                DOUBLE_QUOTE,
                MY_NAME,
                COLON2,
                &*anno_type.as_enum_member(),
                DOUBLE_QUOTE,
                CLOSE_PAREN,
            ]
            .concat()
        } else {
            NONE.to_vec()
        }
    }

    pub fn as_enum_member(&self) -> Vec<u8> {
        let member = match self {
            SAPAggregationRoleProperty::Dimension => "Dimension",
            SAPAggregationRoleProperty::Measure => "Measure",
            SAPAggregationRoleProperty::TotalPropertiesList => "TotalPropertiesList",
        };

        member.as_bytes().to_vec()
    }
}
