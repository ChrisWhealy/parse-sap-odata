use std::{
    fs::File,
    io::{BufReader, Read},
    path::Path,
    str::FromStr,
};

use super::{AssociationSet, EntityContainer};
use crate::test_utils::*;

impl FromStr for EntityContainer {
    type Err = quick_xml::DeError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        quick_xml::de::from_str(s)
    }
}

impl FromStr for AssociationSet {
    type Err = quick_xml::DeError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        quick_xml::de::from_str(s)
    }
}

static TRUE_STR: &str = "true";
static FALSE_STR: &str = "false";

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
#[test]
pub fn should_parse_entity_container() -> Result<(), String> {
    let mut xml_buffer: Vec<u8> = Vec::new();
    let test_data = File::open(Path::new("./test_data/entity_container.xml")).unwrap();
    let _file_size = BufReader::new(test_data).read_to_end(&mut xml_buffer);

    match String::from_utf8(xml_buffer) {
        Ok(xml) => {
            let ent_cont = EntityContainer::from_str(&xml).unwrap();

            handle_test_comparison(&ent_cont.name, &"GWSAMPLE_BASIC_Entities".to_string())?;
            handle_test_comparison(
                &ent_cont.sap_annotations.message_scope_supported.to_string(),
                &"false".to_string(),
            )?;
            handle_test_comparison(&ent_cont.sap_annotations.supported_formats[0], &"atom".to_string())?;
            handle_test_comparison(&ent_cont.sap_annotations.supported_formats[1], &"json".to_string())?;
            handle_test_comparison(&ent_cont.sap_annotations.supported_formats[2], &"xlsx".to_string())?;
            handle_test_comparison(&ent_cont.is_default_entity_container.to_string(), &TRUE_STR.to_string())?;

            Ok(())
        },
        Err(err) => Err(format!("XML test data was not in UTF8 format: {}", err)),
    }
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
#[test]
pub fn should_parse_entity_set() -> Result<(), String> {
    let mut xml_buffer: Vec<u8> = Vec::new();
    let test_data = File::open(Path::new("./test_data/entity_container.xml")).unwrap();
    let _file_size = BufReader::new(test_data).read_to_end(&mut xml_buffer);

    match String::from_utf8(xml_buffer) {
        Ok(xml) => {
            let ent_cont = EntityContainer::from_str(&xml).unwrap();

            handle_test_comparison(&ent_cont.entity_sets.len().to_string(), &"2".to_string())?;
            handle_test_comparison(&ent_cont.entity_sets[0].name, &"BusinessPartnerSet".to_string())?;
            handle_test_comparison(
                &ent_cont.entity_sets[0].entity_type,
                &"GWSAMPLE_BASIC.BusinessPartner".to_string(),
            )?;
            handle_test_comparison(&ent_cont.entity_sets[0].sap_annotations.content_version, &"1".to_string())?;
            handle_test_comparison(
                &ent_cont.entity_sets[0].sap_annotations.is_creatable.to_string(),
                &TRUE_STR.to_string(),
            )?;
            handle_test_comparison(
                &ent_cont.entity_sets[0].sap_annotations.is_deletable.to_string(),
                &TRUE_STR.to_string(),
            )?;
            handle_test_comparison(
                &ent_cont.entity_sets[0].sap_annotations.is_updatable.to_string(),
                &TRUE_STR.to_string(),
            )?;
            handle_test_comparison(
                &ent_cont.entity_sets[0].sap_annotations.is_pageable.to_string(),
                &TRUE_STR.to_string(),
            )?;
            handle_test_comparison(&ent_cont.entity_sets[1].name, &"VH_CategorySet".to_string())?;
            handle_test_comparison(&ent_cont.entity_sets[1].entity_type, &"GWSAMPLE_BASIC.VH_Category".to_string())?;
            handle_test_comparison(&ent_cont.entity_sets[1].sap_annotations.content_version, &"1".to_string())?;
            handle_test_comparison(
                &ent_cont.entity_sets[1].sap_annotations.is_creatable.to_string(),
                &FALSE_STR.to_string(),
            )?;
            handle_test_comparison(
                &ent_cont.entity_sets[1].sap_annotations.is_deletable.to_string(),
                &FALSE_STR.to_string(),
            )?;
            handle_test_comparison(
                &ent_cont.entity_sets[1].sap_annotations.is_updatable.to_string(),
                &FALSE_STR.to_string(),
            )?;
            handle_test_comparison(
                &ent_cont.entity_sets[1].sap_annotations.is_pageable.to_string(),
                &FALSE_STR.to_string(),
            )?;

            Ok(())
        },
        Err(err) => Err(format!("XML test data was not in UTF8 format: {}", err)),
    }
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
#[test]
pub fn should_parse_association_set() -> Result<(), String> {
    let mut xml_buffer: Vec<u8> = Vec::new();
    let test_data = File::open(Path::new("./test_data/association_set.xml")).unwrap();
    let _file_size = BufReader::new(test_data).read_to_end(&mut xml_buffer);

    match String::from_utf8(xml_buffer) {
        Ok(xml) => {
            let assoc_set = AssociationSet::from_str(&xml).unwrap();

            handle_test_comparison(&assoc_set.name, &"Assoc_VH_UnitQuantity_SalesOrderLineItem".to_string())?;
            handle_test_comparison(
                &assoc_set.association,
                &"GWSAMPLE_BASIC.Assoc_VH_UnitQuantity_SalesOrderLineItem".to_string(),
            )?;
            handle_test_comparison(&assoc_set.sap_annotations.content_version, &"1".to_string())?;
            handle_test_comparison(&assoc_set.sap_annotations.is_creatable.to_string(), &FALSE_STR.to_string())?;
            handle_test_comparison(&assoc_set.sap_annotations.is_deletable.to_string(), &FALSE_STR.to_string())?;
            handle_test_comparison(&assoc_set.sap_annotations.is_updatable.to_string(), &FALSE_STR.to_string())?;
            handle_test_comparison(&assoc_set.ends.len().to_string(), &"2".to_string())?;
            handle_test_comparison_opt(&assoc_set.ends[0].entity_set, &Some(String::from("VH_UnitQuantitySet")))?;
            handle_test_comparison(
                &assoc_set.ends[0].role,
                &"FromRole_Assoc_VH_UnitQuantity_SalesOrderLineItem".to_string(),
            )?;

            Ok(())
        },
        Err(err) => Err(format!("XML test data was not in UTF8 format: {}", err)),
    }
}
