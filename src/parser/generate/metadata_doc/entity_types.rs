use std::collections::BTreeSet;

use crate::{
    edmx::data_services::schema::{complex_type::ComplexType, entity_type::EntityType, Schema},
    parser::generate::{syntax_fragments::*, *},
    property::metadata::PropertyType,
    utils::{odata_name_to_rust_safe_name, to_upper_camel_case},
};

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
/// Generate metadata entity type structs, writing output into `out`
pub fn gen_metadata_entity_types_into(out: &mut String, schema: &Schema, skipped_cts: &[String]) {
    out.push_str(&gen_metadata_entity_types(schema, skipped_cts));
}

pub fn gen_metadata_entity_types(schema: &Schema, skipped_cts: &[String]) -> String {
    let mut used_subtypes: BTreeSet<&str> = BTreeSet::new();
    let ets: &Vec<EntityType> = &schema.entity_types;

    let mut out_buffer: String = ets.into_iter().enumerate().fold(
        // Accumulator's initial value is an EntityType comment separator
        gen_comment_separator_for(ENTITY_TYPES),
        |mut acc, (idx, entity)| {
            if idx > 0 {
                acc.push_str(SEPARATOR);
            }

            // Accumulate a set of subtypes used within the SAP Annotations field of each property
            for prop in &entity.properties {
                used_subtypes.extend(prop.sap_annotations.used_subtypes());
            }

            acc.push_str(&gen_metadata_entity_type(entity, &skipped_cts));
            acc.push_str(&gen_metadata_entity_type_impl(entity, &schema.complex_types));

            acc
        },
    );

    // Add usage declaration(s) for all subtypes across all the SAPAnnotationsProperty instances
    for subtype in used_subtypes {
        out_buffer.push_str(&gen_use_path(subtype));
    }

    out_buffer
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
/// EDM EntityType -> Rust metadata instance
fn gen_metadata_entity_type(entity: &EntityType, skipped_cts: &[String]) -> String {
    let struct_name = format!("{}{}", to_upper_camel_case(&entity.name), METADATA);
    let key_type = gen_vector_of_type(PROPERTYREF);

    let mut out_buffer = String::new();
    out_buffer.push_str(RUSTC_ALLOW_DEAD_CODE);
    out_buffer.push_str(&gen_start_struct(&struct_name));

    gen_struct_field_into(&mut out_buffer, FIELD_NAME_KEY, &key_type);

    let mut props: Vec<_> = entity.properties.iter().collect();
    props.sort();

    // Metadata fields are either of type Property or of some complex type
    for prop in props {
        let prop_name = odata_name_to_rust_safe_name(&prop.odata_name);

        match prop.get_property_type() {
            PropertyType::Edm(_, _) => {
                gen_struct_field_into(&mut out_buffer, &prop_name, PROPERTY);
            },

            PropertyType::Complex(cmplx_type) => {
                // Is the current property really a complex type or just a wrapper around a basic Rust type?
                if skipped_cts.contains(&cmplx_type) {
                    // A basic Rust type's metadata is a Property instance
                    gen_struct_field_into(&mut out_buffer, &prop_name, PROPERTY);
                } else {
                    // This really is a complex type
                    let metadata_type_name = format!("{}{}", to_upper_camel_case(&cmplx_type), METADATA);

                    gen_struct_field_into(&mut out_buffer, &prop_name, &metadata_type_name);
                }
            },

            // We should never end up here if the metadata XML is correct...
            PropertyType::Unqualified => {},
        }
    }

    out_buffer.push_str(END_BLOCK);
    out_buffer
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
/// Generates the metadata getter functions for each property in the impl of an EntityType
fn gen_metadata_entity_type_impl(entity: &EntityType, opt_cts: &Option<Vec<ComplexType>>) -> String {
    let struct_name = format!("{}{METADATA}", to_upper_camel_case(&entity.name));
    let mut out_buffer: String = gen_impl_start_for(&struct_name);
    let keys = &entity.key.property_refs;

    // Add a get_key function
    out_buffer.push_str(&gen_fn_signature(
        KEY,
        true,
        false,
        None,
        Some(&gen_vector_of_type(PROPERTYREF)),
    ));
    out_buffer.push_str(OPEN_CURLY);
    out_buffer.push_str(LINE_FEED);
    out_buffer.push_str(VEC_BANG);
    out_buffer.push_str(
        &keys.into_iter()
            .map(|pr| format!("{pr}"))
            .collect::<Vec<_>>()
            .join(","),
    );
    out_buffer.push_str(CLOSE_SQR);
    out_buffer.push_str(CLOSE_CURLY);
    out_buffer.push_str(LINE_FEED);

    let mut props: Vec<_> = entity.properties.iter().collect();
    props.sort();

    // One getter function per property
    for prop in props {
        let safe_name = odata_name_to_rust_safe_name(&prop.odata_name);
        let fn_name = format!("{PREFIX_SNAKE_GET}{safe_name}");

        match prop.get_property_type() {
            PropertyType::Edm(_, _) => {
                let mut prop = prop.clone();
                prop.deserializer_fn = gen_custom_deserializer_info(&prop);
                gen_pub_getter_fn_of_type_into(&mut out_buffer, &fn_name, PROPERTY, &prop)
            }

            PropertyType::Complex(cmplx_type) => {
                let err_msg = format!(
                    "Error: ComplexType property {cmplx_type} found for which there is no corresponding type declaration"
                );

                if let Some(cts) = opt_cts {
                    if let Some(ct) = cts.iter().find(|ct| ct.name.eq(&cmplx_type)) {
                        gen_pub_getter_fn_of_type_into(&mut out_buffer, &fn_name, COMPLEX_TYPE, ct);
                    } else {
                        println!("{err_msg}");
                        println!(
                            "Found complex types {}",
                            cts.iter().map(|ct| ct.name.as_str()).collect::<Vec<_>>().join(",")
                        );
                    }
                } else {
                    println!("{err_msg}");
                }
            },

            PropertyType::Unqualified => {},
        }
    }

    out_buffer.push_str(END_BLOCK);
    out_buffer
}
