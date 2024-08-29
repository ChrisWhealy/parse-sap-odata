#[cfg(feature = "parser")]
pub mod rust_tools;

use std::{collections::HashSet, str::FromStr};

use check_keyword::CheckKeyword;
use convert_case::{Case, Casing};
use serde::{Deserialize, Deserializer};

pub fn default_true() -> bool {
    true
}
pub fn default_false() -> bool {
    false
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
/// Deserialize string to Boolean
pub fn de_str_to_bool<'de, D>(deserializer: D) -> Result<bool, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    bool::from_str(&s).or_else(|_| Ok(false))
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
/// Deserialize space-delimited string to list
pub fn de_str_to_list<'de, D>(deserializer: D) -> Result<Vec<String>, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    Ok(s.split(" ").map(|fmt| String::from(fmt)).collect())
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
/// Case conversion
pub fn to_pascal_case(odata_name: &str) -> String {
    Casing::to_case(&odata_name, Case::Pascal)
}
pub fn to_upper_camel_case(odata_name: &str) -> String {
    Casing::to_case(&odata_name, Case::UpperCamel)
}
pub fn to_snake_case(odata_name: &str) -> String {
    Casing::to_case(&odata_name, Case::Snake)
}

pub fn odata_name_to_rust_safe_name(odata_name: &str) -> String {
    CheckKeyword::into_safe(to_snake_case(odata_name))
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
pub fn dedup_vec_of_string(str_vec: Vec<String>) -> Vec<String> {
    str_vec
        .clone()
        .into_iter()
        .collect::<HashSet<String>>()
        .into_iter()
        .collect::<Vec<String>>()
}

pub fn dedup_vec_of_u8_array(u8_vec: Vec<&[u8]>) -> Vec<&[u8]> {
    u8_vec
        .clone()
        .into_iter()
        .collect::<HashSet<&[u8]>>()
        .into_iter()
        .collect::<Vec<&[u8]>>()
}
