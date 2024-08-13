use std::collections::HashSet;

use crate::{
    edmx::data_services::schema::{complex_type::ComplexType, entity_type::EntityType, Schema},
    parser::syntax_fragments::{
        fragment_generators::{
            comment_for, gen_fn_sig, gen_impl_start, gen_start_struct, gen_struct_field, gen_vector_of_type,
        },
        gen_use_path, CLOSE_CURLY, CLOSE_SQR, COLON, COMMA, COMPLEX_TYPE, END_BLOCK, ENTITY_TYPES, FIELD_NAME_KEY, KEY,
        LINE_FEED, METADATA, OPEN_CURLY, PROPERTY, PROPERTYREF, PUBLIC, RUSTC_ALLOW_DEAD_CODE, SEPARATOR, VEC_BANG,
        PREFIX_SNAKE_GET
    },
    property::metadata::PropertyType,
    utils::odata_name_to_rust_safe_name,
};
use crate::utils::to_upper_camel_case;

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
/// Generate metadata entity type structs
pub fn gen_metadata_entity_types(schema: &Schema, skipped_cts: Vec<String>) -> Vec<u8> {
    let mut used_subtypes: Vec<&[u8]> = vec![];
    let mut out_buffer: Vec<u8> = comment_for(ENTITY_TYPES);
    let ets: &Vec<EntityType> = &schema.entity_types;

    for (idx, entity) in ets.into_iter().enumerate() {
        if idx > 0 {
            out_buffer.append(&mut SEPARATOR.to_vec());
        }

        // Accumulate a list of subtypes used within the SAP Annotations field of each property
        for prop in &entity.properties {
            used_subtypes.append(&mut prop.sap_annotations.used_subtypes());
        }

        out_buffer.append(&mut gen_metadata_entity_type(entity, &skipped_cts));
        out_buffer.append(&mut gen_metadata_entity_type_impl(entity, &schema.complex_types));
    }

    // De-dup the list of used subtypes
    let unique_subtypes = used_subtypes
        .clone()
        .into_iter()
        .collect::<HashSet<&[u8]>>()
        .into_iter()
        .collect::<Vec<&[u8]>>();

    // Add usage declaration(s) for all subtypes across all the SAPAnnotationsProperty instances
    for subtype in unique_subtypes {
        out_buffer.append(&mut gen_use_path(subtype));
    }

    out_buffer
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
/// EDM EntityType -> Rust metadata instance
fn gen_metadata_entity_type(entity: &EntityType, skipped_cts: &Vec<String>) -> Vec<u8> {
    let struct_name = format!("{}{}", to_upper_camel_case(&entity.name), METADATA);
    let mut out_buffer: Vec<u8> = [
        RUSTC_ALLOW_DEAD_CODE,
        LINE_FEED,
        &*gen_start_struct(&struct_name),
        &*gen_struct_field(FIELD_NAME_KEY, &gen_vector_of_type(PROPERTYREF).to_vec()),
    ]
    .concat();

    let mut props = entity.properties.clone();
    props.sort();

    // Metadata fields are either of type Property or of some complex type
    for prop in props {
        let prop_name = odata_name_to_rust_safe_name(&prop.odata_name);

        match prop.get_property_type() {
            PropertyType::Edm(_) => {
                out_buffer.append(&mut [PUBLIC, prop_name.as_bytes(), COLON, PROPERTY, COMMA, LINE_FEED].concat())
            },

            PropertyType::Complex(cmplx_type) => {
                // Is the current property type really complex or just a simple Rust type?
                if skipped_cts.contains(&cmplx_type) {
                    // Its a simple type pretending to be complex, so its metadata is a Property instance
                    out_buffer.append(&mut [PUBLIC, prop_name.as_bytes(), COLON, PROPERTY, COMMA, LINE_FEED].concat());
                } else {
                    // This really is a complex type
                    let metadata_type_name =
                        [to_upper_camel_case(&cmplx_type).as_bytes(), METADATA.as_bytes()].concat();

                    out_buffer.append(
                        &mut [PUBLIC, prop_name.as_bytes(), COLON, &*metadata_type_name, COMMA, LINE_FEED].concat(),
                    );
                }
            },

            // We should never end up here if the metadata XML is correct...
            PropertyType::Unqualified => {},
        }
    }

    out_buffer.append(&mut END_BLOCK.to_vec());
    out_buffer
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
/// Generates the metadata getter functions for each property in the impl of an EntityType
fn gen_metadata_entity_type_impl(entity: &EntityType, opt_cts: &Option<Vec<ComplexType>>) -> Vec<u8> {
    let struct_name = format!("{}{METADATA}", to_upper_camel_case(&entity.name));
    let mut out_buffer: Vec<u8> = gen_impl_start(&struct_name);
    let keys = &entity.key.property_refs;

    // Add a get_key function
    out_buffer.append(
        &mut [
            &gen_fn_sig(&KEY.to_vec(), true, false, None, Some(&gen_vector_of_type(PROPERTYREF))),
            OPEN_CURLY,
            LINE_FEED,
            VEC_BANG,
            keys.into_iter()
                .map(|pr| format!("{pr}"))
                .collect::<Vec<String>>()
                .join(",")
                .as_bytes(),
            CLOSE_SQR,
            CLOSE_CURLY,
            LINE_FEED,
        ]
        .concat(),
    );

    let mut props = entity.properties.clone();
    props.sort();

    // One getter function per property
    for prop in props {
        let fn_name = format!("{PREFIX_SNAKE_GET}{}", odata_name_to_rust_safe_name(&prop.odata_name)).into_bytes();

        match prop.get_property_type() {
            PropertyType::Edm(_) => {
                out_buffer.append(
                    &mut [
                        &*gen_fn_sig(&fn_name, true, false, None, Some(PROPERTY)),
                        OPEN_CURLY,
                        LINE_FEED,
                        &*format!("{prop}").into_bytes(),
                        END_BLOCK,
                    ]
                    .concat(),
                );
            },

            PropertyType::Complex(cmplx_type) => {
                let err_msg = format!("Error: ComplexType property {cmplx_type} found for which there is no corresponding type declaration");

                if let Some(cts) = opt_cts {
                    if let Some(ct) = cts.iter().find(|ct| ct.name.eq(&cmplx_type)) {
                        out_buffer.append(
                            &mut [
                                &*gen_fn_sig(&fn_name, true, false, None, Some(COMPLEX_TYPE)),
                                OPEN_CURLY,
                                LINE_FEED,
                                &*format!("{ct}").into_bytes(),
                                END_BLOCK,
                            ]
                            .concat(),
                        );
                    } else {
                        let ct_names = cts.iter().fold(vec![], |mut acc, ct| {
                            acc.push(ct.name.clone());
                            acc
                        });
                        println!("{err_msg}");
                        println!("Found complex types {}", ct_names.join(","));
                    }
                } else {
                    println!("{err_msg}");
                }
            },

            PropertyType::Unqualified => {},
        }
    }

    out_buffer.append(&mut END_BLOCK.to_vec());
    out_buffer
}
