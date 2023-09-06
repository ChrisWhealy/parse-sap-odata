pub mod association_set_sap_annotations;
pub mod entity_container_sap_annotations;
pub mod entity_set_sap_annotations;
pub mod entity_type_sap_annotations;
pub mod function_import_parameter_sap_annotations;
pub mod function_import_sap_annotations;
pub mod navigation_property_sap_annotations;
pub mod property_sap_annotations;
pub mod schema_sap_annotations;

pub fn default_sap_content_version() -> String {
    "1".to_string()
}

pub fn default_sap_schema_version() -> String {
    "1".to_string()
}

pub fn default_entity_container_supported_formats() -> Vec<String> {
    vec![String::from("atom json")]
}
