use serde::{Deserialize, Deserializer, Serialize};
use crate::property::edm_primitive::EdmPrimitive;
use crate::utils::to_upper_camel_case;

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd, Serialize)]
pub enum EdmType {
    Primitive(EdmPrimitive), // "Edm.Xxx"
    Complex(String),         // "Namespace.TypeName"
    Unknown(String),         // anything unrecognised — retains raw value
}

impl std::fmt::Display for EdmType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            Self::Primitive(p) => write!(f, "{}", p),
            EdmType::Complex(ct) => write!(f, "{}", ct),
            EdmType::Unknown(s) => write!(f, "{}", s),
        }
    }
}

impl From<&str> for EdmType {
    fn from(s: &str) -> Self {
        match s.split_once('.') {
            None => {
                if s.starts_with("CT_") {
                    EdmType::Complex(to_upper_camel_case(s))
                } else {
                    EdmType::Unknown(s.to_owned())
                }
            },
            Some(("Edm", prim)) => EdmType::Primitive(EdmPrimitive::from(prim)),
            Some((_, type_name)) if type_name.starts_with("CT_") => {
                EdmType::Complex(to_upper_camel_case(type_name))
            },
            Some((_, _)) => EdmType::Unknown(s.to_owned()),
        }
    }
}

impl<'de> Deserialize<'de> for EdmType {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        let s = String::deserialize(d)?;

        Ok(match s.split_once('.') {
            None => {
                if s.starts_with("CT_") {
                    EdmType::Complex(to_upper_camel_case(&s))
                } else {
                    EdmType::Unknown(s)
                }
            },
            Some(("Edm", prim)) => EdmType::Primitive(EdmPrimitive::from(prim)),
            Some((_, type_name)) if type_name.starts_with("CT_") => {
                EdmType::Complex(to_upper_camel_case(type_name))
            },
            Some((_, _)) => EdmType::Unknown(s),
        })
    }
}
