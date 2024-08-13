use std::{
    fs::File,
    io::{BufReader, Read},
    path::Path,
    str::FromStr
};

use crate::{
    edmx::data_services::schema::association::end::End,
    test_utils::*
};

impl FromStr for End {
    type Err = quick_xml::DeError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        quick_xml::de::from_str(s)
    }
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
#[test]
pub fn should_parse_end() -> Result<(), String> {
    let mut xml_buffer: Vec<u8> = Vec::new();
    let test_data = File::open(Path::new("./test_data/end.xml")).unwrap();
    let _file_size = BufReader::new(test_data).read_to_end(&mut xml_buffer);

    let xml = match String::from_utf8(xml_buffer) {
        Ok(xml) => xml,
        Err(err) => return Err(format!("XML test data was not in UTF8 format: {}", err)),
    };

    let end = End::from_str(&xml).unwrap();

    handle_test_comparison_opt(&end.end_type, &Some("GWSAMPLE_BASIC.VH_UnitQuantity".to_string()))?;
    handle_test_comparison_opt(&end.multiplicity, &Some("1".to_string()))?;
    handle_test_comparison(&end.role, &"FromRole_Assoc_VH_UnitQuantity_SalesOrderLineItem".to_string())?;

    Ok(())
}
