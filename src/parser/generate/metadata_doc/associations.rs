use crate::{
    edmx::data_services::schema::{association::metadata::normalise_assoc_name, Schema},
    parser::generate::{
        syntax_fragments::derive_traits::{gen_derive_str, DeriveTraits},
        *,
    },
    utils::{to_snake_case, to_upper_camel_case},
};

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
pub fn gen_metadata_associations_into(out: &mut String, odata_srv_name: &str, schema: &Schema) {
    out.push_str(&gen_metadata_associations(odata_srv_name, schema));
}

/// Generate association structs
pub fn gen_metadata_associations(odata_srv_name: &str, schema: &Schema) -> String {
    let mut out = String::new();

    // In a very small number of cases, it is possible for an OData service to contain zero associations
    // E.G. If the service contains only one entity set
    if schema.associations.is_empty() {
        return out;
    }

    let enum_name = &*format!("{}{ASSOCIATIONS}", to_upper_camel_case(odata_srv_name));

    // Start Association enum block
    let mut association_enum = String::new();

    association_enum.push_str(LINE_FEED);
    gen_comment_separator_for(&mut association_enum, ASSOCIATIONS);
    gen_use_path(&mut association_enum, PATH_TO_EDMX_SCHEMA_ASSOCIATION_TYPES);
    association_enum.push_str(LINE_FEED);
    association_enum.push_str(&gen_derive_str(&[
        DeriveTraits::COPY,
        DeriveTraits::CLONE,
        DeriveTraits::DEBUG
    ]));
    gen_enum_start(&mut association_enum, enum_name);

    // Start block containing Association impl functions related to enum iterator
    let mut association_impl_iter_fn = String::new();
    gen_enum_fn_iter_start(&mut association_impl_iter_fn, enum_name);

    // Output the start of the "variant_name" function within the enum implementation
    let mut association_impl_variant_name_fn = String::new();
    gen_enum_impl_fn_variant_name(&mut association_impl_variant_name_fn);

    // Start block containing Association impl getter functions
    let mut association_impl_getter_fns = String::new();

    // Sort references, not owned Association values
    let mut assocs: Vec<_> = schema.associations.iter().collect();
    assocs.sort();

    for (idx, assoc) in assocs.into_iter().enumerate() {
        let stripped_name = normalise_assoc_name(&assoc.name);
        let enum_variant_name = to_upper_camel_case(&stripped_name);

        gen_enum_variant_into(&mut association_enum, &enum_variant_name);
        gen_fq_enum_variant_into(&mut association_impl_iter_fn, enum_name, &enum_variant_name);
        gen_enum_match_arm_into(
            &mut association_impl_variant_name_fn,
            enum_name,
            &enum_variant_name,
            &assoc.name,
        );

        if idx > 0 {
            association_impl_getter_fns.push_str(SEPARATOR);
        }

        let fn_name = format!("{PREFIX_SNAKE_GET}{}", to_snake_case(&enum_variant_name));

        gen_pub_getter_fn_of_type_into(&mut association_impl_getter_fns, &fn_name, ASSOCIATION, assoc);
    }

    // End Association enum block and function blocks
    association_enum.push_str(END_BLOCK);
    gen_end_iter_fn(&mut association_impl_iter_fn);
    association_impl_variant_name_fn.push_str(CLOSE_CURLY);
    association_impl_variant_name_fn.push_str(END_BLOCK);

    out.push_str(&association_enum);

    // Output the start of an enum implementation
    // impl Associations {
    gen_impl_start_for(&mut out, enum_name);
    out.push_str(&association_impl_iter_fn);
    out.push_str(&association_impl_variant_name_fn);
    gen_enum_fn_variant_names(&mut out, &enum_name);
    out.push_str(LINE_FEED);
    out.push_str(&association_impl_getter_fns);
    out.push_str(END_BLOCK);
    out
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
/// Generate association set structs
pub fn gen_metadata_association_sets_into(out: &mut String, odata_srv_name: &str, schema: &Schema) {
    // In a very small number of cases, it is possible for an OData service to contain zero association sets
    // E.G. If the service contains only one entity set
    let mut assoc_sets: Vec<_> = if let Some(ent_cont) = &schema.entity_container {
        if ent_cont.association_sets.is_empty() {
            return
        }

        ent_cont.association_sets.iter().collect()
    } else {
        return
    };
    assoc_sets.sort();

    let enum_name = &*format!("{}{ASSOCIATION_SETS}", to_upper_camel_case(odata_srv_name));

    // Start Association enum block
    let mut association_set_enum = String::new();
    association_set_enum.push_str(LINE_FEED);
    gen_comment_separator_for(&mut association_set_enum, ASSOCIATION_SETS);
    gen_use_path(&mut association_set_enum, PATH_TO_EDMX_SCHEMA_ASSOCIATION_SETS);
    gen_use_path(&mut association_set_enum, PATH_TO_SAP_ANNOTATIONS_ASSOCIATION_SET);
    association_set_enum.push_str(LINE_FEED);
    association_set_enum.push_str(&gen_derive_str(&[DeriveTraits::COPY, DeriveTraits::CLONE, DeriveTraits::DEBUG]));
    gen_enum_start(&mut association_set_enum, enum_name);

    // Start block containing AssociationSets impl functions related to enum iterator
    let mut association_sets_impl_iter_fn = String::new();
    gen_enum_fn_iter_start(&mut association_sets_impl_iter_fn, &enum_name);

    // Output the start of the "variant_name" function within the enum implementation
    let mut association_sets_impl_variant_name_fn = String::new();
    gen_enum_impl_fn_variant_name(&mut association_sets_impl_variant_name_fn);

    // Start block containing AssociationSets impl getter functions
    let mut association_sets_impl_getter_fns = String::new();

    for (idx, assoc_set) in assoc_sets.into_iter().enumerate() {
        let stripped_name = normalise_assoc_name(&assoc_set.name);
        let enum_variant = to_upper_camel_case(&stripped_name);

        gen_enum_variant_into(&mut association_set_enum, &enum_variant);
        gen_fq_enum_variant_into(&mut association_sets_impl_iter_fn, enum_name, &enum_variant);
        gen_enum_match_arm_into(
            &mut association_sets_impl_variant_name_fn,
            &enum_name,
            &enum_variant,
            &assoc_set.name,
        );

        if idx > 0 {
            association_sets_impl_getter_fns.push_str(SEPARATOR);
        }

        gen_pub_getter_fn_of_type_into(
            &mut association_sets_impl_getter_fns,
            &to_snake_case(&enum_variant),
            ASSOCIATION_SET,
            assoc_set,
        );
    }

    // End AssociationSet enum block and function blocks
    association_set_enum.push_str(END_BLOCK);
    gen_end_iter_fn(&mut association_sets_impl_iter_fn);
    association_sets_impl_variant_name_fn.push_str(CLOSE_CURLY);
    association_sets_impl_variant_name_fn.push_str(END_BLOCK);

    // Output the start of an enum implementation
    // impl <schema_name>AssociationSets {↩︎
    out.push_str(&association_set_enum);
    gen_impl_start_for(out, enum_name);
    out.push_str(&association_sets_impl_iter_fn);
    out.push_str(&association_sets_impl_variant_name_fn);
    gen_enum_fn_variant_names(out, &enum_name);
    out.push_str(LINE_FEED);
    out.push_str(&association_sets_impl_getter_fns);
    out.push_str(END_BLOCK);
}
