use crate::{
    edmx::data_services::schema::{entity_type::EntityType, Schema},
    parser::{
        generate_complex_types::gen_complex_types,
        syntax_fragments::{
            derive_traits::*,
            fragment_generators::{
                comment_for, gen_mod_start, gen_start_struct, gen_type_name_upper_camel, impl_from_str_for,
            },
            serde_fragments::*,
            END_BLOCK, SEPARATOR,
        },
        AsRustSrc,
    },
    property::{metadata::PropertyType, Property},
};

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
/// Generate entity type structs
fn gen_entity_types(ets: &Vec<EntityType>) -> Vec<u8> {
    let mut out_buffer: Vec<u8> = Vec::new();

    out_buffer.append(&mut comment_for("ENTITY TYPES"));

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
    let mut out_buffer: Vec<u8> = Vec::new();
    let struct_name = gen_type_name_upper_camel(&entity.name);
    out_buffer.append(&mut derive_str(vec![
        DeriveTraits::CLONE,
        DeriveTraits::DEBUG,
        DeriveTraits::DEFAULT,
        DeriveTraits::SERIALIZE,
        DeriveTraits::DESERIALIZE,
    ]));
    out_buffer.append(&mut SERDE_RENAME_ALL_PASCAL_CASE.to_vec());
    out_buffer.append(&mut gen_start_struct(&struct_name));

    let mut props = entity.properties.clone();
    props.sort();

    for mut prop in props {
        // Check whether this property needs a custom deserializer module/function
        match Property::get_property_type(&prop) {
            PropertyType::Edm(edm_type) => {
                if edm_type.eq("DateTime") || edm_type.eq("DateTimeOffset") {
                    prop.deserializer_fn = if prop.nullable { SERDE_DE_DATETIME_OPT } else { SERDE_DE_DATETIME }
                } else if edm_type.eq("Decimal") {
                    prop.deserializer_module = if prop.nullable { SERDE_DE_DECIMAL_OPT } else { SERDE_DE_DECIMAL }
                }
            },

            PropertyType::Complex(_) => {},

            PropertyType::Unqualified => {
                // TODO This is probably an error condition.  Need to decide how to handle it
                println!(
                    "Malformed property type. Expected <namespace>.<type> or Edm.<type>.  Got {}",
                    prop.edm_type
                );
            },
        };

        out_buffer.append(&mut prop.to_rust())
    }

    out_buffer.append(&mut END_BLOCK.to_vec());

    // Implement `from_str` for this struct
    out_buffer.append(&mut impl_from_str_for(&struct_name));
    out_buffer
}

// ---------------------------------------------------------------------------------------------------------------------
// PUBLIC API
// ---------------------------------------------------------------------------------------------------------------------
pub fn gen_srv_doc_module(odata_srv_name: &str, schema: &Schema) -> Vec<u8> {
    let mut out_buffer: Vec<u8> = Vec::new();

    // Start module definition
    out_buffer.append(&mut gen_mod_start(odata_srv_name));
    out_buffer.append(&mut USE_SERDE.to_vec());

    if let Some(cts) = &schema.complex_types {
        out_buffer.append(&mut gen_complex_types(cts));
    }

    out_buffer.append(&mut gen_entity_types(&schema.entity_types));

    // Create enum + impl for the entity container element
    // This enum acts as a proxy for the service document
    if let Some(ent_cont) = &schema.entity_container {
        out_buffer.append(&mut comment_for("ENTITY SETS ENUM"));
        out_buffer.append(&mut ent_cont.to_enum_with_impl());
    }

    // End module definition
    out_buffer.append(&mut END_BLOCK.to_vec());
    out_buffer
}
