use crate::parser::generate::gen_start_struct;

use super::{
    derive_traits::{gen_derive_str, DeriveTraits},
    CLOSE_PAREN, CLOSE_SQR, DOUBLE_QUOTE, LINE_FEED,
};

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
// Reference the custom DateTime deserializer functions that exist in the parse-odata-atom-feed crate
pub fn gen_datetime_deserializer_fn(is_nullable: bool) -> String {
    format!(
        "parse_sap_atom_feed::deserializers::edm_datetime::to_naive_date_time{}",
        if is_nullable { "_opt" } else { "" }
    )
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
// Reference the custom decimal deserializer functions that exist in the parse-odata-atom-feed crate
pub fn gen_decimal_deserializer_ref(is_nullable: bool, scale: Option<u16>) -> String {
    format!(
        "parse_sap_atom_feed::deserializers::edm_decimal::to_rust_decimal_{}dp{}",
        scale.unwrap_or(0),
        if is_nullable { "_opt" } else { "" }
    )
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
pub fn gen_deserialize_with(fn_name: &str) -> String {
    [
        "#[serde(deserialize_with = ",
        DOUBLE_QUOTE,
        fn_name,
        DOUBLE_QUOTE,
        CLOSE_PAREN,
        CLOSE_SQR,
        LINE_FEED,
    ]
        .concat()
}

pub fn gen_serde_rename(odata_name: &str) -> String {
    [
        "#[serde(rename = ",
        DOUBLE_QUOTE,
        odata_name,
        DOUBLE_QUOTE,
        CLOSE_PAREN,
        CLOSE_SQR,
        LINE_FEED,
    ]
        .concat()
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
pub fn gen_deserializable_struct(struct_name: &str) -> String {
    let mut out = String::new();

    out.push_str(
        &gen_derive_str(&[
            DeriveTraits::CLONE,
            DeriveTraits::DEBUG,
            DeriveTraits::DEFAULT,
            DeriveTraits::SERIALIZE,
            DeriveTraits::DESERIALIZE,
        ]));
    out.push_str("#[serde(rename_all = \"PascalCase\")]");
    out.push_str(LINE_FEED);
    gen_start_struct(&mut out, struct_name);

    out
}
