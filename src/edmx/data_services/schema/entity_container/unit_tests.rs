use std::{
    fs::File,
    io::{BufReader, Read},
    path::Path,
    str::FromStr,
};

use super::{AssociationSet, EntityContainer};

impl std::str::FromStr for EntityContainer {
    type Err = quick_xml::DeError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        quick_xml::de::from_str(s)
    }
}

impl std::str::FromStr for AssociationSet {
    type Err = quick_xml::DeError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        quick_xml::de::from_str(s)
    }
}

#[test]
pub fn should_parse_entity_container() {
    let mut xml_buffer: Vec<u8> = Vec::new();
    let test_data = File::open(Path::new("./test_data/entity_container.xml")).unwrap();
    let _file_size = BufReader::new(test_data).read_to_end(&mut xml_buffer);

    match String::from_utf8(xml_buffer) {
        Ok(xml) => {
            let ent_cont = EntityContainer::from_str(&xml).unwrap();

            assert_eq!(ent_cont.name, "GWSAMPLE_BASIC_Entities");
            assert_eq!(ent_cont.sap_message_scope_supported, true);
            assert_eq!(ent_cont.sap_supported_formats[0], "atom");
            assert_eq!(ent_cont.sap_supported_formats[1], "json");
            assert_eq!(ent_cont.sap_supported_formats[2], "xlsx");
            assert_eq!(ent_cont.ms_annotations.has_stream, false);
            assert_eq!(ent_cont.ms_annotations.is_default_entity_container, true);
        },
        Err(err) => println!("XML test data was not in UTF8 format: {}", err),
    };
}

#[test]
pub fn should_parse_entity_set() {
    let mut xml_buffer: Vec<u8> = Vec::new();
    let test_data = File::open(Path::new("./test_data/entity_container.xml")).unwrap();
    let _file_size = BufReader::new(test_data).read_to_end(&mut xml_buffer);

    match String::from_utf8(xml_buffer) {
        Ok(xml) => {
            let ent_cont = EntityContainer::from_str(&xml).unwrap();
            assert_eq!(ent_cont.entity_sets.len(), 2);

            assert_eq!(ent_cont.entity_sets[0].name, "BusinessPartnerSet");
            assert_eq!(ent_cont.entity_sets[0].entity_type, "GWSAMPLE_BASIC.BusinessPartner");
            assert_eq!(ent_cont.entity_sets[0].sap_content_version, "1");
            assert_eq!(ent_cont.entity_sets[0].sap_creatable, true);
            assert_eq!(ent_cont.entity_sets[0].sap_deletable, true);
            assert_eq!(ent_cont.entity_sets[0].sap_updatable, true);
            assert_eq!(ent_cont.entity_sets[0].sap_pageable, true);

            assert_eq!(ent_cont.entity_sets[1].name, "VH_CategorySet");
            assert_eq!(ent_cont.entity_sets[1].entity_type, "GWSAMPLE_BASIC.VH_Category");
            assert_eq!(ent_cont.entity_sets[1].sap_content_version, "1");
            assert_eq!(ent_cont.entity_sets[1].sap_creatable, false);
            assert_eq!(ent_cont.entity_sets[1].sap_deletable, false);
            assert_eq!(ent_cont.entity_sets[1].sap_updatable, false);
            assert_eq!(ent_cont.entity_sets[1].sap_pageable, false);
        },
        Err(err) => println!("XML test data was not in UTF8 format: {}", err),
    };
}

#[test]
pub fn should_parse_association_set() {
    let mut xml_buffer: Vec<u8> = Vec::new();
    let test_data = File::open(Path::new("./test_data/association_set.xml")).unwrap();
    let _file_size = BufReader::new(test_data).read_to_end(&mut xml_buffer);

    match String::from_utf8(xml_buffer) {
        Ok(xml) => {
            let assoc_set = AssociationSet::from_str(&xml).unwrap();
            assert_eq!(assoc_set.name, "Assoc_VH_UnitQuantity_SalesOrderLineItem");
            assert_eq!(assoc_set.association, "GWSAMPLE_BASIC.Assoc_VH_UnitQuantity_SalesOrderLineItem");
            assert_eq!(assoc_set.sap_creatable, false);
            assert_eq!(assoc_set.sap_deletable, false);
            assert_eq!(assoc_set.sap_updatable, false);
            assert_eq!(assoc_set.sap_content_version, "1");

            assert_eq!(assoc_set.ends.len(), 2);

            assert_eq!(assoc_set.ends[0].entity_set, Some(String::from("VH_UnitQuantitySet")));
            assert_eq!(assoc_set.ends[0].role, "FromRole_Assoc_VH_UnitQuantity_SalesOrderLineItem");
        },
        Err(err) => println!("XML test data was not in UTF8 format: {}", err),
    };
}
