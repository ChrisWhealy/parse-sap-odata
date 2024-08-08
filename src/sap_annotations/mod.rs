use crate::utils::{de_str_to_bool, default_false, default_true};

pub mod aggregation_role;
pub mod association_set;
pub mod display_format;
pub mod entity_container;
pub mod entity_set;
pub mod entity_type;
pub mod field_control;
pub mod filter_restriction;
pub mod function_import;
pub mod navigation_property;
pub mod parameter;
pub mod property;
pub mod schema;

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
static ATOM: &str = "atom";
static ONE: &str = "1";
static JSON: &str = "json";

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
pub fn default_sap_content_version() -> String {
    ONE.to_string()
}
pub fn default_sap_schema_version() -> String {
    ONE.to_string()
}
pub fn default_entity_container_supported_formats() -> Vec<String> {
    vec![String::from(ATOM), String::from(JSON)]
}
