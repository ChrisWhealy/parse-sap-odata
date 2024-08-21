use crate::{
    edmx::data_services::schema::{association::metadata::normalise_assoc_name, Schema},
    parser::generate::{
        syntax_fragments::derive_traits::{derive_str, DeriveTraits},
        *,
    },
    utils::{to_snake_case, to_upper_camel_case},
};

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
/// Generate association structs
pub fn gen_metadata_associations(odata_srv_name: &str, schema: &Schema) -> Vec<u8> {
    // In a very small number of cases, it is possible for an OData service to contain zero associations
    // E.G. If the service contains only one entity set
    if schema.associations.is_empty() {
        return Vec::new();
    }

    let enum_name = &*format!("{}{ASSOCIATIONS}", to_upper_camel_case(odata_srv_name));

    // Start Association enum block
    let mut association_enum: Vec<u8> = [
        LINE_FEED,
        &*gen_comment_separator_for(ASSOCIATIONS),
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
        let enum_variant_name = to_upper_camel_case(&stripped_name);

        association_enum.append(&mut gen_enum_variant(&enum_variant_name));
        association_impl_iter_fn.append(&mut gen_fq_enum_variant(enum_name, &enum_variant_name));
        association_impl_variant_name_fn.append(&mut gen_enum_match_arm(&enum_name, &enum_variant_name, &assoc.name));

        if idx > 0 {
            association_impl_getter_fns.append(&mut SEPARATOR.to_vec());
        }

        let fn_name = [PREFIX_SNAKE_GET.as_bytes(), to_snake_case(&enum_variant_name).as_bytes()].concat();
        association_impl_getter_fns.append(&mut gen_pub_getter_fn_of_type(fn_name, ASSOCIATION, assoc));
    }

    // End Association enum block and function blocks
    association_enum.append(&mut END_BLOCK.to_vec());
    association_impl_iter_fn.append(&mut gen_end_iter_fn());
    association_impl_variant_name_fn.append(&mut [CLOSE_CURLY, END_BLOCK].concat());

    [
        &*association_enum,
        // Output the start of an enum implementation
        // impl <schema_name>Associations {↩︎
        &*gen_impl_start_for(enum_name),
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
/// Generate association structs
pub fn gen_metadata_association_sets(odata_srv_name: &str, schema: &Schema) -> Vec<u8> {
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

    let enum_name = &*format!("{}{ASSOCIATION_SETS}", to_upper_camel_case(odata_srv_name));

    // Start Association enum block
    let mut association_set_enum: Vec<u8> = [
        LINE_FEED,
        &*gen_comment_separator_for(ASSOCIATION_SETS),
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
        let enum_variant = to_upper_camel_case(&stripped_name);

        association_set_enum.append(&mut gen_enum_variant(&enum_variant));
        association_sets_impl_iter_fn.append(&mut gen_fq_enum_variant(enum_name, &enum_variant));
        association_sets_impl_variant_name_fn
            .append(&mut gen_enum_match_arm(&enum_name, &enum_variant, &assoc_set.name));

        if idx > 0 {
            association_sets_impl_getter_fns.append(&mut SEPARATOR.to_vec());
        }

        association_sets_impl_getter_fns.append(&mut gen_pub_getter_fn_of_type(
            to_snake_case(&enum_variant).into_bytes(),
            ASSOCIATION_SET,
            assoc_set,
        ));
    }

    // End AssociationSet enum block and function blocks
    association_set_enum.append(&mut END_BLOCK.to_vec());
    association_sets_impl_iter_fn.append(&mut gen_end_iter_fn());
    association_sets_impl_variant_name_fn.append(&mut [CLOSE_CURLY, END_BLOCK].concat());

    [
        &*association_set_enum,
        // Output the start of an enum implementation
        // impl <schema_name>AssociationSets {↩︎
        &*gen_impl_start_for(enum_name),
        &*association_sets_impl_iter_fn,
        &*association_sets_impl_variant_name_fn,
        &*gen_enum_fn_variant_names(&enum_name),
        LINE_FEED,
        &*association_sets_impl_getter_fns,
        END_BLOCK,
    ]
    .concat()
}
