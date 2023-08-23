use std::{
    fs::File,
    io::{BufReader, Read},
    path::Path,
    str::FromStr,
};

use super::ReferentialConstraint;

impl std::str::FromStr for ReferentialConstraint {
    type Err = quick_xml::DeError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        quick_xml::de::from_str(s)
    }
}

#[test]
pub fn should_parse_referntial_constraint() {
    let mut xml_buffer: Vec<u8> = Vec::new();
    let test_data = File::open(Path::new("./test_data/referential_constraint.xml")).unwrap();
    let _file_size = BufReader::new(test_data).read_to_end(&mut xml_buffer);

    match String::from_utf8(xml_buffer) {
        Ok(xml) => {
            let ref_con = ReferentialConstraint::from_str(&xml).unwrap();

            assert_eq!(ref_con.principal.role, "FromRole_Assoc_VH_UnitLength_Products");
            assert_eq!(ref_con.dependent.role, "ToRole_Assoc_VH_UnitLength_Products");

            assert_eq!(ref_con.principal.property_refs.len(), 1);
            assert_eq!(ref_con.principal.property_refs[0].name, "Msehi");

            assert_eq!(ref_con.dependent.property_refs.len(), 1);
            assert_eq!(ref_con.dependent.property_refs[0].name, "DimUnit");
        },
        Err(err) => println!("XML test data was not in UTF8 format: {}", err),
    };
}
