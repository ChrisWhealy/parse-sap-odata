use crate::parser::syntax_fragments::{CLOSE_PAREN, COLON2, DOUBLE_QUOTE, NONE, OPEN_PAREN, SOME};

use super::SAPParameterProperty;

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
impl SAPParameterProperty {
    pub fn opt_anno_type(opt_self: &Option<SAPParameterProperty>) -> Vec<u8> {
        let own_name: &[u8] = "SAPParameterProperty".as_bytes();

        if let Some(anno_type) = opt_self {
            [
                SOME,
                OPEN_PAREN,
                DOUBLE_QUOTE,
                own_name,
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
            SAPParameterProperty::Mandatory => "Mandatory",
            SAPParameterProperty::Optional => "Optional",
        };

        member.as_bytes().to_vec()
    }
}
