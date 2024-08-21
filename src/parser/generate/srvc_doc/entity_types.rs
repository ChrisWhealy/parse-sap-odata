use crate::{
    edmx::data_services::schema::entity_type::EntityType,
    parser::{
        generate::{
            gen_comment_separator_for,
            gen_impl_from_str_for, gen_start_struct, syntax_fragments::{
                derive_traits::{derive_str, DeriveTraits},
                serde_fragments::*,
                EDMX_DATE_TIME, EDMX_DATE_TIME_OFFSET, EDMX_DECIMAL, END_BLOCK, ENTITY_TYPES, SEPARATOR,
            },
        },
        AsRustSrc,
    },
    property::{metadata::PropertyType, Property},
    utils::to_upper_camel_case,
};

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
/// Generate entity type structs
pub fn gen_entity_types(ets: &Vec<EntityType>) -> Vec<u8> {
    let mut out_buffer: Vec<u8> = gen_comment_separator_for(ENTITY_TYPES);

    for (idx, entity) in ets.into_iter().enumerate() {
        if idx > 0 {
            out_buffer.append(&mut SEPARATOR.to_vec());
        }

        out_buffer.append(&mut gen_entity_type(entity));
    }

    out_buffer
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
/// EDM EntityType Instance -> Rust declaration
fn gen_entity_type(entity: &EntityType) -> Vec<u8> {
    let struct_name = to_upper_camel_case(&entity.name);
    let mut out_buffer: Vec<u8> = [
        &*derive_str(vec![
            DeriveTraits::CLONE,
            DeriveTraits::DEBUG,
            DeriveTraits::DEFAULT,
            DeriveTraits::SERIALIZE,
            DeriveTraits::DESERIALIZE,
        ]),
        SERDE_RENAME_ALL_PASCAL_CASE,
        &*gen_start_struct(&struct_name),
    ]
        .concat();

    let mut props = entity.properties.clone();
    props.sort();

    for mut prop in props {
        // Check whether this property needs a custom deserializer
        match Property::get_property_type(&prop) {
            PropertyType::Edm(edm_type) => {
                let edm_type_str = edm_type.as_str();

                // Do we need to make a forward reference to a custom deserializer in the parse-sap-atom-feed crate?
                if edm_type_str.eq(EDMX_DATE_TIME) || edm_type_str.eq(EDMX_DATE_TIME_OFFSET) {
                    prop.deserializer_fn = gen_datetime_deserializer_ref(prop.nullable);
                } else if edm_type_str.eq(EDMX_DECIMAL) {
                    prop.deserializer_fn = gen_decimal_deserializer_ref(prop.nullable, prop.scale);
                }
            },

            PropertyType::Complex(_) => {},

            PropertyType::Unqualified => {
                // TODO This is probably an error condition.  Need to decide how to handle it
                println!(
                    "Malformed property type. Expected pattern of <namespace>.<type> or Edm.<type>.  Instead got '{}'",
                    prop.edm_type
                );
            },
        };

        out_buffer.append(&mut prop.to_rust())
    }

    out_buffer.append(
        &mut [
            END_BLOCK,
            // Implement `from_str` for this struct
            &*gen_impl_from_str_for(&struct_name),
        ]
            .concat(),
    );

    out_buffer
}
