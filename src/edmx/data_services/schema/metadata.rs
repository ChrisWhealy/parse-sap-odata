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
        let mut output_enum = gen_derive_str(vec![DeriveTraits::DEBUG]);
        output_enum.append(&mut gen_enum_start(&upper_camel_entity_types));

        // Output the start of the "variant_name" function within the enum implementation
        let mut fn_variant_name = gen_enum_impl_fn_variant_name();

        // Create entity type enum
        for ent_type in self.entity_types.iter() {
            let ent_type_name_camel = to_upper_camel_case(&ent_type.name);

            // Add variant to enum and value function
            output_enum.append(&mut gen_enum_variant(&ent_type_name_camel));
            fn_variant_name.append(&mut gen_enum_match_arm(
                &upper_camel_entity_types,
                &ent_type_name_camel,
                &ent_type.name,
            ));
        }

        output_enum.append(&mut END_BLOCK.to_vec());
        fn_variant_name.append(&mut [END_BLOCK, END_BLOCK].concat());

        [
            &*output_enum,
            &*gen_impl_start_for(&upper_camel_entity_types),
            &*fn_variant_name,
            END_BLOCK,
        ]
        .concat()
    }
}
