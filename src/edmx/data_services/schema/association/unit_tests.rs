use std::{
    fs::File,
    io::{BufReader, Read},
    path::Path,
    str::FromStr,
};

use super::Association;

impl std::str::FromStr for Association {
    type Err = quick_xml::DeError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        quick_xml::de::from_str(s)
    }
}

#[test]
pub fn should_parse_association_set() {
    let mut xml_buffer: Vec<u8> = Vec::new();
    let test_data = File::open(Path::new("./test_data/association.xml")).unwrap();
    let _file_size = BufReader::new(test_data).read_to_end(&mut xml_buffer);

    match String::from_utf8(xml_buffer) {
        Ok(xml) => {
            let assoc = Association::from_str(&xml).unwrap();

            assert_eq!(assoc.name, "Assoc_VH_UnitQuantity_SalesOrderLineItem");
            assert_eq!(assoc.sap_content_version, "1");

            assert_eq!(assoc.ends.len(), 2);
            assert_eq!(assoc.ends[0].end_type, Some(String::from("GWSAMPLE_BASIC.VH_UnitQuantity")));
            assert_eq!(assoc.ends[0].multiplicity, Some(String::from("1")));
            assert_eq!(assoc.ends[0].role, "FromRole_Assoc_VH_UnitQuantity_SalesOrderLineItem");

            assert_eq!(assoc.ends[1].end_type, Some(String::from("GWSAMPLE_BASIC.SalesOrderLineItem")));
            assert_eq!(assoc.ends[1].multiplicity, Some(String::from("*")));
            assert_eq!(assoc.ends[1].role, "ToRole_Assoc_VH_UnitQuantity_SalesOrderLineItem");
        },
        Err(err) => println!("XML test data was not in UTF8 format: {}", err),
    };
}
