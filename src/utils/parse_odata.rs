use std::{
    env,
    fs::{File, OpenOptions},
    io::{BufReader, Read, Write},
    path::Path,
    str::FromStr,
};

use check_keyword::CheckKeyword;

use crate::{edmx::Edmx, property::Property, utils::parse_error::ParseError, utils::run_rustfmt};

static DEFAULT_INPUT_DIR: &str = &"./odata";

static LINE_FEED: &[u8] = &[0x0a];
static SPACE: &[u8] = &[0x20];

static DERIVE_CLONE_DEBUG: &[u8] = &"#[derive(Clone, Debug, Default)]".as_bytes();
static START_PUB_STRUCT: &[u8] = &"pub struct ".as_bytes();
static OPEN_CURLY: &[u8] = &"{".as_bytes();
static CLOSE_CURLY: &[u8] = &"}".as_bytes();

fn start_struct(struct_name: String, derive: &[u8]) -> Vec<u8> {
    [
        LINE_FEED,
        &derive,
        LINE_FEED,
        START_PUB_STRUCT,
        SPACE,
        struct_name.as_bytes(),
        OPEN_CURLY,
    ]
    .concat()
}
fn end_struct() -> Vec<u8> {
    [LINE_FEED, CLOSE_CURLY, LINE_FEED, LINE_FEED].concat()
}

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

            // I can haz namespace?
            if let Some(schema) = edmx.data_services.fetch_schema(namespace) {
                // - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
                // If present, transform ComplexType definitions to Rust structs
                // - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
                if let Some(cts) = &schema.complex_types {
                    for ct in cts {
                        let trimmed_name = Property::trim_complex_type_name(&ct.name, namespace);
                        let ct_name = convert_case::Casing::to_case(
                            &String::from_utf8(trimmed_name).unwrap(),
                            convert_case::Case::UpperCamel,
                        );

                        // If the complex type contains only one property and the name suffix is a Rust type, then a
                        // struct does not need to be generated.  This happens with SAP complex types such as
                        // `CT_String` which only contain a single property called `String`.  Such "complex" types are
                        // collapsed down to a single native Rust type
                        if ct.properties.len() > 1 && !ct_name.is_keyword() {
                            let mut props = ct.properties.clone();
                            props.sort();

                            out_buffer.append(&mut start_struct(ct_name, DERIVE_CLONE_DEBUG));

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
                // - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
                for entity in &schema.entity_types {
                    let struct_name = convert_case::Casing::to_case(
                        &String::from_utf8(entity.name.clone().into_bytes()).unwrap(),
                        convert_case::Case::UpperCamel,
                    );
                    out_buffer.append(&mut start_struct(struct_name, DERIVE_CLONE_DEBUG));

                    let mut props = entity.properties.clone();
                    props.sort();

                    for prop in props {
                        out_buffer.append(&mut prop.to_rust(namespace));
                    }

                    out_buffer.append(&mut end_struct());
                }

                // - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
                // TODO Generate function imports
                // - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -

                // Syntax check and format the generated code
                match run_rustfmt(&out_buffer, &metadata_file_name) {
                    Ok(formatted_bytes) => {
                        out_file.write_all(&formatted_bytes).unwrap();

                        // Tell cargo to watch the input file
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
