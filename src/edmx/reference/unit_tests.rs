use std::{
    fs::File,
    io::{BufReader, Read},
    path::Path,
    str::FromStr,
};

use super::Reference;
use crate::{test_utils::*, xml::default_xml_namespace_oasis};

impl FromStr for Reference {
    type Err = quick_xml::DeError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        quick_xml::de::from_str(s)
    }
}

#[test]
pub fn should_parse_edmx_reference() -> Result<(), String> {
    let mut xml_buffer: Vec<u8> = Vec::new();
    let test_data = File::open(Path::new("./test_data/edmx_reference.xml")).unwrap();
    let _file_size = BufReader::new(test_data).read_to_end(&mut xml_buffer);

    match String::from_utf8(xml_buffer) {
        Ok(xml) => {
            let expected_uri = "https://sap/opu/odata/IWFND/CATALOGSERVICE;v=2/Vocabularies(TechnicalName='%2FIWBEP%2FVOC_COMMON',Version='0001',SAP__Origin='LOCAL')/$value".to_string();
            let edmx_ref = Reference::from_str(&xml).unwrap();

            handle_test_comparison_opt(&edmx_ref.uri, &Some(expected_uri))?;
            handle_test_comparison(&edmx_ref.xml_namespace_edmx, &default_xml_namespace_oasis())?;

            if let Some(include) = edmx_ref.include {
                handle_test_comparison(&include.alias, &"Common".to_string())?;
                handle_test_comparison(&include.namespace, &"com.sap.vocabularies.Common.v1".to_string())?;
            }

            Ok(())
        },
        Err(err) => Err(format!("XML test data was not in UTF8 format: {}", err)),
    }
}
