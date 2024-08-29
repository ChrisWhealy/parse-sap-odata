use crate::{
    edmx::data_services::schema::complex_type::ComplexType,
    parser::generate::srvc_doc::complex_types::gen_complex_types, property::metadata::PropertyType,
    test_utils::{handle_test_comparison, handle_test_bool},
    utils::dedup_vec_of_string,
};

use chrono;
use rust_decimal;
use serde::Deserialize;
use std::{
    fs::File,
    io::{BufReader, Read},
    path::Path,
    str::FromStr,
};

static PATH_TO_COMPLEX_TYPE_METADATA: &str = "./test_data/complex_type_metadata.xml";
static PATH_TO_COMPLEX_TYPE: &str = "./test_data/complex_type_pallet.xml";

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
impl FromStr for ComplexType {
    type Err = quick_xml::DeError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        quick_xml::de::from_str(s)
    }
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
#[test]
fn should_deserialize_ct_pallet_metadata() -> Result<(), String> {
    let mut xml_buffer: Vec<u8> = Vec::new();
    let test_data = File::open(Path::new(PATH_TO_COMPLEX_TYPE_METADATA)).unwrap();
    let _file_size = BufReader::new(test_data).read_to_end(&mut xml_buffer);

    match String::from_utf8(xml_buffer) {
        Ok(xml) => {
            let result = ComplexType::from_str(&xml).unwrap();
            handle_test_comparison(&result.name, &"CT_Pallet".to_string())?;
            handle_test_comparison(&result.properties.len(), &6)?;

            let depth = result.properties[0].clone();
            handle_test_comparison(&depth.precision.unwrap(), &12)?;
            handle_test_comparison(&depth.scale.unwrap(), &2)?;
            handle_test_comparison(
                &depth.get_property_type(),
                &PropertyType::Edm("Decimal".to_string(), "rust_decimal".to_string()),
            )?;

            let loaded_at = result.properties[5].clone();
            handle_test_comparison(
                &loaded_at.get_property_type(),
                &PropertyType::Edm("DateTime".to_string(), "chrono".to_string()),
            )
        },
        Err(err) => Err(format!("XML test data was not in UTF8 format: {err}")),
    }
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
#[derive(Clone, Debug, Default, Deserialize)]
#[serde(rename_all = "PascalCase")]
struct CtPallet {
    #[serde(deserialize_with = "parse_sap_atom_feed::deserializers::edm_decimal::to_rust_decimal_2dp")]
    pub max_weight: rust_decimal::Decimal,
    #[serde(deserialize_with = "parse_sap_atom_feed::deserializers::edm_decimal::to_rust_decimal_3dp")]
    pub width: rust_decimal::Decimal,
    #[serde(deserialize_with = "parse_sap_atom_feed::deserializers::edm_decimal::to_rust_decimal_3dp")]
    pub depth: rust_decimal::Decimal,
    #[serde(deserialize_with = "parse_sap_atom_feed::deserializers::edm_decimal::to_rust_decimal_3dp")]
    pub height: rust_decimal::Decimal,
    #[serde(rename = "ShippingID")]
    pub shipping_id: Option<String>,
    #[serde(deserialize_with = "parse_sap_atom_feed::deserializers::edm_datetime::to_naive_date_time")]
    pub loaded_at: chrono::NaiveDateTime,
}

impl FromStr for CtPallet {
    type Err = quick_xml::DeError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        quick_xml::de::from_str(s)
    }
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
#[test]
fn should_deserialize_ct_pallet() -> Result<(), String> {
    let mut xml_buffer: Vec<u8> = Vec::new();
    let test_data = File::open(Path::new(PATH_TO_COMPLEX_TYPE)).unwrap();
    let _file_size = BufReader::new(test_data).read_to_end(&mut xml_buffer);

    match String::from_utf8(xml_buffer) {
        Ok(xml) => {
            let result = CtPallet::from_str(&xml).unwrap();

            handle_test_comparison(&result.max_weight.to_string(), &"215.75".to_string())?;
            handle_test_comparison(&result.width.to_string(), &"1.750".to_string())?;
            handle_test_comparison(&result.depth.to_string(), &"1.750".to_string())?;
            handle_test_comparison(&result.height.to_string(), &"2.250".to_string())?;
            handle_test_comparison(&result.shipping_id.unwrap(), &"QS-23-VRT1".to_string())?;
            handle_test_comparison(
                &result.loaded_at.to_string(),
                &chrono::NaiveDateTime::from_str("2024-08-28T12:41:50.0000000")
                    .unwrap()
                    .to_string(),
            )
        },
        Err(err) => Err(format!("XML test data was not in UTF8 format: {err}")),
    }
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
#[test]
fn should_generate_extern_crate_refs() -> Result<(), String> {
    let mut xml_buffer: Vec<u8> = Vec::new();
    let test_data = File::open(Path::new(PATH_TO_COMPLEX_TYPE_METADATA)).unwrap();
    let _file_size = BufReader::new(test_data).read_to_end(&mut xml_buffer);

    match String::from_utf8(xml_buffer) {
        Ok(xml) => {
            let result = ComplexType::from_str(&xml).unwrap();
            let (_src_code, crate_refs) = gen_complex_types(&vec![result]);
            let crs = dedup_vec_of_string(crate_refs);

            handle_test_comparison(&crs.len(), &2)?;
            handle_test_bool(crs.iter().find(|cr| cr.as_str().eq("rust_decimal")).is_some())?;
            handle_test_bool(crs.iter().find(|cr| cr.as_str().eq("chrono")).is_some())
        },
        Err(err) => Err(format!("XML test data was not in UTF8 format: {err}")),
    }
}
