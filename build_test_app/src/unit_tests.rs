use std::{
    fs::File,
    io::{BufReader, Read},
    path::Path,
    string::FromUtf8Error,
};

use crate::{xml::sanitise_invalid_etag_values, Feed};

static FEED_XML_BASE: &str = "https://SAPES5.SAPDEVCENTER.COM:443/sap/opu/odata/iwbep/GWSAMPLE_BASIC/";

include!(concat!(env!("OUT_DIR"), "/gwsample_basic.rs"));

fn fetch_xml_as_string(filename: &str) -> Result<String, FromUtf8Error> {
    let mut xml_buffer: Vec<u8> = Vec::new();
    let test_data = File::open(Path::new(&format!("./test_data/{}", filename))).unwrap();
    let _file_size = BufReader::new(test_data).read_to_end(&mut xml_buffer);

    String::from_utf8(xml_buffer)
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
#[test]
pub fn should_parse_business_partner_set() {
    static ENTITY_SET_NAME: &str = "BusinessPartnerSet";
    let base_service_name = format!("{}{}", FEED_XML_BASE, ENTITY_SET_NAME);

    match fetch_xml_as_string(&format!("{}-etags.xml", ENTITY_SET_NAME)) {
        Ok(xml) => {
            let bp_feed = Feed::<BusinessPartner>::from_str(&sanitise_invalid_etag_values(xml)).unwrap();

            assert_eq!(bp_feed.namespace, "http://www.w3.org/2005/Atom");
            assert_eq!(bp_feed.xml_base, Some(String::from(FEED_XML_BASE)));
            assert_eq!(bp_feed.id, base_service_name);
            assert_eq!(bp_feed.title, ENTITY_SET_NAME);

            assert_eq!(bp_feed.links.len(), 1);
            assert_eq!(bp_feed.links[0].href, ENTITY_SET_NAME);

            if let Some(entries) = bp_feed.entries {
                assert_eq!(entries.len(), 5);
                assert_eq!(entries[0].id, format!("{}('0100000000')", base_service_name));

                assert_eq!(entries[0].content.properties.address.city, Some("Walldorf".to_owned()));
                assert_eq!(entries[0].content.properties.company_name, "SAP");
                assert_eq!(entries[0].content.properties.currency_code, "EUR");
            } else {
                assert!(
                    1 == 2,
                    "{}",
                    format!("Entity set {} should not be empty!", String::from(ENTITY_SET_NAME))
                )
            }
        },
        Err(err) => println!("XML test data was not in UTF8 format: {}", err),
    };
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
#[test]
pub fn should_parse_contact_set() {
    static ENTITY_SET_NAME: &str = "ContactSet";
    let base_service_name = format!("{}{}", FEED_XML_BASE, ENTITY_SET_NAME);

    match fetch_xml_as_string(&format!("{}.xml", ENTITY_SET_NAME)) {
        Ok(xml) => {
            let c_feed = Feed::<Contact>::from_str(&sanitise_invalid_etag_values(xml)).unwrap();

            assert_eq!(c_feed.namespace, "http://www.w3.org/2005/Atom");
            assert_eq!(c_feed.xml_base, Some(String::from(FEED_XML_BASE)));
            assert_eq!(c_feed.id, base_service_name);
            assert_eq!(c_feed.title, ENTITY_SET_NAME);

            assert_eq!(c_feed.links.len(), 1);
            assert_eq!(c_feed.links[0].href, ENTITY_SET_NAME);

            if let Some(entries) = c_feed.entries {
                assert_eq!(entries.len(), 5);
                assert_eq!(
                    entries[0].content.properties.address.street,
                    Some("Robert-Koch-Straße".to_owned())
                );
                assert_eq!(entries[0].content.properties.first_name, "Karl");
                assert_eq!(entries[0].content.properties.last_name, Some("Müller".to_owned()));
            } else {
                assert!(
                    1 == 2,
                    "{}",
                    format!("Entity set {} should not be empty!", String::from(ENTITY_SET_NAME))
                )
            }
        },
        Err(err) => println!("XML test data was not in UTF8 format: {}", err),
    };
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
#[test]
pub fn should_parse_product_set() {
    static ENTITY_SET_NAME: &str = "ProductSet";
    let base_service_name = format!("{}{}", FEED_XML_BASE, ENTITY_SET_NAME);

    match fetch_xml_as_string(&format!("{}.xml", ENTITY_SET_NAME)) {
        Ok(xml) => {
            let p_feed = Feed::<Product>::from_str(&sanitise_invalid_etag_values(xml)).unwrap();

            assert_eq!(p_feed.namespace, "http://www.w3.org/2005/Atom");
            assert_eq!(p_feed.xml_base, Some(String::from(FEED_XML_BASE)));
            assert_eq!(p_feed.id, base_service_name);
            assert_eq!(p_feed.title, ENTITY_SET_NAME);

            assert_eq!(p_feed.links.len(), 1);
            assert_eq!(p_feed.links[0].href, ENTITY_SET_NAME);

            if let Some(entries) = p_feed.entries {
                assert_eq!(entries.len(), 5);
                assert_eq!(entries[0].content.properties.product_id, "2GOYBEBFLB");
                assert_eq!(entries[0].content.properties.category, "Notebooks");
                // assert_eq!(entries[0].content.properties.weight_measure, Decimal::new(4200000, 3));
                assert_eq!(entries[0].content.properties.weight_measure, 4200.0);
            } else {
                assert!(
                    1 == 2,
                    "{}",
                    format!("Entity set {} should not be empty!", String::from(ENTITY_SET_NAME))
                )
            }
        },
        Err(err) => println!("XML test data was not in UTF8 format: {}", err),
    };
}
