pub mod association_set;
pub mod entity_set;
pub mod function_import;

#[cfg(feature = "parser")]
use crate::parser::syntax_fragments::*;

use crate::{
    sap_annotations::SAPAnnotationsEntityContainer,
    utils::{de_str_to_bool, default_false},
};

use association_set::AssociationSet;
use serde::{Deserialize, Serialize};

use entity_set::EntitySet;
use function_import::FunctionImport;

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
    /// * `pub const fn value(&self) -> &'static str { /* SNIP */ }`
    /// * `pub fn iterator() -> impl Iterator<Item = GwsampleBasicEntities> { /* SNIP */ }`
    /// * `pub fn as_list() -> Vec<&'static str> { /* SNIP */ }`
    pub fn to_enum_with_impl(&self) -> Vec<u8> {
        let cont_name_camel = convert_case::Casing::to_case(&self.name, convert_case::Case::UpperCamel);

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

        // Output the start of the "value" function within the enum implementation
        //   pub const fn value(&self) -> &'static str {↩︎
        //       match *self {↩︎
        let mut fn_value = gen_enum_value_fn_start();

        // Output the start of the "iterator" function within the enum implementation
        //   pub fn iterator() -> impl Iterator<Item = GwsampleBasicEntities> {↩︎
        //       match *self {↩︎
        let mut fn_iterator = gen_enum_iter_fn_start(&cont_name_camel);

        // Create entity set enum
        for ent_set in self.entity_sets.iter() {
            let ent_set_name_camel = convert_case::Casing::to_case(&ent_set.name, convert_case::Case::UpperCamel);

            // Add variant to enum, value and iterator functions
            output_enum.append(&mut gen_enum_variant(&ent_set_name_camel));
            fn_value.append(&mut gen_enum_match_arm(&cont_name_camel, &ent_set_name_camel, &ent_set.name));
            fn_iterator.append(&mut gen_fq_enum_variant(&cont_name_camel, &ent_set_name_camel));
        }

        output_enum.append(&mut end_block());
        fn_value.append(&mut end_block());
        fn_value.append(&mut end_block());
        fn_iterator.append(&mut end_iter_fn());

        return [
            output_enum,
            output_impl,
            fn_value,
            fn_iterator,
            gen_enum_as_list_fn(&cont_name_camel),
            end_block(),
        ]
        .concat();
    }
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
#[cfg(test)]
pub mod unit_tests;
