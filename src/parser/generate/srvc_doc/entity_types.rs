use crate::{
    edmx::data_services::schema::entity_type::EntityType,
    parser::{
        generate::{
            gen_comment_separator_for, gen_impl_from_str_for, gen_serde_custom_deserializer_attrib,
            syntax_fragments::{serde_fragments::*, END_BLOCK, ENTITY_TYPES, SEPARATOR},
        },
        AsRustSrc,
    },
    utils::to_upper_camel_case,
};

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
/// Generate entity type structs
pub fn gen_entity_types(ets: &Vec<EntityType>) -> Vec<u8> {
    ets.into_iter()
        .enumerate()
        .fold(gen_comment_separator_for(ENTITY_TYPES), |mut acc, (idx, entity)| {
            if idx > 0 {
                acc.append(&mut SEPARATOR.to_vec());
            }
            acc.append(&mut gen_entity_type(entity));
            acc
        })
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
/// EDM EntityType Instance -> Rust declaration
fn gen_entity_type(entity: &EntityType) -> Vec<u8> {
    let struct_name = to_upper_camel_case(&entity.name);
    let mut props = entity.properties.clone();
    props.sort();

    let mut out_buffer: Vec<u8> =
        props
            .into_iter()
            .fold(gen_deserializable_struct(&struct_name), |mut acc, mut prop| {
                prop.deserializer_fn = gen_serde_custom_deserializer_attrib(&prop);
                acc.append(&mut prop.to_rust());
                acc
            });

    // End the struct declaration then generate from_str implementation
    out_buffer.append(&mut [END_BLOCK, &*gen_impl_from_str_for(&struct_name)].concat());
    out_buffer
}
