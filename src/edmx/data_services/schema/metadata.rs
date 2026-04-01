use super::Schema;

use crate::{
    parser::generate::{
        gen_enum_impl_fn_variant_name_into, gen_enum_match_arm_into, gen_enum_start_into, gen_enum_variant_into,
        gen_impl_start_for_into,
        syntax_fragments::{
            derive_traits::{gen_derive_str, DeriveTraits},
            END_BLOCK,
        },
    },
    utils::to_upper_camel_case,
};

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
impl Schema {
    pub fn to_entity_types_enum(&self) -> String {
        let mut out = String::new();
        let upper_camel_entity_types = format!("{}EntityTypes", to_upper_camel_case(&self.namespace));

        // Output the start of an enum that collates all the entity type names
        // #[derive(Debug)]↩︎
        // pub enum <schema_namespace>EntityTypes {↩︎
        let mut output_enum = gen_derive_str(&[DeriveTraits::DEBUG]);
        gen_enum_start_into(&mut output_enum, &upper_camel_entity_types);

        // Output the start of the "variant_name" function within the enum implementation
        let mut fn_variant_name = String::new();
        gen_enum_impl_fn_variant_name_into(&mut fn_variant_name);

        // Create entity type enum
        for ent_type in self.entity_types.iter() {
            let ent_type_name_camel = to_upper_camel_case(&ent_type.name);

            // Add variant to enum and value function
            gen_enum_variant_into(&mut output_enum, &ent_type_name_camel);
            gen_enum_match_arm_into(
                &mut fn_variant_name,
                &upper_camel_entity_types,
                &ent_type_name_camel,
                &ent_type.name,
            );
        }

        output_enum.push_str(END_BLOCK);
        fn_variant_name.push_str(END_BLOCK);
        fn_variant_name.push_str(END_BLOCK);

        out.push_str(&output_enum);
        gen_impl_start_for_into(&mut out, &upper_camel_entity_types);
        out.push_str(&fn_variant_name);
        out.push_str(END_BLOCK);

        out
    }
}
