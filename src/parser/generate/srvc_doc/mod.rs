mod complex_types;
mod entity_types;

use crate::{
    edmx::data_services::schema::Schema,
    parser::generate::{gen_comment_separator_for, gen_module_start},
};

use complex_types::gen_complex_types;
use entity_types::gen_entity_types;
use crate::parser::generate::syntax_fragments::{serde_fragments::*, END_BLOCK};

// ---------------------------------------------------------------------------------------------------------------------
// PUBLIC API
// ---------------------------------------------------------------------------------------------------------------------
pub fn gen_srv_doc_module(odata_srv_name: &str, schema: &Schema) -> Vec<u8> {
    // Start module definition
    let mut out_buffer: Vec<u8> = [&*gen_module_start(odata_srv_name), USE_SERDE].concat();

    if let Some(cts) = &schema.complex_types {
        out_buffer.append(&mut gen_complex_types(cts));
    }

    out_buffer.append(&mut gen_entity_types(&schema.entity_types));

    // Create enum + impl for the entity container element
    // This enum acts as a proxy for the service document
    if let Some(ent_cont) = &schema.entity_container {
        out_buffer.append(&mut gen_comment_separator_for("ENTITY SETS ENUM"));
        out_buffer.append(&mut ent_cont.to_enum_with_impl());
    }

    // End module definition
    out_buffer.append(&mut END_BLOCK.to_vec());
    out_buffer
}
