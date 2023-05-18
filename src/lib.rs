pub mod edmx;
pub mod property;
pub mod sap_annotations;

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
// XML Defaults
// TODO: Consider moving these into a separate module?
// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
pub fn default_true() -> bool {
    true
}
pub fn default_false() -> bool {
    false
}
pub fn default_xml_language() -> String {
    "en".to_string()
}
pub fn default_xml_namespace() -> String {
    "http://schemas.microsoft.com/ado/2008/09/edm".to_string()
}
pub fn default_xml_namespace_atom() -> String {
    "http://www.w3.org/2005/Atom".to_string()
}
pub fn default_xml_namespace_edmx() -> String {
    "http://schemas.microsoft.com/ado/2007/06/edmx".to_string()
}
pub fn default_xml_namespace_m() -> String {
    "http://schemas.microsoft.com/ado/2007/08/dataservices/metadata".to_string()
}
pub fn default_xml_namespace_sap() -> String {
    "http://www.sap.com/Protocols/SAPData".to_string()
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
// Tests
// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
#[cfg(test)]
mod unit_tests;
