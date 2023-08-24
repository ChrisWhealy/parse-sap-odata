use std::{
    fs::File,
    io::{BufReader, Read},
    path::Path,
    str::FromStr,
};

use super::Reference;

impl std::str::FromStr for Reference {
    type Err = quick_xml::DeError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        quick_xml::de::from_str(s)
    }
}

#[test]
pub fn should_parse_edmx_reference() {
    let mut xml_buffer: Vec<u8> = Vec::new();
    let test_data = File::open(Path::new("./test_data/edmx_reference.xml")).unwrap();
    let _file_size = BufReader::new(test_data).read_to_end(&mut xml_buffer);

    match String::from_utf8(xml_buffer) {
        Ok(xml) => {
            let edmx_ref = Reference::from_str(&xml).unwrap();

            assert_eq!(edmx_ref.uri,
              Some(
                String::from("https://sap/opu/odata/IWFND/CATALOGSERVICE;v=2/Vocabularies(TechnicalName='%2FIWBEP%2FVOC_COMMON',Version='0001',SAP__Origin='LOCAL')/$value")
              )
            );
            assert_eq!(edmx_ref.xml_namespace_edmx, "http://docs.oasis-open.org/odata/ns/edmx");

            if let Some(include) = edmx_ref.include {
                assert_eq!(include.alias, "Common");
                assert_eq!(include.namespace, "com.sap.vocabularies.Common.v1");
            }
        },
        Err(err) => println!("XML test data was not in UTF8 format: {}", err),
    };
}
