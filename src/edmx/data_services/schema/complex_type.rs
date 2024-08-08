#[cfg(feature = "parser")]
use std::fmt::Formatter;

use serde::{Deserialize, Serialize};

#[cfg(feature = "parser")]
use crate::parser::syntax_fragments::{fragment_generators::gen_owned_string, *};
use crate::property::Property;

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
pub enum ComplexTypeMetadata {
    Name,
    Properties,
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
impl ComplexTypeMetadata {
    pub fn get_field_name(prop_name: ComplexTypeMetadata) -> Vec<u8> {
        let member = match prop_name {
            ComplexTypeMetadata::Name => "name",
            ComplexTypeMetadata::Properties => "properties",
        };

        member.as_bytes().to_vec()
    }
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
/// Represents a `<ComplexType>` tag
///
/// # Child Nodes
/// `1:n Property`
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ComplexType {
    #[serde(rename = "@Name")]
    pub name: String,
    #[serde(rename = "Property", default)]
    pub properties: Vec<Property>,
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
#[cfg(feature = "parser")]
fn line_from(prop_md: ComplexTypeMetadata, val: Vec<u8>) -> Vec<u8> {
    [&*ComplexTypeMetadata::get_field_name(prop_md), COLON, &val, COMMA, LINE_FEED].concat()
}

#[cfg(feature = "parser")]
// Output a ComplexType instance as its own source code
impl std::fmt::Display for ComplexType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut out_buffer: Vec<u8> = [
            // Start ComplexType declaration
            COMPLEX_TYPE,
            OPEN_CURLY,
            &*line_from(ComplexTypeMetadata::Name, gen_owned_string(&self.name)),
            // Start vector of properties
            &*ComplexTypeMetadata::get_field_name(ComplexTypeMetadata::Properties),
            COLON,
            VEC_BANG,
            LINE_FEED,
        ]
        .concat();

        for prop in &self.properties {
            out_buffer.append(&mut [format!("{prop}").as_bytes(), COMMA, LINE_FEED].concat());
        }

        // End vector of properties and ComplexType declaration
        out_buffer.append(&mut [CLOSE_SQR, END_BLOCK].concat());

        write!(f, "{}", String::from_utf8(out_buffer).unwrap())
    }
}
