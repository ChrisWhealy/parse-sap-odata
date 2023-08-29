// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
// Single characters
pub static LINE_FEED: &[u8] = &[0x0a];
pub static SPACE: &[u8] = &[0x20];
pub static DOUBLE_QUOTE: &[u8] = &[0x22];
pub static OPEN_PAREN: &[u8] = &[0x28];
pub static CLOSE_PAREN: &[u8] = &[0x29];
pub static COMMA: &[u8] = &[0x2C];
pub static COLON: &[u8] = &[0x3A];
pub static OPEN_ANGLE: &[u8] = &[0x3C];
pub static CLOSE_ANGLE: &[u8] = &[0x3E];
pub static OPEN_SQR: &[u8] = &[0x5B];
pub static CLOSE_SQR: &[u8] = &[0x5D];
pub static OPEN_CURLY: &[u8] = &[0x7B];
pub static CLOSE_CURLY: &[u8] = &[0x7D];

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
// Syntactical separators
pub static COLON2: &[u8] = &[0x3A, 0x3A];
pub static FAT_ARROW: &[u8] = &[0x3D, 0x3E];

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
// Type names
pub static BOOLEAN: &[u8] = "bool".as_bytes();
pub static F32: &[u8] = "f32".as_bytes();
pub static F64: &[u8] = "f64".as_bytes();
pub static I8: &[u8] = "i8".as_bytes();
pub static I16: &[u8] = "i16".as_bytes();
pub static I32: &[u8] = "i32".as_bytes();
pub static I64: &[u8] = "i64".as_bytes();
pub static U8: &[u8] = "u8".as_bytes();
pub static STRING: &[u8] = "String".as_bytes();
pub static VECTOR_U8: &[u8] = "Vec<u8>".as_bytes();

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
// Keywords or keyword fragments
pub static OPTION: &[u8] = "Option".as_bytes();
pub static PUBLIC: &[u8] = "pub".as_bytes();
pub static START_ENUM: &[u8] = "pub enum ".as_bytes();
pub static START_IMPL: &[u8] = "impl ".as_bytes();

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
// External types
pub static DECIMAL: &[u8] = "rust_decimal::Decimal".as_bytes();
pub static NAIVE_DATE_TIME: &[u8] = "chrono::NaiveDateTime".as_bytes();
pub static STD_TIME_SYSTEMTIME: &[u8] = "std::time::SystemTime".as_bytes();
pub static UUID: &[u8] = "uuid::Uuid".as_bytes();

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
// Function calls and larger code fragments
pub static FN_VALUE_DECL: &[u8] = "pub const fn value(&self) -> &'static str {".as_bytes();
pub static FN_ITERATOR_DECL_START: &[u8] = "pub fn iterator() -> impl Iterator<Item = ".as_bytes();
pub static FN_ITERATOR_DECL_END: &[u8] = "> {".as_bytes();
pub static MATCH_SELF: &[u8] = "match *self {".as_bytes();
pub static CALL_ITER: &[u8] = ".iter()".as_bytes();
pub static CALL_COPIED: &[u8] = ".copied()".as_bytes();
pub static AS_OPT_LIST_START: &[u8] = "
pub fn as_list() -> Vec<&'static str> {
  let mut list = "
    .as_bytes();
pub static AS_OPT_LIST_END: &[u8] = "::iterator().fold(Vec::new(), |mut acc: Vec<&'static str>, es| {
  acc.insert(0, &mut es.value());
  acc
});
list.reverse();
list
}
"
.as_bytes();

pub static START_PUB_STRUCT: &[u8] = &"pub struct ".as_bytes();

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
/// `from_str` implementation for each entity set struct
pub fn impl_from_str_for(struct_name: &str) -> Vec<u8> {
    static FN_START: &[u8] = "
  impl std::str::FromStr for "
        .as_bytes();
    static FN_END: &[u8] = " {
    type Err = quick_xml::DeError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        quick_xml::de::from_str(s)
    }
}

"
    .as_bytes();

    [FN_START, struct_name.as_bytes(), FN_END].concat()
}
// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
/// Derivable traits.
///
/// The Serde traits `Serialize` and `Deserialize` are also included
pub enum DeriveTraits {
    EQ,
    PARTIALEQ,
    ORD,
    PARTIALORD,
    CLONE,
    COPY,
    DEBUG,
    DEFAULT,
    HASH,
    SERIALIZE,
    DESERIALIZE,
}

impl DeriveTraits {
    /// Helper function to return the enum variant name as a static string slice
    pub const fn value(&self) -> &'static str {
        match *self {
            DeriveTraits::EQ => "Eq",
            DeriveTraits::PARTIALEQ => "PartialEq",
            DeriveTraits::ORD => "Ord",
            DeriveTraits::PARTIALORD => "PartialOrd",
            DeriveTraits::CLONE => "Clone",
            DeriveTraits::COPY => "Copy",
            DeriveTraits::DEBUG => "Debug",
            DeriveTraits::DEFAULT => "Default",
            DeriveTraits::HASH => "Hash",
            DeriveTraits::DESERIALIZE => "Deserialize",
            DeriveTraits::SERIALIZE => "Serialize",
        }
    }
}

pub fn derive_str(traits: Vec<DeriveTraits>) -> Vec<u8> {
    let mut d_str = Vec::new();
    d_str.extend_from_slice(DERIVE_START);

    for (idx, d) in traits.iter().enumerate() {
        d_str.extend_from_slice(d.value().as_bytes());

        if idx < traits.len() - 1 {
            d_str.extend([COMMA, SPACE].concat());
        }
    }

    d_str.extend([DERIVE_END, LINE_FEED].concat());
    d_str
}

pub static DERIVE_START: &[u8] = "#[derive(".as_bytes();
pub static DERIVE_END: &[u8] = ")]".as_bytes();

pub static USE_SERDE: &[u8] = "use serde::{Deserialize, Serialize};".as_bytes();
pub static USE_STD_STR: &[u8] = "use std::str::FromStr;".as_bytes();
pub static SERDE_RENAME_PASCAL_CASE: &[u8] = "#[serde(rename_all = \"PascalCase\")]".as_bytes();
pub static SERDE_RENAME_SNAKE_CASE: &[u8] = "#[serde(rename_all = \"snake_case\")]".as_bytes();
pub static SERDE_RENAME: &[u8] = "#[serde(rename = \"".as_bytes();
