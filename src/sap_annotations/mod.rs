use crate::parser::generate::syntax_fragments::{COLON2, DOUBLE_QUOTE};
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
pub fn generate_fq_name(my_name: &'static [u8], member_name: &'static [u8]) -> Vec<u8> {
    let mut fq_name: Vec<u8> = Vec::new();
    fq_name.extend_from_slice(DOUBLE_QUOTE);
    fq_name.extend_from_slice(my_name);
    fq_name.extend_from_slice(COLON2);
    fq_name.extend_from_slice(member_name);
    fq_name.extend_from_slice(DOUBLE_QUOTE);
    fq_name
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
pub trait AnnotationType {
    fn member_name(&self) -> &'static [u8];
}

pub trait OptionalAnnotationType {
    fn opt_anno_type<T: AnnotationType>(&self, opt_self: &Option<T>) -> Vec<u8>;
}

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
