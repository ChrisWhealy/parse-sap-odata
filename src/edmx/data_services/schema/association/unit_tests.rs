use std::{
    fs::File,
    io::{BufReader, Read},
    path::Path,
    str::FromStr,
};

use super::{metadata::normalise_assoc_name, Association};
use crate::test_utils::*;

impl FromStr for Association {
    type Err = quick_xml::DeError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        quick_xml::de::from_str(s)
    }
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
fn handle_normalise_call(assoc_name: &str, expected_val: &str) -> Result<(), String> {
    handle_test_comparison(&normalise_assoc_name(assoc_name), &expected_val.to_string())
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
#[test]
pub fn should_parse_association() -> Result<(), String> {
    let mut xml_buffer: Vec<u8> = Vec::new();
    let test_data = File::open(Path::new("./test_data/association.xml")).unwrap();
    let _file_size = BufReader::new(test_data).read_to_end(&mut xml_buffer);

    match String::from_utf8(xml_buffer) {
        Ok(xml) => {
            let assoc = Association::from_str(&xml).unwrap();

            handle_test_comparison(&assoc.name, &"Assoc_VH_UnitQuantity_SalesOrderLineItem".to_string())?;
            handle_test_comparison(&assoc.sap_content_version, &"1".to_string())?;

            handle_test_comparison(&assoc.ends.len().to_string(), &"2".to_string())?;
            handle_test_comparison_opt(&assoc.ends[0].end_type, &Some("GWSAMPLE_BASIC.VH_UnitQuantity".to_string()))?;
            handle_test_comparison_opt(&assoc.ends[0].multiplicity, &Some("1".to_string()))?;
            handle_test_comparison(
                &assoc.ends[0].role,
                &"FromRole_Assoc_VH_UnitQuantity_SalesOrderLineItem".to_string(),
            )?;

            handle_test_comparison_opt(
                &assoc.ends[1].end_type,
                &Some("GWSAMPLE_BASIC.SalesOrderLineItem".to_string()),
            )?;
            handle_test_comparison_opt(&assoc.ends[1].multiplicity, &Some("*".to_string()))?;
            handle_test_comparison(
                &assoc.ends[1].role,
                &"ToRole_Assoc_VH_UnitQuantity_SalesOrderLineItem".to_string(),
            )?;

            Ok(())
        },
        Err(err) => Err(format!("XML test data was not in UTF8 format: {}", err)),
    }
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
#[test]
pub fn should_normalise_association_name_variations() -> Result<(), String> {
    handle_normalise_call("Assoc_VH_UnitQuantity_SalesOrderLineItem", "VH_UnitQuantity_SalesOrderLineItem")?;
    handle_normalise_call(
        "VH_UnitQuantity_SalesOrderLineItem_AssocSet",
        "VH_UnitQuantity_SalesOrderLineItem",
    )?;
    handle_normalise_call(
        "Assoc_VH_UnitQuantity_SalesOrderLineItem_AssocSet",
        "VH_UnitQuantity_SalesOrderLineItem",
    )?;
    handle_normalise_call(
        "Assoc_VH_UnitQuantity_SalesOrderLineItem_AssocSe",
        "VH_UnitQuantity_SalesOrderLineItem",
    )?;
    handle_normalise_call(
        "Assoc_VH_UnitQuantity_SalesOrderLineItem_AssocS",
        "VH_UnitQuantity_SalesOrderLineItem",
    )?;
    handle_normalise_call(
        "Assoc_VH_UnitQuantity_SalesOrderLineItem_Assoc",
        "VH_UnitQuantity_SalesOrderLineItem",
    )?;
    handle_normalise_call(
        "Assoc_VH_UnitQuantity_SalesOrderLineItem_Asso",
        "VH_UnitQuantity_SalesOrderLineItem",
    )?;
    handle_normalise_call(
        "Assoc_VH_UnitQuantity_SalesOrderLineItem_Ass",
        "VH_UnitQuantity_SalesOrderLineItem",
    )?;
    handle_normalise_call(
        "Assoc_VH_UnitQuantity_SalesOrderLineItem_As",
        "VH_UnitQuantity_SalesOrderLineItem",
    )?;
    handle_normalise_call(
        "Assoc_VH_UnitQuantity_SalesOrderLineItem_A",
        "VH_UnitQuantity_SalesOrderLineItem",
    )?;
    handle_normalise_call(
        "Assoc_VH_UnitQuantity_SalesOrderLineItem_",
        "VH_UnitQuantity_SalesOrderLineItem",
    )?;
    handle_normalise_call("Assoc_VH_UnitQuantity_SalesOrderLineItem", "VH_UnitQuantity_SalesOrderLineItem")?;

    Ok(())
}
