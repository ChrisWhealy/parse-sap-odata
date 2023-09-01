use std::{
    fs::File,
    io::{BufReader, Read},
    path::Path,
    str::FromStr,
};

use super::AtomLink;

impl std::str::FromStr for AtomLink {
    type Err = quick_xml::DeError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        quick_xml::de::from_str(s)
    }
}

#[test]
pub fn should_parse_atom_link() {
    let mut xml_buffer: Vec<u8> = Vec::new();
    let test_data = File::open(Path::new("./test_data/atom_link.xml")).unwrap();
    let _file_size = BufReader::new(test_data).read_to_end(&mut xml_buffer);

    match String::from_utf8(xml_buffer) {
        Ok(xml) => {
            let atom_link = AtomLink::from_str(&xml).unwrap();

            assert_eq!(atom_link.xml_namespace_atom, Some(String::from("http://www.w3.org/2005/Atom")));
            assert_eq!(atom_link.rel, "latest-version");
            assert_eq!(
                atom_link.href,
                "https://SAPES5.SAPDEVCENTER.COM:443/sap/opu/odata/iwbep/GWSAMPLE_BASIC/$metadata"
            );
        },
        Err(err) => println!("XML test data was not in UTF8 format: {}", err),
    };
}
