mod complex_types;
mod entity_types;

use complex_types::gen_complex_types_into;
use entity_types::gen_entity_types_into;

use std::collections::BTreeSet;

use crate::{
    edmx::data_services::schema::Schema,
    parser::generate::{
        gen_comment_separator_for_into, gen_extern_crate_into, gen_module_start_into,
        syntax_fragments::{
            gen_use_path_into, CRATE_QUICK_XML, CRATE_SERDE, END_BLOCK, PATH_TO_SERDE_SERIALIZE_DESERIALIZE,
        },
    },
};

// ---------------------------------------------------------------------------------------------------------------------
pub fn gen_srv_doc_module(odata_srv_name: &str, schema: &Schema) -> Vec<u8> {
    let mut crate_refs: BTreeSet<String> = BTreeSet::new();
    let mut out_buffer = Vec::new();

    // External crate dependencies on serde and quick_xml are always required
    gen_extern_crate_into(&mut out_buffer, CRATE_QUICK_XML);
    gen_extern_crate_into(&mut out_buffer, CRATE_SERDE);
    gen_module_start_into(&mut out_buffer, odata_srv_name);
    gen_use_path_into(&mut out_buffer, PATH_TO_SERDE_SERIALIZE_DESERIALIZE);

    if let Some(cts) = &schema.complex_types {
        crate_refs.extend(gen_complex_types_into(&mut out_buffer, cts));
    }

    crate_refs.extend(gen_entity_types_into(&mut out_buffer, &schema.entity_types));

    // Create enum + impl for the entity container element
    // This enum acts as a proxy for the list of Collections in the service document
    if let Some(ent_cont) = &schema.entity_container {
        gen_comment_separator_for_into(&mut out_buffer, "ENTITY SETS ENUM");
        ent_cont.to_enum_with_impl_into(&mut out_buffer);
    }

    // End module definition
    out_buffer.extend_from_slice(END_BLOCK);

    // Add any external crate references
    for cr in crate_refs {
        gen_extern_crate_into(&mut out_buffer, &cr);
    }

    out_buffer
}

// ---------------------------------------------------------------------------------------------------------------------
#[cfg(test)]
mod unit_tests;
