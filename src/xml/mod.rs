pub static ISO_CODE_ENGLISH: &[u8] = "en".as_bytes();

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
// XML Namespaces
pub static XML_NAMESPACE_MS_ATOM: &[u8] = "http://www.w3.org/2005/Atom".as_bytes();
pub static XML_NAMESPACE_MS_DATA_SERVICES: &[u8] = "http://schemas.microsoft.com/ado/2007/08/dataservices".as_bytes();
pub static XML_NAMESPACE_MS_DATA_SERVICES_METADATA: &[u8] =
    "http://schemas.microsoft.com/ado/2007/08/dataservices/metadata".as_bytes();
pub static XML_NAMESPACE_MS_EDM: &[u8] = "http://schemas.microsoft.com/ado/2008/09/edm".as_bytes();
pub static XML_NAMESPACE_MS_EDMX: &[u8] = "http://schemas.microsoft.com/ado/2007/06/edmx".as_bytes();
pub static XML_NAMESPACE_MS_OASIS_EDMX: &[u8] = "http://docs.oasis-open.org/odata/ns/edmx".as_bytes();
pub static XML_NAMESPACE_SAP_ODATA: &[u8] = "http://www.sap.com/Protocols/SAPData".as_bytes();

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
// Serde default deserialization functions
pub fn default_xml_language() -> String {
    String::from_utf8(ISO_CODE_ENGLISH.to_vec()).unwrap()
}
pub fn default_xml_namespace() -> String {
    String::from_utf8(XML_NAMESPACE_MS_EDM.to_vec()).unwrap()
}
pub fn default_xml_namespace_atom() -> Option<String> {
    Some(String::from_utf8(XML_NAMESPACE_MS_ATOM.to_vec()).unwrap())
}
pub fn default_xml_namespace_edmx() -> String {
    String::from_utf8(XML_NAMESPACE_MS_EDMX.to_vec()).unwrap()
}
pub fn default_xml_namespace_d() -> String {
    String::from_utf8(XML_NAMESPACE_MS_DATA_SERVICES.to_vec()).unwrap()
}
pub fn default_xml_namespace_m() -> String {
    String::from_utf8(XML_NAMESPACE_MS_DATA_SERVICES_METADATA.to_vec()).unwrap()
}
pub fn default_xml_namespace_oasis() -> String {
    String::from_utf8(XML_NAMESPACE_MS_OASIS_EDMX.to_vec()).unwrap()
}
pub fn default_xml_namespace_sap() -> String {
    String::from_utf8(XML_NAMESPACE_SAP_ODATA.to_vec()).unwrap()
}
