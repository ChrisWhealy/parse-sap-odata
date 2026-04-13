use std::collections::BTreeSet;

use crate::{
    edmx::data_services::schema::{complex_type::ComplexType, entity_type::EntityType, Schema},
    parser::generate::{syntax_fragments::*, *},
    property::edm_type::EdmType,
    utils::{odata_name_to_rust_safe_name, to_upper_camel_case},
};

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
/// Generate metadata entity type structs, writing output into `out`
pub fn gen_metadata_entity_types_into(out: &mut String, schema: &Schema, skipped_cts: &BTreeSet<String>) {
    let mut used_subtypes: BTreeSet<&str> = BTreeSet::new();
    let ets: &Vec<EntityType> = &schema.entity_types;

    gen_comment_separator_for_into(out, ENTITY_TYPES);

    for (idx, entity) in ets.iter().enumerate() {
        if idx > 0 {
            out.push_str(SEPARATOR);
        }

        // Accumulate a set of subtypes used within the SAP Annotations field of each property
        for prop in &entity.properties {
            used_subtypes.extend(prop.sap_annotations.used_subtypes());
        }

        gen_metadata_entity_type(out, entity, skipped_cts);
        gen_metadata_entity_type_impl(out, entity, &schema.complex_types);
    }

    // Add usage declaration(s) for all subtypes across all the SAPAnnotationsProperty instances
    for subtype in used_subtypes {
        gen_use_path_into(out, subtype);
    }
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
/// EDM EntityType -> Rust metadata instance
fn gen_metadata_entity_type(out: &mut String, entity: &EntityType, skipped_cts: &BTreeSet<String>) {
    let struct_name = format!("{}{}", to_upper_camel_case(&entity.name), METADATA);

    out.push_str(RUSTC_ALLOW_DEAD_CODE);
    gen_start_struct_into(out, &struct_name);
    out.push_str(PUBLIC);
    out.push_str(FIELD_NAME_KEY);
    out.push_str(COLON);
    gen_vector_of_type_into(out, PROPERTYREF);
    out.push_str(COMMA);
    out.push_str(LINE_FEED);

    let mut props: Vec<_> = entity.properties.iter().collect();
    props.sort();

    // Metadata fields are either of type Property or of some complex type
    for prop in props {
        let prop_name = odata_name_to_rust_safe_name(&prop.odata_name);

        match &prop.edm_type {
            EdmType::Primitive(_) => {
                gen_struct_field_into(out, &prop_name, PROPERTY);
            },

            EdmType::Complex(cmplx_type) => {
                // cmplx_type is already UpperCamelCase with no namespace prefix (e.g. "CtAddress")
                if skipped_cts.contains(cmplx_type.as_str()) {
                    // A basic Rust type's metadata is a Property instance
                    gen_struct_field_into(out, &prop_name, PROPERTY);
                } else {
                    // This really is a complex type
                    gen_struct_field_into(out, &prop_name, &format!("{cmplx_type}{METADATA}"));
                }
            },

            // Truly unrecognised type — no metadata struct generated
            EdmType::Unknown(_) => {},
        }
    }

    out.push_str(END_BLOCK);
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
/// Generates the metadata getter functions for each property in the impl of an EntityType
fn gen_metadata_entity_type_impl(out: &mut String, entity: &EntityType, opt_cts: &Option<Vec<ComplexType>>) {
    let struct_name = format!("{}{METADATA}", to_upper_camel_case(&entity.name));
    gen_impl_start_for_into(out, &struct_name);
    let keys = &entity.key.property_refs;

    // Add the key function
    gen_fn_signature_into(out, KEY, true, false, None, Some("Vec<PropertyRef>"));
    out.push_str(OPEN_CURLY);
    out.push_str(LINE_FEED);
    out.push_str(VEC_BANG);
    out.push_str(&keys.iter().map(|pr| format!("{pr}")).collect::<Vec<_>>().join(","));
    out.push_str(CLOSE_SQR);
    out.push_str(CLOSE_CURLY);
    out.push_str(LINE_FEED);

    let mut props: Vec<_> = entity.properties.iter().collect();
    props.sort();

    // One getter function per property
    for prop in props {
        let safe_name = odata_name_to_rust_safe_name(&prop.odata_name);
        let fn_name = format!("{PREFIX_SNAKE_GET}{}", safe_name.strip_prefix("r#").unwrap_or(&safe_name));

        match &prop.edm_type {
            EdmType::Primitive(_) => {
                let mut prop = prop.clone();
                prop.deserializer_fn = gen_custom_deserializer_info(&prop);
                gen_pub_getter_fn_of_type_into(out, &fn_name, PROPERTY, &prop)
            },

            EdmType::Complex(cmplx_type) => {
                // cmplx_type is already UpperCamelCase; match against ct.name via to_upper_camel_case
                let err_msg = format!(
                    "Error: ComplexType property {cmplx_type} found for which there is no corresponding type declaration"
                );

                if let Some(cts) = opt_cts {
                    if let Some(ct) = cts.iter().find(|ct| to_upper_camel_case(&ct.name).eq(cmplx_type)) {
                        gen_pub_getter_fn_of_type_into(out, &fn_name, COMPLEX_TYPE, ct);
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

            // Truly unrecognised type — no getter generated
            EdmType::Unknown(_) => {},
        }
    }

    out.push_str(END_BLOCK);
}
