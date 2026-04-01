use crate::parser::generate::gen_start_struct_into;

use super::{
    derive_traits::{gen_derive_str, DeriveTraits},
    CLOSE_PAREN, CLOSE_SQR, DOUBLE_QUOTE, LINE_FEED,
};

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
// Reference the custom DateTime deserializer functions that exist in the parse-odata-atom-feed crate
pub fn gen_datetime_deserializer_fn(is_nullable: bool) -> String {
    let mut out = String::from("parse_sap_atom_feed::deserializers::edm_datetime::to_naive_date_time");

    if is_nullable {
        out.push_str("_opt")
    }

    out
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
// Reference the custom decimal deserializer functions that exist in the parse-odata-atom-feed crate
pub fn gen_decimal_deserializer_ref(is_nullable: bool, scale: Option<u16>) -> String {
    let mut out = String::from("parse_sap_atom_feed::deserializers::edm_decimal::to_rust_decimal_");
    out.push_str(&scale.unwrap_or(0).to_string());
    out.push_str("dp");

    if is_nullable {
        out.push_str("_opt")
    }

    out
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
pub fn gen_deserialize_with(fn_name: &str) -> String {
    let mut out = String::new();
    out.push_str("#[serde(deserialize_with = ");
    out.push_str(DOUBLE_QUOTE);
    out.push_str(fn_name);
    out.push_str(DOUBLE_QUOTE);
    out.push_str(CLOSE_PAREN);
    out.push_str(CLOSE_SQR);
    out.push_str(LINE_FEED);

    out
}

pub fn gen_serde_rename(odata_name: &str) -> String {
    let mut out = String::new();

    out.push_str("#[serde(rename = ");
    out.push_str(DOUBLE_QUOTE);
    out.push_str(odata_name);
    out.push_str(DOUBLE_QUOTE);
    out.push_str(CLOSE_PAREN);
    out.push_str(CLOSE_SQR);
    out.push_str(LINE_FEED);

    out
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
pub fn gen_deserializable_struct(struct_name: &str) -> String {
    let mut out = String::new();

    out.push_str(&gen_derive_str(&[
        DeriveTraits::CLONE,
        DeriveTraits::DEBUG,
        DeriveTraits::DEFAULT,
        DeriveTraits::SERIALIZE,
        DeriveTraits::DESERIALIZE,
    ]));
    out.push_str("#[serde(rename_all = \"PascalCase\")]");
    out.push_str(LINE_FEED);
    gen_start_struct_into(&mut out, struct_name);

    out
}
