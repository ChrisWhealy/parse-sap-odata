#[cfg(feature = "parser")]
pub mod rust_tools;

use std::str::FromStr;

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
    Ok(s.split_whitespace().map(str::to_owned).collect())
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
/// Convert an arbitrary OData namespace or service name into a valid Rust module name.
///
/// Applies snake_case conversion and then repairs version suffixes that `convert_case` splits
/// incorrectly: the letter `v` (version) immediately followed by digits must not be separated
/// by an underscore.
///
/// | Input | Output |
/// |---|---|
/// | `"GWSAMPLE_BASIC"` | `"gwsample_basic"` |
/// | `"service.ProjectServiceV2"` | `"service_project_service_v2"` |
/// | `"CatalogServiceV1_0_0"` | `"catalog_service_v1_0_0"` |
pub fn to_module_name(s: &str) -> String {
    // Dots in OData namespace strings (e.g. "service.ProjectServiceV2") are not treated as word
    // boundaries by convert_case, so replace them with underscores before the case conversion.
    let normalised = s.replace('.', "_");
    collapse_version_numbers(&to_snake_case(&normalised))
}

/// After `to_snake_case`, a version suffix like `V2` becomes `v_2`.
/// This function rejoins any isolated `v` segment with an immediately following all-digit segment.
fn collapse_version_numbers(s: &str) -> String {
    let parts: Vec<&str> = s.split('_').collect();
    let mut result: Vec<String> = Vec::with_capacity(parts.len());
    let mut i = 0;

    while i < parts.len() {
        if parts[i] == "v"
            && i + 1 < parts.len()
            && !parts[i + 1].is_empty()
            && parts[i + 1].chars().all(|c| c.is_ascii_digit())
        {
            result.push(format!("v{}", parts[i + 1]));
            i += 2;
        } else {
            result.push(parts[i].to_string());
            i += 1;
        }
    }

    result.join("_")
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
#[cfg(test)]
pub mod unit_tests;
