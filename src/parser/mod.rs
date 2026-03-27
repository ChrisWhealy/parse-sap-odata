pub mod generate;

mod io;

use crate::utils::rust_tools::run_rustfmt;
use generate::{metadata_doc::*, srvc_doc::*, syntax_fragments::SUFFIX_SNAKE_METADATA};
use io::*;

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
pub trait AsRustSrc {
    type CrateRef;
    fn to_rust(&self) -> (Vec<u8>, Self::CrateRef);
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
/// Format `buf` with rustfmt and write the result to `mod_name` in `$OUT_DIR`
fn emit_module(mod_name: &str, buf: &Vec<u8>) {
    match run_rustfmt(buf, mod_name) {
        Ok(formatted_bytes) => {
            if let Err(err) = write_buffer_to_file(mod_name, formatted_bytes) {
                println!("Error: writing module '{}' failed: {}", mod_name, err);
            }
        },
        Err(err) => println!("Error: rustfmt for module '{}' ended with {}", mod_name, err),
    }
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
/// Generate the service document and metadata modules
pub fn gen_src(odata_srv_name: &str, namespace: &str) {
    match deserialize_sap_metadata(odata_srv_name) {
        // Deserialization can fail sometimes!
        // This can happen for example, when a quoted XML attribute value contains an unescaped double quote character
        //
        // The Atom `<feed>` document returned from the entity sets of certain SAP OData services has been known to
        // contain `<entry>` elements whose `m:etag` attribute contains such an incorrectly quoted value
        Err(err) => println!("Error: {}", err),
        Ok(edmx) => {
            // Write cargo build script directive as soon as the input path is known to be valid
            println!(
                "cargo:rerun-if-changed={}",
                format!("{}/{}.xml", DEFAULT_INPUT_DIR, odata_srv_name)
            );

            if let Some(schema) = edmx.data_services.fetch_schema(namespace) {
                emit_module(&format!("{}.rs", odata_srv_name), &gen_srv_doc_module(odata_srv_name, &schema));
                emit_module(
                    &format!("{odata_srv_name}{SUFFIX_SNAKE_METADATA}.rs"),
                    &gen_metadata_module(odata_srv_name, &schema),
                );
            } else {
                println!(
                    "Error: OData schema for namespace '{}' cannot be found or this is not OData V2 XML",
                    namespace
                );
            }
        },
    }
}
