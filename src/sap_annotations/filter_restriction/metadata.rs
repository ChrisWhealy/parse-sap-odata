use crate::parser::syntax_fragments::{CLOSE_PAREN, COLON2, DOUBLE_QUOTE, NONE, OPEN_PAREN, SOME};

use super::SAPFilterRestrictionProperty;

static MY_NAME: &[u8] = "SAPFilterRestrictionProperty".as_bytes();

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
impl SAPFilterRestrictionProperty {
    pub fn opt_anno_type(opt_self: &Option<SAPFilterRestrictionProperty>) -> Vec<u8> {
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
            SAPFilterRestrictionProperty::SingleValue => "SingleValue",
            SAPFilterRestrictionProperty::MultiValue => "MultiValue",
            SAPFilterRestrictionProperty::Interval => "Interval",
        };

        member.as_bytes().to_vec()
    }
}
