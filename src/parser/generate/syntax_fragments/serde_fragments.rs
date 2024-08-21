use crate::parser::generate::gen_start_struct;

use super::{
    derive_traits::{derive_str, DeriveTraits},
    CLOSE_PAREN, CLOSE_SQR, DOUBLE_QUOTE, LINE_FEED,
};

pub static USE_SERDE: &[u8] = "use serde::{Deserialize, Serialize};
"
.as_bytes();
pub static SERDE_RENAME_ALL_PASCAL_CASE: &[u8] = "#[serde(rename_all = \"PascalCase\")]
"
.as_bytes();

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
// Reference the custom DateTime deserializer functions that exist in the parse-odata-atom-feed crate
static DESER_EDM_DATETIME_TO_NAIVE_DATETIME: &str =
    "parse_sap_atom_feed::deserializers::edm_datetime::to_naive_date_time";
static DESER_EDM_DATETIME_TO_NAIVE_DATETIME_OPT: &str =
    "parse_sap_atom_feed::deserializers::edm_datetime::to_naive_date_time_opt";

pub fn gen_datetime_deserializer_fn(is_nullable: bool) -> String {
    (if is_nullable {
        DESER_EDM_DATETIME_TO_NAIVE_DATETIME_OPT
    } else {
        DESER_EDM_DATETIME_TO_NAIVE_DATETIME
    })
    .to_string()
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
// Reference the custom decimal deserializer functions that exist in the parse-odata-atom-feed crate
static DESER_EDM_DECIMAL_TO_RUST_DECIMAL_PREFIX: &str =
    "parse_sap_atom_feed::deserializers::edm_decimal::to_rust_decimal_";

pub fn gen_decimal_deserializer_ref(is_nullable: bool, scale: Option<u16>) -> String {
    format!(
        "{DESER_EDM_DECIMAL_TO_RUST_DECIMAL_PREFIX}{}dp{}",
        scale.unwrap_or(0),
        if is_nullable { "_opt" } else { "" }
    )
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
static SERDE_DESERIALIZE_WITH: &[u8] = "#[serde(deserialize_with = \"".as_bytes();
pub fn deserialize_with(fn_name: &str) -> Vec<u8> {
    [
        SERDE_DESERIALIZE_WITH,
        fn_name.as_bytes(),
        DOUBLE_QUOTE,
        CLOSE_PAREN,
        CLOSE_SQR,
        LINE_FEED,
    ]
    .concat()
}

static SERDE_RENAME: &[u8] = "#[serde(rename = \"".as_bytes();
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
pub fn gen_deserializable_struct(struct_name: &str) -> Vec<u8> {
    [
        &*derive_str(vec![
            DeriveTraits::CLONE,
            DeriveTraits::DEBUG,
            DeriveTraits::DEFAULT,
            DeriveTraits::SERIALIZE,
            DeriveTraits::DESERIALIZE,
        ]),
        SERDE_RENAME_ALL_PASCAL_CASE,
        &*gen_start_struct(struct_name),
    ]
    .concat()
}
