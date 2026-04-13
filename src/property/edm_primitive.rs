use serde::{Deserialize, Serialize};
use crate::parser::generate::syntax_fragments::{CRATE_CHRONO, CRATE_GUID, CRATE_RUST_DECIMAL};

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
/// EDM primitive datatypes
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd, Serialize, Deserialize)]
pub enum EdmPrimitive {
    Binary,
    Boolean,
    Byte,
    DateTime,
    DateTimeOffset,
    Decimal,
    Double,
    Guid,
    Int16,
    Int32,
    Int64,
    Null,
    SByte,
    Single,
    String,
    Time,
    Unknown(String),
}

impl EdmPrimitive {
    /// Return the crate reference for those EdmPrimitive types that need custom deserializers
    pub fn get_crate_ref(&self) -> &'static str {
        match &self {
            EdmPrimitive::DateTime => CRATE_CHRONO,
            EdmPrimitive::DateTimeOffset => CRATE_CHRONO,
            EdmPrimitive::Decimal => CRATE_RUST_DECIMAL,
            EdmPrimitive::Guid => CRATE_GUID,
            _ => "",
        }
    }
}

impl From<&str> for EdmPrimitive {
    fn from(s: &str) -> Self {
        match s {
            "Binary" => Self::Binary,
            "Boolean" => Self::Boolean,
            "Byte" => Self::Byte,
            "DateTime" => Self::DateTime,
            "DateTimeOffset" => Self::DateTimeOffset,
            "Decimal" => Self::Decimal,
            "Double" => Self::Double,
            "Guid" => Self::Guid,
            "Int16" => Self::Int16,
            "Int32" => Self::Int32,
            "Int64" => Self::Int64,
            "Null" => Self::Null,
            "SByte" => Self::SByte,
            "Single" => Self::Single,
            "String" => Self::String,
            "Time" => Self::Time,
            _ => Self::Unknown(s.to_owned()),
        }
    }
}

impl std::fmt::Display for EdmPrimitive {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            Self::Binary => write!(f, "EDM.Binary"),
            Self::Boolean => write!(f, "EDM.Boolean"),
            Self::Byte => write!(f, "EDM.Byte"),
            Self::DateTime => write!(f, "EDM.DateTime"),
            Self::DateTimeOffset => write!(f, "EDM.DateTimeOffset"),
            Self::Decimal => write!(f, "EDM.Decimal"),
            Self::Double => write!(f, "EDM.Double"),
            Self::Guid => write!(f, "EDM.Guid"),
            Self::Int16 => write!(f, "EDM.Int16"),
            Self::Int32 => write!(f, "EDM.Int32"),
            Self::Int64 => write!(f, "EDM.Int64"),
            Self::Null => write!(f, "EDM.Null"),
            Self::SByte => write!(f, "EDM.SByte"),
            Self::Single => write!(f, "EDM.Single"),
            Self::String => write!(f, "EDM.String"),
            Self::Time => write!(f, "EDM.Time"),
            Self::Unknown(s) => write!(f, "{s}"),
        }
    }
}
