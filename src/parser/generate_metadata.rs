use std::collections::HashSet;

use crate::{
    edmx::data_services::schema::{
        association::{metadata::normalise_assoc_name, Association},
        complex_type::ComplexType,
        entity_container::association_set::AssociationSet,
        entity_type::EntityType,
        Schema,
    },
    parser::{
        generate_metadata_complex_types::*,
        syntax_fragments::{
            derive_traits::{derive_str, DeriveTraits},
            fragment_generators::*,
            *,
        },
    },
    property::metadata::PropertyType,
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
    let struct_name = format!("{}{}", gen_type_name_upper_camel(&entity.name), METADATA);
    let mut out_buffer: Vec<u8> = [
        RUSTC_ALLOW_DEAD_CODE,
        &*gen_start_struct(&struct_name),
        &*gen_struct_field("key", &gen_vector_of_type(PROPERTYREF).to_vec()),
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
                        [gen_type_name_upper_camel(&cmplx_type).as_bytes(), METADATA.as_bytes()].concat();

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
    let struct_name = format!("{}{}", gen_type_name_upper_camel(&entity.name), METADATA);
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
        let rust_name = odata_name_to_rust_safe_name(&prop.odata_name);
        let fn_name = format!("get_{}", rust_name).into_bytes();

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

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
/// Generate association structs
fn gen_metadata_associations(odata_srv_name: &str, schema: &Schema) -> Vec<u8> {
    // In a very small number of cases, it is possible for an OData service to contain zero associations
    // E.G. If the service contains only one entity set
    if schema.associations.is_empty() {
        return Vec::new();
    }

    let enum_name = &*format!("{}Associations", gen_type_name_upper_camel(odata_srv_name));

    // Start Association enum block
    let mut association_enum: Vec<u8> = [
        LINE_FEED,
        &*comment_for("ASSOCIATIONS"),
        &*gen_use_path(PATH_TO_EDMX_SCHEMA_ASSOCIATION_TYPES),
        LINE_FEED,
        &*derive_str(vec![DeriveTraits::COPY, DeriveTraits::CLONE, DeriveTraits::DEBUG]),
        &*gen_enum_start(enum_name),
    ]
    .concat();

    // Start block containing Association impl functions related to enum iterator
    let mut association_impl_iter_fn = gen_enum_fn_iter_start(&enum_name);

    // Output the start of the "variant_name" function within the enum implementation
    let mut association_impl_variant_name_fn = gen_enum_impl_fn_variant_name();

    // Start block containing Association impl getter functions
    let mut association_impl_getter_fns: Vec<u8> = Vec::new();

    let mut assocs = schema.associations.clone();
    assocs.sort();

    for (idx, assoc) in assocs.into_iter().enumerate() {
        let stripped_name = normalise_assoc_name(&assoc.name);
        let enum_variant = gen_type_name_upper_camel(&stripped_name);

        association_enum.append(&mut gen_enum_variant(&enum_variant));
        association_impl_iter_fn.append(&mut gen_fq_enum_variant(enum_name, &enum_variant));
        association_impl_variant_name_fn.append(&mut gen_enum_match_arm(&enum_name, &enum_variant, &assoc.name));

        if idx > 0 {
            association_impl_getter_fns.append(&mut SEPARATOR.to_vec());
        }

        association_impl_getter_fns.append(&mut gen_metadata_association_getter_fn(&enum_variant, &assoc));
    }

    // End Association enum block and function blocks
    association_enum.append(&mut END_BLOCK.to_vec());
    association_impl_iter_fn.append(&mut end_iter_fn());
    association_impl_variant_name_fn.append(&mut [CLOSE_CURLY, END_BLOCK].concat());

    [
        &*association_enum,
        // Output the start of an enum implementation
        // impl <schema_name>Associations {↩︎
        &*gen_impl_start(enum_name),
        &*association_impl_iter_fn,
        &*association_impl_variant_name_fn,
        &*gen_enum_fn_variant_names(&enum_name),
        LINE_FEED,
        &*association_impl_getter_fns,
        END_BLOCK,
    ]
    .concat()
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
/// EDM EntityType Instance -> Rust declaration
fn gen_metadata_association_getter_fn(enum_variant: &str, assoc: &Association) -> Vec<u8> {
    let fn_name = ["get_".as_bytes(), gen_type_name_snake(&enum_variant).as_bytes()].concat();

    [
        &*gen_fn_sig(&fn_name, true, false, None, Some("Association".as_bytes())),
        OPEN_CURLY,
        LINE_FEED,
        format!("{}", assoc).as_bytes(),
        END_BLOCK,
        LINE_FEED,
    ]
    .concat()
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
/// EDM EntityType Instance -> Rust declaration
fn gen_metadata_association_set_getter_fn(enum_variant: &str, assoc_set: &AssociationSet) -> Vec<u8> {
    let fn_name = gen_type_name_snake(&enum_variant);

    [
        &*gen_fn_sig(&fn_name.into_bytes(), true, false, None, Some("AssociationSet".as_bytes())),
        OPEN_CURLY,
        LINE_FEED,
        format!("{}", assoc_set).as_bytes(),
        END_BLOCK,
        LINE_FEED,
    ]
    .concat()
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
/// Generate association structs
fn gen_metadata_association_sets(odata_srv_name: &str, schema: &Schema) -> Vec<u8> {
    // In a very small number of cases, it is possible for an OData service to contain zero association sets
    // E.G. If the service contains only one entity set
    let mut assoc_sets = if let Some(ent_cont) = &schema.entity_container {
        if ent_cont.association_sets.len() == 0 {
            return Vec::new();
        }

        ent_cont.association_sets.clone()
    } else {
        return Vec::new();
    };
    assoc_sets.sort();

    let enum_name = &*format!("{}AssociationSets", gen_type_name_upper_camel(odata_srv_name));

    // Start Association enum block
    let mut association_set_enum: Vec<u8> = [
        LINE_FEED,
        &*comment_for("ASSOCIATION SETS"),
        &*gen_use_path(PATH_TO_EDMX_SCHEMA_ASSOCIATION_SETS),
        &*gen_use_path(PATH_TO_SAP_ANNOTATIONS_ASSOCIATION_SET),
        LINE_FEED,
        &*derive_str(vec![DeriveTraits::COPY, DeriveTraits::CLONE, DeriveTraits::DEBUG]),
        &*gen_enum_start(enum_name),
    ]
    .concat();

    // Start block containing AssociationSets impl functions related to enum iterator
    let mut association_sets_impl_iter_fn = gen_enum_fn_iter_start(&enum_name);

    // Output the start of the "variant_name" function within the enum implementation
    let mut association_sets_impl_variant_name_fn = gen_enum_impl_fn_variant_name();

    // Start block containing AssociationSets impl getter functions
    let mut association_sets_impl_getter_fns: Vec<u8> = Vec::new();

    for (idx, assoc_set) in assoc_sets.into_iter().enumerate() {
        let stripped_name = normalise_assoc_name(&assoc_set.name);
        let enum_variant = gen_type_name_upper_camel(&stripped_name);

        association_set_enum.append(&mut gen_enum_variant(&enum_variant));
        association_sets_impl_iter_fn.append(&mut gen_fq_enum_variant(enum_name, &enum_variant));
        association_sets_impl_variant_name_fn
            .append(&mut gen_enum_match_arm(&enum_name, &enum_variant, &assoc_set.name));

        if idx > 0 {
            association_sets_impl_getter_fns.append(&mut SEPARATOR.to_vec());
        }

        association_sets_impl_getter_fns.append(&mut gen_metadata_association_set_getter_fn(&enum_variant, &assoc_set));
    }

    // End AssociationSet enum block and function blocks
    association_set_enum.append(&mut END_BLOCK.to_vec());
    association_sets_impl_iter_fn.append(&mut end_iter_fn());
    association_sets_impl_variant_name_fn.append(&mut [CLOSE_CURLY, END_BLOCK].concat());

    [
        &*association_set_enum,
        // Output the start of an enum implementation
        // impl <schema_name>AssociationSets {↩︎
        &*gen_impl_start(enum_name),
        &*association_sets_impl_iter_fn,
        &*association_sets_impl_variant_name_fn,
        &*gen_enum_fn_variant_names(&enum_name),
        LINE_FEED,
        &*association_sets_impl_getter_fns,
        END_BLOCK,
    ]
    .concat()
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
/// Generate a module containing the metadata structs and their respective instances
pub fn gen_metadata_module(odata_srv_name: &str, schema: &Schema) -> Vec<u8> {
    let mut out_buffer: Vec<u8> = Vec::new();

    // Start module definition
    out_buffer.append(
        &mut [
            &*gen_mod_start(&format!("{odata_srv_name}_metadata")),
            &*gen_use_path(PATH_TO_SAP_ODATA_PROPERTIES),
            &*gen_use_path(PATH_TO_SAP_ANNOTATIONS_PROPERTY),
        ]
        .concat(),
    );

    // Do we need to generate any complex types?
    let skipped_cts = if let Some(cts) = &schema.complex_types {
        out_buffer.append(&mut gen_use_path(PATH_TO_EDMX_COMPLEX_TYPE));
        let (mut cmplx_type_metadata, skipped_cts) = gen_metadata_complex_types(cts);
        out_buffer.append(&mut cmplx_type_metadata);
        skipped_cts
    } else {
        Vec::new()
    };

    out_buffer.append(
        &mut [
            &*gen_metadata_entity_types(&schema, skipped_cts),
            &*gen_metadata_associations(odata_srv_name, &schema),
            &*gen_metadata_association_sets(odata_srv_name, &schema),
            // Close module definition
            END_BLOCK,
        ]
        .concat(),
    );

    out_buffer
}
