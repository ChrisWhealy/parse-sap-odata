use std::{env, fs::OpenOptions, io::Write, path::Path, str::FromStr};

use crate::{edmx::Edmx, parser::error::ParseError};

pub static DEFAULT_INPUT_DIR: &str = "./odata";

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
fn fetch_xml_as_string(filename: &str) -> Result<String, ParseError> {
    let xml_input_pathname = format!("{}/{}.xml", DEFAULT_INPUT_DIR, filename);
    Ok(std::fs::read_to_string(&xml_input_pathname)?)
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
/// Write buffer to $OUT_DIR
pub fn write_buffer_to_file(filename: &str, buf: Vec<u8>) -> Result<(), ParseError> {
    let out_dir = env::var("OUT_DIR").map_err(|e| ParseError { msg: e.to_string() })?;
    let mut output_file = OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open(Path::new(&out_dir).join(filename))?;

    output_file.write_all(&buf)?;
    Ok(())
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
/// Deserialize an SAP OData metadata document
///
/// The metadata file must exist in the root level project directory `./odata` and have the `.xml` extension.
/// For example:
///
/// `odata/`<br>
/// `└── gwsample_basic.xml`
pub fn deserialize_sap_metadata(metadata_file_name: &str) -> Result<Edmx, ParseError> {
    let xml = fetch_xml_as_string(metadata_file_name)?;
    let edmx = Edmx::from_str(&xml)?;

    Ok(edmx)
}
