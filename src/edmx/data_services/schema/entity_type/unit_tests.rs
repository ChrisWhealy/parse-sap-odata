use std::{
    fs::File,
    io::{BufReader, Read},
    path::Path,
    str::FromStr,
};

use super::EntityType;
use crate::test_utils::*;

impl FromStr for EntityType {
    type Err = quick_xml::de::DeError;

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
            handle_test_comparison(&ent_type.has_stream, &true)?;
            handle_test_comparison(&ent_type.key.property_refs.len(), &1)?;
            handle_test_comparison(&ent_type.key.property_refs[0].name, &"BusinessPartnerID".to_string())?;
            handle_test_comparison(&ent_type.properties.len(), &12)?;
            handle_test_comparison(&ent_type.properties[0].odata_name, &"Address".to_string())?;
            handle_test_comparison(&ent_type.properties[0].edm_type, &"GWSAMPLE_BASIC.CT_Address".to_string())?;
            handle_test_comparison(&ent_type.properties[0].nullable, &false)?;
            handle_test_comparison(&ent_type.properties[1].odata_name, &"BusinessPartnerID".to_string())?;
            handle_test_comparison(&ent_type.properties[1].edm_type, &"Edm.String".to_string())?;
            handle_test_comparison(&ent_type.properties[1].nullable, &false)?;
            handle_test_comparison_opt(&ent_type.properties[1].max_length, &Some(10))?;
            handle_test_comparison(&ent_type.properties[1].sap_annotations.is_unicode, &false)?;
            handle_test_comparison_opt(
                &ent_type.properties[1].sap_annotations.label,
                &Some(String::from("Bus. Part. ID")),
            )?;
            handle_test_comparison(&ent_type.properties[1].sap_annotations.is_creatable, &false)?;
            handle_test_comparison(&ent_type.properties[1].sap_annotations.is_updatable, &false)?;
            handle_test_comparison(&ent_type.navigations.len(), &3)?;
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
            handle_test_comparison(&ent_type.has_stream, &true)?;
            handle_test_comparison(&ent_type.sap_content_version, &"1".to_string())?;
            handle_test_comparison(&ent_type.key.property_refs.len(), &1)?;
            handle_test_comparison(&ent_type.key.property_refs[0].name, &"ProductID".to_string())?;
            handle_test_comparison(&ent_type.properties.len(), &21)?;

            // - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
            // Test property values for Price
            let property_price = &ent_type.properties[14];

            handle_test_comparison(&property_price.odata_name, &"Price".to_string())?;
            handle_test_comparison(&property_price.edm_type, &"Edm.Decimal".to_string())?;
            handle_test_comparison_opt(&property_price.precision, &Some(16))?;
            handle_test_comparison_opt(&property_price.scale, &Some(3))?;
            handle_test_comparison(&property_price.nullable, &true)?;
            handle_test_comparison(&property_price.sap_annotations.is_unicode, &false)?;
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
            handle_test_comparison(&property_changed_at.sap_annotations.is_unicode, &false)?;
            handle_test_comparison(&property_changed_at.sap_annotations.is_creatable, &false)?;
            handle_test_comparison(&property_changed_at.sap_annotations.is_updatable, &false)?;
            handle_test_comparison(&ent_type.navigations.len(), &2)?;
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

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
#[test]
pub fn should_parse_entity_type_groups() -> Result<(), String> {
    let mut xml_buffer: Vec<u8> = Vec::new();
    let test_data = File::open(Path::new("./test_data/entity_type_groups.xml")).unwrap();
    let _file_size = BufReader::new(test_data).read_to_end(&mut xml_buffer);

    match String::from_utf8(xml_buffer) {
        Ok(xml) => {
            let ent_type = EntityType::from_str(&xml).unwrap();

            handle_test_comparison(&ent_type.name, &"Groups".to_string())?;
            handle_test_comparison(&ent_type.has_stream, &false)?;
            handle_test_comparison(&ent_type.sap_content_version, &"1".to_string())?;
            handle_test_comparison(&ent_type.key.property_refs.len(), &1)?;
            handle_test_comparison(&ent_type.key.property_refs[0].name, &"groupId".to_string())?;
            handle_test_comparison(&ent_type.properties.len(), &5)?;

            // - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
            // Test property values for groupMembershipId
            let group_id_prop = &ent_type.properties[0];

            handle_test_comparison(&group_id_prop.odata_name, &"groupId".to_string())?;
            handle_test_comparison(&group_id_prop.edm_type, &"Edm.String".to_string())?;
            handle_test_comparison_opt(&group_id_prop.max_length, &Some(36))?;
            handle_test_comparison(&group_id_prop.nullable, &false)?;
            handle_test_comparison(&group_id_prop.sap_annotations.is_unicode, &false)?;

            let type_prop = &ent_type.properties[1];
            handle_test_comparison(&type_prop.odata_name, &"type".to_string())?;
            handle_test_comparison(&group_id_prop.edm_type, &"Edm.String".to_string())?;
            handle_test_comparison_opt(&group_id_prop.max_length, &Some(36))?;
            handle_test_comparison(&group_id_prop.nullable, &false)?;

            let group_membership_id_nav_prop = &ent_type.navigations[0];
            handle_test_comparison(&group_membership_id_nav_prop.name, &"toGroupMemberships".to_string())?;
            handle_test_comparison(
                &group_membership_id_nav_prop.relationship,
                &"service.ProjectServiceV2.GroupMemberships_toGroup".to_string(),
            )?;
            handle_test_comparison(&group_membership_id_nav_prop.from_role, &"Groups".to_string())?;
            handle_test_comparison(&group_membership_id_nav_prop.to_role, &"GroupMemberships".to_string())?;

            Ok(())
        },
        Err(err) => Err(format!("XML test data was not in UTF8 format: {}", err)),
    }
}
