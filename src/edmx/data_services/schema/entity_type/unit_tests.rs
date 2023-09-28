use std::{
    fs::File,
    io::{BufReader, Read},
    path::Path,
    str::FromStr,
};

use super::EntityType;

impl std::str::FromStr for EntityType {
    type Err = quick_xml::DeError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        quick_xml::de::from_str(s)
    }
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
#[test]
pub fn should_parse_entity_type_business_partner() {
    let mut xml_buffer: Vec<u8> = Vec::new();
    let test_data = File::open(Path::new("./test_data/entity_type_business_partner.xml")).unwrap();
    let _file_size = BufReader::new(test_data).read_to_end(&mut xml_buffer);

    match String::from_utf8(xml_buffer) {
        Ok(xml) => {
            let ent_type = EntityType::from_str(&xml).unwrap();

            assert_eq!(ent_type.name, "BusinessPartner");
            assert_eq!(ent_type.sap_content_version, "1");
            assert_eq!(ent_type.has_stream, true);

            assert_eq!(ent_type.key.property_refs.len(), 1);
            assert_eq!(ent_type.key.property_refs[0].name, "BusinessPartnerID");

            assert_eq!(ent_type.properties.len(), 12);
            assert_eq!(ent_type.properties[0].odata_name, "Address");
            assert_eq!(ent_type.properties[0].edm_type, "GWSAMPLE_BASIC.CT_Address");
            assert_eq!(ent_type.properties[0].nullable, false);

            assert_eq!(ent_type.properties[1].odata_name, "BusinessPartnerID");
            assert_eq!(ent_type.properties[1].edm_type, "Edm.String");
            assert_eq!(ent_type.properties[1].nullable, false);
            assert_eq!(ent_type.properties[1].max_length, Some(10));
            assert_eq!(ent_type.properties[1].sap_annotations.is_unicode, false);
            assert_eq!(
                ent_type.properties[1].sap_annotations.label,
                Some(String::from("Bus. Part. ID"))
            );
            assert_eq!(ent_type.properties[1].sap_annotations.is_creatable, false);
            assert_eq!(ent_type.properties[1].sap_annotations.is_updatable, false);

            assert_eq!(ent_type.navigations.len(), 3);
            assert_eq!(ent_type.navigations[0].name, "ToSalesOrders");
            assert_eq!(
                ent_type.navigations[0].relationship,
                "GWSAMPLE_BASIC.Assoc_BusinessPartner_SalesOrders"
            );
            assert_eq!(ent_type.navigations[0].to_role, "ToRole_Assoc_BusinessPartner_SalesOrders");
        },
        Err(err) => println!("XML test data was not in UTF8 format: {}", err),
    };
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
#[test]
pub fn should_parse_entity_type_product() {
    let mut xml_buffer: Vec<u8> = Vec::new();
    let test_data = File::open(Path::new("./test_data/entity_type_product.xml")).unwrap();
    let _file_size = BufReader::new(test_data).read_to_end(&mut xml_buffer);

    match String::from_utf8(xml_buffer) {
        Ok(xml) => {
            let ent_type = EntityType::from_str(&xml).unwrap();

            assert_eq!(ent_type.name, "Product");
            assert_eq!(ent_type.has_stream, true);
            assert_eq!(ent_type.sap_content_version, "1");

            assert_eq!(ent_type.key.property_refs.len(), 1);
            assert_eq!(ent_type.key.property_refs[0].name, "ProductID");

            assert_eq!(ent_type.properties.len(), 21);

            // - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
            // Test property values for Price
            let property_price = &ent_type.properties[14];

            assert_eq!(property_price.odata_name, "Price");
            assert_eq!(property_price.edm_type, "Edm.Decimal");
            assert_eq!(property_price.precision, Some(16));
            assert_eq!(property_price.scale, Some(3));
            assert_eq!(property_price.nullable, true);
            assert_eq!(property_price.sap_annotations.is_unicode, false);
            assert_eq!(property_price.sap_annotations.unit, Some(String::from("CurrencyCode")));
            assert_eq!(property_price.sap_annotations.label, Some(String::from("Unit Price")));

            // - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
            // Test property values for a ChangedAt date
            let property_changed_at = &ent_type.properties[20];

            assert_eq!(property_changed_at.odata_name, "ChangedAt");
            assert_eq!(property_changed_at.edm_type, "Edm.DateTime");
            assert_eq!(property_changed_at.precision, Some(7));
            assert_eq!(property_changed_at.scale, None);
            assert_eq!(property_changed_at.concurrency_mode, Some(String::from("Fixed")));
            assert_eq!(property_changed_at.sap_annotations.label, Some(String::from("Time Stamp")));
            assert_eq!(property_changed_at.sap_annotations.is_unicode, false);
            assert_eq!(property_changed_at.sap_annotations.is_creatable, false);
            assert_eq!(property_changed_at.sap_annotations.is_updatable, false);

            assert_eq!(ent_type.navigations.len(), 2);
            assert_eq!(ent_type.navigations[0].name, "ToSupplier");
            assert_eq!(
                ent_type.navigations[0].relationship,
                "GWSAMPLE_BASIC.Assoc_BusinessPartner_Products"
            );

            assert_eq!(ent_type.navigations[0].name, "ToSupplier");
            assert_eq!(
                ent_type.navigations[0].relationship,
                "GWSAMPLE_BASIC.Assoc_BusinessPartner_Products"
            );
            assert_eq!(ent_type.navigations[0].from_role, "FromRole_Assoc_BusinessPartner_Products");
            assert_eq!(ent_type.navigations[0].to_role, "ToRole_Assoc_BusinessPartner_Products");

            assert_eq!(ent_type.navigations[1].name, "ToSalesOrderLineItems");
            assert_eq!(
                ent_type.navigations[1].relationship,
                "GWSAMPLE_BASIC.Assoc_Product_SalesOrderLineItems"
            );
            assert_eq!(ent_type.navigations[1].from_role, "FromRole_Assoc_Product_SalesOrderLineItems");
            assert_eq!(ent_type.navigations[1].to_role, "ToRole_Assoc_Product_SalesOrderLineItems");
        },
        Err(err) => println!("XML test data was not in UTF8 format: {}", err),
    };
}
