use crate::{property::PropertyRef, utils::odata_name_to_rust_safe_name};

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
// Single characters
pub static LINE_FEED: &[u8] = &[0x0a];
pub static SPACE: &[u8] = &[0x20];
pub static DOUBLE_QUOTE: &[u8] = &[0x22];
pub static OPEN_PAREN: &[u8] = &[0x28];
pub static CLOSE_PAREN: &[u8] = &[0x29];
pub static COMMA: &[u8] = &[0x2C];
pub static COLON: &[u8] = &[0x3A];
pub static SEMI_COLON: &[u8] = &[0x3B];
pub static OPEN_ANGLE: &[u8] = &[0x3C];
pub static CLOSE_ANGLE: &[u8] = &[0x3E];
pub static OPEN_SQR: &[u8] = &[0x5B];
pub static BACK_SLASH: &[u8] = &[0x5C];
pub static CLOSE_SQR: &[u8] = &[0x5D];
pub static OPEN_CURLY: &[u8] = &[0x7B];
pub static CLOSE_CURLY: &[u8] = &[0x7D];

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
// Comment separators
pub static COMMMENT_LINE: &[u8] = "// ".as_bytes();
pub static SEPARATOR: &[u8] = "// -----------------------------------------------------------------------------
"
.as_bytes();

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
pub static UNIT: &[u8] = "()".as_bytes();
pub static STRING: &[u8] = "String".as_bytes();
pub static VECTOR_U8: &[u8] = "Vec<u8>".as_bytes();
pub static VECTOR_STATIC_STR: &[u8] = "Vec<&'static str>".as_bytes();

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
// Keywords or keyword fragments
pub static ENUM: &[u8] = "enum ".as_bytes();
pub static FOR: &[u8] = "for ".as_bytes();
pub static FN: &[u8] = "fn ".as_bytes();
pub static IMPL: &[u8] = "impl ".as_bytes();
pub static MOD: &[u8] = "mod ".as_bytes();
pub static OPTION: &[u8] = "Option".as_bytes();
pub static PUBLIC: &[u8] = "pub ".as_bytes();
pub static TRAIT: &[u8] = "trait ".as_bytes();

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
// External types
pub static DECIMAL: &[u8] = "rust_decimal::Decimal".as_bytes();
pub static NAIVE_DATE_TIME: &[u8] = "chrono::NaiveDateTime".as_bytes();
pub static STD_TIME_SYSTEMTIME: &[u8] = "std::time::SystemTime".as_bytes();
pub static UUID: &[u8] = "uuid::Uuid".as_bytes();

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
// Function calls and larger code fragments
pub static FN_VALUE_DECL: &[u8] = "pub const fn value(&self) -> &'static str {
"
.as_bytes();
pub static FN_ITERATOR_DECL_START: &[u8] = "pub fn iterator() -> impl Iterator<Item = ".as_bytes();
pub static FN_ITERATOR_DECL_END: &[u8] = "> {
"
.as_bytes();
pub static MATCH_SELF: &[u8] = "match *self {
"
.as_bytes();
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
/// Compiler attributes
pub static RUSTC_ALLOW_DEAD_CODE: &[u8] = "#[allow(dead_code)]
"
.as_bytes();

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
/// Derivable traits.
///
/// The Serde traits `Serialize` and `Deserialize` are also included
pub enum DeriveTraits {
    CLONE,
    COPY,
    DEBUG,
    DEFAULT,
    DESERIALIZE,
    EQ,
    HASH,
    ITERATOR,
    ORD,
    PARTIALEQ,
    PARTIALORD,
    SERIALIZE,
}

