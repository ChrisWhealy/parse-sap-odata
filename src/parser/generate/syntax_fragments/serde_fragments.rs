use super::{CLOSE_PAREN, CLOSE_SQR, DOUBLE_QUOTE, LINE_FEED};

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
