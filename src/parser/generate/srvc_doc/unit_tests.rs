use std::collections::BTreeSet;

use crate::{
    edmx::data_services::schema::complex_type::ComplexType,
    parser::generate::srvc_doc::complex_types::gen_complex_types,
    property::{edm_primitive::EdmPrimitive, edm_type::EdmType},
    test_utils::*,
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
    type Err = quick_xml::de::DeError;

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
            handle_test_comparison(&depth.edm_type, &EdmType::Primitive(EdmPrimitive::Decimal))?;

            let loaded_at = result.properties[5].clone();
            handle_test_comparison(&loaded_at.edm_type, &EdmType::Primitive(EdmPrimitive::DateTime))
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
    type Err = quick_xml::de::DeError;

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
            let crs: BTreeSet<String> = crate_refs.into_iter().collect();

            handle_test_comparison(&crs.len(), &2)?;
            handle_test_bool(crs.iter().find(|cr| cr.as_str().eq("rust_decimal")).is_some())?;
            handle_test_bool(crs.iter().find(|cr| cr.as_str().eq("chrono")).is_some())
        },
        Err(err) => Err(format!("XML test data was not in UTF8 format: {err}")),
    }
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
#[test]
fn should_generate_valid_ct_pallet_struct() -> Result<(), String> {
    let mut xml_buffer: Vec<u8> = Vec::new();
    let test_data = File::open(Path::new(PATH_TO_COMPLEX_TYPE_METADATA)).unwrap();
    let _file_size = BufReader::new(test_data).read_to_end(&mut xml_buffer);

    match String::from_utf8(xml_buffer) {
        Ok(xml) => {
            let ct = ComplexType::from_str(&xml).unwrap();
            let (src, _) = gen_complex_types(&[ct]);

            // The generated source must parse as valid Rust
            let module = parse_module(&src)?;
            let ct_struct = find_struct(&module, "CtPallet")?;

            // Struct must carry #[derive(...)] and #[serde(rename_all = "PascalCase")]
            handle_test_bool(ct_struct.attrs.iter().any(|a| a.path().is_ident("derive")))?;
            handle_test_bool(ct_struct.attrs.iter().any(|a| a.path().is_ident("serde")))?;

            // Properties are sorted alphabetically before emission
            let names = struct_field_names(ct_struct);
            handle_test_comparison(&names.len(), &6)?;
            handle_test_comparison(&names[0], &"depth".to_string())?;
            handle_test_comparison(&names[1], &"height".to_string())?;
            handle_test_comparison(&names[2], &"loaded_at".to_string())?;
            handle_test_comparison(&names[3], &"max_weight".to_string())?;
            handle_test_comparison(&names[4], &"shipping_id".to_string())?;
            handle_test_comparison(&names[5], &"width".to_string())?;

            // No property in the XML carries Nullable="false", so all fields must be Option<T>.
            // Each field also carries a serde attribute: decimal/datetime fields get a custom
            // deserializer, and ShippingID gets a rename (non-standard casing).
            let syn::Fields::Named(ref named) = ct_struct.fields else {
                return Err("Expected named fields on CtPallet struct".to_string());
            };

            for field in &named.named {
                handle_test_comparison(&outer_type_name(field)?, &"Option".to_string())?;
                handle_test_bool(has_serde_attr(field))?;
            }

            Ok(())
        },
        Err(err) => Err(format!("XML test data was not in UTF8 format: {err}")),
    }
}
