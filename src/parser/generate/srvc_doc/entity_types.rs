use crate::{
    edmx::data_services::schema::entity_type::EntityType,
    parser::{
        generate::{
            gen_comment_separator_for, gen_impl_from_str_for,
            syntax_fragments::{serde_fragments::*, END_BLOCK, ENTITY_TYPES, SEPARATOR},
        },
        AsRustSrc,
    },
    utils::to_upper_camel_case,
};

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
/// Generate entity type structs, writing output into `out` and returning crate references
pub fn gen_entity_types_into(out: &mut String, ets: &[EntityType]) -> Vec<String> {
    let (src, crs) = gen_entity_types(ets);
    out.push_str(&src);
    crs
}

pub fn gen_entity_types(ets: &[EntityType]) -> (String, Vec<String>) {
    ets.into_iter().enumerate().fold(
        // Accumulator's initial value is a comment separator
        (gen_comment_separator_for(ENTITY_TYPES), vec![]),
        |(mut acc_src, mut acc_crs), (idx, entity)| {
            if idx > 0 {
                acc_src.push_str(SEPARATOR);
            }

            let (et_src, mut crs) = gen_entity_type(entity);
            if !crs.is_empty() {
                acc_crs.append(&mut crs)
            }
            acc_src.push_str(&et_src);
            (acc_src, acc_crs)
        },
    )
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
/// EDM EntityType Instance -> Rust declaration
fn gen_entity_type(entity: &EntityType) -> (String, Vec<String>) {
    let struct_name = to_upper_camel_case(&entity.name);
    let mut crate_refs: Vec<String> = vec![];
    let mut props: Vec<_> = entity.properties.iter().collect();
    props.sort();

    let mut out_buffer: String = props.into_iter().fold(
        // Accumulator's initial value is the derive and serde attributes plus the struct declaration
        gen_deserializable_struct(&struct_name),
        |mut acc, prop| {
            let (prop_src, cr) = prop.to_rust();
            if !cr.is_empty() {
                crate_refs.push(cr)
            }

            acc.push_str(&prop_src);
            acc
        },
    );

    // End the struct declaration then generate from_str implementation
    out_buffer.push_str(END_BLOCK);
    out_buffer.push_str(&gen_impl_from_str_for(&struct_name));

    (out_buffer, crate_refs)
}
