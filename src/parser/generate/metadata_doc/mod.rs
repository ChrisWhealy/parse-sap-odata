mod associations;
mod complex_types;
mod entity_types;

use crate::{
    edmx::data_services::schema::Schema,
    parser::generate::{
        gen_extern_crate_into, gen_module_start_into,
        syntax_fragments::{gen_use_path_into, *},
    },
};

use super::CRATE_PARSE_SAP_ATOM_FEED;

use associations::*;
use complex_types::*;
use entity_types::*;

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
/// Generate a module containing the metadata structs and their respective instances
pub fn gen_metadata_module(odata_srv_name: &str, schema: &Schema) -> Vec<u8> {
    let mod_name = format!("{odata_srv_name}{SUFFIX_SNAKE_METADATA}");
    let mut out_buffer = Vec::new();

    // Start module definition
    gen_extern_crate_into(&mut out_buffer, CRATE_PARSE_SAP_ATOM_FEED);
    gen_module_start_into(&mut out_buffer, &mod_name);
    gen_use_path_into(&mut out_buffer, PATH_TO_SAP_ODATA_PROPERTIES);
    gen_use_path_into(&mut out_buffer, PATH_TO_SAP_ANNOTATIONS_PROPERTY);

    // Do we need to generate any complex types?
    let skipped_cts = if let Some(cts) = &schema.complex_types {
        gen_use_path_into(&mut out_buffer, PATH_TO_EDMX_COMPLEX_TYPE);
        let (mut cmplx_type_metadata, skipped_cts) = gen_metadata_complex_types(cts);
        out_buffer.append(&mut cmplx_type_metadata);
        skipped_cts
    } else {
        Vec::new()
    };

    out_buffer.append(&mut gen_metadata_entity_types(&schema, &skipped_cts));
    out_buffer.append(&mut gen_metadata_associations(odata_srv_name, &schema));
    out_buffer.append(&mut gen_metadata_association_sets(odata_srv_name, &schema));
    // Close module definition
    out_buffer.extend_from_slice(END_BLOCK);

    out_buffer
}
