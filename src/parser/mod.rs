pub mod generate;

mod io;

use crate::utils::{rust_tools::run_rustfmt, to_module_name};
use generate::{metadata_doc::*, srvc_doc::*, syntax_fragments::SUFFIX_SNAKE_METADATA};
use io::*;

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
pub trait AsRustSrc {
    type CrateRef;
    fn to_rust(&self) -> (String, Self::CrateRef);
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
/// Format `buf` with rustfmt and write the result to `mod_name` in `$OUT_DIR`
fn emit_module(mod_name: &str, buf: &[u8]) {
    match run_rustfmt(buf, mod_name) {
        Ok(formatted_bytes) => {
            if let Err(err) = write_buffer_to_file(mod_name, &formatted_bytes) {
                println!("Error: writing module '{mod_name}' failed: {err}");
            }
        },
        Err(err) => println!("Error: rustfmt for module '{mod_name}' ended with {err}"),
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
        Err(err) => println!("Error: {err}"),
        Ok(edmx) => {
            // Write cargo build script directive as soon as the input path is known to be valid
            println!("cargo:rerun-if-changed={DEFAULT_INPUT_DIR}/{odata_srv_name}.xml");

            if let Some(schema) = edmx.data_services.fetch_schema(namespace) {
                // The module name must be valid Rust snake_case regardless of what the caller
                // supplied as odata_srv_name (e.g. "service_ProjectServiceV2" → "service_project_service_v2").
                // The original odata_srv_name is intentionally kept for the input-file lookup above.
                let mod_name = to_module_name(odata_srv_name);
                emit_module(
                    &format!("{mod_name}.rs"),
                    gen_srv_doc_module(&mod_name, schema).as_bytes(),
                );
                emit_module(
                    &format!("{mod_name}{SUFFIX_SNAKE_METADATA}.rs"),
                    gen_metadata_module(&mod_name, schema).as_bytes(),
                );
            } else {
                println!("Error: OData schema for namespace '{namespace}' cannot be found or this is not OData V2 XML");
            }
        },
    }
}
