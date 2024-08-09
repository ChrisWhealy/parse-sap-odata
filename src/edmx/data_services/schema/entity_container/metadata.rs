use crate::{
    edmx::data_services::schema::entity_container::EntityContainer,
    parser::syntax_fragments::{
        derive_traits::*,
        fragment_generators::{
            end_iter_fn, gen_enum_fn_iter_start, gen_enum_fn_variant_names, gen_enum_impl_fn_variant_name,
            gen_enum_match_arm, gen_enum_start, gen_enum_variant, gen_fq_enum_variant, gen_impl_start,
            gen_type_name_upper_camel,
        },
        *,
    },
};

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
impl EntityContainer {
    /// Transforms an `EntityContainer` into an enumeration of entity set names.
    /// Additionally, this enumeration is given three helper functions:
    /// * `pub fn iterator() -> impl Iterator<Item = GwsampleBasicEntities> { /* SNIP */ }`
    /// * `pub const fn variant_name(&self) -> &'static str { /* SNIP */ }`
    /// * `pub fn variant_names() -> Vec<&'static str> { /* SNIP */ }`
    pub fn to_enum_with_impl(&self) -> Vec<u8> {
        let cont_name_camel = gen_type_name_upper_camel(&self.name);

        // Output the start of an enum for this entity container
        // #[derive(Copy, Clone, Debug)]↩︎
        // #[allow(dead_code)]↩︎
        // pub enum <entity_container_name> {↩︎
        let mut entities_enum = [
            derive_str(vec![DeriveTraits::COPY, DeriveTraits::CLONE, DeriveTraits::DEBUG]),
            RUSTC_ALLOW_DEAD_CODE.to_vec(),
            gen_enum_start(&cont_name_camel),
        ]
        .concat();

        // Output the start of the "iterator" function within the enum implementation
        //   pub fn iterator() -> impl Iterator<Item = GwsampleBasicEntities> {↩︎
        //       match *self {↩︎
        let mut enum_fn_iterator = gen_enum_fn_iter_start(&cont_name_camel);

        // Output the start of the "variant_name" function within the enum implementation
        let mut enum_fn_variant_name = gen_enum_impl_fn_variant_name();

        // Create entity set enum
        for ent_set in self.entity_sets.iter() {
            let ent_set_name_camel = gen_type_name_upper_camel(&ent_set.name);

            // Add variant to enum, iterator, and variant_name functions
            entities_enum.append(&mut gen_enum_variant(&ent_set_name_camel));
            enum_fn_variant_name.append(&mut gen_enum_match_arm(&cont_name_camel, &ent_set_name_camel, &ent_set.name));
            enum_fn_iterator.append(&mut gen_fq_enum_variant(&cont_name_camel, &ent_set_name_camel));
        }

        // End enum and function blocks
        entities_enum.append(&mut END_BLOCK.to_vec());
        enum_fn_iterator.append(&mut end_iter_fn());
        enum_fn_variant_name.append(&mut [CLOSE_CURLY, END_BLOCK].concat());

        [
            // EntityContainer enum
            &*entities_enum,
            // Output the start of an enum implementation
            // #[allow(dead_code)]↩︎
            // impl <entity_container_name> {↩︎
            RUSTC_ALLOW_DEAD_CODE,
            &*gen_impl_start(&cont_name_camel),
            &*enum_fn_iterator,
            &*enum_fn_variant_name,
            &*gen_enum_fn_variant_names(&cont_name_camel),
            END_BLOCK,
        ]
        .concat()
    }
}
