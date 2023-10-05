use crate::parser::error::ParseError;

use std::{
    env,
    fs::{File, OpenOptions},
    io::{BufReader, Read, Write},
    path::Path,
    str::FromStr,
};

use crate::edmx::Edmx;

pub static DEFAULT_INPUT_DIR: &str = "./odata";

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
pub fn fetch_xml_as_string(filename: &str) -> Result<String, ParseError> {
    let mut xml_buffer: Vec<u8> = Vec::new();
    let xml_input_pathname = format!("{}/{}.xml", DEFAULT_INPUT_DIR, filename);

    let f_xml = File::open(Path::new(&xml_input_pathname))?;
    let _file_size = BufReader::new(f_xml).read_to_end(&mut xml_buffer);

    Ok(String::from_utf8(xml_buffer)?)
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
/// Write buffer to $OUT_DIR
pub fn write_buffer_to_file(filename: &str, buf: Vec<u8>) {
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
pub fn deserialize_sap_metadata(metadata_file_name: &str) -> Result<Edmx, ParseError> {
    let xml = fetch_xml_as_string(metadata_file_name)?;
    let edmx = Edmx::from_str(&xml)?;

    return Ok(edmx);
}
