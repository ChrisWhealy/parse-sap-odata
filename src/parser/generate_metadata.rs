use std::collections::HashSet;

use crate::{
    edmx::data_services::schema::{complex_type::ComplexType, entity_type::EntityType, Schema},
    parser::{
        generate_metadata_complex_types::*,
        syntax_fragments::{
            fragment_generators::{
                comment_for, gen_getter_fn_for_property_of_type, gen_impl_start, gen_mod_start, gen_struct_field,
                gen_type_name, gen_vector_of_type, start_struct,
            },
            *,
        },
    },
    property::PropertyType,
    utils::odata_name_to_rust_safe_name,
};

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
/// Generate metadata entity type structs
fn gen_metadata_entity_types(schema: &Schema, skipped_cts: Vec<String>) -> Vec<u8> {
    let mut out_buffer: Vec<u8> = Vec::new();
    let mut used_subtypes: Vec<&[u8]> = vec![];
    let ets: &Vec<EntityType> = &schema.entity_types;

    out_buffer.append(&mut comment_for("ENTITY TYPES"));

    for (idx, entity) in ets.into_iter().enumerate() {
        if idx > 0 {
            out_buffer.append(&mut SEPARATOR.to_vec());
        }

        // Accumulate the subtypes used within the SAP Annotations field of each property
        for prop in &entity.properties {
            used_subtypes.append(&mut prop.sap_annotations.used_subtypes());
        }

        out_buffer.append(&mut gen_metadata_entity_type(entity, &skipped_cts));
        out_buffer.append(&mut gen_metadata_entity_type_impl(entity, &schema.complex_types));
    }

    // De-dup the used subtypes
    let unique_subtypes = used_subtypes
        .clone()
        .into_iter()
        .collect::<HashSet<&[u8]>>()
        .into_iter()
        .collect::<Vec<&[u8]>>();

    // Declare usage of all subtypes across all the SAPAnnotationsProperty instances
    for subtype in unique_subtypes {
        out_buffer.append(&mut gen_use_path(subtype));
    }

    out_buffer
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
/// EDM EntityType -> Rust metadata instance
fn gen_metadata_entity_type(entity: &EntityType, skipped_cts: &Vec<String>) -> Vec<u8> {
    let mut out_buffer: Vec<u8> = Vec::new();
    let struct_name = format!("{}{}", gen_type_name(&entity.name), METADATA);

    out_buffer.append(&mut RUSTC_ALLOW_DEAD_CODE.to_vec());
    out_buffer.append(&mut start_struct(&struct_name));
    out_buffer.append(&mut gen_struct_field("key", &gen_vector_of_type(PROPERTYREF).to_vec()));

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
                    let metadata_type_name = [gen_type_name(&cmplx_type).as_bytes(), METADATA.as_bytes()].concat();

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
/// Generates the metadata getter functions for each property in an Entity Type
fn gen_metadata_entity_type_impl(entity: &EntityType, opt_cts: &Option<Vec<ComplexType>>) -> Vec<u8> {
    let mut out_buffer: Vec<u8> = Vec::new();
    let struct_name = format!("{}{}", gen_type_name(&entity.name), METADATA);

    out_buffer.append(&mut gen_impl_start(&struct_name));
    let keys = &entity.key.property_refs;

    // Add a get_key function
    out_buffer.append(
        &mut [
            &gen_getter_fn_for_property_of_type(KEY, &*gen_vector_of_type(PROPERTYREF)),
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
        let rust_name = odata_name_to_rust_safe_name(&prop.odata_name);

        match prop.get_property_type() {
            PropertyType::Edm(_) => {
                out_buffer.append(&mut gen_getter_fn_for_property_of_type(rust_name.as_bytes(), PROPERTY));
                out_buffer.append(&mut format!("{}", prop).into_bytes());
                out_buffer.append(&mut END_BLOCK.to_vec());
            },

            PropertyType::Complex(cmplx_type) => {
                let err_msg = format!("Error: ComplexType property {cmplx_type} found for which there is no corresponding type declaration");

                if let Some(cts) = opt_cts {
                    if let Some(ct) = cts.iter().find(|ct| ct.name.eq(&cmplx_type)) {
                        out_buffer.append(&mut gen_getter_fn_for_property_of_type(rust_name.as_bytes(), COMPLEX_TYPE));
                        out_buffer.append(&mut format!("{ct}").into_bytes());
                        out_buffer.append(&mut END_BLOCK.to_vec());
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

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
/// Generate a module containing the metadata structs and their respective instances
pub fn gen_metadata_module(odata_srv_name: &str, schema: &Schema) -> Vec<u8> {
    let mut out_buffer: Vec<u8> = Vec::new();

    // Start module definition
    out_buffer.append(&mut gen_mod_start(&format!("{odata_srv_name}_metadata")));
    out_buffer.append(&mut gen_use_path(PATH_TO_SAP_ODATA_PROPERTIES));
    out_buffer.append(&mut gen_use_path(PATH_TO_SAP_ANNOTATIONS_PROPERTY));

    // Do we need to generate any complex types?
    let skipped_cts = if let Some(cts) = &schema.complex_types {
        out_buffer.append(&mut gen_use_path(PATH_TO_EDMX_COMPLEX_TYPE));
        let (mut cmplx_type_metadata, skipped_cts) = gen_metadata_complex_types(cts);
        out_buffer.append(&mut cmplx_type_metadata);
        skipped_cts
    } else {
        Vec::new()
    };

    out_buffer.append(&mut gen_metadata_entity_types(&schema, skipped_cts));

    // Close module definition
    out_buffer.append(&mut END_BLOCK.to_vec());
    out_buffer
}
