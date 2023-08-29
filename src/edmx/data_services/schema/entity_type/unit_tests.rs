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
            assert_eq!(ent_type.has_stream, false);

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
            assert_eq!(ent_type.properties[1].sap_annotations.sap_unicode, false);
            assert_eq!(
                ent_type.properties[1].sap_annotations.sap_label,
                Some(String::from("Bus. Part. ID"))
            );
            assert_eq!(ent_type.properties[1].sap_annotations.sap_creatable, false);
            assert_eq!(ent_type.properties[1].sap_annotations.sap_updatable, false);

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

#[test]
pub fn should_parse_entity_type_product() {
    let mut xml_buffer: Vec<u8> = Vec::new();
    let test_data = File::open(Path::new("./test_data/entity_type_product.xml")).unwrap();
    let _file_size = BufReader::new(test_data).read_to_end(&mut xml_buffer);

    match String::from_utf8(xml_buffer) {
        Ok(xml) => {
            let ent_type = EntityType::from_str(&xml).unwrap();

            assert_eq!(ent_type.name, "Product");
            assert_eq!(ent_type.sap_content_version, "1");

            assert_eq!(ent_type.key.property_refs.len(), 1);
            assert_eq!(ent_type.key.property_refs[0].name, "ProductID");

            assert_eq!(ent_type.properties.len(), 21);
            assert_eq!(ent_type.properties[14].odata_name, "Price");
            assert_eq!(ent_type.properties[14].edm_type, "Edm.Decimal");
            assert_eq!(ent_type.properties[14].precision, Some(16));
            assert_eq!(ent_type.properties[14].scale, Some(3));
            assert_eq!(ent_type.properties[14].nullable, true);
            assert_eq!(ent_type.properties[14].sap_annotations.sap_unicode, false);
            assert_eq!(
                ent_type.properties[14].sap_annotations.sap_unit,
                Some(String::from("CurrencyCode"))
            );
            assert_eq!(
                ent_type.properties[14].sap_annotations.sap_label,
                Some(String::from("Unit Price"))
            );
            assert_eq!(ent_type.properties[1].sap_annotations.sap_creatable, true);
            assert_eq!(ent_type.properties[1].sap_annotations.sap_updatable, true);

            assert_eq!(ent_type.navigations.len(), 2);
            assert_eq!(ent_type.navigations[0].name, "ToSupplier");
            assert_eq!(
                ent_type.navigations[0].relationship,
                "GWSAMPLE_BASIC.Assoc_BusinessPartner_Products"
            );
            assert_eq!(ent_type.navigations[0].from_role, "ToRole_Assoc_BusinessPartner_Products");
            assert_eq!(ent_type.navigations[0].to_role, "FromRole_Assoc_BusinessPartner_Products");
        },
        Err(err) => println!("XML test data was not in UTF8 format: {}", err),
    };
}
