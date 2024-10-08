pub mod generate;

mod error;
mod io;

use crate::utils::rust_tools::run_rustfmt;
use generate::{
    metadata_doc::*, srvc_doc::*, syntax_fragments::SUFFIX_SNAKE_METADATA,
};
use io::*;

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
pub trait AsRustSrc {
    type CrateRef;
    fn to_rust(&self) -> (Vec<u8>, Self::CrateRef);
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
        Err(err) => println!("Error: {}", err.msg),
        Ok(edmx) => {
            if let Some(schema) = edmx.data_services.fetch_schema(namespace) {
                let mod_name = format!("{}.rs", odata_srv_name);

                // Generate the source code for the service document module and run it through rustfmt
                match run_rustfmt(&gen_srv_doc_module(odata_srv_name, &schema), &mod_name) {
                    Ok(formatted_bytes) => {
                        write_buffer_to_file(&mod_name, formatted_bytes);

                        // Tell cargo to watch the input XML file
                        println!(
                            "cargo:rerun-if-changed={}",
                            format!("{}/{}.xml", DEFAULT_INPUT_DIR, odata_srv_name)
                        );
                    },
                    Err(err) => println!("Error: rustfmt for service document module ended with {}", err.to_string()),
                }

                // - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
                // Generate the source code for the metadata document module and run it through rustfmt
                let mod_name = format!("{odata_srv_name}{SUFFIX_SNAKE_METADATA}.rs",);

                match run_rustfmt(&gen_metadata_module(odata_srv_name, &schema), &mod_name) {
                    Ok(formatted_bytes) => write_buffer_to_file(&mod_name, formatted_bytes),
                    Err(err) => println!("Error: rustfmt for metadata document module ended with {}", err.to_string()),
                }
            } else {
                println!(
                    "Error: OData schema for namespace '{}' cannot be found or this is not OData V2 XML",
                    namespace
                );
            }
        },
    }
}
