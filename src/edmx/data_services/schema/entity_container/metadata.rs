use crate::parser::generate::syntax_fragments::{derive_traits::*, *};
use crate::{
    edmx::data_services::schema::entity_container::EntityContainer, parser::generate::*, utils::to_upper_camel_case,
};

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
impl EntityContainer {
    /// Transforms an `EntityContainer` into an enumeration of entity set names.
    /// Additionally, this enumeration is given three helper functions:
    /// * `pub fn iterator() -> impl Iterator<Item = GwsampleBasicEntities> { /* SNIP */ }`
    /// * `pub const fn variant_name(&self) -> &'static str { /* SNIP */ }`
    /// * `pub fn variant_names() -> Vec<&'static str> { /* SNIP */ }`
    pub fn to_enum_with_impl_into(&self, out: &mut String) {
        let cont_name_camel = to_upper_camel_case(&self.name);

        let mut enum_fn_iterator = String::new();
        let mut enum_fn_variant_name = String::new();

        gen_enum_fn_iter_start_into(&mut enum_fn_iterator, &cont_name_camel);
        gen_enum_impl_fn_variant_name_into(&mut enum_fn_variant_name);

        let mut entities_enum = String::new();
        entities_enum.push_str(&gen_derive_str(&[DeriveTraits::COPY, DeriveTraits::CLONE, DeriveTraits::DEBUG]));
        entities_enum.push_str(RUSTC_ALLOW_DEAD_CODE);
        gen_enum_start_into(&mut entities_enum, &cont_name_camel);

        let mut entities_enum = self.entity_sets.iter().fold(entities_enum, |mut acc, ent_set| {
            let ent_set_name_camel = to_upper_camel_case(&ent_set.name);
            gen_enum_variant_into(&mut acc, &ent_set_name_camel);
            gen_enum_match_arm_into(&mut enum_fn_variant_name, &cont_name_camel, &ent_set_name_camel, &ent_set.name);
            gen_fq_enum_variant_into(&mut enum_fn_iterator, &cont_name_camel, &ent_set_name_camel);
            acc
        });

        entities_enum.push_str(END_BLOCK);
        gen_end_iter_fn_into(&mut enum_fn_iterator);
        enum_fn_variant_name.push_str(CLOSE_CURLY);
        enum_fn_variant_name.push_str(END_BLOCK);

        out.push_str(&entities_enum);
        out.push_str(RUSTC_ALLOW_DEAD_CODE);
        gen_impl_start_for_into(out, &cont_name_camel);
        out.push_str(&enum_fn_iterator);
        out.push_str(&enum_fn_variant_name);
        gen_enum_fn_variant_names_into(out, &cont_name_camel);
        out.push_str(END_BLOCK);
    }

    pub fn to_enum_with_impl(&self) -> String {
        let mut out = String::new();
        let cont_name_camel = to_upper_camel_case(&self.name);

        // Output the start of the "iterator" function within the enum implementation
        //   pub fn iterator() -> impl Iterator<Item = GwsampleBasicEntities> {↩︎
        //       match *self {↩︎
        let mut enum_fn_iterator = String::new();
        gen_enum_fn_iter_start_into(&mut enum_fn_iterator, &cont_name_camel);

        // Output the start of the "variant_name" function within the enum implementation
        let mut enum_fn_variant_name = String::new();
        gen_enum_impl_fn_variant_name_into(&mut enum_fn_variant_name);

        // Start entity set enum for this entity container
        // #[derive(Copy, Clone, Debug)]↩︎
        // #[allow(dead_code)]↩︎
        // pub enum <entity_container_name> {↩︎
        let mut entities_enum = String::new();

        entities_enum.push_str(&gen_derive_str(&[DeriveTraits::COPY, DeriveTraits::CLONE, DeriveTraits::DEBUG]));
        entities_enum.push_str(RUSTC_ALLOW_DEAD_CODE);
        gen_enum_start_into(&mut entities_enum, &cont_name_camel);

        for ent_set in self.entity_sets.iter() {
            let ent_set_name_camel = to_upper_camel_case(&ent_set.name);

            // Add variant to enum, iterator, and variant_name functions
            gen_enum_variant_into(&mut entities_enum, &ent_set_name_camel);
            gen_enum_match_arm_into(&mut enum_fn_variant_name, &cont_name_camel, &ent_set_name_camel, &ent_set.name);
            gen_fq_enum_variant_into(&mut enum_fn_iterator, &cont_name_camel, &ent_set_name_camel);
        }

        // End enum and function blocks
        entities_enum.push_str(END_BLOCK);
        gen_end_iter_fn_into(&mut enum_fn_iterator);
        enum_fn_variant_name.push_str(CLOSE_CURLY);
        enum_fn_variant_name.push_str(END_BLOCK);

        // EntityContainer enum
        out.push_str(&entities_enum);

        // Output the start of an enum implementation
        // #[allow(dead_code)]↩︎
        // impl <entity_container_name> {↩︎
        out.push_str(RUSTC_ALLOW_DEAD_CODE);
        gen_impl_start_for_into(&mut out, &cont_name_camel);
        out.push_str(&enum_fn_iterator);
        out.push_str(&enum_fn_variant_name);
        gen_enum_fn_variant_names_into(&mut out, &cont_name_camel);
        out.push_str(END_BLOCK);

        out
    }
}
