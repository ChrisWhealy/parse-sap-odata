use std::{
    fs::File,
    io::{BufReader, Read},
    path::Path,
    str::FromStr,
};

use crate::{test_utils::*, xml::default_xml_namespace_atom};
use super::AtomLink;

impl std::str::FromStr for AtomLink {
    type Err = quick_xml::DeError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        quick_xml::de::from_str(s)
    }
}

#[test]
pub fn should_parse_atom_link() -> Result<(), String> {
    let test_data = File::open(Path::new("./test_data/atom_link.xml")).unwrap();
    let mut xml_buffer: Vec<u8> = Vec::new();
    let _file_size = BufReader::new(test_data).read_to_end(&mut xml_buffer);

    match String::from_utf8(xml_buffer) {
        Ok(xml) => {
            let base_url =
                &"https://SAPES5.SAPDEVCENTER.COM:443/sap/opu/odata/iwbep/GWSAMPLE_BASIC/$metadata".to_string();
            let atom_link = AtomLink::from_str(&xml).unwrap();
            let expected_ns = default_xml_namespace_atom().unwrap();

            handle_test_comparison_opt(&atom_link.xml_namespace_atom, &Some(expected_ns))?;
            handle_test_comparison(&atom_link.rel, &"latest-version".to_string())?;
            handle_test_comparison(&atom_link.href, base_url)?;

            Ok(())
        },

        Err(err) => Err(format!("XML test data was not in UTF8 format: {}", err)),
    }
}
