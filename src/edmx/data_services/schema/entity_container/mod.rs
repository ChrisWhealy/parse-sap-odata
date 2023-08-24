pub mod association_set;
pub mod entity_set;
pub mod function_import;

use crate::utils::{de_str_to_bool, de_str_to_list, default_false, default_list, default_true};

use association_set::AssociationSet;
use serde::{Deserialize, Serialize};

use entity_set::EntitySet;
use function_import::FunctionImport;

static LINE_FEED: &[u8] = &[0x0A];
static SPACE: &[u8] = &[0x20];
static DOUBLE_QUOTE: &[u8] = &[0x22];
static COMMA: &[u8] = &[0x2C];
static COLON2: &[u8] = &[0x3A, 0x3A];
static FAT_ARROW: &[u8] = &[0x3D, 0x3E];
static OPEN_SQR: &[u8] = &[0x5B];
static CLOSE_SQR: &[u8] = &[0x5D];
static OPEN_CURLY: &[u8] = &[0x7B];
static CLOSE_CURLY: &[u8] = &[0x7D];

static COPY_CLONE_DEBUG: &[u8] = "#[derive(Copy, Clone, Debug)]".as_bytes();
static START_ENUM: &[u8] = "pub enum ".as_bytes();
static START_IMPL: &[u8] = "impl ".as_bytes();
static FN_VALUE_DECL: &[u8] = "pub const fn value(&self) -> &'static str {".as_bytes();
static FN_ITERATOR_DECL_START: &[u8] = "pub fn iterator() -> impl Iterator<Item = ".as_bytes();
static FN_ITERATOR_DECL_END: &[u8] = "> {".as_bytes();
static MATCH_SELF: &[u8] = "match *self {".as_bytes();
static CALL_ITER: &[u8] = ".iter()".as_bytes();
static CALL_COPIED: &[u8] = ".copied()".as_bytes();
static AS_OPT_LIST_START: &[u8] = "
pub fn as_list() -> Vec<&'static str> {
  let mut list = "
    .as_bytes();
static AS_OPT_LIST_END: &[u8] = "::iterator().fold(Vec::new(), |mut acc: Vec<&'static str>, es| {
      acc.insert(0, &mut es.value());
      acc
  });
  list.reverse();
  list
}
"
.as_bytes();

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
// EntityContainer
//
// Child Nodes:
//   1:n EntitySet
//   1:n AssociationSet
//   0:n FunctionImport
// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
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

    #[serde(
        rename = "sap:message-scope-supported",
        deserialize_with = "de_str_to_bool",
        default = "default_true"
    )]
    pub sap_message_scope_supported: bool,

    #[serde(
        rename = "sap:supported-formats",
        deserialize_with = "de_str_to_list",
        default = "default_list"
    )]
    pub sap_supported_formats: Vec<String>,

    #[serde(rename = "EntitySet", default)]
    pub entity_sets: Vec<EntitySet>,

    #[serde(rename = "AssociationSet", default)]
    pub association_sets: Vec<AssociationSet>,

    #[serde(rename = "FunctionImport", default)]
    pub function_imports: Option<Vec<FunctionImport>>,
}

impl EntityContainer {
    pub fn to_enum_with_impl(&self) -> Vec<u8> {
        let cont_name_camel = convert_case::Casing::to_case(&self.name, convert_case::Case::UpperCamel);

        // Output the start of an enum for this entity container
        // #[derive(Copy, Clone, Debug)]↩︎
        // pub enum <entity_container_name> {↩︎
        let mut output_enum = [
            COPY_CLONE_DEBUG,
            LINE_FEED,
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
        fn_value.append(&mut [LINE_FEED, CLOSE_CURLY, LINE_FEED, CLOSE_CURLY].concat());
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
                AS_OPT_LIST_START,
                cont_name_camel.as_bytes(),
                AS_OPT_LIST_END,
                LINE_FEED,
                CLOSE_CURLY, // Extra close curly needed to close the impl block
            ]
            .concat(),
        );

        return [output_enum, output_impl, fn_value, fn_iterator].concat();
    }
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
#[cfg(test)]
pub mod unit_tests;
