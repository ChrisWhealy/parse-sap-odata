use super::Schema;

use crate::parser::generate::syntax_fragments::{derive_traits::*, *};
use crate::{parser::generate::*, utils::to_upper_camel_case};

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
impl Schema {
    pub fn to_entity_types_enum(&self) -> Vec<u8> {
        let upper_camel_entity_types = format!("{}EntityTypes", to_upper_camel_case(&self.namespace));

        // Output the start of an enum that collates all the entity type names
        // #[derive(Debug)]↩︎
        // pub enum <schema_namespace>EntityTypes {↩︎
        let mut output_enum = gen_derive_str(&[DeriveTraits::DEBUG]);
        output_enum.append(&mut gen_enum_start(&upper_camel_entity_types));

        // Output the start of the "variant_name" function within the enum implementation
        let mut fn_variant_name = gen_enum_impl_fn_variant_name();

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

        output_enum.extend_from_slice(END_BLOCK);
        fn_variant_name.extend_from_slice(END_BLOCK);
        fn_variant_name.extend_from_slice(END_BLOCK);

        [
            &*output_enum,
            &*gen_impl_start_for(&upper_camel_entity_types),
            &*fn_variant_name,
            END_BLOCK,
        ]
        .concat()
    }
}