impl DeriveTraits {
    /// Helper function to return the enum variant name as a static string slice
    pub const fn value(&self) -> &'static str {
        match *self {
            DeriveTraits::CLONE => "Clone",
            DeriveTraits::COPY => "Copy",
            DeriveTraits::DEBUG => "Debug",
            DeriveTraits::DEFAULT => "Default",
            DeriveTraits::DESERIALIZE => "Deserialize",
            DeriveTraits::EQ => "Eq",
            DeriveTraits::HASH => "Hash",
            DeriveTraits::ITERATOR => "Iterator",
            DeriveTraits::ORD => "Ord",
            DeriveTraits::PARTIALEQ => "PartialEq",
            DeriveTraits::PARTIALORD => "PartialOrd",
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

    d_str.extend(DERIVE_END);
    d_str
}

pub static DERIVE_START: &[u8] = "#[derive(".as_bytes();
pub static DERIVE_END: &[u8] = ")]
"
.as_bytes();

pub static USE_SERDE: &[u8] = "use serde::{Deserialize, Serialize};
"
.as_bytes();
pub static SERDE_RENAME_ALL_PASCAL_CASE: &[u8] = "#[serde(rename_all = \"PascalCase\")]
"
.as_bytes();
pub static SERDE_RENAME: &[u8] = "#[serde(rename = \"".as_bytes();

// Deserializers supplied by the rust_decimal crate
// The consuming application needs to declare 'rust_decimal = { version = "1.nn", features = ["serde-with-str"]}'
pub static SERDE_DE_DECIMAL: &str = "rust_decimal::serde::str";
pub static SERDE_DE_DECIMAL_OPT: &str = "rust_decimal::serde::str_option";

// These declarations make forward references to custom deserializers that exist in the parse-odata-atom-feed crate
pub static SERDE_DE_DATETIME_OPT: &str = "parse_sap_atom_feed::deserializers::de_date_to_optional_naive_date_time";
pub static SERDE_DE_DATETIME: &str = "parse_sap_atom_feed::deserializers::de_date_to_naive_date_time";

pub fn deserialize_with(de: &'static str, de_is_function: bool) -> Vec<u8> {
    String::from_utf8(
        [
            (if de_is_function {
                "#[serde(deserialize_with = \""
            } else {
                "#[serde(with = \""
            })
            .as_bytes(),
            de.as_bytes(),
            DOUBLE_QUOTE,
            CLOSE_PAREN,
            CLOSE_SQR,
            LINE_FEED,
        ]
        .concat(),
    )
    .unwrap()
    .into()
}

pub fn comment_for(something: &str) -> Vec<u8> {
    [SEPARATOR, COMMMENT_LINE, something.as_bytes(), LINE_FEED, SEPARATOR].concat()
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
/// Start a module declaration
pub fn gen_mod_start(mod_name: &str) -> Vec<u8> {
    [MOD, mod_name.as_bytes(), SPACE, OPEN_CURLY, LINE_FEED].concat()
}

pub fn gen_serde_rename(odata_name: &str) -> Vec<u8> {
    [
        SERDE_RENAME,
        odata_name.as_bytes(),
        DOUBLE_QUOTE,
        CLOSE_PAREN,
        CLOSE_SQR,
        LINE_FEED,
    ]
    .concat()
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
/// Start and end of a struct declaration
pub fn start_struct(struct_name: &str) -> Vec<u8> {
    [START_PUB_STRUCT, SPACE, struct_name.as_bytes(), OPEN_CURLY, LINE_FEED].concat()
}

pub fn gen_struct_field(field_name: &str, rust_type: &Vec<u8>) -> Vec<u8> {
    [PUBLIC, field_name.as_bytes(), COLON, rust_type, COMMA, LINE_FEED].concat()
}

pub fn end_block() -> Vec<u8> {
    [CLOSE_CURLY, LINE_FEED, LINE_FEED].concat()
}

pub fn end_iter_fn() -> Vec<u8> {
    [CLOSE_SQR, CALL_ITER, CALL_COPIED, LINE_FEED, &end_block()[..]].concat()
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
/// Start of an implementation
pub fn gen_impl_start(some_name: &str) -> Vec<u8> {
    [IMPL, some_name.as_bytes(), SPACE, OPEN_CURLY, LINE_FEED].concat()
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
/// Start of an enum declaration
pub fn gen_enum_start(enum_name: &str) -> Vec<u8> {
    [PUBLIC, ENUM, enum_name.as_bytes(), SPACE, OPEN_CURLY, LINE_FEED].concat()
}

pub fn gen_fq_enum_variant_name(enum_name: &str, variant_name: &str) -> Vec<u8> {
    [enum_name.as_bytes(), COLON2, variant_name.as_bytes()].concat()
}

pub fn gen_enum_variant(variant_name: &str) -> Vec<u8> {
    [variant_name.as_bytes(), COMMA, LINE_FEED].concat()
}

pub fn gen_fq_enum_variant(enum_name: &str, variant_name: &str) -> Vec<u8> {
    [&gen_fq_enum_variant_name(&enum_name, &variant_name)[..], COMMA, LINE_FEED].concat()
}

pub fn gen_enum_as_list_fn(enum_name: &str) -> Vec<u8> {
    [AS_OPT_LIST_START, enum_name.as_bytes(), AS_OPT_LIST_END].concat()
}

pub fn gen_enum_value_fn_start() -> Vec<u8> {
    [FN_VALUE_DECL, MATCH_SELF].concat()
}

pub fn gen_enum_iter_fn_start(type_name: &str) -> Vec<u8> {
    [
        FN_ITERATOR_DECL_START,
        type_name.as_bytes(),
        FN_ITERATOR_DECL_END,
        OPEN_SQR,
        LINE_FEED,
    ]
    .concat()
}

pub fn gen_enum_match_arm(enum_name: &str, variant_name: &str, variant_value: &str) -> Vec<u8> {
    [
        &gen_fq_enum_variant_name(enum_name, variant_name)[..],
        SPACE,
        FAT_ARROW,
        SPACE,
        DOUBLE_QUOTE,
        variant_value.as_bytes(),
        DOUBLE_QUOTE,
        COMMA,
        LINE_FEED,
    ]
    .concat()
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
/// EntityType public trait declaration and implementation
pub fn gen_entity_type_trait_for(struct_name: &str) -> Vec<u8> {
    [
        RUSTC_ALLOW_DEAD_CODE,
        PUBLIC,
        SPACE,
        TRAIT,
        struct_name.as_bytes(),
        OPEN_CURLY,
        &gen_get_key_fn_sig(),
        SEMI_COLON,
        &end_block()[..],
    ]
    .concat()
}

pub fn impl_entity_type_trait(trait_name: &str, struct_name: &str, key_names: &Vec<PropertyRef>) -> Vec<u8> {
    [
        IMPL,
        trait_name.as_bytes(),
        SPACE,
        FOR,
        struct_name.as_bytes(),
        OPEN_CURLY,
        &gen_get_key_fn_sig(),
        OPEN_CURLY,
        "vec!".as_bytes(),
        OPEN_SQR,
        key_names
            .into_iter()
            .map(|pr| format!("\"{}\"", odata_name_to_rust_safe_name(&pr.name)))
            .collect::<Vec<String>>()
            .join(",")
            .as_bytes(),
        CLOSE_SQR,
        CLOSE_CURLY,
        &end_block()[..],
    ]
    .concat()
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
/// Signature of EntityType trait function get_key()
pub fn gen_get_key_fn_sig() -> Vec<u8> {
    [FN, "get_key() -> ".as_bytes(), VECTOR_STATIC_STR].concat()
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
pub fn gen_option_of_type(ty: &[u8]) -> Vec<u8> {
    [OPTION, OPEN_ANGLE, ty, CLOSE_ANGLE].concat()
}
