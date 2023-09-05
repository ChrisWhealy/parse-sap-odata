pub mod syntax_fragments;

use std::{
    env,
    fmt::Debug,
    fs::{File, OpenOptions},
    io::{BufReader, Read, Write},
    path::Path,
    str::FromStr,
};

use crate::{
    edmx::{
        data_services::schema::{complex_type::ComplexType, entity_type::EntityType},
        Edmx,
    },
    property::Property,
    utils::run_rustfmt,
};

use check_keyword::CheckKeyword;
use syntax_fragments::*;

static DEFAULT_INPUT_DIR: &str = &"./odata";

fn fetch_xml_as_string(filename: &str) -> Result<String, ParseError> {
    let mut xml_buffer: Vec<u8> = Vec::new();
    let xml_input_pathname = format!("{}/{}.xml", DEFAULT_INPUT_DIR, filename);

    let f_xml = File::open(Path::new(&xml_input_pathname))?;
    let _file_size = BufReader::new(f_xml).read_to_end(&mut xml_buffer);

    Ok(String::from_utf8(xml_buffer)?)
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
/// Deserialize an SAP OData metadata document
///
/// The metadata file must exist in the `./odata` directory and have the `.xml` extension.
/// For example:
///
/// `odata/`<br>
/// `└── gwsample_basic.xml`
fn deserialize_sap_metadata(metadata_file_name: &str) -> Result<Edmx, ParseError> {
    let xml = fetch_xml_as_string(metadata_file_name)?;
    let edmx = Edmx::from_str(&xml)?;

    return Ok(edmx);
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
/// Generate a Rust struct for a complex type
fn gen_src_complex_type(ct: &ComplexType, namespace: &str) -> Option<Vec<u8>> {
    let mut out_buffer: Vec<u8> = Vec::new();
    let trimmed_name = Property::trim_complex_type_name(&ct.name, namespace);
    let ct_name =
        convert_case::Casing::to_case(&String::from_utf8(trimmed_name).unwrap(), convert_case::Case::UpperCamel);

    // If the complex type contains only one property and the name suffix is a Rust type, then a
    // struct does not need to be generated.  This happens with SAP complex types such as
    // `CT_String` which contains the single property called `String`.
    // Such complex types are in fact not "complex" at all, and should be replaced with a single
    // native Rust type
    if ct.properties.len() > 1 && !ct_name.is_keyword() {
        let mut props = ct.properties.clone();
        props.sort();

        out_buffer.append(&mut derive_str(vec![
            DeriveTraits::CLONE,
            DeriveTraits::DEBUG,
            DeriveTraits::DEFAULT,
            DeriveTraits::SERIALIZE,
            DeriveTraits::DESERIALIZE,
        ]));
        out_buffer.append(&mut SERDE_RENAME_PASCAL_CASE.to_vec());
        out_buffer.append(&mut start_struct(&ct_name));

        for prop in props {
            out_buffer.append(&mut prop.to_rust(namespace));
        }

        // Add terminating line feed, close curly brace, then two more line feeds
        out_buffer.append(&mut end_struct());

        // Implement `from_str` for this struct
        out_buffer.append(&mut impl_from_str_for(&ct_name));
        Some(out_buffer)
    } else {
        // This is just a simple type pretending to be complex
        None
    }
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
/// Generate a Rust struct for an EntityType
fn gen_src_entity_type(entity: &EntityType, namespace: &str) -> Vec<u8> {
    let mut out_buffer: Vec<u8> = Vec::new();
    let struct_name = convert_case::Casing::to_case(
        &String::from_utf8(entity.name.clone().into_bytes()).unwrap(),
        convert_case::Case::UpperCamel,
    );
    out_buffer.append(&mut derive_str(vec![
        DeriveTraits::CLONE,
        DeriveTraits::DEBUG,
        DeriveTraits::DEFAULT,
        DeriveTraits::SERIALIZE,
        DeriveTraits::DESERIALIZE,
    ]));
    out_buffer.append(&mut SERDE_RENAME_PASCAL_CASE.to_vec());
    out_buffer.append(&mut start_struct(&struct_name));

    let mut props = entity.properties.clone();
    props.sort();

    for prop in props {
        out_buffer.append(&mut prop.to_rust(namespace));
    }

    out_buffer.append(&mut end_struct());

    // Each entity type struct implements the `EntityType` marker trait
    out_buffer.append(&mut impl_marker_trait(&struct_name));

    // Implement `from_str` for this struct
    out_buffer.append(&mut impl_from_str_for(&struct_name));
    out_buffer
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
/// Generate Rust structs and enums from the OData metadata
///
/// Any field whose name clashes with a Rust reserved word is written in raw format:<br>
/// E.G. `type --> r#type`
pub fn gen_src(metadata_file_name: &str, namespace: &str) {
    let mut out_buffer: Vec<u8> = Vec::new();

    match deserialize_sap_metadata(metadata_file_name) {
        // Deserialization can fail sometimes!
        // This can happen for example, when a quoted XML attribute value contains unescaped double quote characters
        //
        // The Atom `<feed>` documents returned from the entity sets of certain SAP OData services have been known to
        // contain `<entry>` elements whose `m:etag` attribute cntains an incorrectly quoted value
        Err(err) => println!("Error: {}", err.msg),
        Ok(edmx) => {
            let out_dir = env::var_os("OUT_DIR").unwrap();
            let odata_srv_output_pathbuf = Path::new(&out_dir).join(format!("{}.rs", metadata_file_name));

            let mut odata_srv_output_file = OpenOptions::new()
                .create(true)
                .write(true)
                .truncate(true)
                .open(&odata_srv_output_pathbuf)
                .unwrap();

            // If this fails, then either the build script been passed the wrong namespace value, or we're trying to
            // parse XML that is not an OData V2 metadata document
            if let Some(schema) = edmx.data_services.fetch_schema(namespace) {
                out_buffer.append(
                    &mut [
                        USE_SERDE,
                        LINE_FEED,
                        USE_STD_STR,
                        LINE_FEED,
                        LINE_FEED,
                        MARKER_TRAIT_ENTITY_TYPE,
                        LINE_FEED,
                        LINE_FEED,
                    ]
                    .concat(),
                );

                // - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
                // Transform ComplexType definitions if present
                if let Some(cts) = &schema.complex_types {
                    let mut ignored_cts: usize = 0;
                    out_buffer.append(&mut comment_for("COMPLEX TYPES"));

                    for (idx, ct) in cts.into_iter().enumerate() {
                        if idx > 0 && idx + ignored_cts + 1 < cts.len() {
                            out_buffer.append(&mut [SEPARATOR, LINE_FEED].concat());
                        }

                        if let Some(mut ct_src) = gen_src_complex_type(ct, namespace) {
                            out_buffer.append(&mut ct_src);
                        } else {
                            ignored_cts += 1;
                        }
                    }
                }

                // - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
                // Transform each EntityType definition into a Rust struct
                out_buffer.append(&mut comment_for("ENTITY TYPES"));

                let ets = &schema.entity_types;

                for (idx, entity) in ets.into_iter().enumerate() {
                    if idx > 0 {
                        out_buffer.append(&mut [SEPARATOR, LINE_FEED].concat());
                    }

                    out_buffer.append(&mut gen_src_entity_type(entity, namespace));
                }

                // Create enum + impl for the entity types
                out_buffer.append(&mut comment_for("ENTITY TYPES ENUM - Is this enum is really needed...?"));
                out_buffer.append(&mut schema.to_entity_types_enum());

                // Create enum + impl for the entity container
                // This enum acts as a proxy for the service document
                if let Some(ent_cont) = &schema.entity_container {
                    out_buffer.append(&mut comment_for("ENTITY SETS ENUM"));
                    out_buffer.append(&mut ent_cont.to_enum_with_impl());
                }

                // - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
                // TODO Generate function imports
                // - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -

                // Syntax check and format the generated code
                match run_rustfmt(&out_buffer, &metadata_file_name) {
                    Ok(formatted_bytes) => {
                        odata_srv_output_file.write_all(&formatted_bytes).unwrap();

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
