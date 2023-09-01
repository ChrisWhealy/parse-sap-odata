use regex::Regex;

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
pub fn default_xml_namespace_atom() -> Option<String> {
    Some("http://www.w3.org/2005/Atom".to_string())
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

/// # CORRECT FORMATTING ERRORS IN RAW XML
///
/// When reading certain entity sets from SAP's demo OData service `GWSAMPLE_BASIC`, various formatting errors have been
/// noticed that will cause an XML parser to throw its toys out of the pram.
/// Whether these errors extend to other SAP-delivered Odata services has not been determined; however, coding is
/// included here to correct those errors detected so far
///
/// 1. Correct potentially invalid `m:etag` attribute values on an `<entry>` tag:
///
///    ```xml
///    <entry m:etag="W/"datetime'2023-08-31T01%3A00%3A06.0000000'"">
///    ```
///
///    Is corrected to:
///
///    ```xml
///    <entry m:etag="datetime'2023-08-31T01%3A00%3A06.0000000'">
///    ```
///
/// 1. Entity set content properties containing test descriptions are not enclosed in double quotes.
///    Special characters in these descriptions tend not to be escaped, so the XML parser barfs.
///    E.G.:
///
///    ```xml
///    <d:Category>PDAs & Organizers</d:Category>
///    ```
///
///    Is corrected to:
///
///    ```xml
///    <d:Category>PDAs &amp; Organizers</d:Category>
///    ```
pub fn sanitise_xml(xml: String) -> String {
    let clean_xml = sanitise_bad_etags(xml);

    sanitise_naked_ampersand(clean_xml)
}

fn sanitise_bad_etags(xml: String) -> String {
    if xml.contains("entry m:etag=\"W/\"") || xml.contains("entry m:etag=\"W/&quot;") {
        let mut clean_xml = xml.replace("m:etag=\"W/\"", "m:etag=\"");
        clean_xml = clean_xml.replace("m:etag=\"W/&quot;", "m:etag=\"");
        clean_xml = clean_xml.replace("'\"\">", "'\">");
        clean_xml = clean_xml.replace("'&quot;\">", "'\">");
        clean_xml
    } else {
        xml
    }
}

#[test]
fn should_correct_bad_etags() {
    assert_eq!(
        sanitise_bad_etags(String::from(
            "<entry m:etag=\"W/\"datetime'2023-08-31T01%3A48%3A52.9972620'\"\">"
        )),
        "<entry m:etag=\"datetime'2023-08-31T01%3A48%3A52.9972620'\">"
    );
    assert_eq!(
        sanitise_bad_etags(String::from(
            "<entry m:etag=\"W/&quot;datetime'2023-08-31T01%3A48%3A52.9972620'&quot;\">"
        )),
        "<entry m:etag=\"datetime'2023-08-31T01%3A48%3A52.9972620'\">"
    );
}

/// Naked ampersand characters might occur in OData properties containing text descriptions.
/// E.G.:
///
/// ```
/// <d:Category>PDAs & Organizers</d:Category>
/// ```
///
/// Such characters must be replaced with the character encoding `&amp;`
///
/// First, search for ampersands with non-whitespace characters immediately before and after,
/// then search for ampersnad characters with a space on either side.
///
/// This functionality assumes that the character string `&amp;` will not occur in the XML
fn sanitise_naked_ampersand(xml: String) -> String {
    let re = Regex::new(r"(\S)\&(\S)").unwrap();
    let clean_xml = re.replace_all(&xml, "$1&amp;$2");

    clean_xml.replace(" & ", " &amp; ")
}

#[test]
fn should_replace_naked_ampersand() {
    assert_eq!(
        sanitise_naked_ampersand(String::from("<d:Landx>St Kitts&Nevis</d:Landx>")),
        "<d:Landx>St Kitts&amp;Nevis</d:Landx>"
    );
    assert_eq!(
        sanitise_naked_ampersand(String::from("<d:Category>PDAs & Organizers</d:Category>")),
        "<d:Category>PDAs &amp; Organizers</d:Category>"
    );
}
