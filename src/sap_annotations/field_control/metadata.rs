use crate::parser::syntax_fragments::{CLOSE_PAREN, COLON2, DOUBLE_QUOTE, NONE, OPEN_PAREN, SOME};

use super::SAPFieldControlProperty;

static MY_NAME: &[u8] = "SAPFieldControlProperty".as_bytes();

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
impl SAPFieldControlProperty {
    pub fn opt_fc_prop(opt_self: &Option<SAPFieldControlProperty>) -> Vec<u8> {
        if let Some(fc_prop) = opt_self {
            [
                SOME,
                OPEN_PAREN,
                DOUBLE_QUOTE,
                MY_NAME,
                COLON2,
                &*fc_prop.as_enum_member(),
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
            SAPFieldControlProperty::Hidden => "Hidden",
            SAPFieldControlProperty::ReadOnly => "ReadOnly",
            SAPFieldControlProperty::Optional => "Optional",
            SAPFieldControlProperty::Mandatory => "Mandatory",
        };

        member.as_bytes().to_vec()
    }
}
