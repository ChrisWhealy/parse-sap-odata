use crate::{
    edmx::data_services::schema::entity_type::EntityType,
    parser::{
        generate::{
            gen_comment_separator_for_into, gen_impl_from_str_for_into,
            syntax_fragments::{serde_fragments::*, END_BLOCK, ENTITY_TYPES, SEPARATOR},
        },
        AsRustSrc,
    },
    utils::to_upper_camel_case,
};

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
/// Generate entity type structs, writing output into `out` and returning crate references
pub fn gen_entity_types_into(out: &mut String, ets: &[EntityType]) -> Vec<String> {
    let mut crate_refs: Vec<String> = Vec::new();

    // Start source code with a comment separator
    gen_comment_separator_for_into(out, ENTITY_TYPES);

    for (idx, entity) in ets.iter().enumerate() {
        if idx > 0 {
            out.push_str(SEPARATOR);
        }

        let (et_src, mut crs) = gen_entity_type(entity);
        if !crs.is_empty() {
            crate_refs.append(&mut crs)
        }
        out.push_str(&et_src);
    }

    crate_refs
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
/// EDM EntityType Instance -> Rust declaration
fn gen_entity_type(entity: &EntityType) -> (String, Vec<String>) {
    let struct_name = to_upper_camel_case(&entity.name);
    let mut crate_refs: Vec<String> = vec![];
    let mut props: Vec<_> = entity.properties.iter().collect();
    props.sort();

    // Accumulator's initial value is the derive and serde attributes plus the struct declaration
    let mut out = String::new();
    out.push_str(&gen_deserializable_struct(&struct_name));

    for prop in props.into_iter() {
        let (prop_src, cr) = prop.to_rust();
        if !cr.is_empty() {
            crate_refs.push(cr)
        }

        out.push_str(&prop_src);
    }

    // End the struct declaration then generate from_str implementation
    out.push_str(END_BLOCK);
    gen_impl_from_str_for_into(&mut out, &struct_name);

    (out, crate_refs)
}
