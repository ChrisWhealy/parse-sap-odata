mod complex_types;
mod entity_types;

use crate::{
    edmx::data_services::schema::Schema,
    parser::generate::{
        gen_comment_separator_for, gen_extern_crate, gen_module_start,
        syntax_fragments::{
            gen_use_path, CRATE_QUICK_XML, CRATE_SERDE, END_BLOCK, PATH_TO_SERDE_SERIALIZE_DESERIALIZE,
        },
    },
    utils::dedup_vec_of_string,
};

use complex_types::gen_complex_types;
use entity_types::gen_entity_types;

// ---------------------------------------------------------------------------------------------------------------------
pub fn gen_srv_doc_module(odata_srv_name: &str, schema: &Schema) -> Vec<u8> {
    let mut crate_refs: Vec<String> = vec![];

    // Start module definition
    let mut out_buffer: Vec<u8> = [
        // External crate dependencies on serde and quick_xml are always required
        &*gen_extern_crate(CRATE_QUICK_XML),
        &*gen_extern_crate(CRATE_SERDE),
        &*gen_module_start(odata_srv_name),
        &*gen_use_path(PATH_TO_SERDE_SERIALIZE_DESERIALIZE),
    ]
    .concat();

    if let Some(cts) = &schema.complex_types {
        let (mut src, mut crs) = gen_complex_types(cts);
        if crs.len() > 0 {
            crate_refs.append(&mut crs)
        };
        out_buffer.append(&mut src);
    }

    let (mut ets_src, mut crs) = gen_entity_types(&schema.entity_types);
    if crs.len() > 0 {
        crate_refs.append(&mut crs)
    }
    out_buffer.append(&mut ets_src);

    // Create enum + impl for the entity container element
    // This enum acts as a proxy for the list of Collections in the service document
    if let Some(ent_cont) = &schema.entity_container {
        out_buffer
            .append(&mut [&*gen_comment_separator_for("ENTITY SETS ENUM"), &*ent_cont.to_enum_with_impl()].concat());
    }

    // End module definition
    out_buffer.append(&mut END_BLOCK.to_vec());

    // Add any external crate references
    for cr in dedup_vec_of_string(crate_refs) {
        out_buffer.append(&mut gen_extern_crate(&cr))
    }

    out_buffer
}

// ---------------------------------------------------------------------------------------------------------------------
#[cfg(test)]
mod unit_tests;
