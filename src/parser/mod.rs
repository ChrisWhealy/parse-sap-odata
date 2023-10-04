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
        data_services::schema::{complex_type::ComplexType, entity_type::EntityType, Schema},
        Edmx,
    },
    property::Property,
    utils::run_rustfmt,
};

use check_keyword::CheckKeyword;
use syntax_fragments::*;

static DEFAULT_INPUT_DIR: &str = "./odata";

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
fn fetch_xml_as_string(filename: &str) -> Result<String, ParseError> {
    let mut xml_buffer: Vec<u8> = Vec::new();
    let xml_input_pathname = format!("{}/{}.xml", DEFAULT_INPUT_DIR, filename);

    let f_xml = File::open(Path::new(&xml_input_pathname))?;
    let _file_size = BufReader::new(f_xml).read_to_end(&mut xml_buffer);

    Ok(String::from_utf8(xml_buffer)?)
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
/// Write generated buffer to $OUT_DIR
fn write_buffer_to_file(filename: &str, buf: Vec<u8>) {
    let out_dir = env::var_os("OUT_DIR").unwrap();
    let mut output_file = OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open(Path::new(&out_dir).join(filename))
        .unwrap();

    output_file.write_all(&buf).unwrap();
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
/// Generate complex type structs
fn gen_complex_types(out_buffer: &mut Vec<u8>, cts: &Vec<ComplexType>, namespace: &str) {
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

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
/// Generate entity type structs
fn gen_entity_types(out_buffer: &mut Vec<u8>, ets: &Vec<EntityType>, namespace: &str) {
    out_buffer.append(&mut comment_for("ENTITY TYPES"));

    for (idx, entity) in ets.into_iter().enumerate() {
        if idx > 0 {
            out_buffer.append(&mut [SEPARATOR, LINE_FEED].concat());
        }

        out_buffer.append(&mut gen_src_entity_type(entity, namespace));
    }
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
/// Generate a Rust struct for a complex type
fn gen_src_complex_type(ct: &ComplexType, namespace: &str) -> Option<Vec<u8>> {
    let mut out_buffer: Vec<u8> = Vec::new();
    let trimmed_name = Property::trim_complex_type_name(&ct.name, namespace);
    let ct_name = convert_case::Casing::to_case(
        &String::from_utf8(trimmed_name.to_vec()).unwrap(),
        convert_case::Case::UpperCamel,
    );

    // If the complex type contains only one property and the name suffix is a Rust type, then it is unneccesary to
    // create a Rust struct as this type can be replaced with a single native Rust type.
    // This happens with SAP complex types such as `CT_String` which just contains a single property called `String`.
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
        out_buffer.append(&mut [SERDE_RENAME_PASCAL_CASE, LINE_FEED].concat().to_vec());
        out_buffer.append(&mut start_struct(&ct_name));

        for mut prop in props {
            out_buffer.append(&mut prop.to_rust(namespace));
        }

        out_buffer.append(&mut end_struct());

        // Implement `from_str` for this struct
        out_buffer.append(&mut impl_from_str_for(&ct_name));
        Some(out_buffer)
    } else {
        // This is just a simple type pretending to have a complex
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
    out_buffer.append(&mut [SERDE_RENAME_PASCAL_CASE, LINE_FEED].concat().to_vec());
    out_buffer.append(&mut start_struct(&struct_name));

    let mut props = entity.properties.clone();
    props.sort();

    for mut prop in props {
        out_buffer.append(&mut prop.to_rust(namespace));
    }

    out_buffer.append(&mut end_struct());

    // Each entity type struct implements the `EntityType` marker trait
    out_buffer.append(&mut impl_marker_trait("EntityType", &struct_name));

    // Implement `from_str` for this struct
    out_buffer.append(&mut impl_from_str_for(&struct_name));
    out_buffer
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
/// Generate a module containing the structs and enums for the service document
fn gen_srv_doc_module(odata_srv_name: &str, namespace: &str, schema: &Schema) -> Vec<u8> {
    let mut out_buffer: Vec<u8> = Vec::new();

    // Start module definition
    out_buffer.append(&mut gen_mod_start(odata_srv_name));
    out_buffer.append(&mut gen_use_serde());
    out_buffer.append(&mut gen_marker_trait_for("EntityType"));

    if let Some(cts) = &schema.complex_types {
        gen_complex_types(&mut out_buffer, cts, namespace);
    }

    gen_entity_types(&mut out_buffer, &schema.entity_types, namespace);

    // Create enum + impl for the entity container
    // This enum acts as a proxy for the service document
    if let Some(ent_cont) = &schema.entity_container {
        out_buffer.append(&mut comment_for("ENTITY SETS ENUM"));
        out_buffer.append(&mut ent_cont.to_enum_with_impl());
    }

    // Close module definition
    out_buffer.append(&mut [CLOSE_CURLY, LINE_FEED].concat());

    out_buffer
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
/// Generate a module containing the metadata objects and their respective instances for this OData service
fn gen_metadata_module(odata_srv_name: &str, _namespace: &str, _schema: &Schema) -> Vec<u8> {
    let mut out_buffer: Vec<u8> = Vec::new();

    // Start module definition
    out_buffer.append(&mut gen_mod_start(&format!("{}_metadata", odata_srv_name)));

    // Close module definition
    out_buffer.append(&mut [CLOSE_CURLY, LINE_FEED].concat());

    out_buffer
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
/// Generate the service document and metadata modules
pub fn gen_src(odata_srv_name: &str, namespace: &str) {
    match deserialize_sap_metadata(odata_srv_name) {
        // Deserialization can fail sometimes!
        // This can happen for example, when a quoted XML attribute value contains an unescaped double quote character
        //
        // The Atom `<feed>` document returned from the entity sets of certain SAP OData services have been known to
        // contain `<entry>` elements whose `m:etag` attribute contains such an incorrectly quoted value
        Err(err) => println!("Error: {}", err.msg),
        Ok(edmx) => {
            // If this "if let" fails, then either the build script been passed the wrong namespace value, or we're
            // trying to parse XML that is not an OData V2 metadata document
            if let Some(schema) = edmx.data_services.fetch_schema(namespace) {
                // Generate the source code for the service document module and run it through rustfmt
                match run_rustfmt(&gen_srv_doc_module(odata_srv_name, namespace, &schema), &odata_srv_name) {
                    Ok(formatted_bytes) => {
                        write_buffer_to_file(&format!("{}.rs", odata_srv_name), formatted_bytes);

                        // Tell cargo to watch the input XML file
                        println!(
                            "cargo:rerun-if-changed={}",
                            format!("{}/{}.xml", DEFAULT_INPUT_DIR, odata_srv_name)
                        );
                    },
                    Err(err) => println!("Error: rustfmt for service document module ended with {}", err.to_string()),
                }

                // Now generate the source code for the metadata document module and run that through rustfmt too
                match run_rustfmt(&gen_metadata_module(odata_srv_name, namespace, &schema), &odata_srv_name) {
                    Ok(formatted_bytes) => {
                        write_buffer_to_file(&format!("{}_metadata.rs", odata_srv_name), formatted_bytes)
                    },
                    Err(err) => println!("Error: rustfmt for metadata document module ended with {}", err.to_string()),
                }
            } else {
                println!("OData schema for namespace '{}' cannot be found", namespace);
            }
        },
    }
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
