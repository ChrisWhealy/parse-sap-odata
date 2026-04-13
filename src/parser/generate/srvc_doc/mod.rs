mod complex_types;
mod entity_types;

use complex_types::gen_complex_types_into;
use entity_types::gen_entity_types_into;

use crate::{
    edmx::data_services::schema::Schema,
    parser::generate::{
        gen_comment_separator_for_into, gen_module_start_into,
        syntax_fragments::{
            gen_use_path_into, END_BLOCK, PATH_TO_SERDE_SERIALIZE_DESERIALIZE,
        },
    },
};

// ---------------------------------------------------------------------------------------------------------------------
pub fn gen_srv_doc_module(odata_srv_name: &str, schema: &Schema) -> String {
    let mut out_buffer = String::new();

    // In Rust 2018+ edition, extern crate declarations are not required for
    // crates listed in Cargo.toml.  Emitting them causes duplicate-definition
    // errors when multiple generated modules are included in the same file.
    gen_module_start_into(&mut out_buffer, odata_srv_name);
    gen_use_path_into(&mut out_buffer, PATH_TO_SERDE_SERIALIZE_DESERIALIZE);

    if let Some(cts) = &schema.complex_types {
        gen_complex_types_into(&mut out_buffer, cts);
    }

    gen_entity_types_into(&mut out_buffer, &schema.entity_types);

    // Create enum + impl for the entity container element
    // This enum acts as a proxy for the list of Collections in the service document
    if let Some(ent_cont) = &schema.entity_container {
        gen_comment_separator_for_into(&mut out_buffer, "ENTITY SETS ENUM");
        ent_cont.to_enum_with_impl_into(&mut out_buffer);
    }

    // End module definition
    out_buffer.push_str(END_BLOCK);

    out_buffer
}

// ---------------------------------------------------------------------------------------------------------------------
#[cfg(test)]
mod unit_tests;
