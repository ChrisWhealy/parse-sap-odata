use crate::utils::to_module_name;

#[test]
fn should_preserve_version_suffix() {
    assert_eq!(to_module_name("service.ProjectServiceV2"), "service_project_service_v2");
}

#[test]
fn should_preserve_multi_digit_version_suffix() {
    assert_eq!(to_module_name("CatalogServiceV12"), "catalog_service_v12");
}

#[test]
fn should_handle_all_caps_namespace() {
    assert_eq!(to_module_name("GWSAMPLE_BASIC"), "gwsample_basic");
}

#[test]
fn should_handle_already_snake_case() {
    assert_eq!(to_module_name("catalogservice"), "catalogservice");
}

#[test]
fn should_not_collapse_full_word_version() {
    // "version" is a full word, not a standalone 'v' abbreviation — must not be collapsed
    assert_eq!(to_module_name("ApiVersion2"), "api_version_2");
}
