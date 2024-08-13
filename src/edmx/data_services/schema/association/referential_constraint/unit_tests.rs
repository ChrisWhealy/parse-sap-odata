use super::ReferentialConstraint;
use crate::test_utils::handle_test_comparison;
use std::{
    fs::File,
    io::{BufReader, Read},
    path::Path,
    str::FromStr,
};

impl std::str::FromStr for ReferentialConstraint {
    type Err = quick_xml::DeError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        quick_xml::de::from_str(s)
    }
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
#[test]
pub fn should_parse_referential_constraint() -> Result<(), String> {
    let test_data = File::open(Path::new("./test_data/referential_constraint.xml")).unwrap();
    let mut xml_buffer: Vec<u8> = Vec::new();
    let _file_size = BufReader::new(test_data).read_to_end(&mut xml_buffer);

    match String::from_utf8(xml_buffer) {
        Ok(xml) => {
            let ref_con = ReferentialConstraint::from_str(&xml).unwrap();

            handle_test_comparison(&ref_con.principal.role, &"FromRole_Assoc_VH_UnitLength_Products".to_string())?;
            handle_test_comparison(&ref_con.principal.property_refs.len().to_string(), &"1".to_string())?;
            handle_test_comparison(&ref_con.principal.property_refs[0].name, &"Msehi".to_string())?;

            handle_test_comparison(&ref_con.dependent.role, &"ToRole_Assoc_VH_UnitLength_Products".to_string())?;
            handle_test_comparison(&ref_con.dependent.property_refs.len().to_string(), &"1".to_string())?;
            handle_test_comparison(&ref_con.dependent.property_refs[0].name, &"DimUnit".to_string())?;

            Ok(())
        },
        Err(err) => Err(format!("XML test data was not in UTF8 format: {}", err)),
    }
}
