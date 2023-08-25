pub mod syntax_fragments;

use std::{
    env,
    fmt::Debug,
    fs::{File, OpenOptions},
    io::{BufReader, Read, Write},
    path::Path,
    str::FromStr,
};

use crate::{edmx::Edmx, property::Property, utils::run_rustfmt};

use check_keyword::CheckKeyword;
use syntax_fragments::*;

fn start_struct(struct_name: String) -> Vec<u8> {
    [START_PUB_STRUCT, SPACE, struct_name.as_bytes(), OPEN_CURLY].concat()
}
fn end_struct() -> Vec<u8> {
    [LINE_FEED, CLOSE_CURLY, LINE_FEED, LINE_FEED].concat()
}

static DEFAULT_INPUT_DIR: &str = &"./odata";

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
// Deserialize a given metadata document
// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
pub fn deserialize_sap_metadata(metadata_file_name: &str) -> Result<Edmx, ParseError> {
    let mut xml_buffer: Vec<u8> = Vec::new();
    let xml_input_pathname = format!("{}/{}.xml", DEFAULT_INPUT_DIR, metadata_file_name);

    let f_xml = File::open(Path::new(&xml_input_pathname))?;
    let _file_size = BufReader::new(f_xml).read_to_end(&mut xml_buffer);
    let xml = String::from_utf8(xml_buffer)?;
    let edmx = Edmx::from_str(&xml)?;

    return Ok(edmx);
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
// Generate Rust structs from the OData metadata
//
// Any field whose name clashes with a Rust reserved word is written in raw format: E.G. "type" --> "r#type"
// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
pub fn gen_src(metadata_file_name: &str, namespace: &str) {
    let mut out_buffer: Vec<u8> = Vec::new();

    match deserialize_sap_metadata(metadata_file_name) {
        // Deserialization can fail sometimes.
        // This can happen for example, when a quoted XML attribute value contains unescaped double quote characters
        Err(err) => println!("Error: {}", err.msg),
        Ok(edmx) => {
            let out_dir = env::var_os("OUT_DIR").unwrap();
            let output_path = Path::new(&out_dir).join(format!("{}.rs", metadata_file_name));

            let mut out_file = OpenOptions::new()
                .create(true)
                .write(true)
                .truncate(true)
                .open(&output_path)
                .unwrap();

            // If this fails, then either the build script is being run with the wrong value for the namespace, or we're
            // trying to parse from XML that is not valid OData metadata
            if let Some(schema) = edmx.data_services.fetch_schema(namespace) {
                out_buffer.append(&mut [USE_SERDE, LINE_FEED, LINE_FEED].concat());

                // - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
                // Transform ComplexType definitions if present
                if let Some(cts) = &schema.complex_types {
                    for ct in cts {
                        let trimmed_name = Property::trim_complex_type_name(&ct.name, namespace);
                        let ct_name = convert_case::Casing::to_case(
                            &String::from_utf8(trimmed_name).unwrap(),
                            convert_case::Case::UpperCamel,
                        );

                        // If the complex type contains only one property and the name suffix is a Rust type, then a
                        // struct does not need to be generated.  This happens with SAP complex types such as
                        // `CT_String` which contains the single property called `String`.
                        // Such complex types are in fact not "complex" at all, and should be replaced with a single
                        // native Rust type
                        if ct.properties.len() > 1 && !ct_name.is_keyword() {
                            let mut props = ct.properties.clone();
                            props.sort();

                            out_buffer.append(&mut derive_str(vec![
                                DeriveDirectives::CLONE,
                                DeriveDirectives::DEBUG,
                                DeriveDirectives::DEFAULT,
                                DeriveDirectives::SERIALIZE,
                                DeriveDirectives::DESERIALIZE,
                            ]));
                            out_buffer.append(&mut SERDE_RENAME_PASCAL_CASE.to_vec());
                            out_buffer.append(&mut start_struct(ct_name));

                            for prop in props {
                                out_buffer.append(&mut prop.to_rust(namespace));
                            }

                            // Add terminating line feed, close curly brace, then two more line feeds
                            out_buffer.append(&mut end_struct());
                        }
                    }
                }

                // - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
                // Transform each EntityType definition to a Rust struct
                //
                // It is possible for an OData service to have zero EntityTypes; in which case, there will also be zero
                // EntitySets. This in turns means that interaction with the OData service is only possible through the
                // FunctionImports
                for entity in &schema.entity_types {
                    let struct_name = convert_case::Casing::to_case(
                        &String::from_utf8(entity.name.clone().into_bytes()).unwrap(),
                        convert_case::Case::UpperCamel,
                    );
                    out_buffer.append(&mut derive_str(vec![
                        DeriveDirectives::CLONE,
                        DeriveDirectives::DEBUG,
                        DeriveDirectives::DEFAULT,
                        DeriveDirectives::SERIALIZE,
                        DeriveDirectives::DESERIALIZE,
                    ]));
                    out_buffer.append(&mut SERDE_RENAME_PASCAL_CASE.to_vec());
                    out_buffer.append(&mut start_struct(struct_name));

                    let mut props = entity.properties.clone();
                    props.sort();

                    for prop in props {
                        out_buffer.append(&mut prop.to_rust(namespace));
                    }

                    out_buffer.append(&mut end_struct());
                }

                // Create enum + impl for the entity container
                // The values in this enum act as a proxy for the service document
                if let Some(ent_cont) = &schema.entity_container {
                    out_buffer.append(&mut ent_cont.to_enum_with_impl());
                }

                // - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
                // TODO Generate function imports
                // - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -

                // Syntax check and format the generated code
                match run_rustfmt(&out_buffer, &metadata_file_name) {
                    Ok(formatted_bytes) => {
                        out_file.write_all(&formatted_bytes).unwrap();

                        // Tell cargo to watch the input XML file
                        println!(
                            "cargo:rerun-if-changed={}",
                            format!("{}/{}.xml", DEFAULT_INPUT_DIR, metadata_file_name)
                        );
                    },
                    Err(err) => println!("Error: rustfmt ended with {}", err.to_string()),
                }
            } else {
                println!("Required namespace '{}' not found in schema", namespace);
            };
        },
    };

    out_buffer.clear();
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
// Handle errors
// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
#[derive(Debug)]
pub struct ParseError {
    pub msg: String,
}

impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.msg)
    }
}

impl std::error::Error for ParseError {
    fn description(&self) -> &str {
        &self.msg
    }
}

impl From<std::io::Error> for ParseError {
    fn from(io_err: std::io::Error) -> ParseError {
        ParseError { msg: io_err.to_string() }
    }
}

impl From<std::string::FromUtf8Error> for ParseError {
    fn from(utf8_err: std::string::FromUtf8Error) -> ParseError {
        ParseError { msg: utf8_err.to_string() }
    }
}

impl From<std::io::ErrorKind> for ParseError {
    fn from(io_err: std::io::ErrorKind) -> ParseError {
        ParseError { msg: io_err.to_string() }
    }
}

impl From<quick_xml::DeError> for ParseError {
    fn from(xml_err: quick_xml::DeError) -> ParseError {
        ParseError { msg: xml_err.to_string() }
    }
}
