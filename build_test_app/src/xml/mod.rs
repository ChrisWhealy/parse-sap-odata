// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
// XML Defaults
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
pub fn default_xml_namespace_d() -> String {
    "http://schemas.microsoft.com/ado/2007/08/dataservices".to_string()
}
pub fn default_xml_namespace_m() -> String {
    "http://schemas.microsoft.com/ado/2007/08/dataservices/metadata".to_string()
}
pub fn default_xml_namespace_oasis() -> String {
    "http://docs.oasis-open.org/odata/ns/edm".to_string()
}
pub fn default_xml_namespace_sap() -> String {
    "http://www.sap.com/Protocols/SAPData".to_string()
}

/// # UGLY HACK
///
/// When reading certain entity sets from SAP's demo OData service `GWSAMPLE_BASIC`, if present, the `m:etag` attribute
/// of each `<entry>` tag may contain an invalid XML value such as:
///
/// ```xml
/// <entry m:etag="W/"datetime'2023-08-31T01%3A00%3A06.0000000'"">
/// ```
///
/// Instead of
///
/// ```xml
/// <entry m:etag="datetime'2023-08-31T01%3A00%3A06.0000000'">
/// ```
///
/// Therefore, before attempting to parse the XML as a `Feed::<impl EntityType>`, we must first check for and then
/// remove any invalid `m:etag` attribute values
pub fn sanitise_invalid_etag_values(xml: String) -> String {
    if xml.contains("entry m:etag=\"W/\"") {
        let mut clean_xml = xml.replace("m:etag=\"W/\"", "m:etag=\"");
        clean_xml = clean_xml.replace("'\"\">", "'\">");
        clean_xml
    } else {
        xml
    }
}
