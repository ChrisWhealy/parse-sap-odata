use crate::{
    edmx::data_services::schema::{complex_type::ComplexType, entity_type::EntityType, Schema},
    parser::generate::{syntax_fragments::*, *},
    property::metadata::PropertyType,
    utils::{dedup_vec_of_u8_array, odata_name_to_rust_safe_name, to_upper_camel_case},
};

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
/// Generate metadata entity type structs
pub fn gen_metadata_entity_types(schema: &Schema, skipped_cts: Vec<String>) -> Vec<u8> {
    let mut used_subtypes: Vec<&[u8]> = vec![];
    let ets: &Vec<EntityType> = &schema.entity_types;

    let mut out_buffer: Vec<u8> = ets.into_iter().enumerate().fold(
        // Accumulator's initial value is an EntityType comment separator
        gen_comment_separator_for(ENTITY_TYPES),
        |mut acc, (idx, entity)| {
            if idx > 0 {
                acc.extend_from_slice(SEPARATOR);
            }

            // Accumulate a list of subtypes used within the SAP Annotations field of each property
            for prop in &entity.properties {
                used_subtypes.append(&mut prop.sap_annotations.used_subtypes());
            }

            acc.append(&mut gen_metadata_entity_type(entity, &skipped_cts));
            acc.append(&mut gen_metadata_entity_type_impl(entity, &schema.complex_types));

            acc
        },
    );

    // Add usage declaration(s) for all subtypes across all the SAPAnnotationsProperty instances
    for subtype in dedup_vec_of_u8_array(used_subtypes) {
        out_buffer.append(&mut gen_use_path(subtype));
    }

    out_buffer
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
/// EDM EntityType -> Rust metadata instance
fn gen_metadata_entity_type(entity: &EntityType, skipped_cts: &Vec<String>) -> Vec<u8> {
    let struct_name = format!("{}{}", to_upper_camel_case(&entity.name), METADATA);
    let key_type = gen_vector_of_type(PROPERTYREF);

    let mut out_buffer: Vec<u8> = [RUSTC_ALLOW_DEAD_CODE, &*gen_start_struct(&struct_name)].concat();
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
                    let metadata_type_name =
                        [to_upper_camel_case(&cmplx_type).as_bytes(), METADATA.as_bytes()].concat();

                    gen_struct_field_into(&mut out_buffer, &prop_name, &metadata_type_name);
                }
            },

            // We should never end up here if the metadata XML is correct...
            PropertyType::Unqualified => {},
        }
    }

    out_buffer.extend_from_slice(END_BLOCK);
    out_buffer
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
/// Generates the metadata getter functions for each property in the impl of an EntityType
fn gen_metadata_entity_type_impl(entity: &EntityType, opt_cts: &Option<Vec<ComplexType>>) -> Vec<u8> {
    let struct_name = format!("{}{METADATA}", to_upper_camel_case(&entity.name));
    let mut out_buffer: Vec<u8> = gen_impl_start_for(&struct_name);
    let keys = &entity.key.property_refs;

    // Add a get_key function
    out_buffer.extend_from_slice(&gen_fn_signature(KEY, true, false, None, Some(&gen_vector_of_type(PROPERTYREF))));
    out_buffer.extend_from_slice(OPEN_CURLY);
    out_buffer.extend_from_slice(LINE_FEED);
    out_buffer.extend_from_slice(VEC_BANG);
    out_buffer.extend_from_slice(
        keys.into_iter()
            .map(|pr| format!("{pr}"))
            .collect::<Vec<_>>()
            .join(",")
            .as_bytes(),
    );
    out_buffer.extend_from_slice(CLOSE_SQR);
    out_buffer.extend_from_slice(CLOSE_CURLY);
    out_buffer.extend_from_slice(LINE_FEED);

    let mut props: Vec<_> = entity.properties.iter().collect();
    props.sort();

    // One getter function per property
    for prop in props {
        let safe_name = odata_name_to_rust_safe_name(&prop.odata_name);
        let fn_name = [PREFIX_SNAKE_GET.as_bytes(), safe_name.as_bytes()].concat();

        let mut prop = prop.clone();
        prop.deserializer_fn = gen_custom_deserializer_info(&prop);

        match prop.get_property_type() {
            PropertyType::Edm(_, _) => gen_pub_getter_fn_of_type_into(&mut out_buffer, &fn_name, PROPERTY, &prop),

            PropertyType::Complex(cmplx_type) => {
                let err_msg = format!(
                    "Error: ComplexType property {cmplx_type} found for which there is no corresponding type declaration"
                );

                if let Some(cts) = opt_cts {
                    if let Some(ct) = cts.iter().find(|ct| ct.name.eq(&cmplx_type)) {
                        gen_pub_getter_fn_of_type_into(&mut out_buffer, &fn_name, COMPLEX_TYPE, ct);
                    } else {
                        println!("{err_msg}");
                        println!("Found complex types {}", cts.iter().map(|ct| ct.name.as_str()).collect::<Vec<_>>().join(","));
                    }
                } else {
                    println!("{err_msg}");
                }
            },

            PropertyType::Unqualified => {},
        }
    }

    out_buffer.extend_from_slice(END_BLOCK);
    out_buffer
}
