mod complex_types;
mod entity_types;
mod associations;

use crate::{
    edmx::data_services::schema::Schema,
    parser::syntax_fragments::{
        fragment_generators::*,
        *,
    },
};

use associations::*;
use complex_types::*;
use entity_types::*;

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
/// Generate a module containing the metadata structs and their respective instances
pub fn gen_metadata_module(odata_srv_name: &str, schema: &Schema) -> Vec<u8> {
    let mod_name = format!("{odata_srv_name}{SUFFIX_SNAKE_METADATA}");

    // Start module definition
    let mut out_buffer: Vec<u8> = [
        &*gen_mod_start(&mod_name),
        &*gen_use_path(PATH_TO_SAP_ODATA_PROPERTIES),
        &*gen_use_path(PATH_TO_SAP_ANNOTATIONS_PROPERTY),
    ]
        .concat();

    // Do we need to generate any complex types?
    let skipped_cts = if let Some(cts) = &schema.complex_types {
        out_buffer.append(&mut gen_use_path(PATH_TO_EDMX_COMPLEX_TYPE));
        let (mut cmplx_type_metadata, skipped_cts) = gen_metadata_complex_types(cts);
        out_buffer.append(&mut cmplx_type_metadata);
        skipped_cts
    } else {
        Vec::new()
    };

    out_buffer.append(
        &mut [
            &*gen_metadata_entity_types(&schema, skipped_cts),
            &*gen_metadata_associations(odata_srv_name, &schema),
            &*gen_metadata_association_sets(odata_srv_name, &schema),
            // Close module definition
            END_BLOCK,
        ]
            .concat(),
    );

    out_buffer
}
