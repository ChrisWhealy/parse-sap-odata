static ISO_LANGUAGE_ENGLISH: &'static str = "en";

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
// XML Namespaces
static XML_NAMESPACE_MS_DATA_SERVICES: &'static str = "http://schemas.microsoft.com/ado/2007/08/dataservices";
static XML_NAMESPACE_MS_DATA_SERVICES_METADATA: &'static str =
    "http://schemas.microsoft.com/ado/2007/08/dataservices/metadata";
static XML_NAMESPACE_MS_EDM: &'static str = "http://schemas.microsoft.com/ado/2008/09/edm";
static XML_NAMESPACE_MS_EDMX: &'static str = "http://schemas.microsoft.com/ado/2007/06/edmx";
static XML_NAMESPACE_OASIS_ODATA_EDMX: &'static str = "http://docs.oasis-open.org/odata/ns/edmx";
static XML_NAMESPACE_SAP_DATA: &'static str = "http://www.sap.com/Protocols/SAPData";
static XML_NAMESPACE_W3_ATOM: &'static str = "http://www.w3.org/2005/Atom";

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
// Serde default deserialization functions
pub fn default_xml_language() -> String {
    ISO_LANGUAGE_ENGLISH.to_string()
}
pub fn default_xml_namespace() -> String {
    XML_NAMESPACE_MS_EDM.to_string()
}
pub fn default_xml_namespace_atom() -> Option<String> {
    Some(XML_NAMESPACE_W3_ATOM.to_string())
}
pub fn default_xml_namespace_edmx() -> String {
    XML_NAMESPACE_MS_EDMX.to_string()
}
pub fn default_xml_namespace_d() -> String {
    XML_NAMESPACE_MS_DATA_SERVICES.to_string()
}
pub fn default_xml_namespace_m() -> String {
    XML_NAMESPACE_MS_DATA_SERVICES_METADATA.to_string()
}
pub fn default_xml_namespace_oasis() -> String {
    XML_NAMESPACE_OASIS_ODATA_EDMX.to_string()
}
pub fn default_xml_namespace_sap() -> String {
    XML_NAMESPACE_SAP_DATA.to_string()
}
