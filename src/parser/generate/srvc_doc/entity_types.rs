use crate::{
    edmx::data_services::schema::entity_type::EntityType,
    parser::{
        generate::{
            gen_comment_separator_for, gen_custom_deserializer_info, gen_impl_from_str_for,
            syntax_fragments::{serde_fragments::*, END_BLOCK, ENTITY_TYPES, SEPARATOR},
        },
        AsRustSrc,
    },
    utils::to_upper_camel_case,
};

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
/// Generate entity type structs
pub fn gen_entity_types(ets: &Vec<EntityType>) -> (Vec<u8>, Vec<String>) {
    ets.into_iter().enumerate().fold(
        // Accumulator's initial value is a comment separator
        (gen_comment_separator_for(ENTITY_TYPES), vec![]),
        |(mut acc_src, mut acc_crs), (idx, entity)| {
            if idx > 0 {
                acc_src.append(&mut SEPARATOR.to_vec());
            }

            let (mut et_src, mut crs) = gen_entity_type(entity);
            if crs.len() > 0 {
                acc_crs.append(&mut crs)
            }
            acc_src.append(&mut et_src);
            (acc_src, acc_crs)
        },
    )
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
/// EDM EntityType Instance -> Rust declaration
fn gen_entity_type(entity: &EntityType) -> (Vec<u8>, Vec<String>) {
    let struct_name = to_upper_camel_case(&entity.name);
    let mut crate_refs: Vec<String> = vec![];
    let mut props = entity.properties.clone();
    props.sort();

    let mut out_buffer: Vec<u8> = props.into_iter().fold(
        // Accumulator's initial value is the derive and serde attributes plus the struct declaration
        gen_deserializable_struct(&struct_name),
        |mut acc, mut prop| {
            prop.deserializer_fn = gen_custom_deserializer_info(&prop);
            let (mut prop_src, cr) = prop.to_rust();
            if !cr.is_empty() {
                crate_refs.push(cr)
            }
            acc.append(&mut prop_src);
            acc
        },
    );

    // End the struct declaration then generate from_str implementation
    out_buffer.append(&mut [END_BLOCK, &*gen_impl_from_str_for(&struct_name)].concat());
    (out_buffer, crate_refs)
}
