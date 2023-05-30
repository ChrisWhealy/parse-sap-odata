use std::fs::{File, OpenOptions};
use std::io::{BufReader, Read, Write};
use std::str::FromStr;

use check_keyword::CheckKeyword;

use crate::edmx::Edmx;
use crate::property::Property;
use crate::utils::parse_error::ParseError;
use crate::utils::run_rustfmt;

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
// Deserialize a given metadata document
// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
pub fn deserialize_sap_metadata(metadata_file_name: &str) -> Result<Edmx, ParseError> {
    let mut xml_buffer: Vec<u8> = Vec::new();
    let xml_input_path = format!("./tests/{}.xml", metadata_file_name);

    let f_xml = File::open(&xml_input_path)?;
    let _file_size = BufReader::new(f_xml).read_to_end(&mut xml_buffer);
    let xml = String::from_utf8(xml_buffer)?;
    let edmx = Edmx::from_str(&xml)?;

    return Ok(edmx);
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
// Generate Rust structs from the OData metadata
//
// Any field whose name clashes with a Rust reserved word is written in raw format: E.G. "type" --> "r#type"
// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
pub fn gen_src(metadata_file_name: &str, namespace: &str) {
    let mut out_buffer: Vec<u8> = Vec::new();

    match deserialize_sap_metadata(metadata_file_name) {
        Err(err) => println!("Error: {}", err.msg),
        Ok(edmx) => {
            let output_file_name = format!("./gen/{}.rs", metadata_file_name);
            let mut out_file = OpenOptions::new()
                .create(true)
                .write(true)
                .truncate(true)
                .open(&output_file_name)
                .unwrap();

            // I can haz namespace?
            if let Some(schema) = edmx.data_services.fetch_schema(namespace) {
                // - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
                // If present, transform ComplexType definitions to Rust structs
                // - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
                if let Some(cts) = &schema.complex_types {
                    for ct in cts {
                        let ct_name = convert_case::Casing::to_case(
                            &Property::trim_complex_type_name(ct.name.as_ref(), namespace),
                            convert_case::Case::Pascal,
                        );

                        // If the complex type contains only one property and the name suffix is a Rust type, then a
                        // struct does not need to be generated
                        // This happens with complex types such as `CT_String` which only contain a single property
                        // called `String`
                        if ct.properties.len() > 1 && !ct_name.is_keyword() {
                            out_buffer.append(&mut "#[derive(Debug)]".as_bytes().to_vec());
                            out_buffer.append(
                                &mut format!("pub struct {} {{", ct_name).as_bytes().to_vec(),
                            );

                            for prop in &ct.properties {
                                out_buffer.append(&mut prop.to_rust(namespace));
                            }

                            // Add terminating line feed, close curly brace, then two more line feeds
                            out_buffer.append(&mut vec![10, 125, 10, 10]);
                        }
                    }
                }

                // - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
                // Transform each EntityType definition to a Rust struct
                // - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
                for entity in &schema.entity_types {
                    out_buffer.append(&mut "#[derive(Debug)]".as_bytes().to_vec());
                    out_buffer
                        .append(&mut format!("pub struct {} {{", entity.name).as_bytes().to_vec());

                    for prop in &entity.properties {
                        out_buffer.append(&mut prop.to_rust(namespace));
                    }

                    // Add terminating line feed, close curly brace, then two more line feeds
                    out_buffer.append(&mut vec![10, 125, 10, 10]);
                }

                // - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
                // TODO Generate function imports before writing output
                // - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
                match run_rustfmt(&out_buffer) {
                    Ok(formatted_bytes) => out_file.write_all(&formatted_bytes).unwrap(),
                    Err(err) => println!("Error: rustfmt ended with {}", err.to_string()),
                }
            } else {
                println!("Namespace {} not found in schema", namespace);
            };
        }
    };

    out_buffer.clear();
}
