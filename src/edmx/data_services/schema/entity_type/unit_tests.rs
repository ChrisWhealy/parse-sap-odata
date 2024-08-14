use std::{
    fs::File,
    io::{BufReader, Read},
    path::Path,
    str::FromStr,
};

use super::EntityType;
use crate::test_utils::*;

static TRUE_STR: &str = "true";
static FALSE_STR: &str = "false";

impl FromStr for EntityType {
    type Err = quick_xml::DeError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        quick_xml::de::from_str(s)
    }
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
#[test]
pub fn should_parse_entity_type_business_partner() -> Result<(), String> {
    let mut xml_buffer: Vec<u8> = Vec::new();
    let test_data = File::open(Path::new("./test_data/entity_type_business_partner.xml")).unwrap();
    let _file_size = BufReader::new(test_data).read_to_end(&mut xml_buffer);

    match String::from_utf8(xml_buffer) {
        Ok(xml) => {
            let ent_type = EntityType::from_str(&xml).unwrap();

            handle_test_comparison(&ent_type.name, &"BusinessPartner".to_string())?;
            handle_test_comparison(&ent_type.sap_content_version, &"1".to_string())?;
            handle_test_comparison(&ent_type.has_stream.to_string(), &TRUE_STR.to_string())?;
            handle_test_comparison(&ent_type.key.property_refs.len().to_string(), &"1".to_string())?;
            handle_test_comparison(&ent_type.key.property_refs[0].name, &"BusinessPartnerID".to_string())?;
            handle_test_comparison(&ent_type.properties.len().to_string(), &"12".to_string())?;
            handle_test_comparison(&ent_type.properties[0].odata_name, &"Address".to_string())?;
            handle_test_comparison(&ent_type.properties[0].edm_type, &"GWSAMPLE_BASIC.CT_Address".to_string())?;
            handle_test_comparison(&ent_type.properties[0].nullable.to_string(), &FALSE_STR.to_string())?;
            handle_test_comparison(&ent_type.properties[1].odata_name, &"BusinessPartnerID".to_string())?;
            handle_test_comparison(&ent_type.properties[1].edm_type, &"Edm.String".to_string())?;
            handle_test_comparison(&ent_type.properties[1].nullable.to_string(), &FALSE_STR.to_string())?;
            handle_test_comparison_opt(&ent_type.properties[1].max_length, &Some(10))?;
            handle_test_comparison(
                &ent_type.properties[1].sap_annotations.is_unicode.to_string(),
                &FALSE_STR.to_string(),
            )?;
            handle_test_comparison_opt(
                &ent_type.properties[1].sap_annotations.label,
                &Some(String::from("Bus. Part. ID")),
            )?;
            handle_test_comparison(
                &ent_type.properties[1].sap_annotations.is_creatable.to_string(),
                &FALSE_STR.to_string(),
            )?;
            handle_test_comparison(
                &ent_type.properties[1].sap_annotations.is_updatable.to_string(),
                &FALSE_STR.to_string(),
            )?;
            handle_test_comparison(&ent_type.navigations.len().to_string(), &"3".to_string())?;
            handle_test_comparison(&ent_type.navigations[0].name, &"ToSalesOrders".to_string())?;
            handle_test_comparison(
                &ent_type.navigations[0].relationship,
                &"GWSAMPLE_BASIC.Assoc_BusinessPartner_SalesOrders".to_string(),
            )?;
            handle_test_comparison(
                &ent_type.navigations[0].to_role,
                &"ToRole_Assoc_BusinessPartner_SalesOrders".to_string(),
            )?;

            Ok(())
        },
        Err(err) => Err(format!("XML test data was not in UTF8 format: {}", err)),
    }
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
#[test]
pub fn should_parse_entity_type_product() -> Result<(), String> {
    let mut xml_buffer: Vec<u8> = Vec::new();
    let test_data = File::open(Path::new("./test_data/entity_type_product.xml")).unwrap();
    let _file_size = BufReader::new(test_data).read_to_end(&mut xml_buffer);

    match String::from_utf8(xml_buffer) {
        Ok(xml) => {
            let ent_type = EntityType::from_str(&xml).unwrap();

            handle_test_comparison(&ent_type.name, &"Product".to_string())?;
            handle_test_comparison(&ent_type.has_stream.to_string(), &TRUE_STR.to_string())?;
            handle_test_comparison(&ent_type.sap_content_version, &"1".to_string())?;
            handle_test_comparison(&ent_type.key.property_refs.len().to_string(), &"1".to_string())?;
            handle_test_comparison(&ent_type.key.property_refs[0].name, &"ProductID".to_string())?;
            handle_test_comparison(&ent_type.properties.len().to_string(), &"21".to_string())?;

            // - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
            // Test property values for Price
            let property_price = &ent_type.properties[14];

            handle_test_comparison(&property_price.odata_name, &"Price".to_string())?;
            handle_test_comparison(&property_price.edm_type, &"Edm.Decimal".to_string())?;
            handle_test_comparison_opt(&property_price.precision, &Some(16))?;
            handle_test_comparison_opt(&property_price.scale, &Some(3))?;
            handle_test_comparison(&property_price.nullable.to_string(), &TRUE_STR.to_string())?;
            handle_test_comparison(&property_price.sap_annotations.is_unicode.to_string(), &FALSE_STR.to_string())?;
            handle_test_comparison_opt(&property_price.sap_annotations.unit, &Some(String::from("CurrencyCode")))?;
            handle_test_comparison_opt(&property_price.sap_annotations.label, &Some(String::from("Unit Price")))?;

            // - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
            // Test property values for a ChangedAt date
            let property_changed_at = &ent_type.properties[20];

            handle_test_comparison(&property_changed_at.odata_name, &"ChangedAt".to_string())?;
            handle_test_comparison(&property_changed_at.edm_type, &"Edm.DateTime".to_string())?;
            handle_test_comparison_opt(&property_changed_at.precision, &Some(7))?;
            handle_test_comparison_opt(&property_changed_at.scale, &None)?;
            handle_test_comparison_opt(&property_changed_at.concurrency_mode, &Some(String::from("Fixed")))?;
            handle_test_comparison_opt(&property_changed_at.sap_annotations.label, &Some(String::from("Time Stamp")))?;
            handle_test_comparison(
                &property_changed_at.sap_annotations.is_unicode.to_string(),
                &FALSE_STR.to_string(),
            )?;
            handle_test_comparison(
                &property_changed_at.sap_annotations.is_creatable.to_string(),
                &FALSE_STR.to_string(),
            )?;
            handle_test_comparison(
                &property_changed_at.sap_annotations.is_updatable.to_string(),
                &FALSE_STR.to_string(),
            )?;
            handle_test_comparison(&ent_type.navigations.len().to_string(), &"2".to_string())?;
            handle_test_comparison(&ent_type.navigations[0].name, &"ToSupplier".to_string())?;
            handle_test_comparison(
                &ent_type.navigations[0].relationship,
                &"GWSAMPLE_BASIC.Assoc_BusinessPartner_Products".to_string(),
            )?;
            handle_test_comparison(&ent_type.navigations[0].name, &"ToSupplier".to_string())?;
            handle_test_comparison(
                &ent_type.navigations[0].relationship,
                &"GWSAMPLE_BASIC.Assoc_BusinessPartner_Products".to_string(),
            )?;
            handle_test_comparison(
                &ent_type.navigations[0].from_role,
                &"FromRole_Assoc_BusinessPartner_Products".to_string(),
            )?;
            handle_test_comparison(
                &ent_type.navigations[0].to_role,
                &"ToRole_Assoc_BusinessPartner_Products".to_string(),
            )?;
            handle_test_comparison(&ent_type.navigations[1].name, &"ToSalesOrderLineItems".to_string())?;
            handle_test_comparison(
                &ent_type.navigations[1].relationship,
                &"GWSAMPLE_BASIC.Assoc_Product_SalesOrderLineItems".to_string(),
            )?;
            handle_test_comparison(
                &ent_type.navigations[1].from_role,
                &"FromRole_Assoc_Product_SalesOrderLineItems".to_string(),
            )?;
            handle_test_comparison(
                &ent_type.navigations[1].to_role,
                &"ToRole_Assoc_Product_SalesOrderLineItems".to_string(),
            )?;

            Ok(())
        },
        Err(err) => Err(format!("XML test data was not in UTF8 format: {}", err)),
    }
}
