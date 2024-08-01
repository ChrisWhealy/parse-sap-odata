use serde::{Deserialize, Serialize};

use association_set::AssociationSet;
use entity_set::EntitySet;
use function_import::FunctionImport;

use crate::{
    sap_annotations::entity_container::SAPAnnotationsEntityContainer,
    utils::{de_str_to_bool, default_false},
};

#[cfg(feature = "parser")]
use crate::parser::syntax_fragments::{
    *,
    derive_traits::*,
    fragment_generators::{
        end_iter_fn, gen_enum_iter_fn_start, gen_enum_match_arm, gen_enum_start, gen_enum_variant,
        gen_enum_variant_name_fn_start, gen_enum_variant_names_fn, gen_fq_enum_variant, gen_impl_start,
        gen_type_name,
    },
};

pub mod association_set;
pub mod entity_set;
pub mod function_import;

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
/// Represents an `<EntityContainer>` tag
///
/// # Child Nodes
/// `1:n EntitySet`<br>
/// `1:n AssociationSet`<br>
/// `0:n FunctionImport`
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct EntityContainer {
    #[serde(rename = "@Name")]
    pub name: String,

    #[serde(
        rename = "@IsDefaultEntityContainer",
        deserialize_with = "de_str_to_bool",
        default = "default_false"
    )]
    pub is_default_entity_container: bool,

    #[serde(flatten)]
    pub sap_annotations: SAPAnnotationsEntityContainer,

    #[serde(rename = "EntitySet", default)]
    pub entity_sets: Vec<EntitySet>,

    #[serde(rename = "AssociationSet", default)]
    pub association_sets: Vec<AssociationSet>,

    #[serde(rename = "FunctionImport", default)]
    pub function_imports: Option<Vec<FunctionImport>>,
}

#[cfg(feature = "parser")]
impl EntityContainer {
    /// Transforms an `EntityContainer` into an enumeration of entity set names.
    /// Additionally, this enumeration is given three helper functions:
    /// * `pub fn iterator() -> impl Iterator<Item = GwsampleBasicEntities> { /* SNIP */ }`
    /// * `pub const fn variant_name(&self) -> &'static str { /* SNIP */ }`
    /// * `pub fn variant_names() -> Vec<&'static str> { /* SNIP */ }`
    pub fn to_enum_with_impl(&self) -> Vec<u8> {
        let cont_name_camel = gen_type_name(&self.name);

        // Output the start of an enum for this entity container
        // #[derive(Copy, Clone, Debug)]↩︎
        // pub enum <entity_container_name> {↩︎
        let mut output_enum = [
            derive_str(vec![DeriveTraits::COPY, DeriveTraits::CLONE, DeriveTraits::DEBUG]).as_slice(),
            RUSTC_ALLOW_DEAD_CODE,
        ]
            .concat();

        output_enum.append(&mut gen_enum_start(&cont_name_camel));

        // Output the start of an enum implementation in which dead code is permitted
        // #[allow(dead_code)]
        // impl <entity_container_name> {↩︎
        let mut output_impl = RUSTC_ALLOW_DEAD_CODE.to_vec();

        output_impl.append(&mut gen_impl_start(&cont_name_camel));

        // Output the start of the "iterator" function within the enum implementation
        //   pub fn iterator() -> impl Iterator<Item = GwsampleBasicEntities> {↩︎
        //       match *self {↩︎
        let mut fn_iterator = gen_enum_iter_fn_start(&cont_name_camel);

        // Output the start of the "variant_name" function within the enum implementation
        //   pub const fn variant_name(&self) -> &'static str {↩︎
        //       match *self {↩︎
        let mut fn_variant_name = gen_enum_variant_name_fn_start();

        // Create entity set enum
        for ent_set in self.entity_sets.iter() {
            let ent_set_name_camel = gen_type_name(&ent_set.name);

            // Add variant to enum, iterator, and variant_name functions
            output_enum.append(&mut gen_enum_variant(&ent_set_name_camel));
            fn_variant_name.append(&mut gen_enum_match_arm(&cont_name_camel, &ent_set_name_camel, &ent_set.name));
            fn_iterator.append(&mut gen_fq_enum_variant(&cont_name_camel, &ent_set_name_camel));
        }

        output_enum.append(&mut END_BLOCK.to_vec());
        fn_variant_name.append(&mut END_BLOCK.to_vec());
        fn_variant_name.append(&mut END_BLOCK.to_vec());
        fn_iterator.append(&mut end_iter_fn());

        return [
            output_enum,
            output_impl,
            fn_iterator,
            fn_variant_name,
            gen_enum_variant_names_fn(&cont_name_camel),
            END_BLOCK.to_vec(),
        ]
            .concat();
    }
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
#[cfg(test)]
pub mod unit_tests;
