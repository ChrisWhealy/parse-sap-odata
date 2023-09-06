pub mod association_set;
pub mod entity_set;
pub mod function_import;

use crate::{
    parser::syntax_fragments::*,
    sap_annotations::entity_container_sap_annotations::EntityContainerSAPAnnotations,
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
    pub name: String,

    #[serde(
        rename = "m:IsDefaultEntityContainer",
        deserialize_with = "de_str_to_bool",
        default = "default_false"
    )]
    pub is_default_entity_container: bool,

    #[serde(flatten)]
    pub sap_annotations: EntityContainerSAPAnnotations,

    #[serde(rename = "EntitySet", default)]
    pub entity_sets: Vec<EntitySet>,

    #[serde(rename = "AssociationSet", default)]
    pub association_sets: Vec<AssociationSet>,

    #[serde(rename = "FunctionImport", default)]
    pub function_imports: Option<Vec<FunctionImport>>,
}

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
            START_ENUM,
            cont_name_camel.as_bytes(),
            SPACE,
            OPEN_CURLY,
            LINE_FEED,
        ]
        .concat();

        // Output the start of an enum implementation
        // impl <entity_container_name> {↩︎
        let output_impl = [START_IMPL, cont_name_camel.as_bytes(), SPACE, OPEN_CURLY, LINE_FEED].concat();

        // Output the start of a "value" function within the enum implementation
        //   pub const fn value(&self) -> &'static str {↩︎
        //       match *self {↩︎
        let mut fn_value = [FN_VALUE_DECL, LINE_FEED, MATCH_SELF, LINE_FEED].concat();

        // Output the start of an "iterator" function within the enum implementation
        //   pub fn iterator() -> impl Iterator<Item = GwsampleBasicEntities> {↩︎
        //       match *self {↩︎
        let mut fn_iterator = [
            FN_ITERATOR_DECL_START,
            cont_name_camel.as_bytes(),
            FN_ITERATOR_DECL_END,
            LINE_FEED,
            OPEN_SQR,
            LINE_FEED,
        ]
        .concat();

        // Output the "as_list" function within the enum implementation
        let fn_as_list = [AS_OPT_LIST_START, cont_name_camel.as_bytes(), AS_OPT_LIST_END].concat();

        // Create entity set enum
        for ent_set in self.entity_sets.iter() {
            let ent_set_name_camel = convert_case::Casing::to_case(&ent_set.name, convert_case::Case::UpperCamel);

            // Add variant to enum
            output_enum.append(&mut [ent_set_name_camel.as_bytes(), COMMA, LINE_FEED].concat());

            // Add variant to value function
            fn_value.append(
                &mut [
                    cont_name_camel.as_bytes(),
                    COLON2,
                    ent_set_name_camel.as_bytes(),
                    SPACE,
                    FAT_ARROW,
                    SPACE,
                    DOUBLE_QUOTE,
                    &ent_set.name.as_bytes(),
                    DOUBLE_QUOTE,
                    COMMA,
                    LINE_FEED,
                ]
                .concat(),
            );

            // Add variant to iterator function
            fn_iterator.append(
                &mut [
                    cont_name_camel.as_bytes(),
                    COLON2,
                    ent_set_name_camel.as_bytes(),
                    COMMA,
                    LINE_FEED,
                ]
                .concat(),
            );
        }

        output_enum.append(&mut [LINE_FEED, CLOSE_CURLY, LINE_FEED, LINE_FEED].concat());
        fn_value.append(&mut [LINE_FEED, CLOSE_CURLY, LINE_FEED, CLOSE_CURLY, LINE_FEED, LINE_FEED].concat());
        fn_iterator.append(
            &mut [
                LINE_FEED,
                CLOSE_SQR,
                CALL_ITER,
                LINE_FEED,
                CALL_COPIED,
                LINE_FEED,
                CLOSE_CURLY,
                LINE_FEED,
            ]
            .concat(),
        );

        return [
            output_enum,
            output_impl,
            fn_value,
            fn_iterator,
            fn_as_list,
            CLOSE_CURLY.to_vec(),
        ]
        .concat();
    }
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
#[cfg(test)]
pub mod unit_tests;
